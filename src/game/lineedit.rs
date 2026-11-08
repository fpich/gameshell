//! Éditeur de ligne façon terminal réel.
//! RÈGLE A11: en mode tty (raw mode), les flèches gauche/droite déplacent le curseur,
//! haut/bas naviguent dans l'historique, les autres séquences ANSI sont ignorées.
//! En mode non-tty (pipe, tests), toute séquence d'échappement est nettoyée.

use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

/// Lit une ligne éditable. Retourne None sur EOF (Ctrl+D sur ligne vide).
pub fn read_line(prompt: &str, history: &mut Vec<String>) -> Option<String> {
    if is_tty() {
        read_line_raw(prompt, history)
    } else {
        print!("{}", prompt);
        io::stdout().flush().ok();
        read_line_plain()
    }
}

fn is_tty() -> bool {
    Command::new("stty")
        .arg("-g")
        .stdin(Stdio::inherit())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Mode non-tty (pipe/test): ligne simple + nettoyage ANSI + troncature (règle A13)
fn read_line_plain() -> Option<String> {
    let mut line = String::new();
    let n = io::stdin().read_line(&mut line).ok()?;
    if n == 0 { return None; }
    if line.len() > 4096 {
        line.truncate(4096);
        eprintln!("(input tronqué à 4096 caractères)");
    }
    Some(strip_ansi(&line))
}

/// Nettoie les séquences d'échappement ANSI (ESC [ ... final, et ESC + 1 char)
pub fn strip_ansi(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    // saute jusqu'à la lettre finale de la séquence CSI (0x40..=0x7E)
                    for c2 in chars.by_ref() {
                        if ('\x40'..='\x7e').contains(&c2) { break; }
                    }
                }
                Some(_) => { chars.next(); } // ESC + 1 caractère: ignoré
                None => {}
            }
            let _ = (bytes, i);
            i += 1;
        } else {
            out.push(c);
        }
    }
    out
}

/// Restaure le terminal même en cas de sortie anticipée
struct TtyGuard(String);
impl Drop for TtyGuard {
    fn drop(&mut self) {
        let _ = Command::new("stty")
            .arg(&self.0)
            .stdin(Stdio::inherit())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

fn stty(args: &str) -> bool {
    Command::new("stty")
        .args(args.split_whitespace())
        .stdin(Stdio::inherit())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Lit une byte (None = EOF)
fn read_byte(lock: &mut impl Read) -> Option<u8> {
    let mut b = [0u8; 1];
    match lock.read(&mut b) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(b[0]),
    }
}

/// Début du caractère précédent (UTF-8)
fn char_left(buf: &[u8], cur: usize) -> usize {
    if cur == 0 { return 0; }
    let mut c = cur - 1;
    while c > 0 && (buf[c] & 0xC0) == 0x80 { c -= 1; }
    c
}

/// Début du caractère suivant (UTF-8)
fn char_right(buf: &[u8], cur: usize) -> usize {
    if cur >= buf.len() { return buf.len(); }
    let mut c = cur + 1;
    while c < buf.len() && (buf[c] & 0xC0) == 0x80 { c += 1; }
    c
}

fn redraw(prompt: &str, buf: &[u8], cur: usize) {
    let mut out = io::stdout();
    out.write_all(b"\r\x1b[2K").ok();
    out.write_all(prompt.as_bytes()).ok();
    out.write_all(buf).ok();
    let tail = String::from_utf8_lossy(&buf[cur.min(buf.len())..]);
    let n = tail.chars().count();
    if n > 0 {
        let _ = write!(out, "\x1b[{}D", n);
    }
    out.flush().ok();
}

/// Mode tty: édition de ligne en raw mode (flèches, historique, backspace...)
fn read_line_raw(prompt: &str, history: &mut Vec<String>) -> Option<String> {
    let out = Command::new("stty").arg("-g").stdin(Stdio::inherit()).output();
    let saved = match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => return read_line_plain(),
    };
    if saved.is_empty() { return read_line_plain(); }
    let _guard = TtyGuard(saved.clone());
    if !stty("raw -echo") { return read_line_plain(); }

    let stdin = io::stdin();
    let mut lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let mut cur = 0usize;
    let mut hist_idx: Option<usize> = None;
    let mut stash = String::new();

    redraw(prompt, &buf, 0);

    loop {
        let c = match read_byte(&mut lock) {
            None => { print!("\n"); io::stdout().flush().ok(); return None; } // EOF
            Some(c) => c,
        };
        match c {
            b'\r' | b'\n' => {
                print!("\n");
                io::stdout().flush().ok();
                let line = String::from_utf8_lossy(&buf).to_string();
                if line.len() > 4096 {
                    eprintln!("(input tronqué à 4096 caractères)");
                    let mut t = line; t.truncate(4096);
                    history.push(t.clone());
                    return Some(t);
                }
                let t = line.trim().to_string();
                if !t.is_empty() && history.last().map(|l| l.as_str()) != Some(t.as_str()) {
                    history.push(t);
                }
                return Some(line);
            }
            0x7f | 0x08 => {
                // Backspace: supprime le caractère avant le curseur
                if cur > 0 {
                    let nc = char_left(&buf, cur);
                    buf.drain(nc..cur);
                    cur = nc;
                    redraw(prompt, &buf, cur);
                }
            }
            0x03 => {
                // Ctrl+C: annule la ligne en cours
                print!("^C\n");
                io::stdout().flush().ok();
                return Some(String::new());
            }
            0x04 => {
                // Ctrl+D: EOF si ligne vide, sinon supprime le caractère sous le curseur
                if buf.is_empty() {
                    print!("\n");
                    io::stdout().flush().ok();
                    return None;
                } else if cur < buf.len() {
                    let nc = char_right(&buf, cur);
                    buf.drain(cur..nc);
                    redraw(prompt, &buf, cur);
                }
            }
            0x01 => { cur = 0; redraw(prompt, &buf, cur); } // Ctrl+A: début
            0x05 => { cur = buf.len(); redraw(prompt, &buf, cur); } // Ctrl+E: fin
            0x15 => { buf.clear(); cur = 0; hist_idx = None; redraw(prompt, &buf, cur); } // Ctrl+U
            0x17 => {
                // Ctrl+W: supprime le mot précédent
                while cur > 0 && buf[cur - 1] == b' ' { let nc = char_left(&buf, cur); buf.drain(nc..cur); cur = nc; }
                while cur > 0 && buf[cur - 1] != b' ' { let nc = char_left(&buf, cur); buf.drain(nc..cur); cur = nc; }
                redraw(prompt, &buf, cur);
            }
            0x1b => {
                // Séquence d'échappement: lit la suite
                match read_byte(&mut lock) {
                    None => {}
                    Some(b'[') | Some(b'O') => {
                        let mut params = String::new();
                        let final_byte = loop {
                            match read_byte(&mut lock) {
                                None => break None,
                                Some(f) if (0x40..=0x7e).contains(&f) => break Some(f),
                                Some(p) => params.push(p as char),
                            }
                        };
                        match final_byte {
                            Some(b'A') => {
                                // Haut: historique précédent
                                if !history.is_empty() {
                                    if hist_idx.is_none() {
                                        stash = String::from_utf8_lossy(&buf).to_string();
                                        hist_idx = Some(history.len() - 1);
                                    } else if let Some(i) = hist_idx {
                                        if i > 0 { hist_idx = Some(i - 1); }
                                    }
                                    if let Some(i) = hist_idx {
                                        buf = history[i].bytes().collect();
                                        cur = buf.len();
                                    }
                                    redraw(prompt, &buf, cur);
                                }
                            }
                            Some(b'B') => {
                                // Bas: historique suivant
                                if let Some(i) = hist_idx {
                                    if i + 1 >= history.len() {
                                        hist_idx = None;
                                        buf = stash.bytes().collect();
                                    } else {
                                        hist_idx = Some(i + 1);
                                        buf = history[i + 1].bytes().collect();
                                    }
                                    cur = buf.len();
                                    redraw(prompt, &buf, cur);
                                }
                            }
                            Some(b'C') => {
                                // Droite
                                if cur < buf.len() { cur = char_right(&buf, cur); redraw(prompt, &buf, cur); }
                            }
                            Some(b'D') => {
                                // Gauche
                                if cur > 0 { cur = char_left(&buf, cur); redraw(prompt, &buf, cur); }
                            }
                            Some(b'H') => { cur = 0; redraw(prompt, &buf, cur); } // Home
                            Some(b'F') => { cur = buf.len(); redraw(prompt, &buf, cur); } // End
                            Some(b'3') if params == "~" => {
                                // Suppr: supprime le caractère sous le curseur
                                if cur < buf.len() {
                                    let nc = char_right(&buf, cur);
                                    buf.drain(cur..nc);
                                    redraw(prompt, &buf, cur);
                                }
                            }
                            _ => {} // autres séquences ANSI ignorées (règle A11)
                        }
                    }
                    Some(_) => {} // ESC + autre chose: ignoré
                    None => {}
                }
            }
            _ if c >= 0x20 => {
                // Caractère imprimable: insère à la position du curseur
                buf.insert(cur, c);
                cur += 1;
                redraw(prompt, &buf, cur);
            }
            _ => {} // autres caractères de contrôle ignorés
        }
    }
}

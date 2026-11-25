//! Éditeur de ligne façon terminal réel (règle A11).
//! tty: mode « raw » via termios (FFI, plus de processus stty), curseur ← →, historique ↑ ↓,
//! Home/End, Ctrl+A/E/U/W/L, Suppr, complétion Tab (commandes + chemins du monde), lignes
//! plus longues que le terminal redessinées correctement, largeur des emoji prise en compte.
//! non-tty (pipe, tests): ligne simple, séquences ANSI nettoyées, troncature à 4096 (A13).

use crate::sys;
use std::io::{self, Read, Write};

/// Contexte de complétion: dossier courant et monde (chemins réels)
pub struct Completer {
    pub cwd: String,
    pub world: String,
    pub home: String,
}

/// Lit une ligne éditable. Retourne None sur EOF (Ctrl+D sur ligne vide).
pub fn read_line(
    prompt: &str,
    history: &mut Vec<String>,
    comp: Option<&Completer>,
) -> Option<String> {
    if sys::is_tty(0) && sys::is_tty(1) {
        if let Some(orig) = sys::get_termios(0) {
            return read_line_raw(prompt, history, comp, orig);
        }
    }
    print!("{}", prompt);
    io::stdout().flush().ok();
    read_line_plain()
}

/// Mode non-tty (pipe/test): ligne simple + nettoyage ANSI + troncature (règle A13)
fn read_line_plain() -> Option<String> {
    // lecture octet par octet SANS tampon: l'entrée restante reste disponible pour les
    // commandes lancées ensuite (ex: passwd qui lit le mot de passe sur l'entrée standard)
    let mut raw: Vec<u8> = Vec::new();
    let mut lock = RawStdin;
    loop {
        match read_byte(&mut lock) {
            None => {
                if raw.is_empty() {
                    return None;
                }
                break;
            }
            Some(b'\n') => break,
            Some(b) => {
                if raw.len() < 1_000_000 {
                    raw.push(b);
                }
            }
        }
    }
    let mut line = String::from_utf8_lossy(&raw).to_string(); // binaire/UTF-8 invalide: jamais de panic
    if line.len() > 4096 {
        let mut cut = 4096;
        while !line.is_char_boundary(cut) {
            cut -= 1;
        }
        line.truncate(cut);
        eprintln!("(input tronqué à 4096 caractères)");
    }
    Some(strip_ansi(&line))
}

/// Nettoie les séquences d'échappement ANSI (ESC [ ... final, et ESC + 1 char)
pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    for c2 in chars.by_ref() {
                        if ('\x40'..='\x7e').contains(&c2) {
                            break;
                        }
                    }
                }
                Some(_) => {
                    chars.next();
                }
                None => {}
            }
        } else if c != '\r' {
            out.push(c);
        }
    }
    out
}

/// Restaure le terminal même en cas de sortie anticipée
struct TtyGuard(sys::Termios);
impl Drop for TtyGuard {
    fn drop(&mut self) {
        sys::set_termios(0, &self.0);
    }
}

/// Entrée standard non tamponnée (descripteur 0)
struct RawStdin;
impl Read for RawStdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        use std::os::fd::FromRawFd;
        let mut f = std::mem::ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(0) });
        f.read(buf)
    }
}

fn read_byte(lock: &mut impl Read) -> Option<u8> {
    let mut b = [0u8; 1];
    loop {
        match lock.read(&mut b) {
            Ok(0) => return None,
            Ok(_) => return Some(b[0]),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => return None,
        }
    }
}

fn char_left(buf: &[u8], cur: usize) -> usize {
    if cur == 0 {
        return 0;
    }
    let mut c = cur - 1;
    while c > 0 && (buf[c] & 0xC0) == 0x80 {
        c -= 1;
    }
    c
}

fn char_right(buf: &[u8], cur: usize) -> usize {
    if cur >= buf.len() {
        return buf.len();
    }
    let mut c = cur + 1;
    while c < buf.len() && (buf[c] & 0xC0) == 0x80 {
        c += 1;
    }
    c
}

/// État d'affichage: nombre de lignes physiques et ligne du curseur lors du dernier rendu
struct Screen {
    cursor_row: usize,
}

fn redraw(scr: &mut Screen, prompt: &str, buf: &[u8], cur: usize) {
    let cols = sys::term_cols().max(10);
    let pw = sys::str_width(prompt);
    let text = String::from_utf8_lossy(buf);
    let total = pw + sys::str_width(&text);
    let cur_w = pw + sys::str_width(&String::from_utf8_lossy(&buf[..cur.min(buf.len())]));
    let mut out = String::new();
    // remonte au début du rendu précédent puis efface jusqu'à la fin de l'écran
    if scr.cursor_row > 0 {
        out.push_str(&format!("\x1b[{}A", scr.cursor_row));
    }
    out.push_str("\r\x1b[J");
    out.push_str(prompt);
    out.push_str(&text);
    // terminal en « retour à la ligne différé »: force le passage à la ligne suivante
    if total > 0 && total.is_multiple_of(cols) {
        out.push_str("\r\n");
    }
    let end_row = total / cols;
    let (crow, ccol) = (cur_w / cols, cur_w % cols);
    if end_row > crow {
        out.push_str(&format!("\x1b[{}A", end_row - crow));
    }
    out.push('\r');
    if ccol > 0 {
        out.push_str(&format!("\x1b[{}C", ccol));
    }
    scr.cursor_row = crow;
    let mut so = io::stdout();
    so.write_all(out.as_bytes()).ok();
    so.flush().ok();
}

/// Termine l'affichage: place le curseur après la fin de la ligne et passe à la ligne
fn finish(scr: &mut Screen, prompt: &str, buf: &[u8], cur: usize, suffix: &str) {
    let cols = sys::term_cols().max(10);
    let total = sys::str_width(prompt) + sys::str_width(&String::from_utf8_lossy(buf));
    let cur_w = sys::str_width(prompt)
        + sys::str_width(&String::from_utf8_lossy(&buf[..cur.min(buf.len())]));
    let down = (total / cols).saturating_sub(cur_w / cols);
    let mut s = String::new();
    if down > 0 {
        s.push_str(&format!("\x1b[{}B", down));
    }
    s.push_str(suffix);
    s.push_str("\r\n"); // P1.11: retour en colonne 0
    scr.cursor_row = 0;
    print!("{}", s);
    io::stdout().flush().ok();
}

const GSH_WORDS: &[&str] = &[
    "gsh help",
    "gsh goal",
    "gsh check",
    "gsh code",
    "gsh cancel",
    "gsh reset",
    "gsh skills",
    "gsh export",
    "gsh restart",
];

fn command_names() -> &'static Vec<String> {
    use std::sync::OnceLock;
    static COMMAND_NAMES: OnceLock<Vec<String>> = OnceLock::new();
    COMMAND_NAMES.get_or_init(|| {
        let mut v: Vec<String> = [
            "cd", "echo", "pwd", "exit", "clear", "gsh", "history", "type", "export", "alias",
            "printf",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        v.extend(crate::sim::NAMES.iter().map(|s| s.to_string()));
        for d in ["/usr/bin", "/usr/sbin"] {
            if let Ok(rd) = std::fs::read_dir(d) {
                for e in rd.flatten() {
                    v.push(e.file_name().to_string_lossy().to_string());
                }
            }
        }
        v.sort();
        v.dedup();
        v
    })
}

/// Candidats pour le mot `word` (en position de commande ou non)
fn candidates(c: &Completer, line_before: &str, word: &str) -> Vec<String> {
    let first =
        line_before.trim_start().is_empty() || line_before.trim_end().ends_with(['|', ';', '&']);
    if line_before.trim_start() == "gsh" || line_before.trim_start().starts_with("gsh ") {
        return GSH_WORDS
            .iter()
            .filter_map(|g| g.strip_prefix("gsh "))
            .filter(|g| g.starts_with(word))
            .map(|s| s.to_string())
            .collect();
    }
    if first && !word.contains('/') {
        return command_names()
            .iter()
            .filter(|n| n.starts_with(word))
            .cloned()
            .collect();
    }
    // chemin: dossier du mot, résolu dans le monde
    let (dir_part, prefix) = match word.rfind('/') {
        Some(i) => (&word[..=i], &word[i + 1..]),
        None => ("", word),
    };
    let real_dir = if let Some(r) = dir_part.strip_prefix("~/") {
        format!("{}/{}", c.home, r)
    } else if dir_part.starts_with('/') {
        format!("{}{}", c.world.trim_end_matches('/'), dir_part)
    } else {
        format!("{}/{}", c.cwd, dir_part)
    };
    let at_root = std::fs::canonicalize(&real_dir).ok() == std::fs::canonicalize(&c.world).ok();
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&real_dir) {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if !name.starts_with(prefix) {
                continue;
            }
            if name.starts_with('.') && !prefix.starts_with('.') {
                continue;
            }
            if at_root && crate::sim::files::SYSTEM_ENTRIES.contains(&name.as_str()) {
                continue;
            }
            let is_dir = e.path().is_dir();
            out.push(format!(
                "{}{}{}",
                dir_part,
                name,
                if is_dir { "/" } else { "" }
            ));
        }
    }
    out.sort();
    out
}

fn common_prefix(v: &[String]) -> String {
    let mut p = v.first().cloned().unwrap_or_default();
    for s in v.iter().skip(1) {
        while !s.starts_with(&p) {
            p.pop();
        }
    }
    p
}

/// Mode tty: édition de ligne en mode raw
fn read_line_raw(
    prompt: &str,
    history: &mut Vec<String>,
    comp: Option<&Completer>,
    orig: sys::Termios,
) -> Option<String> {
    let _guard = TtyGuard(orig);
    if !sys::set_termios(0, &sys::raw_from(&orig, 0)) {
        return read_line_plain();
    }
    let mut lock = RawStdin;
    let mut buf: Vec<u8> = Vec::new();
    let mut cur = 0usize;
    let mut hist_idx: Option<usize> = None;
    let mut stash = String::new();
    let mut scr = Screen { cursor_row: 0 };
    let mut last_tab = false;

    redraw(&mut scr, prompt, &buf, 0);
    loop {
        let c = match read_byte(&mut lock) {
            None => {
                finish(&mut scr, prompt, &buf, cur, "");
                return None;
            } // EOF
            Some(c) => c,
        };
        let was_tab = last_tab;
        last_tab = false;
        match c {
            b'\r' | b'\n' => {
                finish(&mut scr, prompt, &buf, cur, "");
                let mut line = String::from_utf8_lossy(&buf).to_string();
                if line.len() > 4096 {
                    let mut cut = 4096;
                    while !line.is_char_boundary(cut) {
                        cut -= 1;
                    }
                    line.truncate(cut);
                    eprintln!("(input tronqué à 4096 caractères)");
                }
                let t = line.trim().to_string();
                if !t.is_empty() && history.last().map(|l| l.as_str()) != Some(t.as_str()) {
                    history.push(t);
                }
                return Some(line);
            }
            0x7f | 0x08 => {
                if cur > 0 {
                    let nc = char_left(&buf, cur);
                    buf.drain(nc..cur);
                    cur = nc;
                    redraw(&mut scr, prompt, &buf, cur);
                }
            }
            0x03 => {
                finish(&mut scr, prompt, &buf, cur, "^C");
                return Some(String::new());
            } // Ctrl+C: annule la ligne
            0x04 => {
                if buf.is_empty() {
                    finish(&mut scr, prompt, &buf, cur, "");
                    return None;
                }
                if cur < buf.len() {
                    let nc = char_right(&buf, cur);
                    buf.drain(cur..nc);
                    redraw(&mut scr, prompt, &buf, cur);
                }
            }
            0x01 => {
                cur = 0;
                redraw(&mut scr, prompt, &buf, cur);
            }
            0x05 => {
                cur = buf.len();
                redraw(&mut scr, prompt, &buf, cur);
            }
            0x0b => {
                buf.truncate(cur);
                redraw(&mut scr, prompt, &buf, cur);
            } // Ctrl+K
            0x0c => {
                print!("\x1b[2J\x1b[H");
                scr.cursor_row = 0;
                redraw(&mut scr, prompt, &buf, cur);
            } // Ctrl+L
            0x15 => {
                buf.drain(..cur);
                cur = 0;
                hist_idx = None;
                redraw(&mut scr, prompt, &buf, cur);
            } // Ctrl+U
            0x17 => {
                while cur > 0 && buf[cur - 1] == b' ' {
                    let nc = char_left(&buf, cur);
                    buf.drain(nc..cur);
                    cur = nc;
                }
                while cur > 0 && buf[cur - 1] != b' ' {
                    let nc = char_left(&buf, cur);
                    buf.drain(nc..cur);
                    cur = nc;
                }
                redraw(&mut scr, prompt, &buf, cur);
            }
            b'\t' => {
                last_tab = true;
                let Some(cp) = comp else { continue };
                let before = String::from_utf8_lossy(&buf[..cur]).to_string();
                let start = before
                    .rfind([' ', '\t', '|', ';', '&', '<', '>'])
                    .map(|i| i + 1)
                    .unwrap_or(0);
                let word = &before[start..];
                let cands = candidates(cp, &before[..start], word);
                if cands.is_empty() {
                    print!("\x07");
                    io::stdout().flush().ok();
                    continue;
                }
                let completion = if cands.len() == 1 {
                    let c0 = &cands[0];
                    if c0.ends_with('/') {
                        c0.clone()
                    } else {
                        format!("{} ", c0)
                    }
                } else {
                    common_prefix(&cands)
                };
                if completion.len() > word.len() && completion.starts_with(word) {
                    let add = &completion.as_bytes()[word.len()..];
                    for (k, b) in add.iter().enumerate() {
                        buf.insert(cur + k, *b);
                    }
                    cur += add.len();
                    redraw(&mut scr, prompt, &buf, cur);
                } else if was_tab && cands.len() > 1 {
                    // double Tab: liste des possibilités
                    finish(&mut scr, prompt, &buf, cur, "");
                    let names: Vec<String> = cands
                        .iter()
                        .map(|c| {
                            c.trim_end_matches('/')
                                .rsplit('/')
                                .next()
                                .unwrap_or(c)
                                .to_string()
                                + if c.ends_with('/') { "/" } else { "" }
                        })
                        .collect();
                    let shown: Vec<&String> = names.iter().take(60).collect();
                    print!(
                        "{}",
                        shown
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join("  ")
                    );
                    if names.len() > 60 {
                        print!("  … ({} au total)", names.len());
                    }
                    print!("\r\n");
                    redraw(&mut scr, prompt, &buf, cur);
                    last_tab = false;
                } else {
                    print!("\x07");
                    io::stdout().flush().ok();
                }
            }
            0x1b => {
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
                                if !history.is_empty() {
                                    match hist_idx {
                                        None => {
                                            stash = String::from_utf8_lossy(&buf).to_string();
                                            hist_idx = Some(history.len() - 1);
                                        }
                                        Some(i) if i > 0 => hist_idx = Some(i - 1),
                                        _ => {}
                                    }
                                    if let Some(i) = hist_idx {
                                        buf = history[i].bytes().collect();
                                        cur = buf.len();
                                    }
                                    redraw(&mut scr, prompt, &buf, cur);
                                }
                            }
                            Some(b'B') => {
                                if let Some(i) = hist_idx {
                                    if i + 1 >= history.len() {
                                        hist_idx = None;
                                        buf = stash.bytes().collect();
                                    } else {
                                        hist_idx = Some(i + 1);
                                        buf = history[i + 1].bytes().collect();
                                    }
                                    cur = buf.len();
                                    redraw(&mut scr, prompt, &buf, cur);
                                }
                            }
                            Some(b'C') => {
                                if cur < buf.len() {
                                    cur = char_right(&buf, cur);
                                    redraw(&mut scr, prompt, &buf, cur);
                                }
                            }
                            Some(b'D') => {
                                if cur > 0 {
                                    cur = char_left(&buf, cur);
                                    redraw(&mut scr, prompt, &buf, cur);
                                }
                            }
                            Some(b'H') => {
                                cur = 0;
                                redraw(&mut scr, prompt, &buf, cur);
                            }
                            Some(b'F') => {
                                cur = buf.len();
                                redraw(&mut scr, prompt, &buf, cur);
                            }
                            Some(b'~') => match params.as_str() {
                                "3" => {
                                    if cur < buf.len() {
                                        let nc = char_right(&buf, cur);
                                        buf.drain(cur..nc);
                                        redraw(&mut scr, prompt, &buf, cur);
                                    }
                                } // Suppr
                                "1" | "7" => {
                                    cur = 0;
                                    redraw(&mut scr, prompt, &buf, cur);
                                } // Home
                                "4" | "8" => {
                                    cur = buf.len();
                                    redraw(&mut scr, prompt, &buf, cur);
                                } // End
                                _ => {}
                            },
                            _ => {} // autres séquences ANSI ignorées (règle A11)
                        }
                    }
                    Some(_) => {}
                }
            }
            _ if c >= 0x20 => {
                buf.insert(cur, c);
                cur += 1;
                // n'affiche qu'une fois le caractère UTF-8 complet
                if c < 0x80 || std::str::from_utf8(&buf).is_ok() {
                    redraw(&mut scr, prompt, &buf, cur);
                }
            }
            _ => {}
        }
    }
}

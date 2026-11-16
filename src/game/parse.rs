//! Analyse d'une ligne shell (sans l'exécuter) pour la validation des missions (P1.1).
//! - découpe sur | || && ; & (hors guillemets), repère les redirections
//! - « commande effective »: la commande derrière sudo / env / nice / nohup / su -c / bash -c...
//! - correspondance d'une cible de check (commande, sous-commande, option, opérateur)

#[derive(Debug, Default, Clone)]
pub struct Simple {
    pub words: Vec<String>,
    pub redirs: Vec<(String, String)>, // (opérateur, cible)
}

#[derive(Debug, Default, Clone)]
pub struct Parsed {
    pub cmds: Vec<Simple>,
    pub ops: Vec<String>, // opérateurs de liaison rencontrés: | |& || && ; &
}

#[derive(Debug, PartialEq)]
enum Tok {
    Word(String),
    Op(String),
}

fn tokenize(line: &str) -> Vec<Tok> {
    let chars: Vec<char> = line.chars().collect();
    let mut toks = Vec::new();
    let mut cur = String::new();
    let mut has_word = false; // mot en cours (même vide entre guillemets)
    let mut quoted = false; // le mot en cours contient des guillemets
    let mut i = 0;
    let flush = |toks: &mut Vec<Tok>, cur: &mut String, has_word: &mut bool, quoted: &mut bool| {
        if *has_word {
            toks.push(Tok::Word(std::mem::take(cur)));
        }
        *has_word = false;
        *quoted = false;
    };
    while i < chars.len() {
        let c = chars[i];
        match c {
            ' ' | '\t' | '\n' | '\r' => { flush(&mut toks, &mut cur, &mut has_word, &mut quoted); i += 1; }
            '#' if !has_word => break, // commentaire
            '\'' => {
                has_word = true; quoted = true; i += 1;
                while i < chars.len() && chars[i] != '\'' { cur.push(chars[i]); i += 1; }
                i += 1;
            }
            '"' => {
                has_word = true; quoted = true; i += 1;
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < chars.len() && matches!(chars[i + 1], '"' | '\\' | '$' | '`') {
                        cur.push(chars[i + 1]); i += 2;
                    } else { cur.push(chars[i]); i += 1; }
                }
                i += 1;
            }
            '\\' => {
                has_word = true;
                if i + 1 < chars.len() { cur.push(chars[i + 1]); }
                i += 2;
            }
            '$' if i + 1 < chars.len() && chars[i + 1] == '(' => {
                // $( ... ): conservé tel quel dans le mot
                has_word = true;
                let mut depth = 0;
                while i < chars.len() {
                    let ch = chars[i];
                    cur.push(ch);
                    if ch == '(' { depth += 1; }
                    if ch == ')' { depth -= 1; if depth == 0 { i += 1; break; } }
                    i += 1;
                }
            }
            '`' => {
                has_word = true;
                cur.push(c); i += 1;
                while i < chars.len() && chars[i] != '`' { cur.push(chars[i]); i += 1; }
                if i < chars.len() { cur.push('`'); i += 1; }
            }
            '|' | '&' | ';' | '<' | '>' => {
                // descripteur numérique collé: 2> 2>> 1>&2
                let mut op = String::new();
                if (c == '>' || c == '<') && has_word && !quoted && !cur.is_empty() && cur.chars().all(|d| d.is_ascii_digit()) {
                    op.push_str(&cur);
                    cur.clear();
                    has_word = false;
                } else {
                    flush(&mut toks, &mut cur, &mut has_word, &mut quoted);
                }
                let rest: String = chars[i..].iter().take(3).collect();
                let cand = ["<<<", "&>>", ">>", "<<", "||", "&&", "|&", ">&", "&>", ">|", ";;", "|", "&", ";", "<", ">"];
                let m = cand.iter().find(|p| rest.starts_with(*p)).copied().unwrap_or("");
                op.push_str(m);
                i += m.chars().count().max(1);
                toks.push(Tok::Op(op));
            }
            _ => { has_word = true; cur.push(c); i += 1; }
        }
    }
    flush(&mut toks, &mut cur, &mut has_word, &mut quoted);
    toks
}

fn is_redir(op: &str) -> bool {
    let o = op.trim_start_matches(|c: char| c.is_ascii_digit());
    matches!(o, ">" | ">>" | "<" | "<<" | "<<<" | ">&" | "&>" | "&>>" | ">|")
}

/// Analyse une ligne complète
pub fn parse(line: &str) -> Parsed {
    let mut p = Parsed::default();
    let mut cur = Simple::default();
    let mut pending_redir: Option<String> = None;
    for t in tokenize(line) {
        match t {
            Tok::Word(w) => {
                if let Some(op) = pending_redir.take() { cur.redirs.push((op, w)); } else { cur.words.push(w); }
            }
            Tok::Op(op) if is_redir(&op) => {
                if let Some(prev) = pending_redir.take() { cur.redirs.push((prev, String::new())); }
                pending_redir = Some(op);
            }
            Tok::Op(op) => {
                if let Some(prev) = pending_redir.take() { cur.redirs.push((prev, String::new())); }
                if !cur.words.is_empty() || !cur.redirs.is_empty() { p.cmds.push(std::mem::take(&mut cur)); }
                p.ops.push(op);
            }
        }
    }
    if let Some(prev) = pending_redir.take() { cur.redirs.push((prev, String::new())); }
    if !cur.words.is_empty() || !cur.redirs.is_empty() { p.cmds.push(cur); }
    p
}

impl Parsed {
    /// Une seule commande, sans opérateur ni redirection
    pub fn is_simple(&self) -> bool {
        self.cmds.len() == 1 && self.ops.is_empty() && self.cmds[0].redirs.is_empty()
            && !self.cmds[0].words.iter().any(|w| w.contains("$(") || w.contains('`'))
    }

    /// Opérateur présent hors guillemets (| > >> < && || ;)
    pub fn has_operator(&self, op: &str) -> bool {
        match op {
            "|" => self.ops.iter().any(|o| o == "|" || o == "|&"),
            ">" => self.cmds.iter().any(|c| c.redirs.iter().any(|(o, _)| o == ">" || o == "1>" || o == ">|")),
            ">>" => self.cmds.iter().any(|c| c.redirs.iter().any(|(o, _)| o == ">>" || o == "1>>")),
            "<" => self.cmds.iter().any(|c| c.redirs.iter().any(|(o, _)| o == "<")),
            "2>" => self.cmds.iter().any(|c| c.redirs.iter().any(|(o, _)| o == "2>" || o == "2>>")),
            o => self.ops.iter().any(|x| x == o),
        }
    }

    /// Toutes les commandes effectives (base, arguments) de la ligne
    pub fn effective(&self) -> Vec<(String, Vec<String>)> {
        let mut out = Vec::new();
        for c in &self.cmds { effective_of(&c.words, &mut out, 0); }
        out
    }
}

fn is_assignment(w: &str) -> bool {
    match w.split_once('=') {
        Some((name, _)) => !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') && !name.starts_with(|c: char| c.is_ascii_digit()),
        None => false,
    }
}

fn effective_of(words: &[String], out: &mut Vec<(String, Vec<String>)>, depth: usize) {
    if depth > 6 { return; }
    let mut i = 0;
    while i < words.len() && is_assignment(&words[i]) { i += 1; }
    if i >= words.len() { return; }
    let base = words[i].rsplit('/').next().unwrap_or(&words[i]).to_string();
    let args: Vec<String> = words[i + 1..].to_vec();
    out.push((base.clone(), args.clone()));
    let a = &args;
    let mut j = 0;
    let inner_from = |j: usize, out: &mut Vec<(String, Vec<String>)>| {
        if j < a.len() { effective_of(&a[j..], out, depth + 1); }
    };
    match base.as_str() {
        "sudo" => {
            while j < a.len() && a[j].starts_with('-') {
                if matches!(a[j].as_str(), "-u" | "-g" | "-C" | "-h" | "-p" | "-U" | "-r" | "-t" | "-D") { j += 2; } else { j += 1; }
            }
            inner_from(j, out);
        }
        "nice" | "ionice" | "watch" | "timeout" | "stdbuf" | "nohup" | "time" | "command" | "builtin" | "exec" | "xargs" => {
            while j < a.len() && a[j].starts_with('-') {
                let takes = match base.as_str() {
                    "nice" => a[j] == "-n",
                    "ionice" => matches!(a[j].as_str(), "-c" | "-n"),
                    "watch" => matches!(a[j].as_str(), "-n" | "-d"),
                    "xargs" => matches!(a[j].as_str(), "-n" | "-I" | "-d" | "-P" | "-L" | "-s" | "-E" | "-a"),
                    "timeout" => matches!(a[j].as_str(), "-s" | "-k"),
                    _ => false,
                };
                j += if takes { 2 } else { 1 };
            }
            if base == "timeout" { j += 1; }
            inner_from(j, out);
        }
        "env" => {
            while j < a.len() && (a[j].starts_with('-') || is_assignment(&a[j])) {
                j += if a[j] == "-u" { 2 } else { 1 };
            }
            inner_from(j, out);
        }
        "su" | "bash" | "sh" => {
            if let Some(k) = a.iter().position(|x| x == "-c") {
                if let Some(s) = a.get(k + 1) {
                    for c in parse(s).cmds { effective_of(&c.words, out, depth + 1); }
                }
            }
        }
        _ => {}
    }
}

/// Liste (lettre, valeur) d'une option courte groupée: "-bn1" → b, n=1 ; "-la" → l, a
fn short_flags(opt: &str) -> Vec<(char, Option<String>)> {
    let mut v: Vec<(char, Option<String>)> = Vec::new();
    for c in opt.trim_start_matches('-').chars() {
        if c.is_ascii_digit() {
            if let Some(last) = v.last_mut() {
                last.1.get_or_insert_with(String::new).push(c);
                continue;
            }
        }
        v.push((c, None));
    }
    v
}

fn flags_of_args(args: &[String]) -> Vec<(char, Option<String>)> {
    let mut v = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if a.starts_with('-') && !a.starts_with("--") && a.len() > 1 {
            if a[1..].chars().all(|c| c.is_ascii_digit()) {
                v.push(('#', Some(a[1..].to_string())));
            } else {
                let mut f = short_flags(a);
                if let Some(last) = f.last_mut() {
                    if last.1.is_none() {
                        if let Some(next) = args.get(i + 1) {
                            if !next.is_empty() && next.chars().all(|c| c.is_ascii_digit()) { last.1 = Some(next.clone()); }
                        }
                    }
                }
                v.extend(f);
            }
        }
        i += 1;
    }
    v
}

/// L'argument `t` (option, sous-commande, mode...) est-il présent pour la commande `base` ?
pub fn arg_present(base: &str, t: &str, args: &[String]) -> bool {
    if t.starts_with("--") {
        return args.iter().any(|a| a == t || a.starts_with(&format!("{}=", t)));
    }
    if base == "chmod" && t.starts_with('+') {
        let want: Vec<char> = t[1..].chars().collect();
        return args.iter().any(|a| {
            if let Some((_, perms)) = a.split_once('+') {
                want.iter().all(|w| perms.contains(*w))
            } else if t == "+x" && a.len() >= 3 && a.chars().all(|c| ('0'..='7').contains(&c)) {
                let owner = a.chars().nth(a.len() - 3).unwrap_or('0');
                matches!(owner, '1' | '3' | '5' | '7')
            } else { false }
        });
    }
    if t.starts_with('+') {
        return args.iter().any(|a| a == t);
    }
    if t.starts_with('-') && t.len() > 1 {
        if args.iter().any(|a| a == t) { return true; }
        if base == "kill" && t == "-9" {
            let sig9 = |s: &str| matches!(s, "9" | "KILL" | "SIGKILL");
            return args.iter().any(|a| a == "-KILL" || a == "-SIGKILL")
                || args.windows(2).any(|w| w[0] == "-s" && sig9(&w[1]));
        }
        let have = flags_of_args(args);
        if t[1..].chars().all(|c| c.is_ascii_digit()) {
            // head -5 ≡ head -n 5 ≡ head -n5
            let n = &t[1..];
            return have.iter().any(|(c, v)| (*c == '#' || *c == 'n') && v.as_deref() == Some(n))
                || args.iter().any(|a| a == &format!("--lines={}", n));
        }
        let want = short_flags(t);
        return want.iter().all(|(c, v)| have.iter().any(|(hc, hv)| hc == c && (v.is_none() || hv == v)));
    }
    // tar czf / ps aux: groupe de lettres sans tiret
    if (base == "tar" || base == "ps") && !t.is_empty() && t.chars().all(|c| c.is_ascii_alphabetic()) {
        let mut letters: Vec<char> = Vec::new();
        if let Some(first) = args.first() {
            if !first.starts_with('-') { letters.extend(first.chars()); }
        }
        for (c, _) in flags_of_args(args) { letters.push(c); }
        return t.chars().all(|c| letters.contains(&c));
    }
    if base == "ip" {
        // ip a / ip r / ip n: abréviations des objets
        if let Some(obj) = args.iter().find(|a| !a.starts_with('-')) {
            if !obj.is_empty() && t.starts_with(obj.as_str()) { return true; }
        }
    }
    args.iter().any(|a| a == t)
}

const OPERATORS: &[&str] = &["|", ">", ">>", "<", "2>", "&&", "||", ";"];

/// Check `command_executed`: la cible est-elle satisfaite par l'historique (lignes RÉUSSIES) ?
/// - opérateur (| > >>): présent hors guillemets dans une ligne
/// - "base sous-commande" (apt update, systemctl restart): base + mots non-options (options ignorées, A4)
/// - commande seule (pwd, useradd): commande effective (derrière sudo...) — jamais une sous-chaîne
/// - option / argument (-la, -c, +x, status, route): argument de LA commande qui le porte dans la solution
pub fn command_matches(target: &str, rcs: &[String], history: &[Parsed]) -> bool {
    let toks: Vec<&str> = target.split_whitespace().collect();
    if toks.is_empty() { return false; }
    if toks.len() == 1 && OPERATORS.contains(&toks[0]) {
        return history.iter().any(|p| p.has_operator(toks[0]));
    }
    let effs_hist: Vec<(String, Vec<String>)> = history.iter().flat_map(|p| p.effective()).collect();
    if toks.len() > 1 {
        let base = toks[0];
        let need: Vec<&str> = toks[1..].iter().copied().filter(|t| !t.starts_with('-')).collect();
        return effs_hist.iter().any(|(b, args)| b == base && need.iter().all(|n| arg_present(b, n, args)));
    }
    let t = toks[0];
    let rc_effs: Vec<Vec<(String, Vec<String>)>> = rcs.iter().map(|r| parse(r).effective()).collect();
    let is_base = rc_effs.iter().flatten().any(|(b, _)| b == t);
    let assoc = if is_base { None } else {
        rc_effs.iter().find_map(|effs| effs.iter().rev().find(|(_, args)| args.iter().any(|a| a == t)).map(|(b, _)| b.clone()))
    };
    match assoc {
        Some(base) => effs_hist.iter().any(|(b, args)| *b == base && arg_present(b, t, args)),
        None => effs_hist.iter().any(|(b, _)| b == t),
    }
}

const READERS: &[&str] = &["cat", "less", "more", "head", "tail", "grep", "egrep", "fgrep", "zcat", "nl", "tac",
    "sort", "uniq", "strings", "nano", "vi", "vim", "view", "awk", "sed", "cut", "od", "xxd", "hexdump", "bat", "batcat"];

/// Check `file_read`: une commande de LECTURE a reçu le fichier cible en argument (ou en < entrée)
pub fn file_read_matches(target: &str, history: &[Parsed]) -> bool {
    let want = target.trim_end_matches('/').rsplit('/').next().unwrap_or(target);
    let base_of = |a: &str| a.trim_end_matches('/').rsplit('/').next().unwrap_or("").to_string();
    history.iter().any(|p| {
        let by_arg = p.effective().iter().any(|(b, args)| READERS.contains(&b.as_str()) && args.iter().any(|a| !a.starts_with('-') && base_of(a) == want));
        let by_redir = p.cmds.iter().any(|c| c.redirs.iter().any(|(o, f)| o == "<" && base_of(f) == want));
        by_arg || by_redir
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn h(lines: &[&str]) -> Vec<Parsed> { lines.iter().map(|l| parse(l)).collect() }
    fn rc(v: &[&str]) -> Vec<String> { v.iter().map(|s| s.to_string()).collect() }

    #[test]
    fn substring_ne_valide_plus() {
        // P1.1: echo / sous-chaînes ne valident jamais
        assert!(!command_matches("pwd", &rc(&["pwd"]), &h(&["echo slsx pwdx"])));
        assert!(!command_matches("rm", &rc(&["rm a"]), &h(&["ls /armes"])));
        assert!(!command_matches("id", &rc(&["id"]), &h(&["cat guide.txt"])));
        assert!(!command_matches("tar", &rc(&["tar czf a.tgz d"]), &h(&["cat start.txt"])));
        assert!(!command_matches("|", &rc(&["ps aux | grep x"]), &h(&["echo 'a | b'"])));
        assert!(!command_matches(">", &rc(&["echo a > f"]), &h(&["echo '>'"])));
        assert!(!file_read_matches("bienvenue.txt", &h(&["echo bienvenue.txt"])));
    }

    #[test]
    fn commande_de_base_a4() {
        assert!(command_matches("ls -la", &rc(&["ls -la"]), &h(&["ls"])));
        assert!(command_matches("chown", &rc(&["sudo chown x f"]), &h(&["sudo chown capitaine f"])));
        assert!(command_matches("|", &rc(&["ps aux | grep ssh"]), &h(&["ps aux | grep ssh"])));
        assert!(command_matches("du", &rc(&["nice -n 19 du -sh /soutes"]), &h(&["nice -n 19 du -sh /soutes"])));
        assert!(command_matches("whoami", &rc(&["su - m -c whoami"]), &h(&["su - m -c whoami"])));
        assert!(command_matches("apt update", &rc(&["sudo apt update"]), &h(&["sudo apt update"])));
        assert!(!command_matches("apt update", &rc(&["sudo apt update"]), &h(&["sudo apt install x"])));
    }

    #[test]
    fn options_exactes() {
        assert!(command_matches("-la", &rc(&["ls -la"]), &h(&["ls -al"])));
        assert!(command_matches("-la", &rc(&["ls -la"]), &h(&["ls -l -a /"])));
        assert!(!command_matches("-la", &rc(&["ls -la"]), &h(&["ls -l"])));
        assert!(!command_matches("-l", &rc(&["wc -l f"]), &h(&["tar -czf a.tgz -l d", "ls -la"])));
        assert!(command_matches("-c", &rc(&["grep -c Failed f"]), &h(&["grep -c Failed f"])));
        assert!(!command_matches("-c", &rc(&["grep -c Failed f"]), &h(&["ping -c 4 x"])));
        assert!(command_matches("-5", &rc(&["head -5 f"]), &h(&["head -n 5 f"])));
        assert!(command_matches("-bn1", &rc(&["top -bn1"]), &h(&["top -b -n 1"])));
        assert!(command_matches("-aG", &rc(&["sudo usermod -aG g u"]), &h(&["sudo usermod -a -G g u"])));
        assert!(command_matches("-9", &rc(&["kill -9 6666"]), &h(&["kill -KILL 6666"])));
        assert!(command_matches("czf", &rc(&["tar czf a.tgz d"]), &h(&["tar -czvf a.tgz d"])));
        assert!(command_matches("+x", &rc(&["chmod +x f"]), &h(&["chmod u+x f"])));
        assert!(command_matches("route", &rc(&["ip route"]), &h(&["ip r"])));
        assert!(command_matches("restart", &rc(&["sudo systemctl restart ssh"]), &h(&["sudo systemctl restart ssh"])));
        assert!(!command_matches("restart", &rc(&["sudo systemctl restart ssh"]), &h(&["echo restart"])));
    }

    #[test]
    fn lecture_et_redirections() {
        assert!(file_read_matches("bienvenue.txt", &h(&["cat bienvenue.txt"])));
        assert!(file_read_matches("sshd_config", &h(&["grep Permit /securite/config/sshd_config"])));
        assert!(file_read_matches("notes.txt", &h(&["wc -l < notes.txt"])));
        assert!(command_matches(">>", &rc(&["echo a >> f"]), &h(&["echo a >>f"])));
        assert!(!command_matches(">", &rc(&["echo a > f"]), &h(&["echo a >> f"])));
        let p = parse("echo \"a  b\" > /soutes/f 2>/dev/null && cd /pont");
        assert_eq!(p.cmds[0].words, vec!["echo", "a  b"]);
        assert_eq!(p.cmds[0].redirs[0], (">".to_string(), "/soutes/f".to_string()));
        assert_eq!(p.cmds[0].redirs[1].0, "2>");
        assert!(!p.is_simple());
        assert!(parse("pwd").is_simple());
    }
}

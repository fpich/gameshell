//! Table de processus fictive: ps, pgrep, pkill, kill, top.
//! Le « système pirate » (PID 9999, 4242, 6666...) existe VRAIMENT dans le jeu: kill l'élimine.

use super::Ctx;

pub struct Proc {
    pub pid: u32,
    pub user: String,
    pub cpu: String,
    pub mem: String,
    pub stat: String,
    pub start: String,
    pub cmd: String,
}

impl Proc {
    pub fn name(&self) -> String {
        let first = self.cmd.split_whitespace().next().unwrap_or("");
        let first = first.trim_end_matches(':');
        first
            .rsplit('/')
            .next()
            .unwrap_or(first)
            .trim_start_matches('-')
            .to_string()
    }
}

pub fn table(c: &Ctx) -> Vec<Proc> {
    c.lines("processes")
        .iter()
        .filter_map(|l| {
            let f: Vec<&str> = l.splitn(7, '|').collect();
            if f.len() < 7 {
                return None;
            }
            Some(Proc {
                pid: f[0].parse().ok()?,
                user: f[1].into(),
                cpu: f[2].into(),
                mem: f[3].into(),
                stat: f[4].into(),
                start: f[5].into(),
                cmd: f[6].into(),
            })
        })
        .collect()
}

fn save(c: &Ctx, t: &[Proc]) {
    let l: Vec<String> = t
        .iter()
        .map(|p| {
            format!(
                "{}|{}|{}|{}|{}|{}|{}",
                p.pid, p.user, p.cpu, p.mem, p.stat, p.start, p.cmd
            )
        })
        .collect();
    c.set_lines("processes", &l);
}

/// Processus présent (par PID ou par nom) — pour les checks
pub fn running(c: &Ctx, what: &str) -> bool {
    let t = table(c);
    match what.parse::<u32>() {
        Ok(pid) => t.iter().any(|p| p.pid == pid),
        Err(_) => t
            .iter()
            .any(|p| p.name().contains(what) || p.cmd.contains(what)),
    }
}

fn vsz(p: &Proc) -> (u64, u64) {
    let m: f64 = p.mem.replace(',', ".").parse().unwrap_or(0.1);
    let rss = (m * 40960.0) as u64 + 1024;
    (rss * 3 + 5000, rss)
}

pub fn ps(c: &Ctx, a: &[String]) -> i32 {
    let joined: String = a
        .iter()
        .map(|s| s.trim_start_matches('-'))
        .collect::<Vec<_>>()
        .join("");
    let t = table(c);
    let all = joined.contains('a') && joined.contains('x')
        || joined.contains('e')
        || joined.contains('A');
    let user_fmt = joined.contains('u') && !a.iter().any(|x| x == "-u");
    let full = joined.contains('f');
    let filter_user = a
        .iter()
        .position(|x| x == "-u")
        .and_then(|i| a.get(i + 1))
        .cloned();
    let rows: Vec<&Proc> = t
        .iter()
        .filter(|p| {
            if let Some(u) = &filter_user {
                return &p.user == u;
            }
            all || p.user == c.me() && (p.cmd.contains("bash"))
        })
        .collect();
    let tty = |p: &Proc| {
        if p.cmd.starts_with('-') || p.cmd.contains("bash") {
            "pts/0"
        } else {
            "?"
        }
    };
    if user_fmt {
        println!("USER         PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND");
        for p in &rows {
            let (v, r) = vsz(p);
            println!(
                "{:<10} {:>5} {:>4} {:>4} {:>6} {:>5} {:<8} {:<4} {:<5}   0:{:02} {}",
                p.user,
                p.pid,
                p.cpu,
                p.mem,
                v,
                r,
                tty(p),
                p.stat,
                p.start,
                p.pid % 60,
                p.cmd
            );
        }
        if all {
            println!(
                "{:<10} {:>5}  0.0  0.0  11000  3500 pts/0    R+   {:<5}   0:00 ps {}",
                c.me(),
                7777,
                "08:20",
                a.join(" ")
            );
        }
    } else if full {
        println!("UID          PID    PPID  C STIME TTY          TIME CMD");
        for p in &rows {
            println!(
                "{:<10} {:>6} {:>7}  0 {:<5} {:<8} 00:00:0{} {}",
                p.user,
                p.pid,
                if p.pid == 1 { 0 } else { 1 },
                p.start,
                tty(p),
                p.pid % 10,
                p.cmd
            );
        }
    } else {
        println!("    PID TTY          TIME CMD");
        for p in &rows {
            println!("{:>7} {:<8} 00:00:00 {}", p.pid, tty(p), p.name());
        }
        println!("{:>7} pts/0    00:00:00 ps", 7777);
    }
    0
}

fn matches(pat: &str, s: &str) -> bool {
    let (anchored_start, p) = match pat.strip_prefix('^') {
        Some(r) => (true, r),
        None => (false, pat),
    };
    let (anchored_end, p) = match p.strip_suffix('$') {
        Some(r) => (true, r),
        None => (false, p),
    };
    match (anchored_start, anchored_end) {
        (true, true) => s == p,
        (true, false) => s.starts_with(p),
        (false, true) => s.ends_with(p),
        _ => s.contains(p),
    }
}

/// pgrep (kill=false) / pkill / killall (kill=true)
pub fn pgrep(c: &Ctx, a: &[String], kill: bool) -> i32 {
    let mut full = false;
    let mut show = false;
    let mut listname = false;
    let mut user: Option<String> = None;
    let mut pat: Option<String> = None;
    let mut i = 0;
    while i < a.len() {
        let x = a[i].as_str();
        if x == "-u" {
            user = a.get(i + 1).cloned();
            i += 2;
            continue;
        }
        if x.starts_with('-')
            && x.len() > 1
            && !x[1..].chars().all(|ch| ch.is_ascii_digit())
            && !x[1..].chars().all(|ch| ch.is_ascii_uppercase())
        {
            for ch in x[1..].chars() {
                match ch {
                    'f' => full = true,
                    'a' => show = true,
                    'l' => listname = true,
                    _ => {}
                }
            }
        } else if !x.starts_with('-') {
            pat = Some(x.to_string());
        }
        i += 1;
    }
    let pat = match pat {
        Some(p) => p,
        None => {
            eprintln!(
                "{}: aucun critère de correspondance indiqué",
                if kill { "pkill" } else { "pgrep" }
            );
            return 2;
        }
    };
    let t = table(c);
    let hits: Vec<&Proc> = t
        .iter()
        .filter(|p| {
            let subject = if full { p.cmd.clone() } else { p.name() };
            matches(&pat, &subject) && user.as_ref().map(|u| &p.user == u).unwrap_or(true)
        })
        .collect();
    if hits.is_empty() {
        return 1;
    }
    if kill {
        if let Some(p) = hits.iter().find(|p| p.user != c.me() && !c.is_root()) {
            eprintln!(
                "pkill: kill du pid {} a échoué : Opération non permise",
                p.pid
            );
            return 1;
        }
        let pids: Vec<u32> = hits.iter().map(|p| p.pid).collect();
        let keep: Vec<Proc> = t.into_iter().filter(|p| !pids.contains(&p.pid)).collect();
        save(c, &keep);
        for pid in pids {
            c.event(&format!("kill {}", pid));
        }
        return 0;
    }
    for p in hits {
        if show || full && listname {
            println!("{} {}", p.pid, p.cmd);
        } else if listname {
            println!("{} {}", p.pid, p.name());
        } else {
            println!("{}", p.pid);
        }
    }
    0
}

pub fn kill(c: &Ctx, a: &[String]) -> i32 {
    if a.iter().any(|x| x == "-l" || x == "-L") {
        println!(" 1) SIGHUP\t 2) SIGINT\t 3) SIGQUIT\t 9) SIGKILL\t15) SIGTERM\t18) SIGCONT\t19) SIGSTOP");
        return 0;
    }
    let mut pids = Vec::new();
    let mut i = 0;
    while i < a.len() {
        let x = &a[i];
        if x == "-s" || x == "-n" {
            i += 2;
            continue;
        }
        if !x.starts_with('-') {
            pids.push(x.clone());
        }
        i += 1;
    }
    if pids.is_empty() {
        eprintln!("kill : utilisation : kill [-s sigspec | -n signum | -sigspec] pid ... ou kill -l [sigspec]");
        return 2;
    }
    let mut t = table(c);
    let mut rc = 0;
    for p in pids {
        let pid: u32 = match p.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("bash: kill: {} : les arguments doivent être des identifiants de processus ou de tâche", p);
                rc = 1;
                continue;
            }
        };
        match t.iter().position(|x| x.pid == pid) {
            Some(idx) => {
                if t[idx].user != c.me() && !c.is_root() {
                    eprintln!("bash: kill: ({}) - Opération non permise", pid);
                    rc = 1;
                    continue;
                }
                if pid == 1 {
                    eprintln!("bash: kill: (1) - Opération non permise");
                    rc = 1;
                    continue;
                }
                t.remove(idx);
                c.event(&format!("kill {}", pid));
            }
            None => {
                eprintln!("bash: kill: ({}) - Aucun processus de ce type", pid);
                rc = 1;
            }
        }
    }
    save(c, &t);
    rc
}

pub fn top(c: &Ctx, a: &[String]) -> i32 {
    let joined: String = a
        .iter()
        .filter(|x| x.starts_with('-'))
        .map(|s| s.trim_start_matches('-'))
        .collect();
    let batch = joined.contains('b') || !crate::sys::is_tty(1) || !crate::sys::is_tty(0);
    let iters: u32 = {
        let mut n: Option<u32> = None;
        for (i, x) in a.iter().enumerate() {
            if let Some(v) = x.strip_prefix("-bn").or_else(|| x.strip_prefix("-n")) {
                if !v.is_empty() {
                    n = v.parse().ok();
                } else {
                    n = a.get(i + 1).and_then(|s| s.parse().ok());
                }
            }
        }
        n.unwrap_or(if batch { 1 } else { u32::MAX })
    };
    let snapshot = || {
        let t = table(c);
        let total: f64 = t.iter().map(|p| p.cpu.parse::<f64>().unwrap_or(0.0)).sum();
        let now = chrono::Local::now().format("%H:%M:%S");
        println!(
            "top - {} up 1 day,  3:14,  1 user,  load average: {:.2}, 0,37, 0,30",
            now,
            total / 100.0
        );
        println!(
            "Tâches: {} total,   1 en cours, {} en veille,   0 arrêté,   0 zombie",
            t.len(),
            t.len().saturating_sub(1)
        );
        println!(
            "%Cpu(s): {:4.1} ut,  1,2 sy,  0,0 ni, {:4.1} id,  0,1 wa,  0,0 hi,  0,1 si,  0,0 st",
            total.min(99.0),
            (100.0 - total).max(0.5)
        );
        println!("MiB Mem :   3936,4 total,    812,7 libr,   1904,2 util,   1219,5 tamp/cache");
        println!("MiB Éch :    975,0 total,    975,0 libr,      0,0 util.   2032,2 dispo Mem\n");
        println!("    PID UTIL.     PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TEMPS+ COM.");
        let mut rows: Vec<&Proc> = t.iter().collect();
        rows.sort_by(|x, y| {
            y.cpu
                .parse::<f64>()
                .unwrap_or(0.0)
                .partial_cmp(&x.cpu.parse::<f64>().unwrap_or(0.0))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for p in rows {
            let (v, r) = vsz(p);
            println!(
                "{:>7} {:<9} 20   0 {:>7} {:>6} {:>6} {} {:>5} {:>5} {:>4}:{:02}.{:02} {}",
                p.pid,
                p.user,
                v,
                r,
                r / 3,
                &p.stat[..1],
                p.cpu,
                p.mem,
                p.pid % 7,
                p.pid % 60,
                p.pid % 100,
                p.name()
            );
        }
    };
    if batch {
        for k in 0..iters.min(5) {
            if k > 0 {
                println!();
            }
            snapshot();
        }
        return 0;
    }
    // interactif: rafraîchit toutes les 3 s, 'q' ou Ctrl+C pour quitter
    let saved = crate::sys::get_termios(0);
    if let Some(t) = &saved {
        crate::sys::set_termios(0, &crate::sys::raw_from(t, 30));
    }
    crate::sys::catch_sigint();
    let mut n = 0;
    loop {
        print!("\x1b[H\x1b[2J");
        snapshot();
        println!("\n(q pour quitter)");
        n += 1;
        if n >= iters {
            break;
        }
        use std::io::Read;
        let mut b = [0u8; 1];
        let got = std::io::stdin().read(&mut b).unwrap_or(0);
        if crate::sys::sigint_seen() || (got == 1 && (b[0] == b'q' || b[0] == 3)) {
            break;
        }
    }
    if let Some(t) = &saved {
        crate::sys::set_termios(0, t);
    }
    0
}

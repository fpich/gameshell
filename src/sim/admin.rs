//! Administration simulée: services (systemctl, journalctl), pare-feu (ufw, iptables),
//! fail2ban, audit (auditctl), paquets (apt, dpkg — catalogue local, 100% offline), chroot.

use super::{need_root, read_answer, Ctx};

// ─────────────────────────────── services ───────────────────────────────

struct Svc { name: String, active: String, enabled: String, pid: String, desc: String }

fn services(c: &Ctx) -> Vec<Svc> {
    c.lines("services").iter().filter_map(|l| {
        let f: Vec<&str> = l.splitn(5, '|').collect();
        if f.len() < 5 { return None; }
        Some(Svc { name: f[0].into(), active: f[1].into(), enabled: f[2].into(), pid: f[3].into(), desc: f[4].into() })
    }).collect()
}

fn save_services(c: &Ctx, s: &[Svc]) {
    let l: Vec<String> = s.iter().map(|x| format!("{}|{}|{}|{}|{}", x.name, x.active, x.enabled, x.pid, x.desc)).collect();
    c.set_lines("services", &l);
}

fn unit_name(u: &str) -> String {
    let u = u.trim_end_matches(".service");
    match u { "sshd" => "ssh".into(), "fail2ban-server" => "fail2ban".into(), x => x.into() }
}

pub fn systemctl(c: &Ctx, a: &[String]) -> i32 {
    let args: Vec<&String> = a.iter().filter(|x| !x.starts_with('-')).collect();
    let verb = args.first().map(|s| s.as_str()).unwrap_or("list-units");
    let units: Vec<String> = args.iter().skip(1).map(|u| unit_name(u)).collect();
    let mut svcs = services(c);
    match verb {
        "status" => {
            if units.is_empty() {
                println!("● {}\n    State: running\n    Units: {} loaded\n     Jobs: 0 queued\n   Failed: 0 units", c.host, svcs.len());
                return 0;
            }
            let mut rc = 0;
            for u in &units {
                match svcs.iter().find(|s| &s.name == u) {
                    Some(s) => {
                        let on = s.active == "active";
                        println!("{} {}.service - {}", if on { "●" } else { "○" }, s.name, s.desc);
                        println!("     Loaded: loaded (/usr/lib/systemd/system/{}.service; {}; preset: enabled)", s.name, s.enabled);
                        if on { println!("     Active: active (running) since Mon 2024-01-15 08:00:01 CET; 3h 14min ago"); println!("   Main PID: {} ({})", s.pid, s.name); }
                        else { println!("     Active: inactive (dead)"); rc = 3; }
                        println!();
                        for l in c.lines("journal").iter().filter(|l| l.contains(&format!("{}[", s.name)) || (s.name == "ssh" && l.contains("sshd["))).rev().take(4).collect::<Vec<_>>().into_iter().rev() {
                            println!("{}", l);
                        }
                    }
                    None => { eprintln!("Unit {}.service could not be found.", u); rc = 4; }
                }
            }
            rc
        }
        "is-active" | "is-enabled" => {
            let mut rc = 0;
            for u in &units {
                match svcs.iter().find(|s| &s.name == u) {
                    Some(s) => { let v = if verb == "is-active" { &s.active } else { &s.enabled }; println!("{}", v); if v != "active" && v != "enabled" && v != "static" { rc = 3; } }
                    None => { println!("inactive"); rc = 3; }
                }
            }
            rc
        }
        "list-units" | "list-unit-files" => {
            println!("  UNIT                         LOAD   ACTIVE   SUB     DESCRIPTION");
            for s in &svcs { println!("  {:<28} loaded {:<8} {:<7} {}", format!("{}.service", s.name), s.active, if s.active == "active" { "running" } else { "dead" }, s.desc); }
            println!("\n{} unités listées.", svcs.len());
            0
        }
        "start" | "stop" | "restart" | "reload" | "enable" | "disable" => {
            if !c.is_root() {
                eprintln!("==== AUTHENTICATING FOR org.freedesktop.systemd1.manage-units ====");
                eprintln!("Authentification requise pour gérer les services système.");
                eprintln!("Échec: droits administrateur nécessaires (essayez avec sudo).");
                return 1;
            }
            if units.is_empty() { eprintln!("Too few arguments."); return 1; }
            for u in &units {
                let s = match svcs.iter_mut().find(|s| &s.name == u) { Some(s) => s, None => { eprintln!("Failed to {} {}.service: Unit {}.service not found.", verb, u, u); return 5; } };
                match verb {
                    "start" => s.active = "active".into(),
                    "stop" => s.active = "inactive".into(),
                    "restart" | "reload" => s.active = "active".into(),
                    "enable" => { s.enabled = "enabled".into(); println!("Created symlink /etc/systemd/system/multi-user.target.wants/{}.service → /usr/lib/systemd/system/{}.service.", u, u); }
                    "disable" => { s.enabled = "disabled".into(); println!("Removed \"/etc/systemd/system/multi-user.target.wants/{}.service\".", u); }
                    _ => {}
                }
                let jl = match verb { "stop" => format!("Stopped {}.service - {}.", u, s.desc), "start" => format!("Started {}.service - {}.", u, s.desc), "restart" => format!("Started {}.service - {}. (redémarré)", u, s.desc), _ => format!("{} {}.service", verb, u) };
                c.journal("systemd[1]", &jl);
                c.event(&format!("systemctl {} {}", verb, u));
            }
            save_services(c, &svcs);
            0
        }
        "daemon-reload" => { if !c.is_root() { return need_root("systemctl"); } 0 }
        _ => { eprintln!("Unknown command verb {}.", verb); 1 }
    }
}

pub fn journalctl(c: &Ctx, a: &[String]) -> i32 {
    let mut n: Option<usize> = None;
    let mut unit: Option<String> = None;
    let mut follow = false;
    let mut grep: Option<String> = None;
    let mut reverse = false;
    let mut i = 0;
    while i < a.len() {
        let x = a[i].as_str();
        match x {
            "-n" | "--lines" => { n = a.get(i + 1).and_then(|s| s.parse().ok()); i += 1; }
            "-u" | "--unit" => { unit = a.get(i + 1).map(|s| unit_name(s)); i += 1; }
            "-f" | "--follow" => follow = true,
            "-r" | "--reverse" => reverse = true,
            "-e" => n = Some(n.unwrap_or(1000)),
            "-g" | "--grep" => { grep = a.get(i + 1).cloned(); i += 1; }
            "--since" | "--until" | "-p" | "--priority" | "-o" | "-b" => { if x != "-b" { i += 1; } }
            _ => {
                if let Some(v) = x.strip_prefix("-n") { n = v.parse().ok(); }
                if let Some(v) = x.strip_prefix("--lines=") { n = v.parse().ok(); }
                if let Some(v) = x.strip_prefix("--unit=") { unit = Some(unit_name(v)); }
                if let Some(v) = x.strip_prefix("-u") { if !v.is_empty() { unit = Some(unit_name(v)); } }
            }
        }
        i += 1;
    }
    let all = c.lines("journal");
    let mut sel: Vec<&String> = all.iter().filter(|l| match &unit {
        Some(u) => { let tag = if u == "ssh" { "sshd[".to_string() } else { format!("{}[", u) }; l.contains(&tag) || l.contains(&format!(" {}.service", u)) }
        None => true,
    }).filter(|l| grep.as_ref().map(|g| l.contains(g.as_str())).unwrap_or(true)).collect();
    if follow && n.is_none() { n = Some(10); }
    if let Some(k) = n { let len = sel.len(); if k < len { sel = sel[len - k..].to_vec(); } }
    if reverse { sel.reverse(); }
    if sel.is_empty() { println!("-- No entries --"); }
    for l in sel { println!("{}", l); }
    if follow {
        crate::sys::catch_sigint();
        eprintln!("(suivi en direct — Ctrl+C pour arrêter)");
        while !crate::sys::sigint_seen() { std::thread::sleep(std::time::Duration::from_millis(200)); }
    }
    0
}

// ─────────────────────────────── pare-feu UFW ───────────────────────────────

fn ufw_rules(c: &Ctx) -> Vec<String> { c.lines("ufw.rules") }

fn parse_rule(a: &[String]) -> Option<String> {
    // allow 22 | allow 22/tcp | deny from IP | deny from IP to any port 22 | allow in 80 | allow ssh
    let mut it = a.iter().map(|s| s.as_str()).peekable();
    let action = match it.next()? { "allow" => "ALLOW", "deny" => "DENY", "reject" => "REJECT", "limit" => "LIMIT", _ => return None };
    let mut dir = "IN";
    let mut from = "Anywhere".to_string();
    let mut port = "Anywhere".to_string();
    let mut proto = String::new();
    while let Some(x) = it.next() {
        match x {
            "in" => dir = "IN",
            "out" => dir = "OUT",
            "from" => from = it.next()?.to_string().replace("any", "Anywhere"),
            "to" => { let _ = it.next(); }
            "port" => port = it.next()?.to_string(),
            "proto" => proto = it.next()?.to_string(),
            "ssh" => port = "22/tcp".into(),
            "http" => port = "80/tcp".into(),
            "https" => port = "443/tcp".into(),
            p => { if p.split('/').next().map(|n| n.chars().all(|ch| ch.is_ascii_digit() || ch == ':')).unwrap_or(false) { port = p.into(); } else { return None; } }
        }
    }
    if !proto.is_empty() && !port.contains('/') && port != "Anywhere" { port = format!("{}/{}", port, proto); }
    Some(format!("{} {} {} from {}", action, dir, port, from))
}

fn rule_cols(r: &str) -> (String, String, String) {
    // "ALLOW IN 22/tcp from Anywhere" → (To, Action, From)
    let f: Vec<&str> = r.split_whitespace().collect();
    if f.len() >= 5 { (f[2].to_string(), format!("{} {}", f[0], f[1]), f[4..].join(" ")) } else { (r.into(), String::new(), String::new()) }
}

pub fn ufw(c: &Ctx, a: &[String]) -> i32 {
    if !c.is_root() { eprintln!("ERROR: You need to be root to run this script"); eprintln!("(astuce: le pare-feu se pilote avec les droits administrateur)"); return 1; }
    let sub = a.first().map(|s| s.as_str()).unwrap_or("");
    let enabled = c.kv("ufw.conf", "enabled").as_deref() == Some("yes");
    match sub {
        "status" => {
            let verbose = a.iter().any(|x| x == "verbose");
            let numbered = a.iter().any(|x| x == "numbered");
            if !enabled { println!("Status: inactive"); return 0; }
            println!("Status: active");
            if verbose {
                println!("Logging: on ({})", c.kv("ufw.conf", "logging").unwrap_or_else(|| "low".into()));
                println!("Default: {} (incoming), {} (outgoing), disabled (routed)", c.kv("ufw.conf", "default_in").unwrap_or_default(), c.kv("ufw.conf", "default_out").unwrap_or_default());
                println!("New profiles: skip");
            }
            let rules = ufw_rules(c);
            if !rules.is_empty() {
                println!("\n{:<27} {:<11} {}", if numbered { "     To" } else { "To" }, "Action", "From");
                println!("{:<27} {:<11} {}", "--", "------", "----");
                for (k, r) in rules.iter().enumerate() {
                    let (to, act, from) = rule_cols(r);
                    let act = if verbose { act } else { act.replace(" IN", "") };
                    if numbered { println!("[{:>2}] {:<22} {:<11} {}", k + 1, to, act, from); } else { println!("{:<27} {:<11} {}", to, act, from); }
                }
            }
            0
        }
        "enable" => { c.set_kv("ufw.conf", "enabled", "yes"); c.event("ufw enable"); println!("Firewall is active and enabled on system startup"); 0 }
        "disable" => { c.set_kv("ufw.conf", "enabled", "no"); c.event("ufw disable"); println!("Firewall stopped and disabled on system startup"); 0 }
        "reload" => { println!("Firewall reloaded"); 0 }
        "reset" => { c.set_lines("ufw.rules", &[]); c.set_kv("ufw.conf", "enabled", "no"); c.event("ufw reset"); println!("Resetting all rules to installed defaults."); 0 }
        "default" => {
            let pol = a.get(1).map(|s| s.as_str()).unwrap_or("");
            let dir = a.get(2).map(|s| s.as_str()).unwrap_or("incoming");
            if !matches!(pol, "allow" | "deny" | "reject") { eprintln!("ERROR: Invalid syntax"); return 1; }
            let key = match dir { "outgoing" => "default_out", "routed" => "default_routed", _ => "default_in" };
            c.set_kv("ufw.conf", key, pol);
            c.event(&format!("ufw default {} {}", pol, dir));
            println!("Default {} policy changed to '{}'\n(be sure to update your rules accordingly)", dir, pol);
            0
        }
        "allow" | "deny" | "reject" | "limit" => match parse_rule(a) {
            Some(r) => {
                let mut rules = ufw_rules(c);
                if rules.contains(&r) { println!("Skipping adding existing rule"); return 0; }
                rules.push(r.clone());
                c.set_lines("ufw.rules", &rules);
                c.event(&format!("ufw {}", a.join(" ")));
                println!("Rule added");
                0
            }
            None => { eprintln!("ERROR: Wrong number of arguments"); 1 }
        },
        "delete" => {
            let mut rules = ufw_rules(c);
            if let Some(n) = a.get(1).and_then(|s| s.parse::<usize>().ok()) {
                if n == 0 || n > rules.len() { eprintln!("ERROR: Could not find rule '{}'", n); return 1; }
                rules.remove(n - 1);
            } else {
                match parse_rule(&a[1..]) { Some(r) if rules.contains(&r) => rules.retain(|x| *x != r), _ => { eprintln!("Could not delete non-existent rule"); return 1; } }
            }
            c.set_lines("ufw.rules", &rules);
            c.event(&format!("ufw {}", a.join(" ")));
            println!("Rule deleted");
            0
        }
        "app" => { println!("Available applications:\n  OpenSSH"); 0 }
        _ => { println!("Usage: ufw COMMAND\n\nCommands:\n enable | disable | reload | reset\n default ARG\n status [verbose|numbered]\n allow|deny|reject|limit ARGS\n delete RULE|NUM"); if sub.is_empty() { 0 } else { 1 } }
    }
}

// ─────────────────────────────── iptables ───────────────────────────────

pub fn iptables(c: &Ctx, a: &[String]) -> i32 {
    if !c.is_root() { eprintln!("iptables v1.8.10 (nf_tables): Could not fetch rule set generation id: Permission denied (you must be root)"); return 4; }
    let mut rules = c.lines("iptables.rules");
    let first = a.first().map(|s| s.as_str()).unwrap_or("");
    let numeric = a.iter().any(|x| x == "-n" || x.starts_with("-L") && x.contains('n') || x == "-nL");
    let listing = a.iter().any(|x| x == "-L" || x == "--list" || x == "-nL" || x == "-Ln" || x == "-vL" || x == "-L" );
    if listing || a.iter().any(|x| x.starts_with("-L") || (x.starts_with('-') && !x.starts_with("--") && x.contains('L'))) {
        let only = a.iter().skip_while(|x| !x.contains('L')).nth(1).filter(|x| !x.starts_with('-')).cloned();
        let host = |s: &str| if numeric { s.to_string() } else { s.replace("0.0.0.0/0", "anywhere") };
        for chain in ["INPUT", "FORWARD", "OUTPUT"] {
            if let Some(o) = &only { if o != chain { continue; } }
            let pol = rules.iter().find_map(|r| r.strip_prefix(&format!("-P {} ", chain)).map(|s| s.to_string())).unwrap_or_else(|| "ACCEPT".into());
            println!("Chain {} (policy {})", chain, pol);
            println!("target     prot opt source               destination");
            for r in rules.iter().filter(|r| r.starts_with(&format!("-A {} ", chain))) {
                let w: Vec<&str> = r.split_whitespace().collect();
                let get = |k: &str| w.iter().position(|x| *x == k).and_then(|i| w.get(i + 1)).map(|s| s.to_string());
                let target = get("-j").unwrap_or_default();
                let prot = get("-p").unwrap_or_else(|| "all".into());
                let src = get("-s").unwrap_or_else(|| "0.0.0.0/0".into());
                let dst = get("-d").unwrap_or_else(|| "0.0.0.0/0".into());
                let mut extra = String::new();
                if let Some(p) = get("--dport") { extra.push_str(&format!(" {} dpt:{}", prot, p)); }
                if let Some(s) = get("--state") { extra.push_str(&format!(" state {}", s)); }
                if r.contains("-i lo") { extra.push_str(" (lo)"); }
                println!("{:<10} {:<4} --  {:<20} {:<20}{}", target, prot, host(&src), host(&dst), extra);
            }
            println!();
        }
        return 0;
    }
    if first == "-S" || first == "--list-rules" { for r in &rules { println!("{}", r); } return 0; }
    match first {
        "-A" | "-I" | "--append" | "--insert" => {
            let chain = a.get(1).cloned().unwrap_or_default();
            if !["INPUT", "FORWARD", "OUTPUT"].contains(&chain.as_str()) { eprintln!("iptables: Chain '{}' does not exist.", chain); return 1; }
            if !a.iter().any(|x| x == "-j") { eprintln!("iptables: aucune cible (-j ACCEPT|DROP|REJECT) indiquée"); return 2; }
            let r = format!("-A {} {}", chain, a[2..].join(" "));
            if first == "-I" || first == "--insert" {
                let pos = rules.iter().position(|x| x.starts_with(&format!("-A {} ", chain))).unwrap_or(rules.len());
                rules.insert(pos, r);
            } else { rules.push(r); }
            c.set_lines("iptables.rules", &rules);
            c.event(&format!("iptables {}", a.join(" ")));
            0
        }
        "-D" | "--delete" => {
            let chain = a.get(1).cloned().unwrap_or_default();
            if let Some(n) = a.get(2).and_then(|s| s.parse::<usize>().ok()) {
                let idx: Vec<usize> = rules.iter().enumerate().filter(|(_, r)| r.starts_with(&format!("-A {} ", chain))).map(|(i, _)| i).collect();
                if n == 0 || n > idx.len() { eprintln!("iptables: Index of deletion too big."); return 1; }
                rules.remove(idx[n - 1]);
            } else {
                let r = format!("-A {} {}", chain, a[2..].join(" "));
                if !rules.contains(&r) { eprintln!("iptables: Bad rule (does a matching rule exist in that chain?)."); return 1; }
                rules.retain(|x| *x != r);
            }
            c.set_lines("iptables.rules", &rules);
            c.event(&format!("iptables {}", a.join(" ")));
            0
        }
        "-F" | "--flush" => {
            let chain = a.get(1).cloned();
            rules.retain(|r| !r.starts_with("-A ") || chain.as_ref().map(|ch| !r.starts_with(&format!("-A {} ", ch))).unwrap_or(false));
            c.set_lines("iptables.rules", &rules);
            c.event("iptables -F");
            0
        }
        "-P" | "--policy" => {
            let (chain, pol) = (a.get(1).cloned().unwrap_or_default(), a.get(2).cloned().unwrap_or_default());
            if !["ACCEPT", "DROP"].contains(&pol.as_str()) { eprintln!("iptables: Bad policy name."); return 2; }
            rules.retain(|r| !r.starts_with(&format!("-P {} ", chain)));
            rules.insert(0, format!("-P {} {}", chain, pol));
            c.set_lines("iptables.rules", &rules);
            c.event(&format!("iptables -P {} {}", chain, pol));
            0
        }
        _ => { eprintln!("iptables v1.8.10: no command specified\nTry `iptables -h' or 'iptables --help' for more information."); 2 }
    }
}

// ─────────────────────────────── fail2ban ───────────────────────────────

pub fn fail2ban(c: &Ctx, a: &[String]) -> i32 {
    if !c.is_root() { eprintln!("ERROR  Permission denied to socket: /var/run/fail2ban/fail2ban.sock, (you must be root)"); return 255; }
    let mut bans = c.lines("fail2ban.bans");
    let args: Vec<&str> = a.iter().map(|s| s.as_str()).collect();
    match args.as_slice() {
        ["ping"] => { println!("Server replied: pong"); 0 }
        ["status"] => { println!("Status\n|- Number of jail:\t2\n`- Jail list:\tsshd, recidive"); 0 }
        ["status", jail] => {
            if !["sshd", "recidive"].contains(jail) { eprintln!("ERROR  NOK: ('{}',)\nSorry but the jail '{}' does not exist", jail, jail); return 255; }
            let ips: Vec<String> = bans.iter().filter_map(|b| b.strip_prefix(&format!("{} ", jail)).map(|s| s.to_string())).collect();
            println!("Status for the jail: {}\n|- Filter\n|  |- Currently failed:\t3\n|  |- Total failed:\t47\n|  `- Journal matches:\t_SYSTEMD_UNIT=sshd.service + _COMM=sshd\n`- Actions\n   |- Currently banned:\t{}\n   |- Total banned:\t{}\n   `- Banned IP list:\t{}", jail, ips.len(), ips.len() + 3, ips.join(" "));
            0
        }
        ["banned"] => { println!("[{{'sshd': [{}]}}]", bans.iter().map(|b| format!("'{}'", b.split_whitespace().nth(1).unwrap_or(""))).collect::<Vec<_>>().join(", ")); 0 }
        ["set", jail, "banip", ip] => { let e = format!("{} {}", jail, ip); if !bans.contains(&e) { bans.push(e); } c.set_lines("fail2ban.bans", &bans); c.event(&format!("fail2ban ban {}", ip)); println!("1"); 0 }
        ["set", jail, "unbanip", ip] => {
            let e = format!("{} {}", jail, ip);
            if !bans.contains(&e) { println!("0"); return 0; }
            bans.retain(|b| *b != e); c.set_lines("fail2ban.bans", &bans); c.event(&format!("fail2ban unban {}", ip)); println!("1"); 0
        }
        ["unban", "--all"] => { let n = bans.len(); bans.clear(); c.set_lines("fail2ban.bans", &bans); c.event("fail2ban unban --all"); println!("{}", n); 0 }
        ["unban", ips @ ..] if !ips.is_empty() => {
            let before = bans.len();
            bans.retain(|b| !ips.iter().any(|ip| b.ends_with(&format!(" {}", ip))));
            c.set_lines("fail2ban.bans", &bans);
            for ip in ips { c.event(&format!("fail2ban unban {}", ip)); }
            println!("{}", before - bans.len());
            0
        }
        _ => { println!("Usage: fail2ban-client [OPTIONS] <COMMAND>\n  status [JAIL] | set JAIL banip|unbanip IP | unban IP ... | banned | ping"); 1 }
    }
}

// ─────────────────────────────── auditd ───────────────────────────────

pub fn auditctl(c: &Ctx, a: &[String]) -> i32 {
    if !c.is_root() { eprintln!("You must be root to run this program."); return 4; }
    let mut rules = c.lines("audit.rules");
    let first = a.first().map(|s| s.as_str()).unwrap_or("");
    match first {
        "-l" => { if rules.is_empty() { println!("No rules"); } for r in &rules { println!("{}", r); } 0 }
        "-s" => { println!("enabled 1\nfailure 1\npid 2210\nrate_limit 0\nbacklog_limit 8192\nlost 0\nbacklog 0"); 0 }
        "-D" => { rules.clear(); c.set_lines("audit.rules", &rules); c.event("auditctl -D"); println!("No rules"); 0 }
        "-w" | "-W" => {
            let path = a.get(1).cloned().unwrap_or_default();
            if path.is_empty() { eprintln!("option -w requires an argument"); return 1; }
            if std::fs::symlink_metadata(c.real(&path)).is_err() { eprintln!("Error sending add rule data request (No such file or directory)"); return 1; }
            let get = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).cloned();
            let perms = get("-p").unwrap_or_else(|| "rwxa".into());
            let mut r = format!("-w {} -p {}", path, perms);
            if let Some(k) = get("-k") { r.push_str(&format!(" -k {}", k)); }
            if first == "-W" {
                let before = rules.len();
                rules.retain(|x| !x.starts_with(&format!("-w {} ", path)));
                if before == rules.len() { eprintln!("Error sending delete rule data request (No such file or directory)"); return 1; }
            } else {
                if rules.iter().any(|x| x.starts_with(&format!("-w {} ", path))) { eprintln!("Error sending add rule data request (Rule exists)"); return 1; }
                rules.push(r);
            }
            c.set_lines("audit.rules", &rules);
            c.event(&format!("auditctl {}", a.join(" ")));
            0
        }
        _ => { println!("usage: auditctl [options]\n    -l  liste les règles\n    -w <chemin> -p <rwxa> -k <clé>  surveille un fichier\n    -W <chemin>  retire une surveillance\n    -D  supprime toutes les règles"); if first.is_empty() { 0 } else { 1 } }
    }
}

// ─────────────────────────────── paquets ───────────────────────────────

const CATALOG: &[(&str, &str, &str)] = &[
    ("cowsay", "3.03+dfsg2-8", "vache configurable qui parle (annonces de la station)"),
    ("snort", "2.9.20-0+deb13u1", "système de détection d'intrusion réseau (IDS)"),
    ("htop", "3.3.0-4", "visualiseur de processus interactif"),
    ("tree", "2.1.1-2", "affiche l'arborescence des dossiers"),
    ("sl", "5.02-1", "locomotive à vapeur ASCII (pour les fautes de frappe)"),
    ("figlet", "2.2.5-3", "grandes lettres en ASCII"),
    ("nginx", "1.26.0-3", "serveur web et proxy inverse"),
    ("apache2", "2.4.62-1", "serveur web Apache"),
    ("tcpdump", "4.99.5-1", "analyseur de trafic réseau en ligne de commande"),
    ("lynis", "3.1.1-1", "audit de sécurité du système"),
    ("rkhunter", "1.4.6-12", "détecteur de rootkits"),
    ("clamav", "1.4.1+dfsg-1", "antivirus"),
    ("nmap", "7.94+git20230807-2", "scanner de ports réseau"),
    ("traceroute", "1:2.1.5-1", "trace la route des paquets"),
    ("mysql-server", "8.0.39-1", "serveur de base de données"),
    ("openssh-server", "1:9.7p1-7", "serveur SSH sécurisé"),
    ("ufw", "0.36.2-6", "pare-feu simplifié"),
    ("fail2ban", "1.1.0-1", "bannit les IP qui attaquent"),
    ("auditd", "1:3.1.2-4", "démon d'audit de sécurité"),
    ("iptables", "1.8.10-4", "filtrage de paquets du noyau"),
    ("sudo", "1.9.15p5-3", "exécute des commandes avec les droits administrateur"),
    ("curl", "8.8.0-4", "client HTTP en ligne de commande"),
    ("nano", "8.1-1", "éditeur de texte simple"),
    ("tar", "1.35+dfsg-3", "archiveur tar"),
    ("gzip", "1.12-1.1", "compression gzip"),
    ("zip", "3.0-14", "archiveur zip"),
    ("unzip", "6.0-28", "désarchiveur zip"),
    ("net-tools", "2.10-1", "outils réseau classiques (netstat, ifconfig)"),
    ("iproute2", "6.10.0-2", "outils réseau modernes (ip, ss)"),
    ("procps", "2:4.0.4-6", "outils de processus (ps, top, kill)"),
    ("dnsutils", "1:9.20.2-1", "outils DNS (dig, nslookup)"),
    ("systemd", "256.7-3", "gestionnaire de système et de services"),
    ("bash", "5.2.32-1", "le shell GNU Bourne Again"),
    ("coreutils", "9.4-3.1", "utilitaires de base GNU"),
    ("base-files", "13.5", "fichiers de base du système Debian"),
    ("linux-image-6.1.0-old", "6.1.99-1", "ancien noyau (inutile)"),
    ("libcorbeau-outils", "0.6.6-1", "bibliothèque laissée par les pirates (inutile)"),
    ("python3-obsolete", "3.9.2-1", "ancienne version de Python (inutile)"),
];

fn pkg(name: &str) -> Option<&'static (&'static str, &'static str, &'static str)> {
    CATALOG.iter().find(|p| p.0 == name)
}

pub fn installed(c: &Ctx, name: &str) -> bool { c.lines("packages.installed").iter().any(|p| p == name) }

pub fn apt(c: &Ctx, a: &[String]) -> i32 {
    let yes = a.iter().any(|x| x == "-y" || x == "--yes" || x == "--assume-yes");
    let words: Vec<&String> = a.iter().filter(|x| !x.starts_with('-')).collect();
    let sub = words.first().map(|s| s.as_str()).unwrap_or("");
    let names: Vec<String> = words.iter().skip(1).map(|s| s.to_string()).collect();
    let lock = || { eprintln!("E: Impossible d'ouvrir le fichier verrou /var/lib/dpkg/lock-frontend - open (13: Permission non accordée)\nE: Impossible d'obtenir le verrou de dpkg (/var/lib/dpkg/lock-frontend). Avez-vous les droits du superutilisateur ?"); 100 };
    let confirm = |msg: &str| -> bool {
        if yes { return true; }
        let r = read_answer(&format!("{}\nSouhaitez-vous continuer ? [O/n] ", msg));
        let ok = r.is_empty() || matches!(r.as_str(), "o" | "O" | "y" | "Y" | "oui");
        if !ok { println!("Annulation."); }
        ok
    };
    let mut inst = c.lines("packages.installed");
    match sub {
        "update" => {
            if !c.is_root() { eprintln!("Lecture des listes de paquets... Fait\nE: Impossible d'ouvrir le fichier verrou /var/lib/apt/lists/lock - open (13: Permission non accordée)"); return 100; }
            println!("Atteint :1 http://depot.andromede.flotte/debian trixie InRelease\nAtteint :2 http://depot.andromede.flotte/debian-security trixie-security InRelease\nLecture des listes de paquets... Fait\nConstruction de l'arbre des dépendances... Fait\nLecture des informations d'état... Fait");
            println!("Tous les paquets sont à jour.");
            c.set_kv("apt.state", "updated", "yes");
            c.event("apt update");
            0
        }
        "upgrade" | "full-upgrade" | "dist-upgrade" => {
            if !c.is_root() { return lock(); }
            println!("Lecture des listes de paquets... Fait\nCalcul de la mise à jour... Fait\n0 mis à jour, 0 nouvellement installés, 0 à enlever et 0 non mis à jour.");
            c.event(&format!("apt {}", sub));
            0
        }
        "install" | "reinstall" => {
            if !c.is_root() { return lock(); }
            if names.is_empty() { println!("0 mis à jour, 0 nouvellement installés, 0 à enlever et 0 non mis à jour."); return 0; }
            for n in &names { if pkg(n).is_none() { eprintln!("Lecture des listes de paquets... Fait\nE: Impossible de trouver le paquet {}", n); return 100; } }
            let new: Vec<&String> = names.iter().filter(|n| !inst.contains(n)).collect();
            println!("Lecture des listes de paquets... Fait\nConstruction de l'arbre des dépendances... Fait");
            for n in names.iter().filter(|n| inst.contains(n)) { println!("{} est déjà la version la plus récente ({}).", n, pkg(n).map(|p| p.1).unwrap_or("")); }
            if new.is_empty() { println!("0 mis à jour, 0 nouvellement installés, 0 à enlever et 0 non mis à jour."); return 0; }
            let list: Vec<&str> = new.iter().map(|s| s.as_str()).collect();
            if !confirm(&format!("Les NOUVEAUX paquets suivants seront installés :\n  {}\n0 mis à jour, {} nouvellement installés, 0 à enlever et 0 non mis à jour.", list.join(" "), new.len())) { return 1; }
            for n in &new {
                let p = pkg(n).unwrap();
                println!("Réception de :1 http://depot.andromede.flotte/debian trixie/main amd64 {} {} [312 ko]", p.0, p.1);
                println!("Dépaquetage de {} ({}) ...\nParamétrage de {} ({}) ...", p.0, p.1, p.0, p.1);
                inst.push(n.to_string());
                c.event(&format!("apt install {}", n));
            }
            c.set_lines("packages.installed", &inst);
            0
        }
        "remove" | "purge" => {
            if !c.is_root() { return lock(); }
            for n in &names {
                if !inst.contains(n) { println!("Le paquet « {} » n'est pas installé, et ne peut donc être supprimé", n); continue; }
                if !confirm(&format!("Les paquets suivants seront ENLEVÉS :\n  {}", n)) { return 1; }
                inst.retain(|x| x != n);
                println!("Suppression de {} ...", n);
                c.event(&format!("apt {} {}", sub, n));
            }
            c.set_lines("packages.installed", &inst);
            0
        }
        "autoremove" => {
            if !c.is_root() { return lock(); }
            let auto: Vec<String> = c.lines("packages.auto").into_iter().filter(|p| inst.contains(p)).collect();
            println!("Lecture des listes de paquets... Fait\nConstruction de l'arbre des dépendances... Fait");
            if auto.is_empty() { println!("0 mis à jour, 0 nouvellement installés, 0 à enlever et 0 non mis à jour."); c.event("apt autoremove"); return 0; }
            if !confirm(&format!("Les paquets suivants seront ENLEVÉS :\n  {}\n0 mis à jour, 0 nouvellement installés, {} à enlever et 0 non mis à jour.", auto.join(" "), auto.len())) { return 1; }
            for p in &auto { println!("Suppression de {} ...", p); }
            inst.retain(|x| !auto.contains(x));
            c.set_lines("packages.installed", &inst);
            c.event("apt autoremove");
            0
        }
        "clean" | "autoclean" => { if !c.is_root() { return lock(); } c.event(&format!("apt {}", sub)); 0 }
        "search" => {
            let q = names.first().cloned().unwrap_or_default();
            println!("En train de trier... Fait\nRecherche en texte intégral... Fait");
            for p in CATALOG.iter().filter(|p| p.0.contains(&q) || p.2.contains(&q)) { println!("{}/stable {} amd64\n  {}\n", p.0, p.1, p.2); }
            0
        }
        "show" | "policy" => {
            for n in &names { match pkg(n) { Some(p) => println!("Package: {}\nVersion: {}\nDescription: {}\nInstallé: {}\n", p.0, p.1, p.2, if inst.contains(n) { "oui" } else { "non" }), None => { eprintln!("N: Impossible de trouver le paquet {}", n); return 100; } } }
            0
        }
        "list" => {
            let only = a.iter().any(|x| x == "--installed");
            println!("En train de lister... Fait");
            for p in CATALOG.iter().filter(|p| !only || inst.iter().any(|i| i == p.0)) { println!("{}/stable {} amd64{}", p.0, p.1, if inst.iter().any(|i| i == p.0) { " [installé]" } else { "" }); }
            0
        }
        _ => { println!("apt 2.9.8 (amd64)\nUtilisation : apt [options] commande\n\n  update - met à jour la liste des paquets\n  install - installe des paquets\n  remove - supprime des paquets\n  autoremove - supprime les paquets inutiles\n  search - cherche un paquet\n  show - détails d'un paquet\n  list - liste les paquets"); if sub.is_empty() { 0 } else { 1 } }
    }
}

pub fn dpkg(c: &Ctx, a: &[String]) -> i32 {
    let inst = c.lines("packages.installed");
    let opt = a.first().map(|s| s.as_str()).unwrap_or("");
    let names: Vec<&String> = a.iter().skip(1).collect();
    match opt {
        "-l" | "--list" => {
            println!("Souhait=inconnU/Installé/suppRimé/Purgé/H=à garder\n| État=Non/Installé/fichier-Config/dépaqUeté/échec-conFig/H=semi-installé\n||/ Nom                     Version              Architecture Description\n+++-=======================-====================-============-=================================");
            for p in CATALOG.iter().filter(|p| inst.iter().any(|i| i == p.0)).filter(|p| names.is_empty() || names.iter().any(|n| p.0.contains(n.trim_matches('*')))) {
                println!("ii  {:<23} {:<20} amd64        {}", p.0, p.1, p.2);
            }
            0
        }
        "-s" | "--status" => {
            let mut rc = 0;
            for n in names {
                match pkg(n) {
                    Some(p) if inst.contains(n) => println!("Package: {}\nStatus: install ok installed\nPriority: optional\nVersion: {}\nDescription: {}\n", p.0, p.1, p.2),
                    _ => { eprintln!("dpkg-query: le paquet « {} » n'est pas installé et aucune information n'est disponible", n); rc = 1; }
                }
            }
            rc
        }
        "-L" | "--listfiles" => {
            let mut rc = 0;
            for n in names { if inst.contains(n) { println!("/usr/bin/{}\n/usr/share/doc/{}\n/usr/share/man/man1/{}.1.gz", n, n, n); } else { eprintln!("dpkg-query: le paquet « {} » n'est pas installé", n); rc = 1; } }
            rc
        }
        "-i" | "--install" => {
            if !c.is_root() { eprintln!("dpkg: erreur: l'opération demandée nécessite les privilèges du superutilisateur"); return 2; }
            eprintln!("dpkg: erreur: impossible d'accéder à l'archive « {} »: Aucun fichier ou dossier de ce type", names.first().map(|s| s.as_str()).unwrap_or(""));
            1
        }
        "--get-selections" => { for p in &inst { println!("{}\t\t\t\t\tinstall", p); } 0 }
        _ => { eprintln!("dpkg: erreur: il faut indiquer une option d'action (-l, -s, -L, -i)"); 2 }
    }
}

/// Outils fournis par un paquet du catalogue: n'existent qu'une fois installés
pub fn installed_tool(c: &Ctx, name: &str, a: &[String]) -> i32 {
    if !installed(c, name) { eprintln!("bash: {} : commande introuvable", name); return 127; }
    match name {
        "cowsay" => {
            let msg = if a.is_empty() { "Meuh!".to_string() } else { a.join(" ") };
            let w = msg.chars().count();
            println!(" {}\n< {} >\n {}\n        \\   ^__^\n         \\  (oo)\\_______\n            (__)\\       )\\/\\\n                ||----w |\n                ||     ||", "_".repeat(w + 2), msg, "-".repeat(w + 2));
            0
        }
        _ => {
            if a.iter().any(|x| x == "-V" || x == "--version") { println!("   ,,_     -*> Snort! <*-\n  o\"  )~   Version 2.9.20 GRE (Build 82)\n   ''''    By Martin Roesch & The Snort Team"); return 0; }
            println!("Snort: sonde IDS prête. Interface eth0 surveillée (simulation). Ctrl+C pour arrêter.");
            0
        }
    }
}

pub fn chroot(c: &Ctx, a: &[String]) -> i32 {
    if !c.is_root() { eprintln!("chroot: impossible de changer le répertoire racine pour « {} »: Opération non permise", a.first().map(|s| s.as_str()).unwrap_or("")); return 125; }
    let dir = match a.iter().find(|x| !x.starts_with('-')) { Some(d) => d.clone(), None => { eprintln!("chroot: opérande manquant"); return 125; } };
    let real = c.real(&dir);
    if !real.is_dir() { eprintln!("chroot: impossible de changer le répertoire racine pour « {} »: Aucun fichier ou dossier de ce type", dir); return 125; }
    let abs = if dir.starts_with('/') { dir.clone() } else {
        let cwd = std::env::current_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
        let s = format!("{}/{}", cwd.trim_end_matches('/'), dir);
        if c.root != "/" { s.strip_prefix(c.root.trim_end_matches('/')).map(|x| x.to_string()).unwrap_or(s) } else { s }
    };
    println!("🔒 Cellule d'isolement activée: le processus est enfermé dans {} et ne voit plus que ses murs.", abs.trim_end_matches('/'));
    println!("   (/ pour lui = {} — il ne peut plus atteindre le reste du système)", abs.trim_end_matches('/'));
    c.event(&format!("chroot {}", abs.trim_end_matches('/')));
    0
}

//! Réseau scénarisé (le bac à sable n'a AUCUN accès réseau réel): ping, ip, ss, netstat,
//! traceroute, nmap, dig, curl, ssh répondent selon les machines du jeu (10.0.0.x).

use super::{Ctx, Profile};
use std::io::Write;

struct HostInfo {
    name: String,
    ip: String,
    up: bool,
    ports: &'static [(u16, &'static str, &'static str)],
    ttl: u8,
}

fn hosts(c: &Ctx) -> Vec<HostInfo> {
    let p = Profile::of(c.camp);
    let me_ports: &'static [(u16, &'static str, &'static str)] = match c.camp {
        3 => &[
            (22, "ssh", "OpenSSH 9.7p1 Debian"),
            (80, "http", "nginx 1.26.0"),
            (3306, "mysql", "MySQL 8.0.39"),
        ],
        _ => &[(22, "ssh", "OpenSSH 9.7p1 Debian")],
    };
    let local = p.ip.to_string();
    let lname = p.host.to_string();
    vec![
        HostInfo {
            name: "localhost".to_string(),
            ip: "127.0.0.1".to_string(),
            up: true,
            ports: me_ports,
            ttl: 64,
        },
        HostInfo {
            name: lname,
            ip: local,
            up: true,
            ports: me_ports,
            ttl: 64,
        },
        HostInfo {
            name: "passerelle".to_string(),
            ip: "10.0.0.1".to_string(),
            up: true,
            ports: &[
                (22, "ssh", "OpenSSH 9.7p1"),
                (53, "domain", "dnsmasq 2.90"),
                (443, "https", "nginx"),
                (8080, "http-proxy", "API statut NOC"),
            ],
            ttl: 64,
        },
        HostInfo {
            name: "nexus-02".to_string(),
            ip: "10.0.0.2".to_string(),
            up: true,
            ports: &[
                (22, "ssh", "OpenSSH 9.7p1 Debian"),
                (9100, "jetdirect", "node_exporter"),
            ],
            ttl: 64,
        },
        HostInfo {
            name: "corbeau-noir".to_string(),
            ip: "10.0.0.66".to_string(),
            up: true,
            ports: &[
                (22, "ssh", "OpenSSH 7.2 (obsolète)"),
                (3306, "mysql", "MySQL 5.5 (exposé!)"),
                (3333, "dec-notes", "pool de minage"),
                (4444, "krb524", "porte dérobée C2"),
            ],
            ttl: 52,
        },
        HostInfo {
            name: "allie".to_string(),
            ip: "10.0.0.99".to_string(),
            up: true,
            ports: &[(22, "ssh", "OpenSSH 9.7p1")],
            ttl: 64,
        },
        HostInfo {
            name: "fai".to_string(),
            ip: "10.0.0.254".to_string(),
            up: true,
            ports: &[(53, "domain", "unbound"), (179, "bgp", "BIRD")],
            ttl: 63,
        },
    ]
}

fn resolve(c: &Ctx, target: &str) -> Option<(String, String, bool, u8)> {
    let t = target.rsplit('@').next().unwrap_or(target);
    let t = t.trim_end_matches('.');
    for h in hosts(c) {
        let short = t.split('.').next().unwrap_or(t);
        if h.ip == t
            || h.name == t
            || h.name == short
            || (t.ends_with(Profile::of(c.camp).domain) && h.name == short)
        {
            return Some((h.name.to_string(), h.ip.to_string(), h.up, h.ttl));
        }
    }
    let alias = match t {
        "andromede" | "andromede.flotte" | "gateway" | "nexus.neon9.local" | "noc.neon9.local" => {
            Some("10.0.0.1")
        }
        "orion7" | "shiro" | "nexus" => Some(Profile::of(c.camp).ip),
        _ => None,
    };
    if let Some(ip) = alias {
        return Some((t.to_string(), ip.to_string(), true, 64));
    }
    if t.starts_with("10.0.0.") && t[7..].parse::<u8>().is_ok() {
        return Some((t.to_string(), t.to_string(), false, 64));
    }
    None
}

pub fn hostname(c: &Ctx, a: &[String]) -> i32 {
    match a.first().map(|s| s.as_str()) {
        None => println!("{}", c.host),
        Some("-f") | Some("--fqdn") => println!("{}.{}", c.host, Profile::of(c.camp).domain),
        Some("-I") | Some("-i") => println!("{}", Profile::of(c.camp).ip),
        Some("-s") => println!("{}", c.host),
        Some(n) => {
            if !c.is_root() {
                eprintln!("hostname: vous devez être root pour modifier le nom d'hôte");
                return 1;
            }
            println!("(le nom d'hôte du jeu reste {}; {} ignoré)", c.host, n);
        }
    }
    0
}

pub fn uname(c: &Ctx, a: &[String]) -> i32 {
    let flags: String = a.iter().map(|s| s.trim_start_matches('-')).collect();
    let all = flags.contains('a') || a.iter().any(|x| x == "--all");
    let mut out: Vec<String> = Vec::new();
    let host = c.host.clone();
    let rel = format!("6.6.7-{}", host);
    if all || flags.contains('s') || flags.is_empty() {
        out.push("Linux".into());
    }
    if all || flags.contains('n') {
        out.push(host.clone());
    }
    if all || flags.contains('r') {
        out.push(rel);
    }
    if all || flags.contains('v') {
        out.push("#1 SMP PREEMPT_DYNAMIC Debian 6.6.7-1 (2024-01-15)".into());
    }
    if all || flags.contains('m') || flags.contains('p') || flags.contains('i') {
        out.push("x86_64".into());
    }
    if all || flags.contains('o') {
        out.push("GNU/Linux".into());
    }
    println!("{}", out.join(" "));
    0
}

pub fn ping(c: &Ctx, a: &[String]) -> i32 {
    let mut count: Option<u32> = None;
    let mut i = 0;
    let mut target = None;
    while i < a.len() {
        let x = a[i].as_str();
        if x == "-c" {
            count = a.get(i + 1).and_then(|s| s.parse().ok());
            i += 2;
            continue;
        }
        if let Some(v) = x.strip_prefix("-c") {
            if !v.is_empty() {
                count = v.parse().ok();
            }
        } else if matches!(x, "-i" | "-W" | "-w" | "-s" | "-t") {
            i += 2;
            continue;
        } else if !x.starts_with('-') {
            target = Some(x.to_string());
        }
        i += 1;
    }
    let target = match target {
        Some(t) => t,
        None => {
            eprintln!("ping: destination d'adresse requise");
            return 1;
        }
    };
    let (name, ip, up, ttl) = match resolve(c, &target) {
        Some(r) => r,
        None => {
            eprintln!("ping: {}: Nom ou service inconnu", target);
            return 2;
        }
    };
    println!("PING {} ({}) 56(84) octets de données.", name, ip);
    crate::sys::catch_sigint();
    let interactive = crate::sys::is_tty(1);
    let delay = std::time::Duration::from_millis(if interactive { 1000 } else { 150 });
    let mut sent = 0u32;
    let mut times = Vec::new();
    loop {
        sent += 1;
        if up {
            let t = 0.031
                + ((sent * 7 + ip.len() as u32) % 13) as f64
                    * if ip == "10.0.0.66" { 3.1 } else { 0.011 };
            times.push(t);
            println!(
                "64 octets de {} : icmp_seq={} ttl={} temps={:.3} ms",
                ip, sent, ttl, t
            );
        } else {
            println!(
                "Depuis {} icmp_seq={} Destination Host Unreachable",
                Profile::of(c.camp).ip,
                sent
            );
        }
        let _ = std::io::stdout().flush();
        if count.map(|n| sent >= n).unwrap_or(false) {
            break;
        }
        let start = std::time::Instant::now();
        while start.elapsed() < delay && !crate::sys::sigint_seen() {
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        if crate::sys::sigint_seen() {
            println!("^C");
            break;
        }
    }
    let recv = times.len() as u32;
    println!("\n--- statistiques ping {} ---", name);
    println!(
        "{} paquets transmis, {} reçus, {}% packet loss, time {}ms",
        sent,
        recv,
        (sent - recv) * 100 / sent.max(1),
        sent * 1000
    );
    if recv > 0 {
        let min = times.iter().cloned().fold(f64::MAX, f64::min);
        let max = times.iter().cloned().fold(0.0, f64::max);
        let avg = times.iter().sum::<f64>() / recv as f64;
        println!(
            "rtt min/avg/max/mdev = {:.3}/{:.3}/{:.3}/{:.3} ms",
            min,
            avg,
            max,
            (max - min) / 2.0
        );
        0
    } else {
        1
    }
}

pub fn ip(c: &Ctx, a: &[String]) -> i32 {
    let p = Profile::of(c.camp);
    let brief = a.iter().any(|x| x == "-br" || x == "-brief");
    let words: Vec<&String> = a.iter().filter(|x| !x.starts_with('-')).collect();
    let obj = words.first().map(|s| s.as_str()).unwrap_or("");
    let is = |full: &str| !obj.is_empty() && full.starts_with(obj);
    if obj.is_empty() {
        eprintln!("Usage: ip [ OPTIONS ] OBJECT {{ COMMAND | help }}\nwhere  OBJECT := {{ address | link | route | neigh | ... }}");
        return 255;
    }
    if is("address") {
        if brief {
            println!("lo               UNKNOWN        127.0.0.1/8 ::1/128\neth0             UP             {}/24", p.ip);
            return 0;
        }
        println!("1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000\n    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00\n    inet 127.0.0.1/8 scope host lo\n       valid_lft forever preferred_lft forever");
        println!("2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000\n    link/ether 52:54:00:07:{:02x}:01 brd ff:ff:ff:ff:ff:ff\n    inet {}/24 brd 10.0.0.255 scope global eth0\n       valid_lft forever preferred_lft forever", c.camp, p.ip);
    } else if is("route") {
        println!("default via 10.0.0.1 dev eth0 proto static\n10.0.0.0/24 dev eth0 proto kernel scope link src {}", p.ip);
    } else if is("link") {
        println!("1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN mode DEFAULT group default qlen 1000\n    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00");
        println!("2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP mode DEFAULT group default qlen 1000\n    link/ether 52:54:00:07:{:02x}:01 brd ff:ff:ff:ff:ff:ff", c.camp);
    } else if is("neighbour") {
        println!("10.0.0.1 dev eth0 lladdr 52:54:00:00:00:01 REACHABLE\n10.0.0.2 dev eth0 lladdr 52:54:00:00:00:02 STALE\n10.0.0.66 dev eth0 lladdr de:ad:be:ef:00:66 REACHABLE\n10.0.0.254 dev eth0 lladdr 52:54:00:00:00:fe STALE");
    } else {
        eprintln!("Object \"{}\" is unknown, try \"ip help\".", obj);
        return 1;
    }
    0
}

pub fn ifconfig(c: &Ctx) -> i32 {
    let p = Profile::of(c.camp);
    println!("eth0: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500\n        inet {}  netmask 255.255.255.0  broadcast 10.0.0.255\n        ether 52:54:00:07:{:02x}:01  txqueuelen 1000  (Ethernet)\n\nlo: flags=73<UP,LOOPBACK,RUNNING>  mtu 65536\n        inet 127.0.0.1  netmask 255.0.0.0\n        loop  txqueuelen 1000  (Boucle locale)", p.ip, c.camp);
    0
}

pub fn ss(c: &Ctx, a: &[String], netstat: bool) -> i32 {
    let p = Profile::of(c.camp);
    let flags: String = a
        .iter()
        .filter(|x| x.starts_with('-'))
        .map(|s| s.trim_start_matches('-'))
        .collect();
    let listen = flags.contains('l');
    let established = a.iter().any(|x| x == "established");
    let procs = flags.contains('p');
    let mut listening: Vec<(&str, u16, &str)> = vec![
        ("0.0.0.0", 22, "sshd,pid=812"),
        ("127.0.0.1", 25, "exim4,pid=640"),
    ];
    if c.camp == 3 {
        listening.push(("0.0.0.0", 80, "nginx,pid=2222"));
        listening.push(("0.0.0.0", 3306, "mysqld,pid=3306"));
    }
    let backdoor = match c.camp {
        1 => Some((4444u16, "backdoor,pid=9999")),
        _ => None,
    };
    if let Some((port, prc)) = backdoor {
        if super::procs::running(c, "9999") {
            listening.push(("0.0.0.0", port, prc));
        }
    }
    let estab: Vec<(String, String, &str)> = vec![
        (
            format!("{}:22", p.ip),
            "10.0.0.1:40022".into(),
            "sshd,pid=1500",
        ),
        (
            format!("{}:22", p.ip),
            "10.0.0.66:51533".into(),
            match c.camp {
                2 => "sshd,pid=6667",
                3 => "sshd,pid=6670",
                _ => "sshd,pid=9998",
            },
        ),
    ];
    if netstat {
        if flags.contains('r') {
            println!("Table de routage IP du noyau\nDestination     Passerelle      Genmask         Indic   MSS Fenêtre irtt Iface\n0.0.0.0         10.0.0.1        0.0.0.0         UG        0 0          0 eth0\n10.0.0.0        0.0.0.0         255.255.255.0   U         0 0          0 eth0");
            return 0;
        }
        println!(
            "Connexions Internet actives {}",
            if listen {
                "(seulement serveurs)"
            } else {
                "(serveurs et établies)"
            }
        );
        println!(
            "Proto Recv-Q Send-Q Adresse locale          Adresse distante        Etat       {}",
            if procs { "PID/Program name" } else { "" }
        );
        for (addr, port, prc) in &listening {
            let pr = if procs {
                prc.replace(",pid=", "/")
                    .split('/')
                    .rev()
                    .collect::<Vec<_>>()
                    .join("/")
            } else {
                String::new()
            };
            println!(
                "tcp        0      0 {:<23} 0.0.0.0:*               LISTEN     {}",
                format!("{}:{}", addr, port),
                pr
            );
        }
        if !listen {
            for (l, r, prc) in &estab {
                println!(
                    "tcp        0     36 {:<23} {:<23} ESTABLISHED {}",
                    l,
                    r,
                    if procs {
                        prc.to_string()
                    } else {
                        String::new()
                    }
                );
            }
        }
        return 0;
    }
    if flags.contains('s') && a.len() == 1 {
        println!(
            "Total: 214\nTCP:   {} (estab 2, closed 0, orphaned 0, timewait 0)",
            listening.len() + 2
        );
        return 0;
    }
    println!("State      Recv-Q Send-Q Local Address:Port    Peer Address:Port Process");
    if listen
        || (!established && !flags.contains('t') && !flags.contains('u'))
        || (!established && listen)
    {
        for (addr, port, prc) in &listening {
            println!(
                "LISTEN     0      128    {:<21} {:<17} {}",
                format!("{}:{}", addr, port),
                "0.0.0.0:*",
                if procs {
                    format!("users:((\"{}\"))", prc.replace(",pid=", "\",pid="))
                } else {
                    String::new()
                }
            );
        }
    }
    if !listen || established {
        for (l, r, prc) in &estab {
            println!(
                "ESTAB      0      36     {:<21} {:<17} {}",
                l,
                r,
                if procs {
                    format!("users:((\"{}\"))", prc.replace(",pid=", "\",pid="))
                } else {
                    String::new()
                }
            );
        }
    }
    0
}

pub fn traceroute(c: &Ctx, a: &[String]) -> i32 {
    let target = match a.iter().find(|x| !x.starts_with('-')) {
        Some(t) => t.clone(),
        None => {
            eprintln!("Usage: traceroute [ -46dFITnreAUDV ] host");
            return 2;
        }
    };
    let (name, ip, up, _) = match resolve(c, &target) {
        Some(r) => r,
        None => {
            eprintln!("{}: Nom ou service inconnu\nCannot handle \"host\" cmdline arg `{}' on position 1 (argc 1)", target, target);
            return 2;
        }
    };
    println!(
        "traceroute to {} ({}), 30 hops max, 60 byte packets",
        name, ip
    );
    println!(" 1  passerelle (10.0.0.1)  0.412 ms  0.388 ms  0.371 ms");
    if ip == "10.0.0.1" {
        return 0;
    }
    if ip == "10.0.0.66" {
        println!(" 2  fai (10.0.0.254)  2.104 ms  2.087 ms  2.071 ms");
        println!(" 3  relais-sombre.secteur7g (172.16.66.1)  18.442 ms  18.410 ms  18.398 ms");
        println!(" 4  * * *");
    }
    if up {
        println!(
            " {}  {} ({})  {} ms",
            if ip == "10.0.0.66" { 5 } else { 2 },
            name,
            ip,
            if ip == "10.0.0.66" { "31.337" } else { "0.512" }
        );
        0
    } else {
        println!(" 2  * * *\n 3  * * *");
        0
    }
}

pub fn nmap(c: &Ctx, a: &[String]) -> i32 {
    let syn = a.iter().any(|x| x == "-sS");
    if syn && !c.is_root() {
        eprintln!("You requested a scan type which requires root privileges.\nQUITTING!");
        return 1;
    }
    let ports_opt: Option<Vec<u16>> = a
        .iter()
        .position(|x| x == "-p")
        .and_then(|i| a.get(i + 1))
        .or_else(|| a.iter().find(|x| x.starts_with("-p") && x.len() > 2))
        .map(|s| {
            let s = s.trim_start_matches("-p");
            let mut v = Vec::new();
            for part in s.split(',') {
                if let Some((x, y)) = part.split_once('-') {
                    if let (Ok(x), Ok(y)) = (x.parse::<u16>(), y.parse::<u16>()) {
                        for p in x..=y.min(x.saturating_add(2000)) {
                            v.push(p);
                        }
                    }
                } else if let Ok(p) = part.parse() {
                    v.push(p);
                }
            }
            v
        });
    let skip_next = ["-p", "-oN", "-oG", "-T"];
    let mut target = None;
    for (i, x) in a.iter().enumerate() {
        if x.starts_with('-') {
            continue;
        }
        if i > 0 && skip_next.contains(&a[i - 1].as_str()) {
            continue;
        }
        target = Some(x.clone());
    }
    let target = match target {
        Some(t) => t,
        None => {
            eprintln!("WARNING: No targets were specified, so 0 hosts scanned.");
            return 1;
        }
    };
    let version = a.iter().any(|x| x == "-sV" || x == "-A");
    println!(
        "Starting Nmap 7.94SVN ( https://nmap.org ) at {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M %Z")
    );
    let hs = hosts(c);
    let (name, ip, up, _) = match resolve(c, &target) {
        Some(r) => r,
        None => {
            println!(
                "Failed to resolve \"{}\".\nNmap done: 0 IP addresses (0 hosts up) scanned",
                target
            );
            return 0;
        }
    };
    if !up {
        println!(
            "Note: Host seems down.\nNmap done: 1 IP address (0 hosts up) scanned in 3.04 seconds"
        );
        return 0;
    }
    let open: Vec<(u16, &str, &str)> = hs
        .iter()
        .find(|h| h.ip == ip)
        .map(|h| h.ports.to_vec())
        .unwrap_or_default();
    println!(
        "Nmap scan report for {} ({})\nHost is up (0.00{}s latency).",
        name,
        ip,
        ip.len()
    );
    let wanted: Vec<u16> = ports_opt
        .clone()
        .unwrap_or_else(|| open.iter().map(|p| p.0).collect());
    if ports_opt.is_none() {
        println!("Not shown: 996 closed tcp ports (reset)");
    }
    println!(
        "PORT     STATE  SERVICE{}",
        if version { "     VERSION" } else { "" }
    );
    for port in wanted {
        match open.iter().find(|p| p.0 == port) {
            Some((p, s, v)) => println!(
                "{:<8} open   {:<11}{}",
                format!("{}/tcp", p),
                s,
                if version {
                    v.to_string()
                } else {
                    String::new()
                }
            ),
            None => println!("{:<8} closed unknown", format!("{}/tcp", port)),
        }
    }
    println!(
        "\nNmap done: 1 IP address (1 host up) scanned in 0.{}2 seconds",
        ip.len()
    );
    0
}

pub fn dig(c: &Ctx, a: &[String], tool: &str) -> i32 {
    let short = a.iter().any(|x| x == "+short");
    let name = match a.iter().find(|x| {
        !x.starts_with('+')
            && !x.starts_with('-')
            && !x.starts_with('@')
            && !matches!(x.as_str(), "A" | "MX" | "NS" | "AAAA" | "TXT" | "ANY")
    }) {
        Some(n) => n.clone(),
        None => {
            if tool == "dig" {
                println!("; <<>> DiG 9.20.2 <<>>\n;; global options: +cmd");
                return 0;
            }
            eprintln!("Usage: {} nom", tool);
            return 1;
        }
    };
    let known: &[(&str, &str)] = &[
        ("nexus.neon9.local", "10.0.0.9"),
        ("noc.neon9.local", "10.0.0.9"),
        ("cluster.neon9.local", "10.0.0.2"),
        ("nexus-02.neon9.local", "10.0.0.2"),
        ("api.neon9.local", "10.0.0.1"),
        ("gw.neon9.local", "10.0.0.1"),
        ("andromede.flotte", "10.0.0.1"),
        ("orion7.flotte", "10.0.0.7"),
        ("shiro.clan", "10.0.0.10"),
        ("corbeau-noir.secteur7g", "10.0.0.66"),
        ("localhost", "127.0.0.1"),
    ];
    let ip = known
        .iter()
        .find(|(n, _)| *n == name.trim_end_matches('.'))
        .map(|x| x.1.to_string())
        .or_else(|| resolve(c, &name).map(|r| r.1));
    match tool {
        "host" => match &ip {
            Some(ip) => {
                println!("{} has address {}", name, ip);
                0
            }
            None => {
                println!("Host {} not found: 3(NXDOMAIN)", name);
                1
            }
        },
        "nslookup" => match &ip {
            Some(ip) => {
                println!(
                    "Server:\t\t10.0.0.1\nAddress:\t10.0.0.1#53\n\nName:\t{}\nAddress: {}",
                    name, ip
                );
                0
            }
            None => {
                println!("** server can't find {}: NXDOMAIN", name);
                1
            }
        },
        _ => {
            if short {
                if let Some(ip) = &ip {
                    println!("{}", ip);
                }
                return 0;
            }
            println!("; <<>> DiG 9.20.2-1-Debian <<>> {}\n;; global options: +cmd\n;; Got answer:\n;; ->>HEADER<<- opcode: QUERY, status: {}, id: 4242\n\n;; QUESTION SECTION:\n;{}.\t\t\tIN\tA\n", name, if ip.is_some() { "NOERROR" } else { "NXDOMAIN" }, name);
            if let Some(ip) = &ip {
                println!(";; ANSWER SECTION:\n{}.\t300\tIN\tA\t{}\n", name, ip);
            }
            println!(";; Query time: 1 msec\n;; SERVER: 10.0.0.1#53(10.0.0.1) (UDP)\n;; MSG SIZE  rcvd: 62");
            0
        }
    }
}

pub fn curl(c: &Ctx, a: &[String], wget: bool) -> i32 {
    let silent = a
        .iter()
        .any(|x| x == "-s" || x == "--silent" || x == "-sS" || x == "-q");
    let head = a.iter().any(|x| x == "-I" || x == "--head");
    let out = a
        .iter()
        .position(|x| x == "-o" || x == "-O" && wget)
        .and_then(|i| a.get(i + 1))
        .cloned();
    let url = match a
        .iter()
        .find(|x| x.contains("://") || (!x.starts_with('-') && x.contains('.')))
    {
        Some(u) => u.clone(),
        None => {
            eprintln!(
                "{}: try '{} --help' for more information",
                if wget { "wget" } else { "curl" },
                if wget { "wget" } else { "curl" }
            );
            return 2;
        }
    };
    let rest = url.split("://").nth(1).unwrap_or(&url).to_string();
    let (hostport, path) = match rest.split_once('/') {
        Some((h, p)) => (h.to_string(), format!("/{}", p)),
        None => (rest.clone(), "/".into()),
    };
    let host = hostport.split(':').next().unwrap_or("").to_string();
    let port = hostport
        .split(':')
        .nth(1)
        .unwrap_or(if url.starts_with("https") {
            "443"
        } else {
            "80"
        })
        .to_string();
    let r = resolve(c, &host);
    let body: Option<String> = match (r.as_ref().map(|x| x.1.as_str()), port.as_str(), path.as_str()) {
        (Some("10.0.0.1"), "8080", "/status") => Some(format!("{{\"noc\":\"neon-9\",\"status\":\"DEGRADED\",\"cluster\":\"nexus-02 OK\",\"intrusion\":\"10.0.0.66\",\"firewall\":\"{}\",\"code\":\"NOC-API-8080\"}}\n", if c.kv("ufw.conf", "enabled").as_deref() == Some("yes") { "active" } else { "inactive" })),
        (Some("10.0.0.1"), "8080", _) => Some("{\"error\":\"not found\",\"hint\":\"/status\"}\n".into()),
        (Some("10.0.0.1"), "80" | "443", _) => Some("<html><body><h1>Passerelle de la flotte</h1><p>Accès réservé.</p></body></html>\n".into()),
        (Some("127.0.0.1"), "80", _) | (Some("10.0.0.9"), "80", _) if c.camp == 3 => Some("<html><body><h1>Welcome to nginx!</h1></body></html>\n".into()),
        _ => None,
    };
    let body = match body {
        Some(b) => b,
        None => {
            if !silent {
                eprintln!(
                    "{}",
                    if r.is_none() {
                        format!("curl: (6) Could not resolve host: {}", host)
                    } else {
                        format!("curl: (7) Failed to connect to {} port {} after 3 ms: Couldn't connect to server", host, port)
                    }
                );
            }
            return if r.is_none() { 6 } else { 7 };
        }
    };
    let status = if body.contains("not found") {
        "404 Not Found"
    } else {
        "200 OK"
    };
    if head {
        println!(
            "HTTP/1.1 {}\nServer: nginx\nContent-Type: {}\nContent-Length: {}\n",
            status,
            if body.starts_with('{') {
                "application/json"
            } else {
                "text/html"
            },
            body.len()
        );
        return 0;
    }
    let target_file = if wget {
        out.or_else(|| {
            Some(
                path.rsplit('/')
                    .next()
                    .filter(|s| !s.is_empty())
                    .unwrap_or("index.html")
                    .to_string(),
            )
        })
    } else {
        out
    };
    match target_file {
        Some(f) => {
            if std::fs::write(c.real(&f), &body).is_err() {
                eprintln!("Impossible d'écrire {}", f);
                return 23;
            }
            if wget {
                println!("« {} » sauvegardé [{}/{}]", f, body.len(), body.len());
            } else if !silent {
                eprintln!(
                    "  % Total    % Received\n100   {}  100   {}",
                    body.len(),
                    body.len()
                );
            }
        }
        None => print!("{}", body),
    }
    0
}

pub fn ssh(c: &Ctx, a: &[String]) -> i32 {
    let mut i = 0;
    let mut dest: Option<String> = None;
    let mut cmd: Vec<String> = Vec::new();
    while i < a.len() {
        let x = &a[i];
        if dest.is_none() {
            if matches!(
                x.as_str(),
                "-p" | "-i" | "-l" | "-o" | "-F" | "-J" | "-L" | "-R"
            ) {
                i += 2;
                continue;
            }
            if x.starts_with('-') {
                i += 1;
                continue;
            }
            dest = Some(x.clone());
        } else {
            cmd.push(x.clone());
        }
        i += 1;
    }
    let dest = match dest {
        Some(d) => d,
        None => {
            eprintln!("usage: ssh [-p port] [utilisateur@]hôte [commande]");
            return 255;
        }
    };
    let user = if dest.contains('@') {
        dest.split('@').next().unwrap_or("").to_string()
    } else {
        c.me().to_string()
    };
    let host = dest.rsplit('@').next().unwrap_or(&dest).to_string();
    let (name, ip, up, _) = match resolve(c, &host) {
        Some(r) => r,
        None => {
            eprintln!(
                "ssh: Could not resolve hostname {}: Nom ou service inconnu",
                host
            );
            return 255;
        }
    };
    if !up {
        eprintln!("ssh: connect to host {} port 22: No route to host", host);
        return 255;
    }
    if ip == "10.0.0.66" {
        eprintln!("ssh: connect to host {} port 22: Connection refused (le corbeau ne vous laissera pas entrer)", host);
        return 255;
    }
    let remote = if ip == "127.0.0.1" || ip == Profile::of(c.camp).ip {
        c.host.clone()
    } else {
        name.clone()
    };
    eprintln!(
        "🔐 Canal chiffré établi avec {} ({}) — clé ED25519 vérifiée.",
        remote, ip
    );
    let env: Vec<(&str, String)> = vec![
        ("GSH_HOSTNAME", remote.clone()),
        ("GSH_EUID", user.clone()),
        ("USER", user.clone()),
    ];
    if !cmd.is_empty() {
        return super::run_shell(&cmd.join(" "), &env);
    }
    println!("Linux {} 6.6.7-{} #1 SMP PREEMPT_DYNAMIC x86_64\n\nLast login: Mon Jan 15 08:15:42 2024 from 10.0.0.1", remote, remote);
    if crate::sys::is_tty(0) {
        let rc = super::run_as(
            &["/bin/bash".into(), "--norc".into(), "-i".into()],
            &[env, vec![("PS1", format!("{}@{}:\\w$ ", user, remote))]].concat(),
        );
        println!("Connection to {} closed.", host);
        return rc;
    }
    // non interactif: la session lit (et ignore) l'entrée puis se ferme
    let mut s = String::new();
    while std::io::stdin().read_line(&mut s).unwrap_or(0) > 0 {
        if s.trim() == "exit" {
            break;
        }
        s.clear();
    }
    println!("logout\nConnection to {} closed.", host);
    0
}

#[cfg(test)]
mod tests {
    use super::{resolve, Ctx};

    fn ctx(camp: u8) -> Ctx {
        use super::Profile;
        let p = Profile::of(camp);
        Ctx {
            root: "/".into(),
            etc: "/etc".into(),
            camp,
            user: p.user.into(),
            euid: p.user.into(),
            host: p.host.into(),
            homebase: p.homebase.into(),
        }
    }

    #[test]
    fn resolve_noms_et_adresses_connus() {
        let c = ctx(1);
        let (n, ip, up, ttl) = resolve(&c, "localhost").unwrap();
        assert_eq!(
            (n.as_str(), ip.as_str(), up, ttl),
            ("localhost", "127.0.0.1", true, 64)
        );
        let (n, ip, ..) = resolve(&c, "nexus-02").unwrap();
        assert_eq!((n.as_str(), ip.as_str()), ("nexus-02", "10.0.0.2"));
        let (n, ip, ..) = resolve(&c, "10.0.0.66").unwrap();
        assert_eq!((n.as_str(), ip.as_str()), ("corbeau-noir", "10.0.0.66"));
        // le nom propre des camnaines locales est résolu par nom
        let (n, ip, ..) = resolve(&c, "orion7").unwrap();
        assert_eq!((n.as_str(), ip.as_str()), ("orion7", "10.0.0.7"));
    }

    #[test]
    fn resolve_suffixe_domaine_et_at_prefixe() {
        let c = ctx(1);
        // suffixe de domaine : le nom court est retenu
        let (n, ip, ..) = resolve(&c, "passerelle.orion7.flotte").unwrap();
        assert_eq!((n.as_str(), ip.as_str()), ("passerelle", "10.0.0.1"));
        // préfixe utilisateur@ (ssh) : le host compte
        let (n, ip, ..) = resolve(&c, "netrunner@passerelle.orion7.flotte").unwrap();
        assert_eq!((n.as_str(), ip.as_str()), ("passerelle", "10.0.0.1"));
        // point final : normalisé
        let (n, ..) = resolve(&c, "passerelle.").unwrap();
        assert_eq!(n, "passerelle");
    }

    #[test]
    fn resolve_alias_et_host_inconnu() {
        let c = ctx(1);
        // alias « gateway » → passerelle
        let (_, ip, up, ..) = resolve(&c, "gateway").unwrap();
        assert_eq!((ip.as_str(), up), ("10.0.0.1", true));
        // hôte privé connu mais inconnu du jeu → down (host 10.0.0.x)
        let (n, ..) = resolve(&c, "10.0.0.200").unwrap();
        assert_eq!(n, "10.0.0.200");
        // IP invalide
        assert!(resolve(&c, "10.0.0.300").is_none());
        // absent complet
        assert!(resolve(&c, "inconnu.com").is_none());
        assert!(resolve(&c, "").is_none());
    }
}

#[cfg(test)]
mod hosts_tests {
    use super::{hosts, Ctx, Profile};

    fn ctx(camp: u8) -> Ctx {
        let p = Profile::of(camp);
        Ctx {
            root: "/".into(),
            etc: "/etc".into(),
            camp,
            user: p.user.into(),
            euid: p.user.into(),
            host: p.host.into(),
            homebase: p.homebase.into(),
        }
    }

    #[test]
    fn hosts_ports_par_campagne() {
        // campagnes 1 et 2: 1 port local (ssh) au localhost et à l'hôte local
        for camp in [1u8, 2] {
            let hs = hosts(&ctx(camp));
            let local = hs.iter().find(|h| h.ip == Profile::of(camp).ip).unwrap();
            assert_eq!(
                local.ports.len(),
                1,
                "campagne {}: un seul port attendu",
                camp
            );
            assert_eq!(
                hs.iter()
                    .find(|h| h.name == "localhost")
                    .unwrap()
                    .ports
                    .len(),
                1
            );
        }
        // campagne 3: 3 ports (ssh, http, mysql) sur localhost et hôte local
        let hs = hosts(&ctx(3));
        let local = hs.iter().find(|h| h.ip == Profile::of(3).ip).unwrap();
        assert_eq!(local.ports.len(), 3);
        assert_eq!(
            hs.iter()
                .find(|h| h.name == "localhost")
                .unwrap()
                .ports
                .len(),
            3
        );
        // les autres serveurs sont stables
        assert!(hs.iter().any(|h| h.name == "passerelle"));
        assert!(hs.iter().any(|h| h.name == "nexus-02"));
        assert!(hs.iter().any(|h| h.name == "corbeau-noir"));
        // l'hôte local porte son propre ip (campagne 3: 10.0.0.9)
        assert_eq!(local.ip, "10.0.0.9");
    }
}

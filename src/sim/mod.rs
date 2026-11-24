//! Simulateurs des commandes système (P0.2): AUCUNE commande d'administration ne touche
//! la machine hôte. Le binaire du jeu est « multi-appel » (comme busybox): installé dans
//! le PATH du bac à sable sous les noms useradd, ufw, systemctl, ps, kill, ping, ssh...,
//! il lit/écrit un état fictif propre à chaque campagne dans <systeme>/etc/ (passwd, group,
//! shadow) et <systeme>/etc/gsh/ (processus, services, pare-feu, paquets, audit...).

pub mod admin;
pub mod files;
pub mod net;
pub mod procs;
pub mod users;

use std::io::{BufRead, Write};
use std::path::PathBuf;

/// Noms sous lesquels le binaire est installé dans le PATH du jeu
pub const NAMES: &[&str] = &[
    "sudo",
    "su",
    "whoami",
    "id",
    "groups",
    "logname",
    "hostname",
    "uname",
    "who",
    "w",
    "ps",
    "pgrep",
    "pkill",
    "killall",
    "kill",
    "top",
    "systemctl",
    "service",
    "journalctl",
    "ufw",
    "iptables",
    "fail2ban-client",
    "auditctl",
    "apt",
    "apt-get",
    "dpkg",
    "cowsay",
    "snort",
    "chroot",
    "mount",
    "df",
    "findmnt",
    "useradd",
    "adduser",
    "userdel",
    "groupadd",
    "groupdel",
    "usermod",
    "gpasswd",
    "passwd",
    "chown",
    "chgrp",
    "ping",
    "ip",
    "ifconfig",
    "ss",
    "netstat",
    "traceroute",
    "nmap",
    "dig",
    "host",
    "nslookup",
    "curl",
    "wget",
    "ssh",
    "ls",
    "find",
    "nano",
    "zip",
    "unzip",
];

pub fn is_sim(name: &str) -> bool {
    NAMES.contains(&name)
}

/// Contexte d'exécution d'un simulateur (variables posées par le moteur)
#[derive(Clone)]
pub struct Ctx {
    pub root: String, // racine du monde ("/" dans le bac à sable)
    pub etc: String,  // /etc du jeu
    pub camp: u8,
    pub user: String, // compte du joueur (capitaine / seigneur / netrunner)
    pub euid: String, // utilisateur effectif (root sous sudo, autre sous su)
    pub host: String,
    pub homebase: String, // /cabines, /quartiers, /noc
}

impl Ctx {
    pub fn from_env() -> Ctx {
        let camp: u8 = std::env::var("GSH_CAMPAIGN")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        let p = Profile::of(camp);
        let user = std::env::var("GSH_USER").unwrap_or_else(|_| p.user.into());
        Ctx {
            root: std::env::var("GSH_ROOT").unwrap_or_else(|_| "/".into()),
            etc: std::env::var("GSH_ETC").unwrap_or_else(|_| "/etc".into()),
            camp,
            euid: std::env::var("GSH_EUID").unwrap_or_else(|_| user.clone()),
            user,
            host: std::env::var("GSH_HOSTNAME").unwrap_or_else(|_| p.host.into()),
            homebase: p.homebase.into(),
        }
    }

    /// Contexte côté moteur (checks), chemins de l'hôte
    pub fn host(camp: u8, world: &str, etc: &str) -> Ctx {
        let p = Profile::of(camp);
        Ctx {
            root: world.into(),
            etc: etc.into(),
            camp,
            user: p.user.into(),
            euid: p.user.into(),
            host: p.host.into(),
            homebase: p.homebase.into(),
        }
    }

    pub fn is_root(&self) -> bool {
        self.euid == "root"
    }

    /// Chemin réel d'un chemin du jeu (absolu ou relatif au dossier courant)
    pub fn real(&self, p: &str) -> PathBuf {
        if let Some(rest) = p.strip_prefix("/etc") {
            if rest.is_empty() || rest.starts_with('/') {
                return PathBuf::from(format!("{}{}", self.etc, rest));
            }
        }
        if p.starts_with('/') && self.root != "/" {
            return PathBuf::from(format!("{}{}", self.root.trim_end_matches('/'), p));
        }
        PathBuf::from(p)
    }

    pub fn state(&self, name: &str) -> PathBuf {
        PathBuf::from(format!("{}/gsh/{}", self.etc, name))
    }
    pub fn etc_file(&self, name: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}", self.etc, name))
    }

    pub fn lines(&self, name: &str) -> Vec<String> {
        read_lines(&self.state(name))
    }
    pub fn set_lines(&self, name: &str, l: &[String]) {
        write_lines(&self.state(name), l)
    }
    pub fn kv(&self, name: &str, key: &str) -> Option<String> {
        self.lines(name)
            .iter()
            .find_map(|l| l.strip_prefix(&format!("{}=", key)).map(|s| s.to_string()))
    }
    pub fn set_kv(&self, name: &str, key: &str, val: &str) {
        let mut l = self.lines(name);
        let pre = format!("{}=", key);
        match l.iter_mut().find(|x| x.starts_with(&pre)) {
            Some(x) => *x = format!("{}{}", pre, val),
            None => l.push(format!("{}{}", pre, val)),
        }
        self.set_lines(name, &l);
    }
    /// Journal des actions d'administration (utilisé par les checks `sim_event`)
    pub fn event(&self, e: &str) {
        let mut l = self.lines("events.log");
        l.push(e.to_string());
        self.set_lines("events.log", &l);
    }
    /// Entrée du journal système (journalctl)
    pub fn journal(&self, unit: &str, msg: &str) {
        let mut l = self.lines("journal");
        let ts = chrono::Local::now().format("%b %d %H:%M:%S").to_string();
        l.push(format!("{} {} {}: {}", ts, self.host, unit, msg));
        self.set_lines("journal", &l);
    }
    pub fn me(&self) -> &str {
        &self.euid
    }
}

pub fn read_lines(p: &std::path::Path) -> Vec<String> {
    std::fs::read_to_string(p)
        .map(|s| s.lines().map(|l| l.to_string()).collect())
        .unwrap_or_default()
}

pub fn write_lines(p: &std::path::Path, l: &[String]) {
    if let Some(d) = p.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let mut s = l.join("\n");
    if !s.is_empty() {
        s.push('\n');
    }
    let _ = std::fs::write(p, s);
}

/// Paramètres d'une campagne
pub struct Profile {
    pub user: &'static str,
    pub host: &'static str,
    pub homebase: &'static str,
    pub ip: &'static str,
    pub groups: [(&'static str, u32); 2],
    pub domain: &'static str,
}

impl Profile {
    pub fn of(camp: u8) -> Profile {
        match camp {
            2 => Profile {
                user: "seigneur",
                host: "shiro",
                homebase: "/quartiers",
                ip: "10.0.0.10",
                groups: [("samourai", 1001), ("gardes", 1002)],
                domain: "shiro.clan",
            },
            3 => Profile {
                user: "netrunner",
                host: "nexus",
                homebase: "/noc",
                ip: "10.0.0.9",
                groups: [("netops", 1001), ("securite", 1002)],
                domain: "neon9.local",
            },
            _ => Profile {
                user: "capitaine",
                host: "orion7",
                homebase: "/cabines",
                ip: "10.0.0.7",
                groups: [("equipage_pont", 1001), ("securite", 1002)],
                domain: "orion7.flotte",
            },
        }
    }
}

/// Hachage NON cryptographique des mots de passe du jeu (FNV-1a): c'est un jeu,
/// rien n'est un vrai secret. Évite simplement de stocker le mot de passe en clair.
pub fn hash_pw(pw: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in pw.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("$gsh${:016x}", h)
}

/// Lit une ligne sur stdin (sans écho si terminal)
pub fn read_secret(prompt: &str) -> Option<String> {
    eprint!("{}", prompt);
    let _ = std::io::stderr().flush();
    let tty = crate::sys::is_tty(0);
    let saved = if tty {
        crate::sys::get_termios(0)
    } else {
        None
    };
    if let Some(t) = &saved {
        crate::sys::set_termios(0, &crate::sys::noecho_from(t));
    }
    let mut s = String::new();
    let n = std::io::stdin().lock().read_line(&mut s).unwrap_or(0);
    if let Some(t) = &saved {
        crate::sys::set_termios(0, t);
    }
    if tty {
        eprintln!();
    }
    if n == 0 {
        None
    } else {
        Some(s.trim_end_matches(['\n', '\r']).to_string())
    }
}

pub fn read_answer(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = std::io::stdout().flush();
    let mut s = String::new();
    let _ = std::io::stdin().lock().read_line(&mut s);
    s.trim().to_string()
}

/// Exécute une ligne de commande dans le monde, avec une identité/environnement modifiés
pub fn run_as(args: &[String], env: &[(&str, String)]) -> i32 {
    if args.is_empty() {
        return 0;
    }
    let mut c = std::process::Command::new(&args[0]);
    c.args(&args[1..]);
    for (k, v) in env {
        c.env(k, v);
    }
    match c.status() {
        Ok(s) => s.code().unwrap_or(1),
        Err(_) => {
            eprintln!("{}: {} : commande introuvable", args[0], args[0]);
            127
        }
    }
}

pub fn run_shell(cmd: &str, env: &[(&str, String)]) -> i32 {
    run_as(
        &[
            "/bin/bash".to_string(),
            "-c".to_string(),
            format!("enable -n kill 2>/dev/null; {}", cmd),
        ],
        env,
    )
}

/// Chemin de la vraie commande (hors simulateurs), ou None
pub fn real_bin(name: &str) -> Option<String> {
    for d in ["/usr/bin", "/bin", "/usr/sbin", "/sbin"] {
        let p = format!("{}/{}", d, name);
        if std::path::Path::new(&p).is_file() {
            return Some(p);
        }
    }
    None
}

pub fn exec_real(name: &str, args: &[String]) -> i32 {
    match real_bin(name) {
        Some(p) => {
            use std::os::unix::process::CommandExt;
            let e = std::process::Command::new(&p).arg0(name).args(args).exec();
            eprintln!("{}: {}", name, e);
            126
        }
        None => {
            eprintln!("bash: {} : commande introuvable", name);
            127
        }
    }
}

/// Message standard « droits root requis »
pub fn need_root(cmd: &str) -> i32 {
    eprintln!(
        "{}: permission refusée (droits administrateur requis — essayez avec sudo)",
        cmd
    );
    1
}

/// Point d'entrée multi-appel
pub fn run(name: &str, args: Vec<String>) -> i32 {
    let ctx = Ctx::from_env();
    let a = args;
    match name {
        "sudo" => users::sudo(&ctx, &a),
        "su" => users::su(&ctx, &a),
        "whoami" => {
            println!("{}", ctx.me());
            0
        }
        "logname" => {
            println!("{}", ctx.user);
            0
        }
        "id" => users::id(&ctx, &a),
        "groups" => users::groups(&ctx, &a),
        "useradd" | "adduser" => users::useradd(&ctx, &a, name == "adduser"),
        "userdel" => users::userdel(&ctx, &a),
        "groupadd" => users::groupadd(&ctx, &a),
        "groupdel" => users::groupdel(&ctx, &a),
        "usermod" => users::usermod(&ctx, &a),
        "gpasswd" => users::gpasswd(&ctx, &a),
        "passwd" => users::passwd(&ctx, &a),
        "chown" | "chgrp" => users::chown(&ctx, &a, name == "chgrp"),
        "who" | "w" => {
            users::who(&ctx, name == "w");
            0
        }
        "hostname" => net::hostname(&ctx, &a),
        "uname" => net::uname(&ctx, &a),
        "ps" => procs::ps(&ctx, &a),
        "pgrep" => procs::pgrep(&ctx, &a, false),
        "pkill" | "killall" => procs::pgrep(&ctx, &a, true),
        "kill" => procs::kill(&ctx, &a),
        "top" => procs::top(&ctx, &a),
        "systemctl" => admin::systemctl(&ctx, &a),
        "service" => {
            // service ssh restart → systemctl restart ssh
            if a.len() >= 2 {
                admin::systemctl(&ctx, &[a[1].clone(), a[0].clone()])
            } else {
                eprintln!("Usage: service <service> <action>");
                1
            }
        }
        "journalctl" => admin::journalctl(&ctx, &a),
        "ufw" => admin::ufw(&ctx, &a),
        "iptables" => admin::iptables(&ctx, &a),
        "fail2ban-client" => admin::fail2ban(&ctx, &a),
        "auditctl" => admin::auditctl(&ctx, &a),
        "apt" | "apt-get" => admin::apt(&ctx, &a),
        "dpkg" => admin::dpkg(&ctx, &a),
        "cowsay" | "snort" => admin::installed_tool(&ctx, name, &a),
        "chroot" => admin::chroot(&ctx, &a),
        "mount" | "findmnt" => files::mount(&ctx),
        "df" => files::df(&ctx, &a),
        "ping" => net::ping(&ctx, &a),
        "ip" => net::ip(&ctx, &a),
        "ifconfig" => net::ifconfig(&ctx),
        "ss" | "netstat" => net::ss(&ctx, &a, name == "netstat"),
        "traceroute" => net::traceroute(&ctx, &a),
        "nmap" => net::nmap(&ctx, &a),
        "dig" | "host" | "nslookup" => net::dig(&ctx, &a, name),
        "curl" | "wget" => net::curl(&ctx, &a, name == "wget"),
        "ssh" => net::ssh(&ctx, &a),
        "ls" => files::ls(&ctx, &a),
        "find" => files::find(&ctx, &a),
        "nano" => files::nano(&a),
        "zip" => files::zip(&a),
        "unzip" => files::unzip(&a),
        _ => {
            eprintln!("{}: simulateur inconnu", name);
            127
        }
    }
}

// ───────────────────────── État initial d'une campagne ─────────────────────────

/// Crée l'état système fictif de la campagne (passwd, group, shadow, processus, services...)
/// dans `etc`. `force` = écrase (gsh reset / restart).
pub fn init_system(etc: &str, camp: u8, force: bool) {
    let marker = format!("{}/gsh/version", etc);
    if !force && std::path::Path::new(&marker).exists() {
        return;
    }
    let _ = std::fs::remove_dir_all(etc);
    let _ = std::fs::create_dir_all(format!("{}/gsh", etc));
    let p = Profile::of(camp);
    let ctx = Ctx::host(camp, "/", etc);
    let w = |name: &str, content: String| {
        let _ = std::fs::write(format!("{}/{}", etc, name), content);
    };
    let home = format!("{}/{}", p.homebase, p.user);
    w("passwd", format!(
        "root:x:0:0:root:/root:/bin/bash\ndaemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin\nsshd:x:100:65534::/run/sshd:/usr/sbin/nologin\n{u}:x:1000:1000:{u}:{h}:/bin/bash\nnobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin\n",
        u = p.user, h = home));
    w("group", format!(
        "root:x:0:\nadm:x:4:{u}\nsudo:x:27:{u}\n{u}:x:1000:\n{g1}:x:{i1}:{u}\n{g2}:x:{i2}:{u}\nnogroup:x:65534:\n",
        u = p.user, g1 = p.groups[0].0, i1 = p.groups[0].1, g2 = p.groups[1].0, i2 = p.groups[1].1));
    w("shadow", format!("root:*:19700:0:99999:7:::\ndaemon:*:19700:0:99999:7:::\nsshd:*:19700:0:99999:7:::\n{}:{}:19700:0:99999:7:::\nnobody:*:19700:0:99999:7:::\n", p.user, hash_pw(p.user)));
    w("hostname", format!("{}\n", p.host));
    w("hosts", format!("127.0.0.1\tlocalhost\n127.0.1.1\t{h}\n{ip}\t{h}.{d} {h}\n10.0.0.1\tpasserelle andromede\n10.0.0.2\tnexus-02\n10.0.0.66\tcorbeau-noir\n10.0.0.99\tallie\n10.0.0.254\tfai\n", h = p.host, ip = p.ip, d = p.domain));
    w(
        "nsswitch.conf",
        "passwd: files\ngroup: files\nshadow: files\nhosts: files\n".into(),
    );
    w("os-release", "PRETTY_NAME=\"Debian GNU/Linux 13 (trixie)\"\nNAME=\"Debian GNU/Linux\"\nVERSION_ID=\"13\"\nID=debian\n".into());
    w("resolv.conf", "nameserver 10.0.0.1\n".into());
    w("shells", "/bin/sh\n/bin/bash\n/usr/bin/bash\n".into());
    // fichiers de l'hôte utiles aux outils (fuseau horaire, cache des bibliothèques, configs d'éditeurs)
    for f in [
        "localtime",
        "ld.so.cache",
        "nanorc",
        "manpath.config",
        "inputrc",
        "magic",
        "mime.types",
    ] {
        if let Ok(b) = std::fs::read(format!("/etc/{}", f)) {
            let _ = std::fs::write(format!("{}/{}", etc, f), b);
        }
    }
    w(
        "sudoers",
        format!(
            "root ALL=(ALL:ALL) ALL\n%sudo ALL=(ALL:ALL) ALL\n# {} est membre du groupe sudo\n",
            p.user
        ),
    );

    // Processus fictifs: pid|user|cpu|mem|stat|start|commande
    let mut procs: Vec<String> = vec![
        "1|root|0.0|0.3|Ss|08:00|/sbin/init".into(),
        "412|root|0.0|0.2|Ssl|08:00|/usr/sbin/rsyslogd -n -iNONE".into(),
        "433|root|0.0|0.1|Ss|08:00|/usr/sbin/cron -f".into(),
        "520|root|0.1|1.1|Ssl|08:00|/usr/bin/python3 /usr/bin/fail2ban-server -xf start".into(),
        "812|root|0.0|0.4|Ss|08:00|sshd: /usr/sbin/sshd -D [listener] 0 of 10-100 startups".into(),
        "2210|root|0.0|0.1|S<sl|08:00|/sbin/auditd".into(),
        format!("1337|{}|0.0|0.2|Ss|08:15|-bash", p.user),
        format!("1500|root|0.0|0.3|Ss|08:15|sshd: {} [priv]", p.user),
    ];
    match camp {
        2 => procs.extend([
            "3001|root|0.3|0.5|S|08:01|garde_ronde --porte nord".to_string(),
            "3002|root|0.3|0.5|S|08:01|garde_ronde --porte sud".to_string(),
            "3003|root|0.1|0.2|S|08:01|garde_veilleur --tour est".to_string(),
            format!("6666|{}|66.6|4.2|R|03:33|/tmp/.espion/espion_ninja --cible donjon --relais 10.0.0.66", p.user),
            "6667|root|0.0|0.1|S|03:33|sshd: espion@notty".to_string(),
        ]),
        3 => procs.extend([
            "2222|root|0.2|0.9|Ss|08:00|nginx: master process /usr/sbin/nginx".to_string(),
            "2223|www-data|0.4|1.2|S|08:00|nginx: worker process".to_string(),
            "3306|mysql|1.5|9.8|Ssl|08:00|/usr/sbin/mysqld".to_string(),
            format!("6666|{}|98.7|12.4|R|02:14|/tmp/.x/miner --pool stratum+tcp://10.0.0.66:3333 --threads 8", p.user),
            "6670|root|0.0|0.1|S|02:14|sshd: root@notty".to_string(),
        ]),
        _ => procs.extend([
            "2048|root|1.2|2.0|Ssl|08:00|/usr/bin/bouclier-deflecteur --mode=auto".to_string(),
            "3100|root|0.4|1.0|Ss|08:00|/usr/bin/navigation-hyperespace --secteur 7G".to_string(),
            format!("4242|{}|87.3|6.1|R|04:42|./corbeau_miner --pool 10.0.0.66:3333", p.user),
            format!("9999|{}|12.5|3.3|S|03:13|/tmp/.pirate/backdoor --c2 10.0.0.66 --port 4444", p.user),
            "9998|root|0.0|0.1|S|03:13|sshd: pirate@notty".to_string(),
        ]),
    }
    ctx.set_lines("processes", &procs);

    // Services: nom|actif|activé|pid|description
    let mut svc: Vec<String> = vec![
        "ssh|active|enabled|812|OpenBSD Secure Shell server".into(),
        "cron|active|enabled|433|Regular background program processing daemon".into(),
        "rsyslog|active|enabled|412|System Logging Service".into(),
        "fail2ban|active|enabled|520|Fail2Ban Service".into(),
        "ufw|active|enabled|0|Uncomplicated firewall".into(),
        "auditd|active|enabled|2210|Security Auditing Service".into(),
        "systemd-journald|active|static|210|Journal Service".into(),
    ];
    if camp == 3 {
        svc.push(
            "nginx|active|enabled|2222|A high performance web server and a reverse proxy server"
                .into(),
        );
        svc.push("mysql|active|enabled|3306|MySQL Community Server".into());
    }
    ctx.set_lines("services", &svc);

    // Journal système (journalctl)
    let h = p.host;
    let mut j: Vec<String> = Vec::new();
    j.push(format!(
        "janv. 15 08:00:01 {} systemd[1]: Started ssh.service - OpenBSD Secure Shell server.",
        h
    ));
    j.push(format!("janv. 15 08:00:02 {} systemd[1]: Started cron.service - Regular background program processing daemon.", h));
    j.push(format!(
        "janv. 15 08:00:02 {} systemd[1]: Started fail2ban.service - Fail2Ban Service.",
        h
    ));
    for (i, port) in [51422, 51430, 51444, 51460, 51477, 51490]
        .iter()
        .enumerate()
    {
        j.push(format!("janv. 15 03:1{}:0{} {} sshd[812]: Failed password for root from 10.0.0.66 port {} ssh2", i, i, h, port));
    }
    j.push(format!(
        "janv. 15 03:17:00 {} fail2ban.actions[520]: NOTICE [sshd] Ban 10.0.0.66",
        h
    ));
    j.push(format!("janv. 15 03:20:11 {} kernel: [UFW BLOCK] IN=eth0 OUT= SRC=10.0.0.66 DST={} PROTO=TCP DPT=22", h, p.ip));
    j.push(format!(
        "janv. 15 08:15:42 {} sshd[1500]: Accepted publickey for {} from 10.0.0.1 port 40022 ssh2",
        h, p.user
    ));
    j.push(format!("janv. 15 08:15:42 {} sshd[1500]: pam_unix(sshd:session): session opened for user {}(uid=1000)", h, p.user));
    j.push(format!(
        "janv. 15 09:00:01 {} CRON[2501]: (root) CMD (/usr/local/sbin/sauvegarde.sh)",
        h
    ));
    j.push(format!(
        "janv. 15 09:30:00 {} sshd[812]: Connection closed by 10.0.0.66 port 51501 [preauth]",
        h
    ));
    j.push(format!(
        "janv. 15 09:41:07 {} sshd[812]: Invalid user admin from 10.0.0.66 port 51510",
        h
    ));
    j.push(format!("janv. 15 09:41:09 {} sshd[812]: Failed password for invalid user admin from 10.0.0.66 port 51510 ssh2", h));
    j.push(format!(
        "janv. 15 10:02:33 {} auditd[2210]: Audit daemon rotating log files",
        h
    ));
    j.push(format!("janv. 15 10:12:00 {} kernel: [UFW BLOCK] IN=eth0 OUT= SRC=10.0.0.66 DST={} PROTO=TCP DPT=3306", h, p.ip));
    j.push(format!(
        "janv. 15 11:00:01 {} CRON[2650]: (root) CMD (test -x /usr/sbin/anacron)",
        h
    ));
    j.push(format!("janv. 15 11:20:45 {} sshd[812]: Received disconnect from 10.0.0.99 port 40100:11: disconnected by user", h));
    j.push(format!(
        "janv. 15 11:45:02 {} fail2ban.actions[520]: NOTICE [sshd] Ban 10.0.0.99",
        h
    ));
    j.push(format!("janv. 15 12:00:00 {} systemd[1]: Starting apt-daily.service - Daily apt download activities...", h));
    j.push(format!("janv. 15 12:00:09 {} systemd[1]: Finished apt-daily.service - Daily apt download activities.", h));
    ctx.set_lines("journal", &j);

    // Pare-feu UFW
    ctx.set_lines(
        "ufw.conf",
        &[
            "enabled=no".into(),
            format!("default_in={}", if camp == 3 { "allow" } else { "deny" }),
            "default_out=allow".into(),
            "logging=low".into(),
        ],
    );
    let rules: Vec<String> = match camp {
        1 => vec!["ALLOW IN 22/tcp from Anywhere".into()],
        _ => vec!["ALLOW IN 80/tcp from Anywhere".into()],
    };
    ctx.set_lines("ufw.rules", &rules);
    ctx.set_lines(
        "iptables.rules",
        &[
            "-P INPUT ACCEPT".into(),
            "-P FORWARD DROP".into(),
            "-P OUTPUT ACCEPT".into(),
            "-A INPUT -i lo -j ACCEPT".into(),
            "-A INPUT -m state --state RELATED,ESTABLISHED -j ACCEPT".into(),
            "-A INPUT -p tcp --dport 22 -j ACCEPT".into(),
        ],
    );
    let bans: Vec<String> = match camp {
        3 => vec!["sshd 10.0.0.66".into(), "sshd 10.0.0.254".into()],
        _ => vec!["sshd 10.0.0.66".into(), "sshd 10.0.0.99".into()],
    };
    ctx.set_lines("fail2ban.bans", &bans);
    ctx.set_lines("audit.rules", &[]);
    let mut pk: Vec<String> = [
        "base-files",
        "bash",
        "coreutils",
        "openssh-server",
        "ufw",
        "fail2ban",
        "auditd",
        "iptables",
        "sudo",
        "curl",
        "nano",
        "tar",
        "gzip",
        "zip",
        "unzip",
        "net-tools",
        "iproute2",
        "procps",
        "dnsutils",
        "systemd",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    if camp == 3 {
        pk.extend(
            ["nmap", "nginx", "mysql-server", "traceroute"]
                .iter()
                .map(|s| s.to_string()),
        );
    }
    ctx.set_lines("packages.installed", &pk);
    ctx.set_lines(
        "packages.auto",
        &[
            "linux-image-6.1.0-old".into(),
            "libcorbeau-outils".into(),
            "python3-obsolete".into(),
        ],
    );
    ctx.set_lines("apt.state", &["updated=no".into()]);
    ctx.set_lines("events.log", &[]);
    ctx.set_lines("owners", &[]);
    w("gsh/version", "2\n".into());
}

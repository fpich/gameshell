//! Comptes et identité simulés: sudo, su, id, groups, useradd, groupadd, usermod, passwd, chown...
//! Tout est écrit dans le /etc du JEU (jamais celui de l'hôte).

use super::{hash_pw, need_root, read_lines, read_secret, run_as, run_shell, write_lines, Ctx};

#[allow(dead_code)]
pub struct Pw {
    pub name: String,
    pub uid: u32,
    pub gid: u32,
    pub home: String,
    pub shell: String,
}
pub struct Gr {
    pub name: String,
    pub gid: u32,
    pub members: Vec<String>,
}

pub fn passwd_db(c: &Ctx) -> Vec<Pw> {
    read_lines(&c.etc_file("passwd"))
        .iter()
        .filter_map(|l| {
            let f: Vec<&str> = l.split(':').collect();
            if f.len() < 7 {
                return None;
            }
            Some(Pw {
                name: f[0].into(),
                uid: f[2].parse().ok()?,
                gid: f[3].parse().ok()?,
                home: f[5].into(),
                shell: f[6].into(),
            })
        })
        .collect()
}

pub fn group_db(c: &Ctx) -> Vec<Gr> {
    read_lines(&c.etc_file("group"))
        .iter()
        .filter_map(|l| {
            let f: Vec<&str> = l.split(':').collect();
            if f.len() < 4 {
                return None;
            }
            Some(Gr {
                name: f[0].into(),
                gid: f[2].parse().ok()?,
                members: f[3]
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect(),
            })
        })
        .collect()
}

fn save_groups(c: &Ctx, g: &[Gr]) {
    let l: Vec<String> = g
        .iter()
        .map(|g| format!("{}:x:{}:{}", g.name, g.gid, g.members.join(",")))
        .collect();
    write_lines(&c.etc_file("group"), &l);
}

pub fn user_exists(c: &Ctx, u: &str) -> bool {
    passwd_db(c).iter().any(|p| p.name == u)
}
pub fn group_exists(c: &Ctx, g: &str) -> bool {
    group_db(c).iter().any(|x| x.name == g)
}

/// Groupes d'un utilisateur: (gid, nom), groupe principal d'abord
pub fn user_groups(c: &Ctx, u: &str) -> Vec<(u32, String)> {
    let pw = passwd_db(c);
    let gr = group_db(c);
    let mut out = Vec::new();
    if let Some(p) = pw.iter().find(|p| p.name == u) {
        if let Some(g) = gr.iter().find(|g| g.gid == p.gid) {
            out.push((g.gid, g.name.clone()));
        }
        for g in &gr {
            if g.gid != p.gid && g.members.iter().any(|m| m == u) {
                out.push((g.gid, g.name.clone()));
            }
        }
    }
    out
}

pub fn has_password(c: &Ctx, u: &str) -> bool {
    read_lines(&c.etc_file("shadow")).iter().any(|l| {
        let f: Vec<&str> = l.split(':').collect();
        f.len() > 1 && f[0] == u && f[1].starts_with("$gsh$")
    })
}

fn set_shadow(c: &Ctx, u: &str, val: &str) {
    let mut l = read_lines(&c.etc_file("shadow"));
    let mut found = false;
    for x in l.iter_mut() {
        let mut f: Vec<String> = x.split(':').map(|s| s.to_string()).collect();
        if f.first().map(|s| s == u).unwrap_or(false) {
            if f.len() < 2 {
                f.resize(9, String::new());
            }
            f[1] = val.to_string();
            *x = f.join(":");
            found = true;
        }
    }
    if !found {
        l.push(format!("{}:{}:19700:0:99999:7:::", u, val));
    }
    write_lines(&c.etc_file("shadow"), &l);
}

fn shadow_hash(c: &Ctx, u: &str) -> Option<String> {
    read_lines(&c.etc_file("shadow")).iter().find_map(|l| {
        let f: Vec<&str> = l.split(':').collect();
        if f.len() > 1 && f[0] == u {
            Some(f[1].to_string())
        } else {
            None
        }
    })
}

fn env_for(c: &Ctx, user: &str) -> Vec<(&'static str, String)> {
    let home = if user == "root" {
        "/root".to_string()
    } else {
        passwd_db(c)
            .into_iter()
            .find(|p| p.name == user)
            .map(|p| p.home)
            .unwrap_or_else(|| format!("{}/{}", c.homebase, user))
    };
    vec![
        ("GSH_EUID", user.to_string()),
        ("USER", user.to_string()),
        ("LOGNAME", user.to_string()),
        ("HOME", home),
    ]
}

pub fn sudo(c: &Ctx, a: &[String]) -> i32 {
    let mut i = 0;
    let mut target = "root".to_string();
    let mut shell = false;
    while i < a.len() && a[i].starts_with('-') {
        match a[i].as_str() {
            "-l" | "--list" => {
                println!("Entrées par défaut pour {} sur {} :\n    env_reset, mail_badpass, secure_path=/usr/local/sbin\\:/usr/local/bin\\:/usr/sbin\\:/usr/bin\\:/sbin\\:/bin\n", c.user, c.host);
                println!("L'utilisateur {} peut utiliser les commandes suivantes sur {} :\n    (ALL : ALL) ALL", c.user, c.host);
                return 0;
            }
            "-v" | "-k" | "-K" => return 0,
            "-u" => {
                target = a.get(i + 1).cloned().unwrap_or_default();
                i += 1;
            }
            "-i" | "-s" => shell = true,
            "-h" | "--help" => {
                println!(
                    "usage: sudo [-u utilisateur] commande [arguments]\n       sudo -l | -i | -s"
                );
                return 0;
            }
            _ => {}
        }
        i += 1;
    }
    let in_sudo = super::users::user_groups(c, &c.user)
        .iter()
        .any(|(_, g)| g == "sudo")
        || c.is_root();
    if !in_sudo {
        eprintln!("{} n'apparaît pas dans le fichier sudoers. Cet incident sera signalé à l'administrateur.", c.me());
        return 1;
    }
    if target != "root" && !user_exists(c, &target) {
        eprintln!("sudo: utilisateur inconnu : {}", target);
        return 1;
    }
    let env = env_for(c, &target);
    if shell || i >= a.len() {
        if i >= a.len() && !shell {
            eprintln!("usage: sudo [-u utilisateur] commande");
            return 1;
        }
        return run_as(
            &["/bin/bash".into(), "--norc".into(), "-i".into()],
            &[env, vec![("PS1", format!("{}@{}:\\w# ", target, c.host))]].concat(),
        );
    }
    run_as(&a[i..], &env)
}

pub fn su(c: &Ctx, a: &[String]) -> i32 {
    let mut user = "root".to_string();
    let mut cmd: Option<String> = None;
    let mut i = 0;
    while i < a.len() {
        match a[i].as_str() {
            "-" | "-l" | "--login" => {}
            "-c" | "--command" => {
                cmd = a.get(i + 1).cloned();
                i += 1;
            }
            "-s" => {
                i += 1;
            }
            x if !x.starts_with('-') => user = x.to_string(),
            _ => {}
        }
        i += 1;
    }
    if user != "root" && !user_exists(c, &user) {
        eprintln!(
            "su: l'utilisateur {} n'existe pas ou l'entrée de l'utilisateur n'a pas de shell",
            user
        );
        return 1;
    }
    if !c.is_root() {
        let pw = read_secret("Mot de passe : ").unwrap_or_default();
        let ok = match shadow_hash(c, &user) {
            Some(h) if h.starts_with("$gsh$") => h == hash_pw(&pw),
            _ => false,
        };
        if !ok {
            eprintln!("su: Échec de l'authentification");
            if !has_password(c, &user) && user != "root" {
                eprintln!("(le compte {} n'a pas encore de mot de passe)", user);
            }
            return 1;
        }
    }
    let env = env_for(c, &user);
    match cmd {
        Some(s) => run_shell(&s, &env),
        None => run_as(
            &["/bin/bash".into(), "--norc".into(), "-i".into()],
            &[env, vec![("PS1", format!("{}@{}:\\w$ ", user, c.host))]].concat(),
        ),
    }
}

pub fn id(c: &Ctx, a: &[String]) -> i32 {
    let u = a
        .iter()
        .find(|x| !x.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| c.me().to_string());
    let flags: String = a
        .iter()
        .filter(|x| x.starts_with('-'))
        .map(|x| x.trim_start_matches('-'))
        .collect();
    if u == "root" {
        if flags.contains('u') {
            println!("0");
        } else if flags.contains('n') && flags.contains('G') {
            println!("root");
        } else {
            println!("uid=0(root) gid=0(root) groupes=0(root)");
        }
        return 0;
    }
    let pw = passwd_db(c);
    let p = match pw.iter().find(|p| p.name == u) {
        Some(p) => p,
        None => {
            eprintln!("id: « {} » : utilisateur inexistant", u);
            return 1;
        }
    };
    let gs = user_groups(c, &u);
    if flags.contains('u') {
        println!(
            "{}",
            if flags.contains('n') {
                u.clone()
            } else {
                p.uid.to_string()
            }
        );
        return 0;
    }
    if flags.contains('g') {
        println!(
            "{}",
            if flags.contains('n') {
                gs.first().map(|g| g.1.clone()).unwrap_or_default()
            } else {
                p.gid.to_string()
            }
        );
        return 0;
    }
    if flags.contains('G') {
        let v: Vec<String> = gs
            .iter()
            .map(|(g, n)| {
                if flags.contains('n') {
                    n.clone()
                } else {
                    g.to_string()
                }
            })
            .collect();
        println!("{}", v.join(" "));
        return 0;
    }
    let pg = gs
        .first()
        .map(|g| g.1.clone())
        .unwrap_or_else(|| p.gid.to_string());
    let list: Vec<String> = gs.iter().map(|(g, n)| format!("{}({})", g, n)).collect();
    println!(
        "uid={}({}) gid={}({}) groupes={}",
        p.uid,
        u,
        p.gid,
        pg,
        list.join(",")
    );
    0
}

pub fn groups(c: &Ctx, a: &[String]) -> i32 {
    let users: Vec<String> = if a.is_empty() {
        vec![c.me().to_string()]
    } else {
        a.to_vec()
    };
    for u in &users {
        if u == "root" {
            println!("{}root", if a.is_empty() { "" } else { "root : " });
            continue;
        }
        if !user_exists(c, u) {
            eprintln!("groups: « {} » : utilisateur inexistant", u);
            return 1;
        }
        let names: Vec<String> = user_groups(c, u).into_iter().map(|g| g.1).collect();
        if a.is_empty() {
            println!("{}", names.join(" "));
        } else {
            println!("{} : {}", u, names.join(" "));
        }
    }
    0
}

fn valid_name(n: &str) -> bool {
    !n.is_empty()
        && n.len() <= 32
        && n.chars()
            .next()
            .map(|c| c.is_ascii_lowercase() || c == '_')
            .unwrap_or(false)
        && n.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
}

pub fn useradd(c: &Ctx, a: &[String], adduser: bool) -> i32 {
    let me = if adduser { "adduser" } else { "useradd" };
    if !c.is_root() {
        eprintln!("{me} : Permission non accordée.\n{me} : impossible de verrouiller /etc/passwd ; veuillez réessayer plus tard.");
        eprintln!("(astuce: les comptes se créent avec les droits administrateur)");
        return 1;
    }
    let mut mkhome = adduser;
    let mut shell = "/bin/sh".to_string();
    let mut home: Option<String> = None;
    let mut extra: Vec<String> = Vec::new();
    let mut name: Option<String> = None;
    let mut i = 0;
    while i < a.len() {
        match a[i].as_str() {
            "-m" | "--create-home" => mkhome = true,
            "-M" => mkhome = false,
            "-s" | "--shell" => {
                shell = a.get(i + 1).cloned().unwrap_or_default();
                i += 1;
            }
            "-d" | "--home-dir" => {
                home = a.get(i + 1).cloned();
                i += 1;
            }
            "-G" | "--groups" => {
                extra = a
                    .get(i + 1)
                    .map(|s| s.split(',').map(|x| x.to_string()).collect())
                    .unwrap_or_default();
                i += 1;
            }
            "-g" | "-c" | "-u" | "-e" | "-k" => {
                i += 1;
            }
            "--disabled-password" | "--gecos" if adduser => {
                if a[i] == "--gecos" {
                    i += 1;
                }
            }
            x if !x.starts_with('-') => name = Some(x.to_string()),
            _ => {}
        }
        i += 1;
    }
    let name = match name {
        Some(n) => n,
        None => {
            eprintln!("Utilisation : {me} [options] IDENTIFIANT");
            return 2;
        }
    };
    if !valid_name(&name) {
        eprintln!("{me} : nom d'utilisateur « {} » non valable", name);
        return 3;
    }
    if user_exists(c, &name) {
        eprintln!("{me} : l'utilisateur « {} » existe déjà", name);
        return 9;
    }
    for g in &extra {
        if !group_exists(c, g) {
            eprintln!("{me} : le groupe « {} » n'existe pas", g);
            return 6;
        }
    }
    let pw = passwd_db(c);
    let gr = group_db(c);
    let uid = (1001..60000)
        .find(|u| !pw.iter().any(|p| p.uid == *u) && !gr.iter().any(|g| g.gid == *u))
        .unwrap_or(1999);
    let home = home.unwrap_or_else(|| format!("{}/{}", c.homebase, name));
    if adduser {
        shell = "/bin/bash".into();
    }
    let mut pl = read_lines(&c.etc_file("passwd"));
    let pos = pl
        .iter()
        .position(|l| l.starts_with("nobody:"))
        .unwrap_or(pl.len());
    pl.insert(
        pos,
        format!("{}:x:{}:{}::{}:{}", name, uid, uid, home, shell),
    );
    write_lines(&c.etc_file("passwd"), &pl);
    let mut gr = group_db(c);
    let gpos = gr
        .iter()
        .position(|g| g.name == "nogroup")
        .unwrap_or(gr.len());
    gr.insert(
        gpos,
        Gr {
            name: name.clone(),
            gid: uid,
            members: vec![],
        },
    );
    for g in gr.iter_mut() {
        if extra.contains(&g.name) {
            g.members.push(name.clone());
        }
    }
    save_groups(c, &gr);
    set_shadow(c, &name, "!");
    if mkhome {
        let real = c.real(&home);
        let _ = std::fs::create_dir_all(&real);
        let _ = std::fs::write(
            real.join(".bashrc"),
            "# ~/.bashrc du nouveau membre d'équipage\n",
        );
    }
    c.event(&format!("useradd {}", name));
    if adduser {
        println!("Ajout de l'utilisateur « {} » ...\nAjout du nouveau groupe « {} » ({}) ...\nCréation du répertoire personnel « {} » ...", name, name, uid, home);
        return passwd(c, &[name]);
    }
    0
}

pub fn userdel(c: &Ctx, a: &[String]) -> i32 {
    if !c.is_root() {
        return need_root("userdel");
    }
    let rm = a.iter().any(|x| x == "-r");
    let name = match a.iter().find(|x| !x.starts_with('-')) {
        Some(n) => n.clone(),
        None => {
            eprintln!("Utilisation : userdel [options] IDENTIFIANT");
            return 2;
        }
    };
    if name == c.user || name == "root" {
        eprintln!("userdel: l'utilisateur {} est actuellement connecté", name);
        return 8;
    }
    let p = match passwd_db(c).into_iter().find(|p| p.name == name) {
        Some(p) => p,
        None => {
            eprintln!("userdel : l'utilisateur « {} » n'existe pas", name);
            return 6;
        }
    };
    let pl: Vec<String> = read_lines(&c.etc_file("passwd"))
        .into_iter()
        .filter(|l| !l.starts_with(&format!("{}:", name)))
        .collect();
    write_lines(&c.etc_file("passwd"), &pl);
    let sl: Vec<String> = read_lines(&c.etc_file("shadow"))
        .into_iter()
        .filter(|l| !l.starts_with(&format!("{}:", name)))
        .collect();
    write_lines(&c.etc_file("shadow"), &sl);
    let mut gr = group_db(c);
    gr.retain(|g| !(g.name == name && g.gid == p.gid));
    for g in gr.iter_mut() {
        g.members.retain(|m| *m != name);
    }
    save_groups(c, &gr);
    if rm && p.home.starts_with(&c.homebase) {
        let _ = std::fs::remove_dir_all(c.real(&p.home));
    }
    c.event(&format!("userdel {}", name));
    0
}

pub fn groupadd(c: &Ctx, a: &[String]) -> i32 {
    if !c.is_root() {
        eprintln!("groupadd : Permission non accordée.\ngroupadd : impossible de verrouiller /etc/group ; veuillez réessayer plus tard.");
        return 10;
    }
    let name = match a.iter().find(|x| !x.starts_with('-')) {
        Some(n) => n.clone(),
        None => {
            eprintln!("Utilisation : groupadd [options] GROUPE");
            return 2;
        }
    };
    if !valid_name(&name) {
        eprintln!("groupadd : « {} » n'est pas un nom de groupe valable", name);
        return 3;
    }
    if group_exists(c, &name) {
        eprintln!("groupadd : le groupe « {} » existe déjà", name);
        return 9;
    }
    let mut gr = group_db(c);
    let pw = passwd_db(c);
    let gid = (1001..60000)
        .find(|g| !gr.iter().any(|x| x.gid == *g) && !pw.iter().any(|p| p.uid == *g))
        .unwrap_or(1999);
    let pos = gr
        .iter()
        .position(|g| g.name == "nogroup")
        .unwrap_or(gr.len());
    gr.insert(
        pos,
        Gr {
            name: name.clone(),
            gid,
            members: vec![],
        },
    );
    save_groups(c, &gr);
    c.event(&format!("groupadd {}", name));
    0
}

pub fn groupdel(c: &Ctx, a: &[String]) -> i32 {
    if !c.is_root() {
        return need_root("groupdel");
    }
    let name = match a.iter().find(|x| !x.starts_with('-')) {
        Some(n) => n.clone(),
        None => {
            eprintln!("Utilisation : groupdel GROUPE");
            return 2;
        }
    };
    let mut gr = group_db(c);
    if !gr.iter().any(|g| g.name == name) {
        eprintln!("groupdel : le groupe « {} » n'existe pas", name);
        return 6;
    }
    gr.retain(|g| g.name != name);
    save_groups(c, &gr);
    c.event(&format!("groupdel {}", name));
    0
}

pub fn usermod(c: &Ctx, a: &[String]) -> i32 {
    if !c.is_root() {
        eprintln!("usermod : Permission non accordée.\nusermod : impossible de verrouiller /etc/passwd ; veuillez réessayer plus tard.");
        return 1;
    }
    let mut append = false;
    let mut groups: Option<Vec<String>> = None;
    let mut lock: Option<bool> = None;
    let mut shell: Option<String> = None;
    let mut i = 0;
    let mut user: Option<String> = None;
    while i < a.len() {
        let x = a[i].as_str();
        if x.starts_with('-') && !x.starts_with("--") && x.len() > 2 {
            // options groupées: -aG groupe
            for ch in x[1..].chars() {
                match ch {
                    'a' => append = true,
                    'L' => lock = Some(true),
                    'U' => lock = Some(false),
                    'G' => {
                        groups = a
                            .get(i + 1)
                            .map(|s| s.split(',').map(|g| g.to_string()).collect());
                        i += 1;
                    }
                    _ => {}
                }
            }
        } else {
            match x {
                "-a" | "--append" => append = true,
                "-G" | "--groups" => {
                    groups = a
                        .get(i + 1)
                        .map(|s| s.split(',').map(|g| g.to_string()).collect());
                    i += 1;
                }
                "-L" | "--lock" => lock = Some(true),
                "-U" | "--unlock" => lock = Some(false),
                "-s" | "--shell" => {
                    shell = a.get(i + 1).cloned();
                    i += 1;
                }
                "-g" | "-d" | "-l" | "-c" | "-u" | "-e" => {
                    i += 1;
                }
                x if !x.starts_with('-') => user = Some(x.to_string()),
                _ => {}
            }
        }
        i += 1;
    }
    let user = match user {
        Some(u) => u,
        None => {
            eprintln!("Utilisation : usermod [options] IDENTIFIANT");
            return 2;
        }
    };
    if !user_exists(c, &user) {
        eprintln!("usermod : l'utilisateur « {} » n'existe pas", user);
        return 6;
    }
    if let Some(gs) = &groups {
        for g in gs {
            if !group_exists(c, g) {
                eprintln!("usermod : le groupe « {} » n'existe pas", g);
                return 6;
            }
        }
        let mut gr = group_db(c);
        for g in gr.iter_mut() {
            let wanted = gs.contains(&g.name);
            let member = g.members.contains(&user);
            if wanted && !member {
                g.members.push(user.clone());
            }
            if !wanted && member && !append {
                g.members.retain(|m| *m != user);
            }
        }
        save_groups(c, &gr);
        c.event(&format!("usermod {} groupes={}", user, gs.join(",")));
        if !append {
            eprintln!(
                "(attention: sans -a, {} a été RETIRÉ de ses autres groupes secondaires)",
                user
            );
        }
    }
    if let Some(l) = lock {
        if let Some(h) = shadow_hash(c, &user) {
            let nh = if l {
                if h.starts_with('!') {
                    h
                } else {
                    format!("!{}", h)
                }
            } else {
                h.trim_start_matches('!').to_string()
            };
            set_shadow(c, &user, &nh);
        }
        c.event(&format!(
            "usermod {} {}",
            user,
            if l { "verrouillé" } else { "déverrouillé" }
        ));
    }
    if let Some(s) = shell {
        let pl: Vec<String> = read_lines(&c.etc_file("passwd"))
            .into_iter()
            .map(|l| {
                let mut f: Vec<String> = l.split(':').map(|x| x.to_string()).collect();
                if f.len() >= 7 && f[0] == user {
                    f[6] = s.clone();
                }
                f.join(":")
            })
            .collect();
        write_lines(&c.etc_file("passwd"), &pl);
    }
    0
}

pub fn gpasswd(c: &Ctx, a: &[String]) -> i32 {
    if !c.is_root() {
        return need_root("gpasswd");
    }
    if a.len() == 3 && (a[0] == "-a" || a[0] == "-d") {
        let (user, grp) = (&a[1], &a[2]);
        if !user_exists(c, user) {
            eprintln!("gpasswd : l'utilisateur « {} » n'existe pas", user);
            return 3;
        }
        let mut gr = group_db(c);
        match gr.iter_mut().find(|g| g.name == *grp) {
            Some(g) => {
                if a[0] == "-a" {
                    if !g.members.contains(user) {
                        g.members.push(user.clone());
                    }
                    println!("Ajout de l'utilisateur {} au groupe {}", user, grp);
                } else {
                    g.members.retain(|m| m != user);
                    println!("Suppression de l'utilisateur {} du groupe {}", user, grp);
                }
            }
            None => {
                eprintln!("gpasswd : le groupe « {} » n'existe pas", grp);
                return 3;
            }
        }
        save_groups(c, &gr);
        return 0;
    }
    eprintln!("Utilisation : gpasswd [-a UTILISATEUR | -d UTILISATEUR] GROUPE");
    2
}

pub fn passwd(c: &Ctx, a: &[String]) -> i32 {
    let opt = a.iter().find(|x| x.starts_with('-')).cloned();
    let user = a
        .iter()
        .find(|x| !x.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| c.me().to_string());
    if user != c.me() && !c.is_root() {
        eprintln!("passwd : Vous ne pouvez pas afficher ou modifier les informations de mot de passe pour {}.", user);
        return 1;
    }
    if user != "root" && !user_exists(c, &user) {
        eprintln!("passwd : l'utilisateur « {} » n'existe pas", user);
        return 1;
    }
    match opt.as_deref() {
        Some("-S") => {
            let st = match shadow_hash(c, &user) {
                Some(h) if h.starts_with("$gsh$") => "P",
                Some(h) if h.starts_with('!') => "L",
                _ => "NP",
            };
            println!("{} {} 01/15/2024 0 99999 7 -1", user, st);
            return 0;
        }
        Some("-l") | Some("-u") | Some("-d") => {
            if !c.is_root() {
                return need_root("passwd");
            }
            let h = shadow_hash(c, &user).unwrap_or_default();
            let nh = match opt.as_deref() {
                Some("-l") => format!("!{}", h.trim_start_matches('!')),
                Some("-u") => h.trim_start_matches('!').to_string(),
                _ => String::new(),
            };
            set_shadow(c, &user, &nh);
            println!("passwd : informations d'expiration du mot de passe modifiées.");
            return 0;
        }
        _ => {}
    }
    let p1 = match read_secret("Nouveau mot de passe : ") {
        Some(p) => p,
        None => {
            eprintln!("passwd : erreur de manipulation du jeton d'authentification\npasswd : mot de passe inchangé");
            return 10;
        }
    };
    let p2 = read_secret("Retapez le nouveau mot de passe : ").unwrap_or_default();
    if p1 != p2 {
        eprintln!("Les mots de passe ne correspondent pas.\npasswd : erreur de manipulation du jeton d'authentification\npasswd : mot de passe inchangé");
        return 10;
    }
    if p1.is_empty() {
        eprintln!("Aucun mot de passe n'a été fourni.\npasswd : mot de passe inchangé");
        return 10;
    }
    set_shadow(c, &user, &hash_pw(&p1));
    c.event(&format!("passwd {}", user));
    println!("passwd : mot de passe mis à jour avec succès");
    0
}

/// chown/chgrp simulés: le propriétaire « fictif » est mémorisé dans l'état du jeu
pub fn chown(c: &Ctx, a: &[String], chgrp: bool) -> i32 {
    let me = if chgrp { "chgrp" } else { "chown" };
    let args: Vec<&String> = a.iter().filter(|x| !x.starts_with('-')).collect();
    if args.len() < 2 {
        eprintln!("{me}: opérande manquant\nSaisissez « {me} --help » pour plus d'informations.");
        return 1;
    }
    let spec = args[0].as_str();
    let (owner, group) = if chgrp {
        (None, Some(spec.to_string()))
    } else {
        match spec.split_once(':').or_else(|| spec.split_once('.')) {
            Some((o, g)) => (
                if o.is_empty() {
                    None
                } else {
                    Some(o.to_string())
                },
                if g.is_empty() {
                    None
                } else {
                    Some(g.to_string())
                },
            ),
            None => (Some(spec.to_string()), None),
        }
    };
    if let Some(o) = &owner {
        if o != "root" && !user_exists(c, o) {
            eprintln!("{me}: utilisateur incorrect: « {} »", spec);
            return 1;
        }
    }
    if let Some(g) = &group {
        if !group_exists(c, g) {
            eprintln!("{me}: groupe incorrect: « {} »", spec);
            return 1;
        }
    }
    let mut owners = c.lines("owners");
    let mut rc = 0;
    for f in &args[1..] {
        let real = c.real(f);
        if std::fs::symlink_metadata(&real).is_err() {
            eprintln!(
                "{me}: impossible d'accéder à '{}': Aucun fichier ou dossier de ce type",
                f
            );
            rc = 1;
            continue;
        }
        let cur_owner = current_owner(c, f, &owners);
        let changing_to_other =
            owner.as_ref().map(|o| o != c.me()).unwrap_or(false) || (cur_owner != c.me());
        if !c.is_root() && changing_to_other {
            eprintln!(
                "{me}: modification du propriétaire de '{}': Opération non permise",
                f
            );
            rc = 1;
            continue;
        }
        let key = canonical_key(c, f);
        let prev = owners
            .iter()
            .find(|l| l.split('\t').next() == Some(key.as_str()))
            .cloned();
        let (mut po, mut pg) = match &prev {
            Some(l) => {
                let v: Vec<&str> = l.split('\t').collect();
                (
                    v.get(1).unwrap_or(&"").to_string(),
                    v.get(2).unwrap_or(&"").to_string(),
                )
            }
            None => (c.user.clone(), c.user.clone()),
        };
        if let Some(o) = &owner {
            po = o.clone();
        }
        if let Some(g) = &group {
            pg = g.clone();
        }
        owners.retain(|l| l.split('\t').next() != Some(key.as_str()));
        owners.push(format!("{}\t{}\t{}", key, po, pg));
    }
    c.set_lines("owners", &owners);
    c.event(&format!("{} {}", me, a.join(" ")));
    rc
}

fn canonical_key(c: &Ctx, f: &str) -> String {
    // chemin absolu dans le monde (pour retrouver le propriétaire côté moteur)
    let real = std::fs::canonicalize(c.real(f)).unwrap_or_else(|_| c.real(f));
    let s = real.to_string_lossy().to_string();
    if c.root != "/" {
        s.strip_prefix(c.root.trim_end_matches('/'))
            .map(|x| x.to_string())
            .unwrap_or(s)
    } else {
        s
    }
}

fn current_owner(c: &Ctx, f: &str, owners: &[String]) -> String {
    let key = canonical_key(c, f);
    owners
        .iter()
        .find_map(|l| {
            let v: Vec<&str> = l.split('\t').collect();
            if v.first() == Some(&key.as_str()) {
                v.get(1).map(|s| s.to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| c.user.clone())
}

/// Propriétaire d'un fichier pour les checks (chemin du monde, ex: /pont/x.txt)
pub fn owner_of(c: &Ctx, world_path: &str) -> String {
    c.lines("owners")
        .iter()
        .find_map(|l| {
            let v: Vec<&str> = l.split('\t').collect();
            if v.first() == Some(&world_path) {
                v.get(1).map(|s| s.to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| c.user.clone())
}

pub fn who(c: &Ctx, long: bool) {
    if long {
        println!(" 08:15:42 up 1 day,  3:14,  1 user,  load average: 0,42, 0,37, 0,30");
        println!("UTIL.    TTY      DE               LOGIN@   IDLE   JCPU   PCPU  QUOI");
        println!(
            "{:<8} pts/0    10.0.0.1         08:15    0.00s  0.04s  0.00s -bash",
            c.user
        );
    } else {
        println!("{:<8} pts/0        2024-01-15 08:15 (10.0.0.1)", c.user);
    }
}

#[cfg(test)]
mod valid_name_tests {
    use super::valid_name;

    #[test]
    fn forme_valide() {
        assert!(valid_name("capitaine"));
        assert!(valid_name("_install"));
        assert!(valid_name("a-b-c"));
        assert!(valid_name("test123"));
        assert!(!valid_name(""));
        assert!(!valid_name("1ABC"));
        assert!(!valid_name("MajUsCUlE"));
        assert!(!valid_name("avec espace"));
        assert!(!valid_name("-avec"));
        assert!(!valid_name(&"a".repeat(33)));
        assert!(valid_name(&"a".repeat(32)));
    }
}

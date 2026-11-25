//! Module shell — monde du jeu, bac à sable d'exécution, vérifications natives, sauvegardes.
//!
//! ISOLATION (règle A1/A16, audit P0.1): chaque ligne tapée par le joueur s'exécute dans un
//! bac à sable bubblewrap (`bwrap`): le monde de la campagne devient `/`, /usr de l'hôte est
//! monté en LECTURE SEULE, /etc est celui du JEU (passwd, group...), aucun réseau, aucun
//! utilisateur réel (uid 1000 = capitaine/seigneur/netrunner). Les commandes d'administration
//! (sudo, useradd, ufw, systemctl, kill, ping...) sont des SIMULATEURS (module sim) placés en
//! tête du PATH: rien n'est jamais exécuté sur la machine hôte.
//! Sans bwrap: mode dégradé (réécriture des chemins + garde-fou sur TOUS les chemins
//! absolus qui échapperaient au monde — cd, ls, cat...), signalé au joueur.

use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};

static TEST_MODE: AtomicBool = AtomicBool::new(false);

/// Modes de test (AUTO/EDGE/TORTURE): jamais de sauvegarde ni d'historique (règle A18, P1.9)
pub fn set_test_mode(on: bool) {
    TEST_MODE.store(on, Ordering::SeqCst);
}
pub fn test_mode() -> bool {
    TEST_MODE.load(Ordering::SeqCst)
}

/// Dossier d'installation du jeu (isolé)
pub fn gameshell_home() -> String {
    std::env::var("GAMESHELL_HOME").unwrap_or_else(|_| {
        format!(
            "{}/.gameshell",
            std::env::var("GSH_REAL_HOME")
                .or_else(|_| std::env::var("HOME"))
                .unwrap_or_else(|_| ".".into())
        )
    })
}

/// Chemin vers le monde du jeu (dédié à la campagne choisie — règle D2)
pub fn world_dir() -> String {
    std::env::var("GAMESHELL_WORLD").unwrap_or_else(|_| format!("{}/vaisseau", gameshell_home()))
}

/// Campagne active: 1 = Pirates de l'Espace (facile), 2 = Japon Féodal (moyen), 3 = Nexus Réseau (difficile)
pub fn campaign_id() -> u8 {
    std::env::var("GAMESHELL_CAMPAIGN")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
}

pub fn camp_name(c: u8) -> &'static str {
    match c {
        2 => "japon",
        3 => "reseau",
        _ => "espace",
    }
}

fn world_of(home: &str, campaign: u8) -> String {
    match campaign {
        2 => format!("{}/monde_japon", home),
        3 => format!("{}/monde_reseau", home),
        _ => format!("{}/vaisseau", home),
    }
}

/// Home du joueur DANS le monde (chemin vu dans le jeu): /cabines/capitaine...
pub fn home_in_world(campaign: u8) -> String {
    let p = crate::sim::Profile::of(campaign);
    format!("{}/{}", p.homebase, p.user)
}

/// /etc fictif de la campagne (passwd, group, shadow, état des simulateurs)
pub fn sys_etc(campaign: u8) -> String {
    format!("{}/systeme/{}/etc", gameshell_home(), camp_name(campaign))
}

pub fn sim_ctx() -> crate::sim::Ctx {
    let c = campaign_id();
    crate::sim::Ctx::host(c, &world_dir(), &sys_etc(c))
}

fn essentials(campaign: u8) -> &'static [&'static str] {
    match campaign {
        2 => &[
            "donjon",
            "pagode",
            "jardins",
            "remparts",
            "greniers",
            "quartiers",
        ],
        3 => &["baies", "serveurs", "logs", "configs", "noc", "firewall"],
        _ => &[
            "pont",
            "cabines",
            "soutes",
            "reacteur",
            "hangar",
            "securite",
            "communications",
            "navigation",
            "armes",
            "salle-des-machines",
            "archives",
        ],
    }
}

/// Régénère le monde d'une campagne via le vaisseau-gen.sh embarqué. Retourne false si le générateur manque.
pub fn regenerate_world(campaign: u8) -> bool {
    let home = gameshell_home();
    let gen = format!("{}/vaisseau-gen.sh", home);
    if !Path::new(&gen).exists() {
        return false;
    }
    let world = world_of(&home, campaign);
    Command::new("bash")
        .arg(&gen)
        .arg(camp_name(campaign))
        .arg(&world)
        .env("GAMESHELL_HOME", &home)
        .stdout(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Sélectionne le monde d'une campagne (génère si absent/incomplet) et ajuste HOME — règle D2
pub fn select_world(campaign: u8) {
    let home_dir = gameshell_home();
    if std::env::var("GSH_REAL_HOME").is_err() {
        std::env::set_var("GSH_REAL_HOME", std::env::var("HOME").unwrap_or_default());
    }
    std::env::set_var("GAMESHELL_HOME", &home_dir);
    let world = world_of(&home_dir, campaign);
    let missing = essentials(campaign)
        .iter()
        .any(|d| !Path::new(&format!("{}/{}", world, d)).exists());
    if missing {
        regenerate_world(campaign);
    }
    crate::sim::init_system(&sys_etc(campaign), campaign, false);
    std::env::set_var("GAMESHELL_WORLD", &world);
    std::env::set_var("GAMESHELL_CAMPAIGN", campaign.to_string());
    std::env::set_var("HOME", format!("{}{}", world, home_in_world(campaign)));
}

/// Nom d'hôte du monde courant
pub fn world_hostname() -> &'static str {
    crate::sim::Profile::of(campaign_id()).host
}

/// Nom du joueur dans le monde courant
pub fn world_user() -> &'static str {
    crate::sim::Profile::of(campaign_id()).user
}

/// Home du joueur (chemin réel sur l'hôte)
pub fn player_home() -> String {
    format!("{}{}", world_dir(), home_in_world(campaign_id()))
}

/// Chemin réel → chemin vu dans le jeu (« / » = racine du monde). None si hors du monde.
pub fn to_world(real: &str) -> Option<String> {
    let w = world_dir();
    let w = w.trim_end_matches('/');
    if real == w {
        return Some("/".into());
    }
    real.strip_prefix(&format!("{}/", w))
        .map(|r| format!("/{}", r))
}

// ───────────────────────────── bac à sable ─────────────────────────────

fn which(name: &str) -> Option<String> {
    let path = std::env::var("PATH").unwrap_or_else(|_| "/usr/bin:/bin".into());
    for d in path
        .split(':')
        .chain(["/usr/bin", "/bin", "/usr/local/bin"])
    {
        let p = format!("{}/{}", d, name);
        if Path::new(&p).is_file() {
            return Some(p);
        }
    }
    None
}

static SANDBOX_CHECKED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();

/// bwrap disponible ET fonctionnel (espaces de noms utilisateur autorisés)?
pub fn sandbox_available() -> bool {
    if std::env::var("GSH_NO_SANDBOX").is_ok() {
        return false;
    }
    *SANDBOX_CHECKED.get_or_init(|| {
        which("bwrap")
            .map(|b| {
                Command::new(b)
                    .args(["--unshare-all", "--ro-bind", "/", "/", "true"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    })
}

/// Installe le binaire multi-appel des simulateurs dans <home>/simbin (liens useradd → gsh-sim...)
pub fn ensure_simbin() -> String {
    let dir = format!("{}/simbin", gameshell_home());
    let _ = std::fs::create_dir_all(&dir);
    let target = format!("{}/gsh-sim", dir);
    if let Ok(exe) = std::env::current_exe() {
        let same = match (std::fs::metadata(&exe), std::fs::metadata(&target)) {
            (Ok(a), Ok(b)) => a.len() == b.len() && b.modified().ok() >= a.modified().ok(),
            _ => false,
        };
        if !same {
            let tmp = format!("{}.tmp", target);
            if std::fs::copy(&exe, &tmp).is_ok() {
                let _ = std::fs::rename(&tmp, &target);
            }
        }
    }
    for n in crate::sim::NAMES {
        let l = format!("{}/{}", dir, n);
        if std::fs::symlink_metadata(&l).is_err() {
            let _ = std::os::unix::fs::symlink("gsh-sim", &l);
        }
    }
    dir
}

#[derive(Default)]
pub struct RunOpts {
    pub capture: bool,          // capture stdout+stderr (tests)
    pub quiet: bool,            // sortie supprimée (bancs de test)
    pub stdin: Option<Vec<u8>>, // entrée fournie (bancs de test)
}

pub struct RunResult {
    pub code: i32,
    pub cwd: Option<String>,
    pub out: String,
}

/// Script exécuté dans le bac à sable: la ligne du joueur est passée INTACTE (P1.5) à bash,
/// le dossier final est renvoyé sur le descripteur 3 (cd composés: `cd /pont && ls`, P1.4).
const SCRIPT: &str = r#"__gsh_old="$OLDPWD"
cd -- "$GSH_CWD" 2>/dev/null || cd
OLDPWD="$__gsh_old"
enable -n kill 2>/dev/null
shopt -s expand_aliases
alias ll='ls -alF' la='ls -A' l='ls -CF'
trap '' TSTP
eval "$GSH_LINE" 3>&-
__gsh_rc=$?
{ builtin printf '%s' "$(builtin pwd -P)" >&3; } 2>/dev/null
exit $__gsh_rc"#;

/// Normalise un chemin absolu hôte (lexical: résout "." et "..", clampées au root physique).
fn lexical_normalize(p: &std::path::Path) -> std::path::PathBuf {
    use std::path::Component;
    let mut out = std::path::PathBuf::new();
    for c in p.components() {
        match c {
            Component::RootDir => out.push("/"),
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            Component::Normal(seg) => out.push(seg),
            _ => {}
        }
    }
    out
}

/// Garde-fou de chemins (mode dégradé): lève une erreur si un chemin absolu de la ligne
/// échappe au monde (résolution lexicale après préfixe monde). Les chemins hôtes déjà
/// dans le monde et les chemins techniques hôte (/dev/null, /bin/, /usr/) passent.
fn rewrite_for_host(line: &str, world: &str) -> Result<String, String> {
    let world = world.trim_end_matches('/');
    let inside_prefix = format!("{}/", world);
    let mut out = String::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    let mut in_single = false;
    let mut in_double = false;
    let mut word_start = true;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' && !in_double {
            in_single = !in_single;
            out.push(c);
            i += 1;
            word_start = false;
            continue;
        }
        if c == '"' && !in_single {
            in_double = !in_double;
            out.push(c);
            i += 1;
            continue;
        }
        if c == '/'
            && (word_start || (i > 0 && matches!(chars[i - 1], '"' | '\'' | '=' | '>' | '<' | ':')))
        {
            // borne le token: fin au prochain séparateur non quoté (espace, |, >, (, etc.)
            let mut j = i;
            let mut l_single = in_single;
            let mut l_double = in_double;
            while j < chars.len() {
                let d = chars[j];
                if d == '\'' && !l_double {
                    l_single = !l_single;
                    j += 1;
                    continue;
                }
                if d == '"' && !l_single {
                    l_double = !l_double;
                    j += 1;
                    continue;
                }
                if !l_single
                    && !l_double
                    && matches!(d, ' ' | '\t' | '|' | ';' | '&' | '>' | '<' | '(' | ')')
                {
                    break;
                }
                j += 1;
            }
            let raw: String = chars[i..j].iter().collect();
            let trimmed = raw.trim_end_matches(['\'', '"']);
            let skip = trimmed.starts_with("/dev/null")
                || trimmed.starts_with("/bin/")
                || trimmed.starts_with("/usr/")
                || trimmed == world
                || trimmed.starts_with(&inside_prefix);
            if skip {
                out.extend(chars[i..j].iter());
                i = j;
                word_start = false;
                continue;
            }
            let abs = lexical_normalize(
                &std::path::Path::new(world).join(trimmed.trim_start_matches('/')),
            );
            let inside =
                abs == std::path::Path::new(world) || abs.starts_with(std::path::Path::new(world));
            if !inside {
                return Err(trimmed.to_string());
            }
            let rel = abs.strip_prefix(std::path::Path::new(world)).unwrap();
            let rel_s = rel.to_string_lossy().into_owned();
            let mut new_path = world.to_string();
            if !rel_s.is_empty() {
                new_path = format!("{}/{}", world, rel_s);
            }
            let trailing = if raw.ends_with('\'') || raw.ends_with('"') {
                raw.chars().last().unwrap().to_string()
            } else {
                String::new()
            };
            out.push_str(&new_path);
            if !trailing.is_empty() {
                out.push_str(&trailing);
            }
            i = j;
            word_start = false;
            continue;
        }
        out.push(c);
        word_start = !in_single
            && !in_double
            && (c == ' '
                || c == '\t'
                || c == '|'
                || c == ';'
                || c == '&'
                || c == '>'
                || c == '<'
                || c == '(');
        i += 1;
    }
    Ok(out)
}

/// Exécute une ligne du joueur dans le monde. `cwd_real` = dossier courant (chemin réel).
pub fn run_line(line: &str, cwd_real: &str, oldpwd_real: &str, o: &RunOpts) -> RunResult {
    let camp = campaign_id();
    let p = crate::sim::Profile::of(camp);
    let world = world_dir();
    let simbin = ensure_simbin();
    let etc = sys_etc(camp);
    crate::sim::init_system(&etc, camp, false);
    let sandbox = sandbox_available();
    let home_w = home_in_world(camp);

    let mut cmd;
    let (cwd_arg, oldpwd_arg, line_arg);
    if sandbox {
        let bw = which("bwrap").unwrap_or_else(|| "bwrap".into());
        cmd = Command::new(bw);
        cmd.args([
            "--unshare-all",
            "--die-with-parent",
            "--uid",
            "1000",
            "--gid",
            "1000",
            "--hostname",
            p.host,
        ]);
        cmd.args(["--bind", &world, "/"]);
        cmd.args(["--ro-bind", "/usr", "/usr"]);
        for d in ["bin", "sbin", "lib", "lib32", "lib64", "libx32"] {
            let hp = format!("/{}", d);
            match std::fs::symlink_metadata(&hp) {
                Ok(m) if m.file_type().is_symlink() => {
                    if let Ok(t) = std::fs::read_link(&hp) {
                        cmd.args(["--symlink", &t.to_string_lossy(), &hp]);
                    }
                }
                Ok(m) if m.is_dir() => {
                    cmd.args(["--ro-bind", &hp, &hp]);
                }
                _ => {}
            }
        }
        cmd.args([
            "--tmpfs",
            "/usr/local",
            "--ro-bind",
            &simbin,
            "/usr/local/bin",
        ]);
        cmd.args(["--bind", &etc, "/etc"]);
        cmd.args(["--proc", "/proc", "--dev", "/dev"]);
        cwd_arg = to_world(cwd_real).unwrap_or_else(|| home_w.clone());
        oldpwd_arg = to_world(oldpwd_real).unwrap_or_else(|| cwd_arg.clone());
        cmd.args([
            "--chdir",
            if Path::new(&format!("{}{}", world, cwd_arg)).is_dir() {
                &cwd_arg
            } else {
                "/"
            },
        ]);
        cmd.args([
            "/usr/local/bin/gsh-sim",
            "__exec",
            "/bin/bash",
            "--noprofile",
            "--norc",
            "-c",
            SCRIPT,
        ]);
        line_arg = line.to_string();
    } else {
        cmd = Command::new(format!("{}/gsh-sim", simbin));
        cmd.args(["__exec", "/bin/bash", "--noprofile", "--norc", "-c", SCRIPT]);
        cwd_arg = cwd_real.to_string();
        oldpwd_arg = oldpwd_real.to_string();
        line_arg = match rewrite_for_host(line, &world) {
            Ok(l) => l,
            Err(_) => {
                eprintln!("🚫 Ce passage mène hors du vaisseau: accès refusé.");
                return RunResult {
                    code: 1,
                    cwd: None,
                    out: String::new(),
                };
            }
        };
    }
    cmd.env_clear();
    let path = if sandbox {
        "/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin".to_string()
    } else {
        format!("{}:/usr/bin:/bin:/usr/sbin:/sbin", simbin)
    };
    let home = if sandbox {
        home_w.clone()
    } else {
        format!("{}{}", world, home_w)
    };
    cmd.env("PATH", path)
        .env("HOME", &home)
        .env("USER", p.user)
        .env("LOGNAME", p.user)
        .env("HOSTNAME", p.host)
        .env("SHELL", "/bin/bash")
        .env(
            "TMPDIR",
            if sandbox {
                "/tmp".to_string()
            } else {
                format!("{}/tmp", world)
            },
        )
        .env(
            "TERM",
            std::env::var("TERM").unwrap_or_else(|_| "xterm-256color".into()),
        )
        .env(
            "LANG",
            std::env::var("LANG").unwrap_or_else(|_| "C.UTF-8".into()),
        )
        .env(
            "GSH_ROOT",
            if sandbox {
                "/".to_string()
            } else {
                world.clone()
            },
        )
        .env(
            "GSH_ETC",
            if sandbox {
                "/etc".to_string()
            } else {
                etc.clone()
            },
        )
        .env("GSH_CAMPAIGN", camp.to_string())
        .env("GSH_USER", p.user)
        .env("GSH_HOSTNAME", p.host)
        .env("GSH_CWD", &cwd_arg)
        .env("OLDPWD", &oldpwd_arg)
        .env("GSH_LINE", &line_arg);
    for k in [
        "LC_ALL",
        "LC_MESSAGES",
        "LANGUAGE",
        "COLUMNS",
        "LINES",
        "COLORTERM",
    ] {
        if let Ok(v) = std::env::var(k) {
            cmd.env(k, v);
        }
    }
    if test_mode() {
        cmd.env("GSH_BENCH", "1");
    }

    // Descripteur 3: dossier courant final
    let (mut reader, writer) = match std::io::pipe() {
        Ok(p) => p,
        Err(_) => {
            return RunResult {
                code: -1,
                cwd: None,
                out: String::new(),
            }
        }
    };
    {
        use std::os::fd::AsRawFd;
        use std::os::unix::process::CommandExt;
        let wfd = writer.as_raw_fd();
        unsafe {
            cmd.pre_exec(move || {
                crate::sys::ignore_int_quit();
                if crate::sys::dup_fd(wfd, 3) {
                    Ok(())
                } else {
                    Err(std::io::Error::last_os_error())
                }
            });
        }
    }
    if o.stdin.is_some() {
        cmd.stdin(Stdio::piped());
    }
    if o.capture {
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    } else if o.quiet {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            drop(writer);
            return RunResult {
                code: 127,
                cwd: None,
                out: format!("Erreur d'exécution: {}", e),
            };
        }
    };
    drop(writer);
    if let Some(data) = o.stdin.clone() {
        if let Some(mut si) = child.stdin.take() {
            std::thread::spawn(move || {
                let _ = si.write_all(&data);
            });
        }
    }
    let mut out = String::new();
    if o.capture {
        let mut so = child.stdout.take();
        let mut se = child.stderr.take();
        let t = std::thread::spawn(move || {
            let mut s = String::new();
            if let Some(e) = se.as_mut() {
                let _ = e.read_to_string(&mut s);
            }
            s
        });
        if let Some(s) = so.as_mut() {
            let _ = s.read_to_string(&mut out);
        }
        out.push_str(&t.join().unwrap_or_default());
    }
    let code = child
        .wait()
        .map(|s| {
            s.code().unwrap_or_else(|| {
                use std::os::unix::process::ExitStatusExt;
                128 + s.signal().unwrap_or(0)
            })
        })
        .unwrap_or(-1);
    let mut final_dir = String::new();
    let _ = reader.read_to_string(&mut final_dir);
    let cwd = if final_dir.is_empty() {
        None
    } else if sandbox {
        Some(if final_dir == "/" {
            world.clone()
        } else {
            format!("{}{}", world.trim_end_matches('/'), final_dir)
        })
    } else {
        // mode dégradé: garde-fou, on ne sort jamais du monde
        if to_world(&final_dir).is_some() {
            Some(final_dir)
        } else {
            eprintln!("🚫 Ce passage mène hors du vaisseau: accès refusé.");
            None
        }
    };
    RunResult { code, cwd, out }
}

// ───────────────────────────── vérifications natives (P0.7) ─────────────────────────────

pub fn file_exists(path: &str) -> bool {
    std::fs::symlink_metadata(path).is_ok()
}
pub fn dir_exists(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn get_permissions(path: &str) -> Option<u32> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .ok()
        .map(|m| m.permissions().mode() & 0o7777)
}

pub fn is_executable(path: &str) -> bool {
    get_permissions(path)
        .map(|m| m & 0o100 != 0)
        .unwrap_or(false)
}

pub fn file_contains(path: &str, pattern: &str) -> bool {
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).contains(pattern))
        .unwrap_or(false)
}

pub fn file_contains_count(path: &str, pattern: &str) -> usize {
    if pattern.is_empty() {
        return 0;
    }
    std::fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).matches(pattern).count())
        .unwrap_or(0)
}

pub fn file_size(path: &str) -> u64 {
    std::fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}

pub fn canonical(path: &str) -> Option<String> {
    std::fs::canonicalize(path)
        .ok()
        .map(|p| p.to_string_lossy().to_string())
}

// ───────────────────────────── sauvegardes & historique ─────────────────────────────

/// Nom de fichier sûr pour un joueur
pub fn player_slug(name: &str) -> String {
    let s: String = name
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    let s = s.trim_matches('_').to_string();
    if s.is_empty() {
        "joueur".into()
    } else {
        s.chars().take(40).collect()
    }
}

/// Sauvegarde PAR JOUEUR et par campagne (P1.10): state/save_campagne<N>_<joueur>.json
pub fn save_path(campaign: u8, player: &str) -> String {
    format!(
        "{}/state/save_campagne{}_{}.json",
        gameshell_home(),
        campaign,
        player_slug(player)
    )
}

fn legacy_save_path(campaign: u8) -> String {
    format!("{}/state/save_campagne{}.json", gameshell_home(), campaign)
}

pub fn load_save(campaign: u8, player: &str) -> Option<crate::missions::SaveData> {
    if let Ok(content) = std::fs::read_to_string(save_path(campaign, player)) {
        return serde_json::from_str(&content).ok();
    }
    // ancienne sauvegarde (avant P1.10): reprise seulement si elle n'appartient à personne d'autre
    let content = std::fs::read_to_string(legacy_save_path(campaign)).ok()?;
    let sv: crate::missions::SaveData = serde_json::from_str(&content).ok()?;
    if sv.player_name.is_empty() || player_slug(&sv.player_name) == player_slug(player) {
        Some(sv)
    } else {
        None
    }
}

pub fn write_save(save: &crate::missions::SaveData) -> bool {
    if test_mode() {
        return true;
    }
    let path = save_path(save.campaign, &save.player_name);
    if let Some(parent) = Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match serde_json::to_string_pretty(save) {
        Ok(json) => {
            let tmp = format!("{}.tmp", path);
            std::fs::write(&tmp, json).is_ok() && std::fs::rename(&tmp, &path).is_ok()
        }
        Err(_) => false,
    }
}

pub fn remove_save(campaign: u8, player: &str) {
    let _ = std::fs::remove_file(save_path(campaign, player));
    if let Ok(c) = std::fs::read_to_string(legacy_save_path(campaign)) {
        if let Ok(sv) = serde_json::from_str::<crate::missions::SaveData>(&c) {
            if sv.player_name.is_empty() || player_slug(&sv.player_name) == player_slug(player) {
                let _ = std::fs::remove_file(legacy_save_path(campaign));
            }
        }
    }
}

pub fn history_path(campaign: u8, player: &str) -> String {
    format!(
        "{}/state/history_campagne{}_{}.log",
        gameshell_home(),
        campaign,
        player_slug(player)
    )
}

/// Historique horodaté des actions d'une partie (règle A22)
pub fn log_action(campaign: u8, player: &str, event: &str) {
    if test_mode() {
        return;
    }
    let path = history_path(campaign, player);
    if let Some(parent) = Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        let clean: String = event
            .chars()
            .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
            .collect();
        let _ = writeln!(f, "[{}] {}", ts, clean);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reecriture_mode_degrade() {
        let w = "/m";
        assert_eq!(rewrite_for_host("cat /pont/x", w).unwrap(), "cat /m/pont/x");
        assert_eq!(
            rewrite_for_host("cat \"/pont/x\" >/soutes/f", w).unwrap(),
            "cat \"/m/pont/x\" >/m/soutes/f"
        );
        assert_eq!(
            rewrite_for_host("echo \"a  b\" 2>/dev/null", w).unwrap(),
            "echo \"a  b\" 2>/dev/null"
        );
        assert_eq!(rewrite_for_host("ls '/pont'", w).unwrap(), "ls '/m/pont'");
        assert_eq!(
            rewrite_for_host("ls /pont/../armes", w).unwrap(),
            "ls /m/armes"
        );
        assert_eq!(
            rewrite_for_host("ls /pont/../../etc/passwd", w).unwrap_err(),
            "/pont/../../etc/passwd"
        );
        assert_eq!(rewrite_for_host("ls /..", w).unwrap_err(), "/..");
        assert_eq!(
            rewrite_for_host("ls '/../etc/passwd'", w).unwrap_err(),
            "/../etc/passwd"
        );
        // chemins hôtes déjà dans le monde: inchangés
        assert_eq!(rewrite_for_host("ls /m/pont", w).unwrap(), "ls /m/pont");
        assert_eq!(
            rewrite_for_host("cat /usr/bin/ls", w).unwrap(),
            "cat /usr/bin/ls"
        );
        assert_eq!(rewrite_for_host("cat /mfoo", "/m").unwrap(), "cat /m/mfoo");
    }
    #[test]
    fn slug() {
        assert_eq!(player_slug("Eve \"le pirate\""), "eve__le_pirate");
        assert_eq!(player_slug("  "), "joueur");
    }
    #[test]
    fn lexical_normalize_resolves_dots() {
        use std::path::Path;
        assert_eq!(lexical_normalize(Path::new("/a/b/c")), std::path::PathBuf::from("/a/b/c"));
        assert_eq!(lexical_normalize(Path::new("/a/b/./c")), std::path::PathBuf::from("/a/b/c"));
        assert_eq!(lexical_normalize(Path::new("/a/b/../c")), std::path::PathBuf::from("/a/c"));
        // ParentDir clampé au root physique : jamais de segments au-dessus de /
        assert_eq!(lexical_normalize(Path::new("/../../etc/passwd")), std::path::PathBuf::from("/etc/passwd"));
        // Chemin relatif : on garde le segment du début seulement
        assert_eq!(lexical_normalize(Path::new("a/b/../c")), std::path::PathBuf::from("a/c"));
    }
    #[test]
    fn camp_name_et_profil() {
        assert_eq!(camp_name(1), "espace");
        assert_eq!(camp_name(2), "japon");
        assert_eq!(camp_name(3), "reseau");
        assert_eq!(camp_name(99), "espace");
        assert_eq!(world_of("/home/x", 1), "/home/x/vaisseau");
        assert_eq!(world_of("/home/x", 2), "/home/x/monde_japon");
        assert_eq!(world_of("/home/x", 3), "/home/x/monde_reseau");
        assert_eq!(world_of("/home/x", 7), "/home/x/vaisseau");
        assert_eq!(player_slug("  TEST  "), "test");
        assert_eq!(player_slug(""), "joueur");
        assert_eq!(player_slug("a_b-c d!"), "a_b_c_d");
        // Troncature à 40 caractères
        assert_eq!(player_slug(&"x".repeat(100)).len(), 40);
        // home_in_world selon campagne
        assert_eq!(home_in_world(1), "/cabines/capitaine");
        assert_eq!(home_in_world(2), "/quartiers/seigneur");
        assert_eq!(home_in_world(3), "/noc/netrunner");
    }
}

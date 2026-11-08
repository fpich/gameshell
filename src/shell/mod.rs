//! Module shell - Exécute les vraies commandes dans le système
//! Le monde du jeu est isolé dans GAMESHELL_HOME/vaisseau/

use std::process::{Command, Stdio};
use std::io::Write;

/// Dossier d'installation du jeu (isolé)
pub fn gameshell_home() -> String {
    std::env::var("GAMESHELL_HOME")
        .unwrap_or_else(|_| {
            // Par défaut: ~/.gameshell
            format!("{}/.gameshell", std::env::var("HOME").unwrap_or_else(|_| ".".into()))
        })
}

/// Chemin vers le monde du jeu (dédié à la campagne choisie — règle D2)
pub fn world_dir() -> String {
    std::env::var("GAMESHELL_WORLD")
        .unwrap_or_else(|_| format!("{}/vaisseau", gameshell_home()))
}

/// Campagne active: 1 = Pirates de l'Espace (facile), 2 = Japon Féodal (moyen)
pub fn campaign_id() -> u8 {
    std::env::var("GAMESHELL_CAMPAIGN").ok().and_then(|v| v.parse().ok()).unwrap_or(1)
}

/// Sélectionne le monde d'une campagne (génère si absent) et ajuste HOME — règle D2
pub fn select_world(campaign: u8) {
    let home_dir = gameshell_home();
    let (world, home, camp) = match campaign {
        2 => (format!("{}/monde_japon", home_dir), format!("{}/monde_japon/quartiers/seigneur", home_dir), "japon"),
        3 => (format!("{}/monde_reseau", home_dir), format!("{}/monde_reseau/noc/netrunner", home_dir), "reseau"),
        _ => (format!("{}/vaisseau", home_dir), format!("{}/vaisseau/cabines/capitaine", home_dir), "espace"),
    };
    if !std::path::Path::new(&world).exists() {
        // monde absent (reset supprimé): régénère via vaisseau-gen.sh embarqué
        let gen = format!("{}/vaisseau-gen.sh", home_dir);
        if std::path::Path::new(&gen).exists() {
            let _ = Command::new("bash").arg(&gen).arg(camp).arg(&world).status();
        }
    }
    std::env::set_var("GAMESHELL_WORLD", &world);
    std::env::set_var("GAMESHELL_CAMPAIGN", campaign.to_string());
    std::env::set_var("HOME", &home);
}

/// Nom d'hôte du monde courant
pub fn world_hostname() -> &'static str {
    match campaign_id() { 2 => "shiro", 3 => "nexus", _ => "orion7" }
}

/// Nom du joueur dans le monde courant
pub fn world_user() -> &'static str {
    match campaign_id() { 2 => "seigneur", 3 => "netrunner", _ => "capitaine" }
}

/// Chemin vers le home du joueur dans le monde (défini par campagne via HOME)
pub fn player_home() -> String {
    std::env::var("HOME").unwrap_or_else(|_| format!("{}/cabines/capitaine", world_dir()))
}

/// Exécute une commande shell réelle et retourne (stdout, stderr, exit_code)
pub fn execute_real(command: &str) -> (String, String, i32) {
    // Expand ~ to $HOME since we may be in single quotes
    let home = std::env::var("HOME").unwrap_or_default();
    let expanded = command.replace("~/", &format!("{}/", home));
    let output = Command::new("bash")
        .arg("-c")
        .arg(&expanded)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            (stdout, stderr, out.status.code().unwrap_or(-1))
        }
        Err(e) => (String::new(), format!("Erreur d'exécution: {}", e), -1),
    }
}

/// Exécute une commande en interactif (hérite du terminal)
pub fn execute_interactive(command: &str) -> i32 {
    let home = std::env::var("HOME").unwrap_or_default();
    let expanded = command.replace("~/", &format!("{}/", home));
    let status = Command::new("bash")
        .arg("-c")
        .arg(&expanded)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(s) => s.code().unwrap_or(-1),
        Err(_) => -1,
    }
}

/// Vérifie si un fichier existe
pub fn file_exists(path: &str) -> bool {
    let (_, _, code) = execute_real(&format!("test -e '{}'", path));
    code == 0
}

/// Vérifie si un répertoire existe
pub fn dir_exists(path: &str) -> bool {
    let (_, _, code) = execute_real(&format!("test -d '{}'", path));
    code == 0
}

/// Vérifie si un utilisateur existe
pub fn user_exists(name: &str) -> bool {
    let (_, _, code) = execute_real(&format!("id '{}' 2>/dev/null", name));
    code == 0
}

/// Vérifie si un groupe existe
pub fn group_exists(name: &str) -> bool {
    let (_, _, code) = execute_real(&format!("getent group '{}' >/dev/null 2>&1", name));
    code == 0
}

/// Obtient les permissions d'un fichier (format octal)
pub fn get_permissions(path: &str) -> String {
    let (stdout, _, _) = execute_real(&format!("stat -c '%a' '{}' 2>/dev/null", path));
    stdout.trim().to_string()
}

/// Vérifie si un fichier contient un texte
pub fn file_contains(path: &str, pattern: &str) -> bool {
    let (_, _, code) = execute_real(&format!("grep -q '{}' '{}' 2>/dev/null", pattern, path));
    code == 0
}

/// Taille d'un fichier en octets (0 si absent)
pub fn file_size(path: &str) -> u64 {
    let (stdout, _, _) = execute_real(&format!("stat -c '%s' '{}' 2>/dev/null", path));
    stdout.trim().parse().unwrap_or(0)
}

/// Propriétaire d'un fichier (nom d'utilisateur, vide si absent)
pub fn file_owner(path: &str) -> String {
    let (stdout, _, _) = execute_real(&format!("stat -c '%U' '{}' 2>/dev/null", path));
    stdout.trim().to_string()
}

/// Nombre d'occurrences d'un motif dans un fichier
pub fn file_contains_count(path: &str, pattern: &str) -> usize {
    let (stdout, _, _) = execute_real(&format!("grep -o '{}' '{}' 2>/dev/null | wc -l", pattern, path));
    stdout.trim().parse().unwrap_or(0)
}

/// Vérifie si un processus tourne (par nom)
pub fn process_running(name: &str) -> bool {
    let (_, _, code) = execute_real(&format!("pgrep -x '{}' >/dev/null 2>&1 || pgrep -f '{}' >/dev/null 2>&1", name, name));
    code == 0
}

/// Vérifie si la sortie d'une commande contient un texte
pub fn output_contains(command: &str, pattern: &str) -> bool {
    let (stdout, _, _) = execute_real(command);
    stdout.contains(pattern)
}

/// Lit le contenu d'un fichier
#[allow(dead_code)]
pub fn read_file(path: &str) -> Option<String> {
    let (stdout, _stderr, code) = execute_real(&format!("cat '{}' 2>/dev/null", path));
    if code == 0 { Some(stdout) } else { None }
}

/// Chemin du fichier d'historique
pub fn history_path() -> String {
    std::env::var("GAMESHELL_HISTORY")
        .unwrap_or_else(|_| format!("{}/state/commands.log", gameshell_home()))
}

/// Enregistre une commande dans l'historique du jeu
pub fn log_command(cmd: &str) {
    let log_path = history_path();
    if let Some(parent) = std::path::Path::new(&log_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&log_path) {
        let _ = writeln!(f, "{}", cmd);
    }
}


/// Chemin de la sauvegarde d'une campagne (règle A18)
pub fn save_path(campaign: u8) -> String {
    format!("{}/state/save_campagne{}.json", gameshell_home(), campaign)
}

/// Charge la sauvegarde d'une campagne si elle existe
pub fn load_save(campaign: u8) -> Option<crate::missions::SaveData> {
    let content = std::fs::read_to_string(save_path(campaign)).ok()?;
    serde_json::from_str(&content).ok()
}

/// Écrit la sauvegarde d'une campagne
pub fn write_save(save: &crate::missions::SaveData) -> bool {
    if let Some(parent) = std::path::Path::new(&save_path(save.campaign)).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    match serde_json::to_string_pretty(save) {
        Ok(json) => std::fs::write(save_path(save.campaign), json).is_ok(),
        Err(_) => false,
    }
}

/// Historique horodaté des actions d'une partie (règle A22): state/history_campagne<N>.log
pub fn log_action(campaign: u8, event: &str) {
    let path = format!("{}/state/history_campagne{}.log", gameshell_home(), campaign);
    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        use std::io::Write;
        let _ = writeln!(f, "[{}] {}", ts, event);
    }
}

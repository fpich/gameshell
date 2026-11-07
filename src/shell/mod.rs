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

/// Chemin vers le monde du jeu
pub fn world_dir() -> String {
    format!("{}/vaisseau", gameshell_home())
}

/// Chemin vers le home du joueur dans le monde
#[allow(dead_code)]
pub fn player_home() -> String {
    format!("{}/cabines/capitaine", world_dir())
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

/// Obtient le répertoire courant réel
pub fn get_current_dir() -> String {
    let (stdout, _, _) = execute_real("pwd");
    stdout.trim().to_string()
}

/// Obtient le hostname du monde (orion7) ou le vrai hostname
pub fn get_hostname() -> String {
    "orion7".to_string()
}

/// Obtient l'utilisateur courant
#[allow(dead_code)]
pub fn get_current_user() -> String {
    let (stdout, _, _) = execute_real("whoami");
    stdout.trim().to_string()
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

/// Lit l'historique des commandes
pub fn read_command_history() -> Vec<String> {
    std::fs::read_to_string(history_path())
        .unwrap_or_default()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

/// Réinitialise le monde du jeu (reset)
#[allow(dead_code)]
pub fn reset_world() -> bool {
    let src = format!("{}/vaisseau", gameshell_home());
    let repo_world = concat!(env!("CARGO_MANIFEST_DIR"), "/vaisseau");
    // Copie le monde depuis le repo ou depuis une sauvegarde
    let (_, _, code) = execute_real(&format!(
        "cp -r '{}'/* '{}/' 2>/dev/null || true",
        repo_world, src
    ));
    code == 0
}

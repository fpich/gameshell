//! GameShell - Les Pirates de l'Espace
//! Jeu RPG éducatif pour apprendre l'administration shell Debian 13
//! Conçu pour BTS SIO SISR
//!
//! Architecture: Le binaire s'exécute comme un shell wrapper dans un conteneur
//! Debian 13. Il intercepte les commandes, les exécute réellement dans le shell,
//! puis vérifie si la mission est accomplie.

use clap::Parser;
use colored::Colorize;
use std::io::{self, Write};

mod game;
mod missions;
mod shell;

#[derive(Parser, Debug)]
#[command(name = "gameshell")]
#[command(about = "🚀 GameShell - Les Pirates de l'Espace - Apprenez le shell Debian 13 en jouant!", long_about = None)]
struct Cli {
    /// Mode test (sans interaction)
    #[arg(long)]
    test: bool,

    /// Mode shell direct (lance le jeu immédiatement)
    #[arg(long)]
    shell: bool,

    /// Nom du joueur
    #[arg(short, long)]
    name: Option<String>,
}

fn print_banner() {
    // Clear écran au lancement
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().ok();

    // Espace étoilé avec titre entouré d'étoiles
    let espace = r#"
  *  .  *     .  *    .  *     .  *   .  *  .  *
 .    *    .     *  .    *   .     *    .   *
    *   .  *    .   *    .    *  .    *  .    *
  .    *     .   *   .   *    .   *     .  *
     *   .    *    .  *    .    *   .    *   .

  *   🏴‍☠️   LES PIRATES DE L'ESPACE   🏴‍☠️   *
     *    Mission SISR — Station Orion-7    *

  .   *    .   *    .    *   .    *   .    *
     *    .   *    .   *    .  *    .   *
  *    .    *    .   *    .    *    .    *  .
"#;
    println!("{}", espace.green());
    println!();
}

fn main() {
    let cli = Cli::parse();

    print_banner();

    // Demander le nom du joueur
    let player_name = cli.name.unwrap_or_else(|| {
        let mut hist = Vec::new();
        let name = game::lineedit::read_line("👤 Entrez votre nom de capitaine: ", &mut hist)
            .unwrap_or_default();
        let name = name.trim().to_string();
        if name.is_empty() { "Capitaine".to_string() } else { name }
    });

    println!("\n👋 Bienvenue, {}!", player_name.bright_green().bold());
    println!("📦 Chargement des missions...\n");

    let mut game = game::Game::new(&player_name);

    if cli.test {
        game.run_test_mode();
    } else if std::env::var("GAMESHELL_AUTO").is_ok() {
        // Mode auto-solve (tests d'exhaustivité): joue toutes les missions
        let (ok, total) = game.run_auto();
        std::process::exit(if ok == total { 0 } else { 1 });
    } else {
        game.run();
    }
}

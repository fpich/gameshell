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

fn print_banner(campaign: u8) {
    // Clear écran au lancement
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().ok();

    if campaign == 2 {
        // Bannière Japon Féodal: torii et pagode
        let japon = r#"
           .    .        /
      .        .      /  \
  .     .        .  /____\      .
        .    .     /|    |\   .
   .        .     / |    | \      .
         .     .  |  |    |  |  .
   .        .    _|__|____|__|_    .
        .    .   |   SHIRO    |  .
   .        .    |____________|    .
  ________________/    ||    \_______________
  \   GARDE: [####..]   Honneur: [######..]   /
   \______________________________________/
      |##|                          |##|
      |##|   DOJO        PAGODE     |##|
      |##|                          |##|
"#;
        println!("{}", japon.red());
        println!();
        println!("{}", "  *   🏯   LE JAPON FÉODAL   🏯   *".red().bold());
        println!("{}", "     *  Mission Administration — Château Shiro  *".yellow());
        println!("{}", "  *        Niveau MOYEN — 100 missions        *".red());
    } else {
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
        println!("{}", "  *   🏴‍☠️   LES PIRATES DE L'ESPACE   🏴‍☠️   *".green().bold());
        println!("{}", "     *    Mission SISR — Station Orion-7    *".yellow());
        println!("{}", "  *        Niveau FACILE — 100 missions        *".green());
    }
    println!();
}

fn main() {
    let cli = Cli::parse();

    // Nom du joueur d'abord
    print!("\x1b[2J\x1b[H");
    println!("  ╔══════════════════════════════════════╗");
    println!("  ║          🕹️  GAMESHELL               ║");
    println!("  ║   Jeu RPG éducatif — shell Debian    ║");
    println!("  ╚══════════════════════════════════════╝\n");
    let player_name = cli.name.unwrap_or_else(|| {
        let mut hist = Vec::new();
        let name = game::lineedit::read_line("👤 Entrez votre nom: ", &mut hist)
            .unwrap_or_default();
        let name = name.trim().to_string();
        if name.is_empty() { "Capitaine".to_string() } else { name }
    });

    // Sélection de la campagne (règle C14) — défaut espace si non interactif
    let campaign: u8 = if std::env::var("GAMESHELL_AUTO").is_ok() {
        1 // le mode auto itère sur les 2 campagnes plus bas
    } else {
        println!("\n  ╔══════════════════════════════════════╗");
        println!("  ║     Choisissez votre CAMPAGNE        ║");
        println!("  ╚══════════════════════════════════════╝\n");
        println!("  1) 🏴‍☠️  Les Pirates de l'Espace — Station Orion-7  (niveau FACILE, 100 missions)");
        println!("  2) 🏯  Le Japon Féodal — Château Shiro           (niveau MOYEN, 100 missions)");
        println!();
        let mut hist = Vec::new();
        let choix = game::lineedit::read_line("👉 Numéro de campagne (1 ou 2): ", &mut hist).unwrap_or_default();
        if choix.trim() == "2" { 2 } else { 1 }
    };
    shell::select_world(campaign);
    print_banner(campaign);

    println!("\n👋 Bienvenue, {}!", player_name.bright_green().bold());
    println!("📦 Chargement des missions...\n");

    if cli.test {
        let mut game = game::Game::new(&player_name);
        game.run_test_mode();
    } else if std::env::var("GAMESHELL_AUTO").is_ok() {
        // Mode auto-solve: joue les 2 campagnes (100 missions chacune)
        let mut all_ok = true;
        for c in [1u8, 2] {
            shell::select_world(c);
            let mut game = game::Game::new(&player_name);
            let (ok, total) = game.run_auto();
            println!("🤖 AUTO campagne {} ({}): {}/{} missions résolues",
                c, if c == 2 { "Japon Féodal" } else { "Pirates de l'Espace" }, ok, total);
            if ok != total { all_ok = false; }
        }
        std::process::exit(if all_ok { 0 } else { 1 });
    } else {
        let mut game = game::Game::new(&player_name);
        game.run();
    }
}

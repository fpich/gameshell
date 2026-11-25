//! GameShell — RPG éducatif pour apprendre l'administration shell Debian 13 (BTS SIO SISR)
//!
//! Architecture: le binaire est un shell de jeu. Chaque ligne tapée s'exécute dans un bac à
//! sable bubblewrap où le monde de la campagne est la racine `/` (voir shell/mod.rs); les
//! commandes d'administration sont simulées (module sim) — la machine hôte n'est jamais
//! modifiée. Le même binaire sert de simulateur « multi-appel » (useradd, ufw, ps...).

use clap::Parser;
use colored::Colorize;
use std::io::{self, Write};

mod game;
mod missions;
mod shell;
mod sim;
mod sys;

#[derive(Parser, Debug)]
#[command(name = "gameshell")]
#[command(about = "🚀 GameShell — apprenez le shell Debian 13 en jouant (3 campagnes, 300 missions)", long_about = None)]
struct Cli {
    /// Mode test (liste les missions sans interaction)
    #[arg(long)]
    test: bool,

    /// Mode shell direct (compatibilité, sans effet)
    #[arg(long)]
    shell: bool,

    /// Nom du joueur
    #[arg(short, long)]
    name: Option<String>,

    /// Mode enseignant: impose la campagne (1, 2 ou 3) sans menu interactif
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=3))]
    campaign: Option<u8>,

    /// Mode enseignant: verrouille l'ordre des missions (gsh restart refusé)
    #[arg(long)]
    locked: bool,
}

/// Nettoie un nom de joueur: espaces blanchis avant/après, caractères de contrôle
/// supprimés, 32 caractères maximum. Vide (ou purement contrôle) → « Capitaine ».
fn sanitize_player_name(name: &str) -> String {
    let s: String = name
        .trim()
        .chars()
        .filter(|c| !c.is_control())
        .take(32)
        .collect();
    if s.is_empty() {
        "Capitaine".to_string()
    } else {
        s
    }
}

fn campaign_label(c: u8) -> &'static str {
    match c {
        2 => "Japon Féodal",
        3 => "Nexus Réseau",
        _ => "Pirates de l'Espace",
    }
}

fn print_banner(campaign: u8) {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().ok();
    if campaign == 2 {
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
"#;
        println!("{}", japon.red());
        println!("{}", "  *   🏯   LE JAPON FÉODAL   🏯   *".red().bold());
        println!(
            "{}",
            "     *  Mission Administration — Château Shiro  *".yellow()
        );
        println!(
            "{}",
            "  *        Niveau MOYEN — 100 missions        *".red()
        );
    } else if campaign == 3 {
        // P4.4: les ┌──┐ alignés au-dessus des │OK│
        let reseau = r#"
        ┌─[ NEXUS-9 ]──────────────────────────────┐
        │          ▮▮▮   NOC NEON-9   ▮▮▮          │
        │          ┌──┐  ┌──┐  ┌──┐  ┌──┐   BAIES  │
        │          │OK│  │OK│  │!!│  │OK│          │
        │          └──┘  └──┘  └──┘  └──┘          │
        │          UPLINK: ━━━━▶ 10.0.0.1          │
        └──────────────────────────────────────────┘
"#;
        println!("{}", reseau.cyan());
        println!("{}", "  *   🌐   NEXUS — NOC NEON-9   🌐   *".cyan().bold());
        println!("{}", "     *  Mission Administration Réseau    *".yellow());
        println!(
            "{}",
            "  *        Niveau DIFFICILE — 100 missions   *".cyan()
        );
    } else {
        // Règle B1: espace étoilé compact + titre entouré d'étoiles (une seule fois — P4.5)
        let espace = r#"
  *  .  *     .  *    .  *     .  *   .  *  .  *
 .    *    .     *  .    *   .     *    .   *
    *   .  *    .   *    .    *  .    *  .    *
"#;
        println!("{}", espace.green());
        println!(
            "{}",
            "  *   🏴‍☠️   LES PIRATES DE L'ESPACE   🏴‍☠️   *".green().bold()
        );
        println!(
            "{}",
            "     *    Mission SISR — Station Orion-7    *".yellow()
        );
        println!(
            "{}",
            "  *        Niveau FACILE — 100 missions        *".green()
        );
        println!("{}", "\n  .   *    .   *    .    *   .    *   .    *\n     *    .   *    .   *    .  *    .   *".green());
    }
    println!();
}

fn main() {
    // Binaire multi-appel: lancé sous le nom d'une commande simulée (useradd, ufw, ps...)
    let argv: Vec<String> = std::env::args().collect();
    let argv0 = argv
        .first()
        .map(|a| a.rsplit('/').next().unwrap_or(a).to_string())
        .unwrap_or_default();
    if argv0 != "gameshell" && argv0 != "gsh-sim" && sim::is_sim(&argv0) {
        std::process::exit(sim::run(&argv0, argv[1..].to_vec()));
    }
    if argv0 == "gsh-sim" {
        // relais: rétablit Ctrl+C par défaut puis lance la commande (voir shell::run_line)
        if argv.get(1).map(|s| s.as_str()) == Some("__exec") && argv.len() > 2 {
            use std::os::unix::process::CommandExt;
            sys::default_int_quit();
            let e = std::process::Command::new(&argv[2]).args(&argv[3..]).exec();
            eprintln!("gsh-sim: {}", e);
            std::process::exit(126);
        }
        if let Some(name) = argv.get(1) {
            if sim::is_sim(name) {
                std::process::exit(sim::run(name, argv[2..].to_vec()));
            }
        }
        eprintln!("gsh-sim: simulateur des commandes système de GameShell");
        std::process::exit(2);
    }

    let cli = Cli::parse();
    // P1.3: Ctrl+C interrompt la commande en cours, jamais le jeu
    sys::protect_parent_from_signals();

    let auto = std::env::var("GAMESHELL_AUTO").is_ok();
    let edge = std::env::var("GAMESHELL_EDGE").is_ok();
    let torture = std::env::var("GAMESHELL_TORTURE").is_ok();
    let bench = auto || edge || torture;

    print!("\x1b[2J\x1b[H");
    println!("  ╔══════════════════════════════════════╗");
    println!("  ║           🕹️  GAMESHELL               ║");
    println!("  ║   Jeu RPG éducatif — shell Debian    ║");
    println!("  ╚══════════════════════════════════════╝\n");
    let player_name = cli
        .name
        .clone()
        .map(|n| sanitize_player_name(&n))
        .unwrap_or_else(|| {
            let mut hist = Vec::new();
            let name = game::lineedit::read_line("👤 Entrez votre nom: ", &mut hist, None)
                .unwrap_or_default();
            sanitize_player_name(&name)
        });

    // Sélection de la campagne (règle C14) — mode enseignant: --campaign impose la campagne
    let campaign: u8 = if let Some(c) = cli.campaign {
        c
    } else if bench {
        1
    } else {
        println!("\n  ╔══════════════════════════════════════╗");
        println!("  ║       Choisissez votre CAMPAGNE      ║");
        println!("  ╚══════════════════════════════════════╝\n");
        println!(
            "  1) 🏴‍☠️  Les Pirates de l'Espace — Station Orion-7  (niveau FACILE, 100 missions)"
        );
        println!("  2) 🏯  Le Japon Féodal — Château Shiro           (niveau MOYEN, 100 missions)");
        println!(
            "  3) 🌐  Nexus — NOC Neon-9 (réseau)               (niveau DIFFICILE, 100 missions)"
        );
        println!();
        let mut hist = Vec::new();
        let choix =
            game::lineedit::read_line("👉 Numéro de campagne (1, 2 ou 3): ", &mut hist, None)
                .unwrap_or_default();
        match choix.trim() {
            "2" => 2,
            "3" => 3,
            _ => 1,
        }
    };
    if cli.locked {
        std::env::set_var("GAMESHELL_LOCKED", "1");
    }

    if bench {
        // Bancs de test: les 3 campagnes, sans sauvegarde ni historique (règle A18, P1.9)
        let mut all_ok = true;
        for c in [1u8, 2, 3] {
            shell::select_world(c);
            let mut g = game::Game::new(&player_name, true);
            let name = campaign_label(c);
            if torture {
                let (ok, issues) = g.run_torture();
                println!(
                    "🌪️  TORTURE campagne {} ({}): {}/100 OK, {} problèmes",
                    c,
                    name,
                    ok,
                    issues.len()
                );
                for i in &issues {
                    println!("   ⚠️  {}", i);
                }
                if ok != 100 || !issues.is_empty() {
                    all_ok = false;
                }
            } else if edge {
                let (ok, issues) = g.run_edge();
                println!(
                    "🧪 EDGE campagne {} ({}): {}/100 missions OK, {} problèmes",
                    c,
                    name,
                    ok,
                    issues.len()
                );
                for i in &issues {
                    println!("   ⚠️  {}", i);
                }
                if ok != 100 || !issues.is_empty() {
                    all_ok = false;
                }
            } else {
                let (ok, total) = g.run_auto();
                println!(
                    "🤖 AUTO campagne {} ({}): {}/{} missions résolues",
                    c, name, ok, total
                );
                if ok != total {
                    all_ok = false;
                }
            }
        }
        std::process::exit(if all_ok { 0 } else { 1 });
    }

    shell::select_world(campaign);
    print_banner(campaign);
    println!("👋 Bienvenue, {}!", player_name.bright_green().bold());
    println!("📦 Chargement des missions...\n");

    let mut g = game::Game::new(&player_name, false);
    if cli.test {
        g.run_test_mode();
    } else {
        g.run();
    }
}

#[cfg(test)]
mod tests {
    use super::{campaign_label, sanitize_player_name};

    #[test]
    fn sanitize_nom_nettoyage_et_troncature() {
        assert_eq!(sanitize_player_name("  Capitaine  "), "Capitaine");
        assert_eq!(sanitize_player_name(""), "Capitaine");
        assert_eq!(sanitize_player_name("   "), "Capitaine");
        assert_eq!(sanitize_player_name("\u{0007}\u{001b}"), "Capitaine");
        // caractères de contrôle supprimés (pas les mutateurs)
        assert_eq!(sanitize_player_name("Capi\u{0007}taire"), "Capitaire");
        // troncature à 32 caractères
        assert_eq!(sanitize_player_name(&"x".repeat(100)).len(), 32);
        // nom enseignant transmis tel quel (après nettoyage)
        assert_eq!(sanitize_player_name("Alice 3000"), "Alice 3000");
        // emoji comptés comme caractères (pas octets)
        assert_eq!(sanitize_player_name("🏴‍☠️").chars().count(), 4);
    }

    #[test]
    fn campaign_label_couvre_les_3_campagnes() {
        assert_eq!(campaign_label(1), "Pirates de l'Espace");
        assert_eq!(campaign_label(2), "Japon Féodal");
        assert_eq!(campaign_label(3), "Nexus Réseau");
        // repli sur la campagne 1 (jamais de panic, jamais de titre vide)
        assert_eq!(campaign_label(0), "Pirates de l'Espace");
        assert_eq!(campaign_label(99), "Pirates de l'Espace");
    }
}

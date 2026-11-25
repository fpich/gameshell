//! Moteur GameShell
//! - chaque ligne SHELL s'exécute dans le bac à sable (shell::run_line), jamais sur l'hôte
//! - validation fiable: analyse de la ligne (parse.rs), seules les commandes RÉUSSIES comptent
//! - phase 2 (code) sans bloquer le shell: `gsh code <CODE>`
//! - pas de gsh hint (règle A9), gsh check ne révèle jamais rien (règle A8)

use crate::missions::{load_all_missions, Mission, Skill};
use crate::shell;
use colored::Colorize;
use std::io::{self, Write};

mod bench;
pub mod brief;
pub mod lineedit;
pub mod parse;
pub mod ui;

pub use brief::cmd_brief;

pub struct Game {
    player_name: String,
    campaign: u8,
    missions: Vec<Mission>,
    current_mission: usize,
    xp: u64,
    level: u32,
    skills: Vec<Skill>,
    completed: Vec<String>,
    badges: Vec<String>,
    current_dir: String, // chemin RÉEL (hôte) du dossier courant, toujours dans le monde
    previous_dir: String, // pour `cd -`
    pending_code: Option<String>, // code attendu en phase 2
    check_fails: u32,    // échecs consécutifs de gsh check (UX, règle A14)
    attempts: std::collections::HashMap<String, u32>,
    mission_commands: Vec<String>, // lignes RÉUSSIES tapées dans la mission courante (règle A2)
    history: Vec<String>,          // historique de session (flèches haut/bas)
    test_mode: bool,               // bancs AUTO/EDGE/TORTURE: ni sauvegarde ni historique (P1.9)
    event_mark: usize, // événements d'administration antérieurs à la mission (checks sim_event)
    bench_quiet: bool, // sortie des commandes masquée dans les bancs
}

fn start_skills() -> Vec<Skill> {
    vec![Skill {
        command: "man".into(),
        name: "man".into(),
        description: "man COMMANDE — affiche le manuel (compétence de départ)".into(),
    }]
}

/// Le code de phase 2 peut être tapé seul s'il « ressemble » à un code (MAJUSCULES/chiffres/tirets)
fn looks_like_code(w: &str) -> bool {
    !w.is_empty()
        && w.chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-')
        && w.chars().any(|c| c.is_ascii_alphanumeric())
}

impl Game {
    pub fn new(player_name: &str, test_mode: bool) -> Self {
        let campaign = shell::campaign_id();
        let missions = load_all_missions();
        shell::set_test_mode(test_mode);
        let home = shell::player_home();
        let save = if test_mode {
            None
        } else {
            shell::load_save(campaign, player_name)
        };
        let mut game = Game {
            player_name: player_name.to_string(),
            campaign,
            missions,
            current_mission: 0,
            xp: 0,
            level: 1,
            skills: start_skills(),
            completed: Vec::new(),
            badges: Vec::new(),
            current_dir: home.clone(),
            previous_dir: home,
            pending_code: None,
            check_fails: 0,
            attempts: std::collections::HashMap::new(),
            mission_commands: Vec::new(),
            history: Vec::new(),
            test_mode,
            bench_quiet: test_mode && std::env::var("GSH_VERBOSE").is_err(),
            event_mark: 0,
        };
        game.mark_events();
        if let Some(sv) = save {
            if sv.campaign == game.campaign && sv.current_mission <= game.missions.len() {
                game.current_mission = sv.current_mission;
                game.xp = sv.xp;
                game.level = sv.level;
                game.skills = sv.skills;
                game.completed = sv.completed;
                game.badges = sv.badges;
                game.attempts = sv.attempts;
            }
        }
        game
    }

    /// Les checks `sim_event` ne regardent que les actions faites PENDANT la mission en cours
    fn mark_events(&mut self) {
        self.event_mark = shell::sim_ctx().lines("events.log").len();
    }

    fn log(&self, event: &str) {
        if !self.test_mode {
            shell::log_action(self.campaign, &self.player_name, event);
        }
    }

    /// Écrit la sauvegarde de la campagne (règle A18) — jamais en mode test (P1.9)
    fn save_progress(&self) {
        if self.test_mode {
            return;
        }
        let save = crate::missions::SaveData {
            campaign: self.campaign,
            player_name: self.player_name.clone(),
            current_mission: self.current_mission,
            xp: self.xp,
            level: self.level,
            skills: self.skills.clone(),
            completed: self.completed.clone(),
            badges: self.badges.clone(),
            attempts: self.attempts.clone(),
            saved_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        if !shell::write_save(&save) {
            println!(
                "  ⚠️  Sauvegarde impossible dans {}",
                shell::save_path(self.campaign, &self.player_name)
            );
        }
    }

    pub fn run(&mut self) {
        let (titre, role, camp, niveau) = match self.campaign {
            2 => (
                "🏯 CHÂTEAU SHIRO — Terminal du Seigneur",
                "Seigneur",
                "Le Japon Féodal",
                "MOYEN",
            ),
            3 => (
                "🌐 NEXUS — NOC Neon-9",
                "Netrunner",
                "Nexus Réseau",
                "DIFFICILE",
            ),
            _ => (
                "🚀 ORION-7 — Terminal du Capitaine",
                "Admin",
                "Les Pirates de l'Espace",
                "FACILE",
            ),
        };
        println!();
        println!(
            "{}",
            ui::simple_box(
                &[
                    titre.to_string(),
                    format!("{}: {}", role, self.player_name),
                    format!("Campagne: {}  │  Niveau: {}", camp, niveau)
                ],
                60
            )
        );
        if !shell::sandbox_available() {
            println!(
                "\n  {}",
                "⚠️  Bac à sable bubblewrap indisponible: isolation PARTIELLE du monde."
                    .yellow()
                    .bold()
            );
            println!("     Demandez l'installation du paquet « bubblewrap » (sudo apt install bubblewrap).");
        }
        self.cmd_help();
        self.show_tutorial_if_first_run();
        if self.current_mission >= self.missions.len() {
            self.show_victory_screen();
            println!("  🔄 Tapez 'gsh restart' pour rejouer la campagne.\n");
        } else {
            self.show_mission();
        }

        self.log(&format!(
            "SESSION début — joueur={} campagne={}",
            self.player_name, self.campaign
        ));
        loop {
            let cwd = self.display_path(&self.current_dir);
            let prompt = format!(
                "{}@{}:{}$ [{}/{}] ",
                self.player_name.bright_green(),
                shell::world_hostname().normal(),
                cwd.normal(),
                self.completed.len(),
                self.missions.len()
            );
            let completer = lineedit::Completer {
                cwd: self.current_dir.clone(),
                world: shell::world_dir(),
                home: shell::player_home(),
            };
            // Règle A11: édition de ligne façon terminal (flèches, historique, Tab)
            let line = match lineedit::read_line(&prompt, &mut self.history, Some(&completer)) {
                Some(l) => l,
                None => {
                    println!("\n👋 Déconnexion (EOF).");
                    break;
                } // Ctrl+D = sortie propre
            };
            let cmd = line.trim().to_string();
            if cmd.is_empty() {
                continue;
            }
            if !self.handle_line(&cmd) {
                break;
            }
        }
        self.log("SESSION fin");
    }

    /// Traite une ligne tapée. Retourne false pour quitter.
    fn handle_line(&mut self, cmd: &str) -> bool {
        let words: Vec<&str> = cmd.split_whitespace().collect(); // règle A5
        if words.first() == Some(&"gsh") {
            match words.get(1).copied() {
                Some("help") => self.cmd_help(),
                Some("goal") => self.cmd_goal(),
                Some("check") => {
                    if self.cmd_check() {
                        self.show_mission();
                    }
                }
                Some("code") => match words.get(2) {
                    Some(c) => {
                        if self.submit_code(c) {
                            self.show_mission();
                        }
                    }
                    None => println!("  Usage: gsh code VOTRE-CODE"),
                },
                Some("cancel") => {
                    if self.pending_code.take().is_some() {
                        println!("\n  ↩️  Saisie du code annulée. Refaites 'gsh check' quand vous serez prêt.\n");
                    } else {
                        println!("\n  Aucun code n'est attendu pour le moment.\n");
                    }
                }
                Some("reset") => self.cmd_reset(),
                Some("skills") => self.cmd_skills(),
                Some("export") => self.cmd_export(),
                Some("restart") => self.cmd_restart(),
                _ => println!(
                    "  Commande gsh inconnue. Tapez 'gsh help' pour la liste des commandes gsh."
                ),
            }
            return true;
        }
        // Phase 2: un code tapé seul est accepté (sinon la ligne est une commande SHELL normale)
        if self.pending_code.is_some() && words.len() == 1 && looks_like_code(words[0]) {
            if self.submit_code(words[0]) {
                self.show_mission();
            }
            return true;
        }
        if words.len() == 1 && matches!(words[0], "exit" | "quit" | "logout") {
            println!("\n👋 Déconnexion.");
            return false;
        }
        if words.len() == 1 && words[0] == "clear" {
            print!("\x1b[2J\x1b[H");
            io::stdout().flush().ok();
            return true;
        }
        self.log(&format!("CMD {}", cmd));
        self.exec_shell(cmd);
        true
    }

    /// RÈGLE A8-compatible: rm visant un fichier critique de la mission → avertissement narratif
    fn warn_destructive(&self, parsed: &parse::Parsed) -> bool {
        let m = match self.missions.get(self.current_mission) {
            Some(m) => m,
            None => return false,
        };
        let critical: Vec<String> = m
            .checks
            .iter()
            .filter(|ck| {
                matches!(
                    ck.check_type.as_str(),
                    "file_exists"
                        | "file_read"
                        | "file_size"
                        | "file_owner"
                        | "file_contains_count"
                        | "config_contains"
                        | "permission"
                )
            })
            .map(|ck| ck.target.clone())
            .collect();
        if critical.is_empty() {
            return false;
        }
        let hits = parsed
            .effective()
            .iter()
            .filter(|(b, _)| b == "rm")
            .any(|(_, args)| {
                args.iter().filter(|a| !a.starts_with('-')).any(|a| {
                    let ra = self.resolve_path(a);
                    critical.iter().any(|c| {
                        let rc = self.resolve_path(c);
                        let same = shell::canonical(&ra).unwrap_or(ra.clone())
                            == shell::canonical(&rc).unwrap_or(rc.clone());
                        same || (!c.contains('/') && ra.rsplit('/').next() == Some(c.as_str()))
                    })
                })
            });
        if hits {
            println!("\n  ⚠️  Cette commande est DESTRUCTRICE et viserait un élément important de la mission.");
            println!("  🚀 L'équipage vous déconseille fortement cette action: elle pourrait compromettre la mission.");
            println!("  💡 Si vous avez déjà modifié l'état du monde, tapez 'gsh reset' pour repartir proprement.");
        }
        hits
    }

    /// Exécute une ligne SHELL; enregistre la ligne si elle a RÉUSSI (P1.1.3). Retourne le code de sortie.
    fn exec_shell(&mut self, line: &str) -> i32 {
        self.exec_with(line, None)
    }

    fn exec_with(&mut self, line: &str, stdin: Option<Vec<u8>>) -> i32 {
        let parsed = parse::parse(line);
        if parsed.cmds.is_empty() {
            return 0;
        }
        if self.warn_destructive(&parsed) {
            return 1;
        }
        // Interceptions (uniquement pour une commande simple, sans pipe ni redirection — P1.4)
        if parsed.is_simple() {
            let w = &parsed.cmds[0].words;
            match (w[0].as_str(), w.len()) {
                ("pwd", 1) => {
                    if !self.bench_quiet {
                        println!("{}", self.display_path(&self.current_dir));
                    }
                    self.mission_commands.push(line.to_string());
                    return 0;
                }
                ("man", 2) => {
                    // Règle C7: double définition roleplay PUIS technique
                    let (tech, rp) = cmd_brief(&w[1]);
                    println!("🚀 {}", rp);
                    println!("💻 {}", tech);
                    println!("   (manuel complet: man {} | less)", w[1]);
                    self.mission_commands.push(line.to_string());
                    return 0;
                }
                ("man", 1) => {
                    println!("Quelle page de manuel voulez-vous ?\nPar exemple: man ls — consultez gsh help pour la syntaxe générale.");
                    return 1;
                }
                _ => {}
            }
        }
        let opts = shell::RunOpts {
            capture: false,
            quiet: self.bench_quiet,
            stdin,
        };
        let r = shell::run_line(line, &self.current_dir, &self.previous_dir, &opts);
        if let Some(nd) = r.cwd {
            if nd != self.current_dir {
                self.previous_dir = std::mem::replace(&mut self.current_dir, nd);
            }
        }
        if r.code == 0 {
            self.mission_commands.push(line.to_string());
        }
        r.code
    }

    fn cmd_help(&self) {
        println!("\n  📘 Pour revoir l'aide complète: gsh help");
        println!("\n  🧠 Taxonomie de Bloom: chaque mission a un niveau (Se souvenir → Créer).");
        println!("  🎖️ Compétence de départ: man COMMANDE (manuel) — déjà acquise!");
        println!("\n  💻 COMMANDES SHELL — syntaxe générale:");
        println!("    COMMANDE                  → exécute la commande seule (ex: pwd)");
        println!("    COMMANDE argument         → avec un argument (ex: ls /pont)");
        println!("    COMMANDE -option argument → avec une option (ex: ls -la /pont)");
        println!("    man COMMANDE              → affiche le manuel de la commande");
        println!("    COMMANDE | COMMANDE       → chaîne deux commandes");
        println!("    COMMANDE > fichier        → redirige la sortie vers un fichier");
        println!("\n  🎮 COMMANDES GSH:");
        println!("    gsh check        → valider la mission     gsh goal    → revoir la mission");
        println!(
            "    gsh code CODE    → donner le code demandé gsh cancel  → annuler la saisie du code"
        );
        println!("    gsh skills       → vos compétences        gsh reset   → réinitialiser la mission en cours");
        println!(
            "    gsh export       → export enseignant      gsh restart → recommencer la campagne"
        );
        println!("\n  📚 GLOSSAIRE SHELL (rappel rapide):");
        println!("    pwd  où suis-je?    ls  voir le contenu    cd  changer de dossier");
        println!("    cat  lire un fichier   mkdir  créer un dossier   touch  créer un fichier");
        println!("    cp  copier   mv  déplacer/renommer   rm  supprimer (⚠️ destructeur)");
        println!("    chmod  changer les droits   chown  changer le propriétaire");
        println!("    grep  chercher dans un texte   find  chercher des fichiers");
        println!("    echo  afficher un texte   man  manuel   |  chaîner   >  rediriger");
        println!("    Tab complète les noms, ↑ ↓ rappellent les commandes, Ctrl+C interrompt\n");
    }

    fn cmd_goal(&self) {
        self.show_mission();
    }

    fn cmd_check(&mut self) -> bool {
        if self.current_mission >= self.missions.len() {
            println!("\n  🏆 Campagne terminée! Tapez 'gsh restart' pour rejouer.\n");
            return false;
        }
        if self.pending_code.is_some() {
            println!("\n  🔐 Les actions sont validées: il ne manque que le code. Tapez: gsh code VOTRE-CODE\n");
            return false;
        }
        let (ok, missing) = self.check_mission_detailed();
        let mid = self.missions[self.current_mission].id.clone();
        if !ok {
            let _ = missing; // JAMAIS révéler quoi que ce soit (règle A8)
            self.check_fails += 1;
            *self.attempts.entry(mid.clone()).or_insert(0) += 1;
            self.log(&format!("CHECK mission {} -> échec", mid));
            println!("\n  ❌ Mission non validée. Relisez le contexte et réessayez.");
            if self.check_fails.is_multiple_of(3) {
                // Règle A8: message générique, JAMAIS la commande/fichier manquant (P1.7)
                println!(
                    "  🧭 Relisez le dialogue du PNJ: il vous dit exactement quoi faire et où."
                );
                println!("     Vérifiez que vous êtes au bon endroit (pwd) et que vos commandes n'ont pas affiché d'erreur.");
            } else if self.check_fails % 3 == 2 {
                println!("  💡 Besoin de revoir l'objectif? Tapez 'gsh goal'");
            }
            return false;
        }
        self.log(&format!("CHECK mission {} -> succès", mid));
        // Phase 2: la mission demande un code
        if let Some(code) = self.missions[self.current_mission].code_required.clone() {
            println!(
                "\n  🔐 Actions validées! Dernière étape: donnez le code trouvé dans les fichiers."
            );
            println!(
                "     Tapez: gsh code VOTRE-CODE   (vous pouvez continuer à explorer en attendant)"
            );
            self.pending_code = Some(code);
            return false;
        }
        self.check_fails = 0;
        let xp = self.missions[self.current_mission].xp_reward;
        self.log(&format!("MISSION {} réussie (+{} XP)", mid, xp));
        println!("\n  ✅ Mission réussie!");
        self.complete_mission();
        true
    }

    /// Rétroaction positive contextualisée par niveau de Bloom et par campagne
    fn congrats(&self, m: &Mission) -> (&'static str, &'static str) {
        let a = match shell::campaign_id() {
            2 => "Le château Shiro vous salue, seigneur.",
            3 => "Le NOC Neon-9 valide votre intervention.",
            _ => "La station Orion-7 vous félicite, Capitaine.",
        };
        let b = match m.bloom_level.as_str() {
            "Se souvenir" => "Vous maîtrisez désormais ce réflexe essentiel.",
            "Comprendre" => "Vous ne l'exécutez plus: vous le comprenez.",
            "Appliquer" => "Vous savez mettre cette compétence en pratique.",
            "Analyser" => "Vous savez décomposer et raisonner sur la situation.",
            "Évaluer" => "Vous savez juger et choisir la bonne action.",
            "Créer" => "Vous avez construit quelque chose de nouveau.",
            _ => "L'équipage vous salue avec respect.",
        };
        (a, b)
    }

    fn complete_mission(&mut self) {
        let m = match self.missions.get(self.current_mission) {
            Some(m) => m.clone(),
            None => return,
        };
        let (congrats, bloom_note) = self.congrats(&m);
        println!("  🎖️  {}", congrats);
        println!("  🧠 {}", bloom_note);
        if let Some(skill) = &m.skill {
            if !self.skills.iter().any(|s| s.name == skill.name) {
                let (tech, _) = cmd_brief(&skill.name);
                println!("  🔓 Compétence: {} — {}", skill.name, tech);
                self.skills.push(skill.clone());
            }
        }
        self.xp += m.xp_reward;
        // plusieurs niveaux possibles d'un coup (P1.14)
        while self.xp >= self.level as u64 * 100 {
            self.level += 1;
            println!("  🎉 Niveau {}!", self.level);
        }
        if !self.completed.contains(&m.id) {
            self.completed.push(m.id.clone());
        }
        self.current_mission += 1;
        self.mission_commands.clear(); // règle A2: historique propre à chaque mission
        self.pending_code = None;
        self.mark_events();
        self.update_badges();
        // Règle A7: retour à la cabine (HOME) pour la nouvelle mission
        self.current_dir = shell::player_home();
        self.previous_dir = self.current_dir.clone();
        self.save_progress(); // A18
        if self.current_mission >= self.missions.len()
            && self.completed.len() >= self.missions.len()
        {
            self.show_victory_screen(); // A19
        }
    }

    /// Débloque des badges thématiques selon les compétences acquises
    fn update_badges(&mut self) {
        let has = |cmds: &[&str]| {
            cmds.iter().any(|c| {
                self.skills
                    .iter()
                    .any(|s| s.name.split_whitespace().next() == Some(*c))
            })
        };
        let mut new_badges: Vec<(&str, &str)> = Vec::new();
        if has(&["ls", "cd", "pwd", "mkdir", "touch", "cp", "mv", "rm"]) {
            new_badges.push(("🗂️ Explorateur", "maîtrise des fichiers et répertoires"));
        }
        if has(&["cat", "grep", "head", "tail", "wc", "find", "less"]) {
            new_badges.push(("🔍 Fouineur", "lecture et recherche dans les fichiers"));
        }
        if has(&[
            "chmod", "chown", "useradd", "groupadd", "passwd", "sudo", "su",
        ]) {
            new_badges.push(("🛡️ Gardien", "droits, utilisateurs et sécurité"));
        }
        if has(&["ps", "top", "kill", "pgrep", "systemctl", "journalctl"]) {
            new_badges.push(("⚙️ Mécanicien", "processus et services"));
        }
        if has(&[
            "ip",
            "ping",
            "netstat",
            "nmap",
            "ssh",
            "curl",
            "dig",
            "traceroute",
            "iptables",
            "ufw",
        ]) {
            new_badges.push(("🌐 Réseau", "réseau et connectivité"));
        }
        if has(&["tar", "zip", "unzip", "gzip", "df", "du", "mount"]) {
            new_badges.push(("📦 Logisticien", "archives et stockage"));
        }
        for (name, desc) in new_badges {
            if !self.badges.iter().any(|b| b.starts_with(name)) {
                println!("  🏅 Badge débloqué: {} — {}", name, desc);
                self.badges.push(format!("{} — {}", name, desc));
            }
        }
    }

    /// Écran de fin par campagne (règle A19) — bordures alignées (P4.2)
    fn show_victory_screen(&self) {
        let (campagne, bastion) = match self.campaign {
            2 => ("🏯 LE JAPON FÉODAL", "le château Shiro"),
            3 => ("🌐 NEXUS — NOC NEON-9", "le NOC Nexus"),
            _ => ("🏴‍☠️ LES PIRATES DE L'ESPACE", "Orion-7"),
        };
        let mut lines = vec![
            format!("{} est sauvé(e)!", bastion),
            "----".into(),
            format!("Campagne: {}", campagne),
            format!("Missions: {}/{}", self.completed.len(), self.missions.len()),
            format!("XP: {}  │  Niveau: {}", self.xp, self.level),
            format!(
                "Compétences: {}  │  Badges: {}",
                self.skills.len(),
                self.badges.len()
            ),
            "----".into(),
        ];
        let names: Vec<String> = self
            .skills
            .iter()
            .map(|s| format!("✓ {}", s.name))
            .collect();
        for chunk in names.chunks(3) {
            lines.push(
                chunk
                    .iter()
                    .map(|n| ui::pad(n, 19))
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim_end()
                    .to_string(),
            );
        }
        if !self.badges.is_empty() {
            lines.push("----".into());
            for b in &self.badges {
                lines.push(format!("🏅 {}", b.split(" — ").next().unwrap_or(b)));
            }
        }
        println!("\n{}\n", ui::heavy_box("🏆 FÉLICITATIONS!", &lines, 60));
    }

    /// gsh export: progression en JSON + CSV (règle A20), JSON valide (P1.8)
    fn cmd_export(&self) {
        #[derive(serde::Serialize)]
        struct MissionRow<'a> {
            id: &'a str,
            titre: &'a str,
            statut: &'a str,
            xp: u64,
            tentatives: u32,
        }
        #[derive(serde::Serialize)]
        struct Export<'a> {
            campagne: u8,
            joueur: &'a str,
            date: String,
            missions: Vec<MissionRow<'a>>,
            missions_reussies: usize,
            xp_total: u64,
            niveau: u32,
            competences: Vec<&'a str>,
            badges: Vec<&'a str>,
            historique: Vec<String>,
        }
        let dir = format!("{}/state", shell::gameshell_home());
        let _ = std::fs::create_dir_all(&dir);
        let stamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let slug = shell::player_slug(&self.player_name);
        let json_path = format!(
            "{}/export_campagne{}_{}_{}.json",
            dir, self.campaign, slug, stamp
        );
        let csv_path = format!(
            "{}/export_campagne{}_{}_{}.csv",
            dir, self.campaign, slug, stamp
        );
        let done: std::collections::HashSet<&str> =
            self.completed.iter().map(|s| s.as_str()).collect();
        let hist = std::fs::read_to_string(shell::history_path(self.campaign, &self.player_name))
            .unwrap_or_default();
        let rows: Vec<MissionRow> = self
            .missions
            .iter()
            .map(|m| MissionRow {
                id: &m.id,
                titre: &m.title,
                statut: if done.contains(m.id.as_str()) {
                    "réussie"
                } else {
                    "à faire"
                },
                xp: m.xp_reward,
                tentatives: self.attempts.get(&m.id).copied().unwrap_or(0),
            })
            .collect();
        let csv_field =
            |s: &str| format!("\"{}\"", s.replace('"', "\"\"").replace(['\n', '\r'], " "));
        let mut csv = String::from("id;titre;statut;xp;tentatives\n");
        for r in &rows {
            csv.push_str(&format!(
                "{};{};{};{};{}\n",
                csv_field(r.id),
                csv_field(r.titre),
                csv_field(r.statut),
                r.xp,
                r.tentatives
            ));
        }
        let ex = Export {
            campagne: self.campaign,
            joueur: &self.player_name,
            date: stamp.clone(),
            missions: rows,
            missions_reussies: self.completed.len(),
            xp_total: self.xp,
            niveau: self.level,
            competences: self.skills.iter().map(|s| s.name.as_str()).collect(),
            badges: self.badges.iter().map(|s| s.as_str()).collect(),
            historique: hist.lines().map(|l| l.to_string()).collect(),
        };
        let okj = serde_json::to_string_pretty(&ex)
            .map(|j| std::fs::write(&json_path, j).is_ok())
            .unwrap_or(false);
        let okc = std::fs::write(&csv_path, csv).is_ok();
        self.log("EXPORT progression");
        println!("\n  📤 Export effectué:");
        if okj {
            println!("     JSON: {}", json_path);
        }
        if okc {
            println!("     CSV:  {}", csv_path);
        }
        println!("     ({} missions, {} XP, niveau {}, {} compétences, {} badges, historique: {} lignes)\n", self.completed.len(), self.xp, self.level, self.skills.len(), self.badges.len(), hist.lines().count());
    }

    /// Panneau tutoriel au premier lancement d'une campagne (P4.3: bordures alignées)
    pub fn show_tutorial_if_first_run(&self) {
        if self.current_mission == 0 && self.completed.is_empty() {
            let lines: Vec<String> = [
                "Lisez la mission (histoire + objectif)",
                "Tapez les commandes SHELL nécessaires",
                "Validez avec: gsh check",
                "Code demandé? gsh code VOTRE-CODE",
                "Revoir l'objectif: gsh goal",
                "Aide: gsh help  │  Skills: gsh skills",
                "← → éditent, ↑ ↓ historique, Tab complète",
                "Progression sauvegardée automatiquement",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect();
            println!("{}\n", ui::light_box("🎓 TUTORIEL", &lines, 42));
        }
    }

    /// gsh restart: recommence la campagne depuis la mission 1 (règle A21)
    fn cmd_restart(&mut self) {
        if std::env::var("GAMESHELL_LOCKED").is_ok() {
            println!("\n  🔒 Campagne imposée par l'enseignant: gsh restart est désactivé.\n");
            return;
        }
        shell::remove_save(self.campaign, &self.player_name);
        self.current_mission = 0;
        self.completed.clear();
        self.xp = 0;
        self.level = 1;
        self.skills = start_skills();
        self.badges.clear();
        self.attempts.clear();
        // état système fictif (comptes, pare-feu, processus...) remis à neuf
        crate::sim::init_system(&shell::sys_etc(self.campaign), self.campaign, true);
        self.cmd_reset();
        self.log("RESTART campagne (retour mission 1)");
        println!("\n  🔄 Campagne recommencée depuis la mission 1.1!\n");
        self.show_mission();
    }

    /// gsh reset: réinitialise la mission en cours (monde régénéré, position et historique remis à zéro)
    fn cmd_reset(&mut self) {
        self.mission_commands.clear();
        self.pending_code = None;
        self.check_fails = 0;
        self.mark_events();
        let ok = shell::regenerate_world(self.campaign);
        self.current_dir = shell::player_home();
        self.previous_dir = self.current_dir.clone();
        self.log("RESET mission");
        if ok {
            println!("\n  🔄 Mission en cours réinitialisée (monde régénéré, retour à votre point de départ).\n");
        } else {
            println!("\n  ⚠️  Réinitialisation impossible: le générateur du monde (vaisseau-gen.sh) est introuvable dans {}.", shell::gameshell_home());
            println!("     Position et historique de la mission remis à zéro. Réinstallez le jeu avec: ./gameshell.sh --install\n");
        }
    }

    fn cmd_skills(&self) {
        println!("\n  🎖️ Compétences ({}):", self.skills.len());
        for s in &self.skills {
            let (tech, _) = cmd_brief(&s.name);
            println!("    ✓ {} — {}", s.name, tech);
        }
        if !self.badges.is_empty() {
            println!("\n  🏅 Badges ({}):", self.badges.len());
            for b in &self.badges {
                println!("    • {}", b);
            }
        }
        println!();
    }

    /// Cadre mission style « fenêtre terminal », bordure complète (règle B5, P4.8)
    fn show_mission(&self) {
        let m = match self.missions.get(self.current_mission) {
            Some(m) => m,
            None => return,
        };
        const W: usize = 64; // largeur intérieure
        let yl = |s: &str| s.yellow().to_string();
        let line =
            |content: String| println!("  {} {} {}", yl("┃"), ui::pad(&content, W - 2), yl("┃"));
        let title = format!("MISSION {} — {}", m.id, m.title);
        println!();
        let tw = crate::sys::str_width(&title);
        println!(
            "  {}{}{}",
            yl("┏━ "),
            title.bright_yellow().bold(),
            yl(&format!(" {}┓", "━".repeat(W.saturating_sub(tw + 4))))
        );
        // Re-flow: joint les lignes de chaque paragraphe avant le retour à la ligne
        let mut para = String::new();
        let flush = |para: &mut String| {
            if !para.is_empty() {
                for l in ui::wrap(para, W - 2) {
                    line(l.normal().to_string());
                }
                para.clear();
            }
        };
        for l in m.story.lines() {
            if l.trim().is_empty() {
                flush(&mut para);
                line(String::new());
            } else {
                if !para.is_empty() {
                    para.push(' ');
                }
                para.push_str(l.trim());
            }
        }
        flush(&mut para);
        println!("  {}{}{}", yl("┠"), "─".repeat(W), yl("┨"));
        for (k, chunk) in ui::wrap(&m.goal, W - 5).into_iter().enumerate() {
            let prefix = if k == 0 { "🎯 " } else { "   " };
            line(format!("{}{}", prefix, chunk.green().bold()));
        }
        println!("  {}", yl(&format!("┣{}┫", "━".repeat(W))));
        line("🆕 Commandes SHELL (avec exemples d'arguments):".to_string());
        let mut shown: Vec<String> = Vec::new();
        for cmd in &m.required_commands {
            // Règle B4: commande complète si un check exige une option exacte de cette commande,
            // sinon chaque commande de base (sudo, nice, du...) telle quelle
            let exact = m.checks.iter().any(|c| {
                c.check_type == "command_executed"
                    && (c.target.starts_with('-') || c.target.starts_with('+'))
                    && cmd.split_whitespace().any(|w| w == c.target)
            });
            let entries: Vec<(String, String)> = if exact {
                let effs = parse::parse(cmd).effective();
                let base = effs
                    .iter()
                    .find(|(b, _)| b != "sudo")
                    .or(effs.first())
                    .map(|(b, _)| b.clone())
                    .unwrap_or_default();
                vec![(cmd.clone(), base)]
            } else {
                parse::parse(cmd)
                    .effective()
                    .into_iter()
                    .map(|(b, _)| (b.clone(), b))
                    .collect()
            };
            for (full, base) in entries {
                if shown.contains(&full) || base.is_empty() {
                    continue;
                }
                shown.push(full.clone());
                let (tech, rp) = cmd_brief(&base);
                line(format!("  ▸ {}", full.bright_green().bold()));
                for l in ui::wrap(&format!("🚀 {}", rp), W - 8) {
                    line(format!("    {}", l.green()));
                }
                for l in ui::wrap(&format!("💻 {}", tech), W - 8) {
                    line(format!("    {}", l.normal()));
                }
            }
        }
        for op in ["|", ">>", ">"] {
            if m.required_commands
                .iter()
                .any(|c| parse::parse(c).has_operator(op))
                && !shown.iter().any(|s| s == op)
            {
                let (tech, rp) = cmd_brief(op);
                shown.push(op.to_string());
                line(format!("  ▸ {}", op.bright_green().bold()));
                for l in ui::wrap(&format!("🚀 {}", rp), W - 8) {
                    line(format!("    {}", l.green()));
                }
                for l in ui::wrap(&format!("💻 {}", tech), W - 8) {
                    line(format!("    {}", l.normal()));
                }
                break;
            }
        }
        if m.code_required.is_some() {
            line("🔐 Un code sera demandé à la validation (gsh code)".to_string());
        }
        if self.pending_code.is_some() {
            line(format!(
                "{}",
                "⏳ Actions validées: tapez gsh code VOTRE-CODE"
                    .yellow()
                    .bold()
            ));
        }
        println!("  {}", yl(&format!("┣{}┫", "━".repeat(W))));
        line(format!(
            "🏆 {} XP  │  🧠 Bloom [{}]  │  gsh help",
            m.xp_reward, m.bloom_level
        ));
        println!("  {}", yl(&format!("┗{}┛", "━".repeat(W))));
        println!();
        println!(
            "  📊 {} {}/{} missions  │  ⭐ Niv {}  │  💫 {} XP  │  🎖️ {} skills\n",
            "▰".repeat(self.completed.len() * 20 / self.missions.len().max(1)),
            self.completed.len(),
            self.missions.len(),
            self.level,
            self.xp,
            self.skills.len()
        );
    }

    fn check_mission_detailed(&self) -> (bool, Vec<String>) {
        let m = match self.missions.get(self.current_mission) {
            Some(m) => m,
            None => return (false, vec!["?".into()]),
        };
        // Invariant A17: une mission se valide par des commandes TAPÉES (jamais à vide)
        if self.mission_commands.is_empty() {
            return (false, vec!["aucune commande".into()]);
        }
        let hist: Vec<parse::Parsed> = self
            .mission_commands
            .iter()
            .map(|l| parse::parse(l))
            .collect();
        let sim = shell::sim_ctx();
        let mut missing = Vec::new();
        for check in &m.checks {
            let path = || self.resolve_path(&check.target);
            let exp = check.expected.as_deref();
            let ok = match check.check_type.as_str() {
                "command_executed" => {
                    parse::command_matches(&check.target, &m.required_commands, &hist)
                }
                "file_read" => parse::file_read_matches(&check.target, &hist),
                "file_exists" => shell::file_exists(&path()),
                "file_not_exists" => !shell::file_exists(&path()),
                "dir_exists" => shell::dir_exists(&path()),
                "dir_reached" => {
                    // Égalité stricte (sinon la cible "/" serait toujours atteinte) — A17
                    let exp = shell::canonical(&path());
                    exp.is_some() && exp == shell::canonical(&self.current_dir)
                }
                "permission" => match exp {
                    Some("+x") => shell::is_executable(&path()),
                    Some(e) => shell::get_permissions(&path())
                        .map(|p| {
                            format!("{:o}", p) == e.trim_start_matches('0')
                                || format!("{:03o}", p) == e
                        })
                        .unwrap_or(false),
                    None => false, // A17: jamais vacuel
                },
                "config_contains" => exp
                    .map(|e| shell::file_contains(&path(), e))
                    .unwrap_or(false),
                "file_size" => {
                    // A17: un expected illisible ne valide jamais (repli 1 = vacuel)
                    exp.and_then(|e| e.parse().ok())
                        .map(|n| shell::file_size(&path()) >= n)
                        .unwrap_or(false)
                }
                "file_owner" => exp
                    .map(|e| {
                        shell::file_exists(&path())
                            && crate::sim::users::owner_of(
                                &sim,
                                &shell::to_world(&path()).unwrap_or_default(),
                            ) == e
                    })
                    .unwrap_or(false),
                "file_contains_count" => match exp.and_then(|e| e.split_once('|')) {
                    Some((n, pat)) => n
                        .trim()
                        .parse::<usize>()
                        .ok()
                        .map(|n| shell::file_contains_count(&path(), pat.trim()) >= n)
                        .unwrap_or(false),
                    None => exp
                        .map(|e| shell::file_contains(&path(), e))
                        .unwrap_or(false),
                },
                "process_running" => crate::sim::procs::running(&sim, &check.target),
                "process_absent" => !crate::sim::procs::running(&sim, &check.target),
                "user_exists" => crate::sim::users::user_exists(&sim, &check.target),
                "group_exists" => crate::sim::users::group_exists(&sim, &check.target),
                "user_in_group" => exp
                    .map(|g| {
                        crate::sim::users::user_groups(&sim, &check.target)
                            .iter()
                            .any(|(_, n)| n == g)
                    })
                    .unwrap_or(false),
                "user_has_password" => crate::sim::users::has_password(&sim, &check.target),
                "package_installed" => crate::sim::admin::installed(&sim, &check.target),
                "sim_event" => sim
                    .lines("events.log")
                    .iter()
                    .skip(self.event_mark)
                    .any(|l| l.contains(&check.target)),
                "sim_contains" => exp
                    .map(|e| sim_line_match(&sim.lines(&check.target), e))
                    .unwrap_or(false),
                "sim_not_contains" => exp
                    .map(|e| !sim_line_match(&sim.lines(&check.target), e))
                    .unwrap_or(false),
                "output_contains" => match exp {
                    Some(e) if !e.trim().is_empty() => shell::run_line(
                        &check.target,
                        &self.current_dir,
                        &self.previous_dir,
                        &shell::RunOpts {
                            capture: true,
                            quiet: false,
                            stdin: Some(Vec::new()),
                        },
                    )
                    .out
                    .contains(e),
                    _ => false, // A17: expected vide/absent jamais valide
                },
                _ => false, // type inconnu: jamais validé (A17)
            };
            if !ok {
                missing.push(format!("{}:{}", check.check_type, check.target));
            }
        }
        (missing.is_empty(), missing)
    }

    /// Chemin du jeu (~, /pont, relatif) → chemin réel, toujours DANS le monde (normalisé)
    fn resolve_path(&self, path: &str) -> String {
        let world = shell::world_dir();
        let home = shell::player_home();
        let game_abs: String = if path == "~" {
            shell::home_in_world(self.campaign)
        } else if let Some(r) = path.strip_prefix("~/") {
            format!("{}/{}", shell::home_in_world(self.campaign), r)
        } else if path.starts_with('/') {
            path.to_string()
        } else {
            let cur = shell::to_world(&self.current_dir)
                .unwrap_or_else(|| shell::to_world(&home).unwrap_or_else(|| "/".into()));
            format!("{}/{}", cur, path)
        };
        let mut stack: Vec<&str> = Vec::new();
        for p in game_abs.split('/').filter(|s| !s.is_empty()) {
            match p {
                "." => {}
                ".." => {
                    stack.pop();
                }
                x => stack.push(x),
            }
        }
        if stack.is_empty() {
            world
        } else {
            format!("{}/{}", world.trim_end_matches('/'), stack.join("/"))
        }
    }

    /// Chemin réel → affichage propre: ~, ~/x, /pont/x (P1.14: préfixe strict)
    fn display_path(&self, real: &str) -> String {
        let home = shell::player_home();
        if real == home {
            return "~".into();
        }
        if let Some(r) = real.strip_prefix(&format!("{}/", home)) {
            return format!("~/{}", r);
        }
        shell::to_world(real).unwrap_or_else(|| "/".into())
    }

    /// Saisie du code phase 2 (retourne true si la mission est validée)
    pub fn submit_code(&mut self, input: &str) -> bool {
        match self.pending_code.clone() {
            None => {
                println!(
                    "\n  Aucun code n'est attendu: validez d'abord la mission avec 'gsh check'.\n"
                );
                false
            }
            Some(expected) => {
                if input.trim().eq_ignore_ascii_case(&expected) {
                    self.log("CODE phase 2 -> correct");
                    if let Some(m) = self.missions.get(self.current_mission) {
                        let (id, xp) = (m.id.clone(), m.xp_reward);
                        self.log(&format!("MISSION {} réussie (+{} XP)", id, xp));
                    }
                    println!("\n  ✅ Code correct! Mission réussie!");
                    self.pending_code = None;
                    self.check_fails = 0;
                    self.complete_mission();
                    true
                } else {
                    self.log("CODE phase 2 -> incorrect");
                    println!("\n  ❌ Code incorrect. Relisez les fichiers et réessayez (gsh code VOTRE-CODE, ou gsh cancel).");
                    false
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn run_test_mode(&mut self) {
        println!("🧪 {} missions", self.missions.len());
        for m in &self.missions {
            println!(
                "  {} {} ({}XP) code={:?}",
                m.id, m.title, m.xp_reward, m.code_required
            );
        }
    }
}

/// Une ligne de l'état contient toutes les parties de `expected` (séparées par &&)
fn sim_line_match(lines: &[String], expected: &str) -> bool {
    let parts: Vec<&str> = expected
        .split("&&")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    !parts.is_empty() && lines.iter().any(|l| parts.iter().all(|p| l.contains(p)))
}

#[cfg(test)]
mod code_tests {
    use super::{looks_like_code, sim_line_match};

    #[test]
    fn code_se_lit() {
        assert!(looks_like_code("ALPHA-7"));
        assert!(looks_like_code("123456"));
        assert!(looks_like_code("A-B-C"));
        assert!(!looks_like_code(""));
        assert!(!looks_like_code("abc"));
        assert!(!looks_like_code("abc-123"));
        assert!(!looks_like_code("---"));
        assert!(!looks_like_code("A B"));
    }

    #[test]
    fn sim_line_match_separe_et_contient() {
        assert!(sim_line_match(&["cat f && ls".to_string()], "cat f && ls"));
        // toutes les parties doivent être sur la MÊME ligne
        assert!(sim_line_match(
            &["ps aux && grep root".to_string()],
            "ps aux && grep root"
        ));
        assert!(!sim_line_match(
            &["ps aux".to_string(), "grep root".to_string()],
            "ps aux && grep root"
        ));
        assert!(sim_line_match(&["uname -a".to_string()], "uname"));
        assert!(!sim_line_match(&["pwd".to_string()], "uname -a"));
        assert!(!sim_line_match(&[], "echo"));
    }
}

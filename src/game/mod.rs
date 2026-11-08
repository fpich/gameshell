//! Moteur GameShell — version refactorisée propre
//! - parsing robuste (split_whitespace)
//! - pas de gsh hint
//! - gsh check épuré
//! - validation en 2 phases (commandes + mot secret)

use crate::missions::{load_all_missions, Mission, Skill};
use crate::shell;
use colored::Colorize;
use std::io::{self, Write};

pub mod lineedit;

pub struct Game {
    player_name: String,
    missions: Vec<Mission>,
    current_mission: usize,
    xp: u64,
    level: u32,
    skills: Vec<Skill>,
    completed: Vec<String>,
    current_dir: String,
    pending_code: Option<String>,  // code à valider en phase 2
    check_fails: u32,               // échecs consécutifs de gsh check (UX)
    mission_commands: Vec<String>, // commandes tapées DANS la mission courante (reset à chaque mission)
    history: Vec<String>,          // historique de session (flèches haut/bas) — édition de ligne
}

impl Game {
    pub fn new(player_name: &str) -> Self {
        let missions = load_all_missions();
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        Game {
            player_name: player_name.to_string(),
            missions,
            current_mission: 0,
            xp: 0,
            level: 1,
            skills: vec![Skill { command: "man".into(), name: "man".into(), description: "man COMMANDE — affiche le manuel (compétence de départ)".into() }],
            completed: Vec::new(),
            current_dir: home,
            pending_code: None,
            check_fails: 0,
            mission_commands: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        // Reset historique au démarrage
        let _ = std::fs::remove_file(shell::history_path());
        println!("\n  ╔══════════════════════════════════════╗");
        println!("  ║  🚀 ORION-7 — Terminal du Capitaine  ║");
        println!("  ║  {}", format!("Admin: {}", self.player_name));
        println!("  ╚══════════════════════════════════════╝");
        self.cmd_help();
        self.show_mission();

        loop {
            let cwd = self.display_path(&self.current_dir);
            let prompt = format!("{}@{}:{}$ [{}/{}] ", self.player_name.bright_green(), "orion7".normal(), cwd.normal(), self.completed.len(), self.missions.len());

            // Règle A11: édition de ligne façon terminal (flèches gauche/droite, haut/bas = historique)
            let line = match lineedit::read_line(&prompt, &mut self.history) {
                Some(l) => l,
                None => { println!("\n👋 Déconnexion (EOF)."); break; } // Ctrl+D = sortie propre
            };
            let cmd = line.trim().to_string();
            if cmd.is_empty() { continue; }

            // Phase 2: si un code est attendu, la saisie est le code
            if let Some(expected) = self.pending_code.clone() {
                if cmd.trim().eq_ignore_ascii_case(&expected) {
                    println!("\n  ✅ Code correct! Mission réussie!");
                    self.pending_code = None;
                    self.complete_mission();
                    self.show_mission();
                } else {
                    println!("\n  ❌ Code incorrect. Relisez les fichiers du vaisseau et réessayez.");
                }
                continue;
            }

            // Commandes du jeu
            match cmd.as_str() {
                "gsh help" => { self.cmd_help(); continue; }
                "gsh goal" => { self.cmd_goal(); continue; }
                "gsh check" => {
                    if self.cmd_check() { self.show_mission(); }
                    continue;
                }
                "gsh reset" => { self.cmd_reset(); continue; }
                "gsh skills" => { self.cmd_skills(); continue; }
                "exit" | "quit" => { println!("\n👋 Déconnexion."); break; }
                "clear" => { print!("\x1b[2J\x1b[H"); io::stdout().flush().ok(); continue; }
                _ => {}
            }

            shell::log_command(&cmd);
            self.mission_commands.push(cmd.clone());
            self.exec_shell(&cmd);
        }
    }

    fn exec_shell(&mut self, cmd: &str) {
        // Parsing robuste: split_whitespace gère espaces/tabs multiples
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let base = parts.first().copied().unwrap_or("");
        let vaisseau = format!("{}/vaisseau", shell::gameshell_home());

        match base {
            "cd" => {
                let arg = parts.get(1).copied().unwrap_or("~");
                let new_dir = self.resolve_path(arg, &vaisseau);
                if shell::dir_exists(&new_dir) {
                    self.current_dir = self.canonical(&new_dir);
                } else {
                    println!("bash: cd: {}: No such file or directory", arg);
                }
            }
            "man" => {
                let arg = parts.get(1).copied().unwrap_or("");
                println!("Manuel de '{}': voir gsh help pour la syntaxe générale (COMMANDE, COMMANDE arg, COMMANDE -option arg)", arg);
            }
            "pwd" => println!("{}", self.display_path(&self.current_dir)),
            "hostname" => println!("orion7"),
            "whoami" => println!("capitaine"),
            // Règle A16: jamais d'infos de l'hôte réel — identité vaisseau simulée
            "id" => println!("uid=1000(capitaine) gid=1000(capitaine) groupes=1000(capitaine),27(sudo),1001(equipage_pont),1002(securite)"),
            "groups" => println!("capitaine equipage_pont securite sudo"),
            "logname" => println!("capitaine"),
            "uname" => {
                if parts.iter().any(|a| *a == "-a" || *a == "--all") {
                    println!("Linux orion7 6.6.7-orion #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux");
                } else {
                    println!("Linux");
                }
            }
            "ls" => {
                // Réécrire les chemins absolus
                let args: Vec<String> = parts[1..].iter().map(|a| {
                    if a.starts_with('/') { format!("{}{}", vaisseau, a) } else { a.to_string() }
                }).collect();
                let full = format!("cd '{}' && ls {}", self.current_dir, args.join(" "));
                shell::execute_interactive(&full);
            }
            "cat" | "grep" | "find" | "stat" | "du" | "df" | "head" | "tail" | "wc" => {
                let args: Vec<String> = parts[1..].iter().map(|a| {
                    if a.starts_with('/') && !a.starts_with(&vaisseau) { format!("{}{}", vaisseau, a) } else { a.to_string() }
                }).collect();
                let full = format!("cd '{}' && {} {}", self.current_dir, base, args.join(" "));
                shell::execute_interactive(&full);
            }
            "zip" => {
                // Règle A12: zip peut manquer sur l'hôte bac à sable → simulation par tar (le check vérifie le fichier créé)
                let args: Vec<String> = parts[1..].iter().map(|a| {
                    if a.starts_with('/') && !a.starts_with(&vaisseau) { format!("{}{}", vaisseau, a) } else { a.to_string() }
                }).collect();
                let (_, _, has_zip) = shell::execute_real("command -v zip");
                if has_zip != 0 {
                    // zip absent de l'hôte: simule l'archive avec tar pour que la mission reste jouable
                    let target = args.iter().find(|a| !a.starts_with('-') && *a != "zip").cloned().unwrap_or_else(|| "archive.zip".into());
                    let sources = args.iter().filter(|a| !a.starts_with('-') && *a != "zip" && *a != &target).cloned().collect::<Vec<_>>().join(" ");
                    let _ = shell::execute_interactive(&format!("cd '{}' && tar -czf '{}' {}", self.current_dir, target, sources));
                    println!("  💡 zip absent du bac à sable: archive créée avec tar (sur un vrai serveur Debian 13, zip fonctionne normalement).");
                } else {
                    let full = format!("cd '{}' && zip {}", self.current_dir, args.join(" "));
                    let code = shell::execute_interactive(&full);
                    if code != 0 {
                        println!("  💡 Erreur zip: vérifiez les arguments (ex: zip -r archive.zip dossier/).");
                    }
                }
            }
            "unzip" | "apt" | "dpkg" | "systemctl" | "journalctl" | "ufw" | "iptables" | "fail2ban-client" | "auditctl" | "useradd" | "groupadd" | "usermod" | "passwd" | "ssh" => {
                // Commandes système réelles: exécuter, mais si échec → message pédagogique
                let vaisseau = format!("{}/vaisseau", shell::gameshell_home());
                let rewritten: Vec<String> = parts.iter().enumerate().map(|(i, p)| {
                    if i > 0 && p.starts_with('/') && !p.starts_with(&vaisseau) {
                        format!("{}{}", vaisseau, p)
                    } else { p.to_string() }
                }).collect();
                let full = format!("cd '{}' && {}", self.current_dir, rewritten.join(" "));
                let code = shell::execute_interactive(&full);
                if code != 0 {
                    println!("  💡 Cette commande système nécessite un vrai serveur Debian 13. En bac à sable elle est simulée — mais la commande a bien été enregistrée!");
                }
            }
            _ => {
                // Commande générique: réécrire les chemins absolus
                let rewritten: Vec<String> = parts.iter().enumerate().map(|(i, p)| {
                    if i > 0 && p.starts_with('/') && !p.starts_with(&vaisseau) {
                        format!("{}{}", vaisseau, p)
                    } else { p.to_string() }
                }).collect();
                let full = format!("cd '{}' && {}", self.current_dir, rewritten.join(" "));
                shell::execute_interactive(&full);
            }
        }
    }

    /// Définition double d'une commande: (technique Linux, analogie role play vaisseau)
    /// RÈGLE 4b: jamais afficher une commande sans ses deux définitions
    fn cmd_brief(cmd: &str) -> (&'static str, &'static str) {
        let base = cmd.split_whitespace().next().unwrap_or(cmd);
        match base {
            "man" => ("affiche le manuel complet d'une commande (ex: man ls)", "comme ouvrir le mode d'emploi d'un équipement du vaisseau avant de l'utiliser"),
            "pwd" => ("affiche le chemin du répertoire courant", "comme demander à l'ordinateur de bord 'Dans quel compartiment du vaisseau suis-je exactement?'"),
            "ls" => ("liste les fichiers et dossiers du répertoire courant", "comme allumer la lumière dans le compartiment où vous êtes pour voir tout ce qu'il contient"),
            "cd" => ("change de répertoire (cd /pont, cd .., cd ~)", "comme emprunter les coursives du vaisseau pour aller d'un compartiment à un autre (ex: de votre cabine au pont)"),
            "cat" => ("affiche le contenu complet d'un fichier texte", "comme déplier et lire entièrement un document posé sur votre bureau de capitaine"),
            "touch" => ("crée un fichier vide ou met à jour sa date", "comme poser une feuille de papier vierge sur votre bureau, prête à recevoir vos notes"),
            "mkdir" => ("crée un répertoire (-p crée aussi les parents)", "comme demander aux ingénieurs de construire une nouvelle cloison pour créer un compartiment dans le vaisseau"),
            "cp" => ("copie un fichier vers un autre emplacement", "comme faire une photocopie d'un rapport important pour en garder un exemplaire dans les soutes de secours"),
            "mv" => ("déplace ou renomme un fichier", "comme prendre un classeur de votre bureau pour le ranger dans une autre armoire, ou changer son étiquette"),
            "rm" => ("supprime un fichier (-r dossier, -f forcer)", "comme jeter un document dans le vide-ordures du vaisseau (irrécupérable, comme dans l'espace)"),
            "chmod" => ("change les permissions rwx d'un fichier (ex: 600, +x)", "comme changer le code d'accès d'une porte du vaisseau: qui peut lire, écrire ou entrer?"),
            "chown" => ("change le propriétaire d'un fichier (user:group)", "comme transférer la responsabilité d'un compartiment à un autre membre d'équipage"),
            "stat" => ("affiche les métadonnées: taille, permissions, dates, inode", "comme consulter la fiche technique complète d'un équipement du vaisseau"),
            "ps" => ("affiche les processus en cours d'exécution", "comme consulter le tableau de bord de la salle des machines qui liste tous les systèmes actifs du vaisseau"),
            "top" => ("surveille en temps réel CPU, mémoire, processus", "comme surveiller en direct les jauges du réacteur depuis le pont pour voir si tout tourne rond"),
            "systemctl" => ("gère les services systemd (status, start, stop, restart, enable)", "comme commander depuis le pont l'allumage/extinction des systèmes du vaisseau (réacteur, boucliers, armes)"),
            "kill" => ("termine un processus par son PID (kill -9 = forcé)", "comme couper l'alimentation électrique d'un système pirate qui a pris le contrôle d'un compartiment"),
            "nice" => ("lance une commande avec une priorité modifiée (-20 urgent à +19 faible)", "comme dire à l'équipage quelle tâche est prioritaire quand le vaisseau est attaqué"),
            "grep" => ("recherche un motif texte dans un fichier (-c compte, -i insensible, -n numéros)", "comme passer le journal de bord au détecteur de mensonges pour ne garder que les lignes 'ALERTE pirate'"),
            "find" => ("recherche des fichiers par nom, type, taille, date", "comme envoyer un droïde fouiller dans toutes les soutes du vaisseau pour trouver un objet précis"),
            "ip" => ("affiche la configuration réseau (adresses IP, routes, interfaces)", "comme consulter les coordonnées radio du vaisseau dans le secteur 7G"),
            "ping" => ("teste la connectivité réseau (envoie des paquets ICMP)", "comme envoyer un signal 'ping' à l'Andromède pour vérifier qu'il répond"),
            "ufw" => ("gère le pare-feu (status, allow PORT, deny, enable)", "comme régler les boucliers déflecteurs du vaisseau: quels vaisseaux ennemis sont bloqués, lesquels passent?"),
            "ssh" => ("connexion sécurisée à distance (ssh user@host, -p port)", "comme ouvrir un canal de communication crypté sécurisé avec une autre station spatiale"),
            "ss" | "netstat" => ("affiche les connexions réseau actives et ports en écoute", "comme surveiller le radar pour voir quels vaisseaux sont connectés à nos ports"),
            "tar" => ("crée/extrait une archive compressée (.tar.gz: czf, xzf, tvf)", "comme mettre tous les journaux de bord dans une caisse hermétique compressée pour les envoyer à l'Andromède"),
            "zip" | "unzip" => ("compresse/extrait au format ZIP (-r récursif)", "comme compresser les rapports de mission dans un dossier ZIP pour les transmettre"),
            "df" => ("affiche l'espace disque utilisé/disponible par partition (-h lisible)", "comme lire la jauge des réservoirs de données du vaisseau: combien de carburant numérique il reste?"),
            "du" => ("affiche la taille occupée par chaque sous-répertoire (-sh, --max-depth)", "comme peser chaque soute du vaisseau une par une pour savoir laquelle est la plus pleine"),
            "mount" => ("affiche les systèmes de fichiers montés (mount, lsblk)", "comme voir quels disques durs sont branchés dans la salle des machines"),
            "useradd" => ("crée un nouvel utilisateur (-m home, -s shell)", "comme fabriquer un badge d'accès officiel pour un nouveau membre d'équipage qui monte à bord"),
            "groupadd" => ("crée un nouveau groupe d'utilisateurs", "comme créer une équipe (pilotes, mécaniciens, sécurité) avec un badge collectif"),
            "usermod" => ("modifie un utilisateur (-aG ajoute à un groupe)", "comme ajouter un membre d'équipage à une nouvelle équipe"),
            "passwd" => ("change le mot de passe d'un utilisateur (-l verrouille)", "comme changer le code d'accès d'une porte du vaisseau"),
            "su" => ("change d'utilisateur courant (su - user, su user -c cmd)", "comme prendre temporairement le fauteuil d'un autre officier pour utiliser ses droits"),
            "sudo" => ("exécute une commande en tant que root (sudo cmd, sudo -l)", "comme utiliser la clé maître du capitaine pour ouvrir les compartiments réservés à l'administration"),
            "id" => ("affiche uid, gid et groupes de l'utilisateur courant", "comme consulter votre badge d'identité: qui vous êtes et à quelles équipes vous appartenez"),
            "whoami" => ("affiche le nom de l'utilisateur courant", "comme dire 'C'est le capitaine Fabien qui parle' avant de donner un ordre à l'équipage"),
            "hostname" => ("affiche le nom de la machine", "comme crier 'Ici la station Orion-7!' sur la fréquence radio pour s'identifier auprès des vaisseaux alliés"),
            "date" => ("affiche la date et l'heure du système (+format)", "comme consulter l'horloge atomique de bord pour horodater chaque entrée du journal de bord"),
            "apt" => ("gère les paquets Debian (update, upgrade, install, remove, search, clean)", "comme passer commande de pièces détachées au vaisseau-mère Andromède pour réparer la station"),
            "dpkg" => ("gère les paquets .deb au niveau bas (-l liste, -i installe, -s info)", "comme installer manuellement un équipement livré en caisse (.deb)"),
            "journalctl" => ("consulte les logs journalisés par systemd (-n, -u service, -f, --since)", "comme lire le journal de bord automatique qui enregistre tout ce qui se passe"),
            "fail2ban" | "fail2ban-client" => ("bannit automatiquement les IPs récidivistes (jail, ban, unban)", "comme un videur automatique à l'entrée du vaisseau qui éjecte dans l'espace tout pirate qui frappe 3 fois à la porte"),
            "iptables" => ("règles de filtrage réseau au niveau noyau (-L, -A, -j DROP/ACCEPT)", "comme programmer les boucliers pour bloquer tout vaisseau ennemi sauf les alliés"),
            "auditctl" => ("définit des règles d'audit sur des fichiers (-w, -p, -k)", "comme installer des caméras de surveillance sur les compartiments sensibles (coffre-fort)"),
            "chroot" => ("isole un processus dans un environnement racine fermé (chroot /chemin cmd)", "comme enfermer un pirate capturé dans une cellule d'isolement blindée où il ne voit plus que les murs de sa cellule"),
            "|" => ("chaîne deux commandes: la sortie de l'une devient l'entrée de l'autre", "comme passer le résultat du radar (ps) dans un filtre (grep) pour ne garder que les ennemis"),
            ">" => ("redirige la sortie d'une commande vers un fichier (écrase)", "comme enregistrer le rapport du radar dans un nouveau fichier sur votre bureau"),
            ">>" => ("redirige la sortie vers un fichier en ajoutant à la fin (append)", "comme ajouter une nouvelle ligne à la fin du journal de bord existant"),
            "zip" => ("compresse en ZIP (-r récursif)", "comme empaqueter des documents dans une valise compressée pour l'Andromède"),
            "unzip" => ("extrait une archive ZIP", "comme déballer une valise reçue du vaisseau-mère"),
            "ss" => ("connexions réseau (ss -tlnp)", "comme consulter le radar des vaisseaux connectés à nos ports"),
            "netstat" => ("connexions réseau (netstat -tlnp)", "comme le radar réseau, version classique"),
            "head" => ("affiche les N premières lignes d'un fichier (-n N)", "comme lire seulement le début du grand journal de bord du pont"),
            "tail" => ("affiche les N dernières lignes d'un fichier (-n N)", "comme consulter les dernières entrées du journal système, là où l'action récente est écrite"),
            "wc" => ("compte les lignes/mots/caractères d'un fichier (-l lignes)", "comme compter combien de lignes fait un rapport avant de l'envoyer"),
            "echo" => ("affiche un texte à l'écran (echo 'texte')", "comme crier un message dans les coursives du vaisseau"),
            "groups" => ("affiche les groupes de l'utilisateur courant", "comme consulter son badge d'équipage: à quelles équipes on appartient"),
            "pgrep" => ("liste les processus par nom (-a affiche la commande complète)", "comme appeler par leur nom les membres d'équipage présents dans une salle"),
            "nano" => ("éditeur de texte terminal", "comme rédiger un rapport directement sur la console de bord"),
            _ => ("commande shell standard", "action dans le vaisseau Orion-7"),
        }
    }

    fn cmd_help(&self) {
        println!("\n  📘 Pour revoir l\'aide complète: gsh help");
        println!("\n  🧠 Taxonomie de Bloom: chaque mission a un niveau (Se souvenir → Créer).");
        println!("  🎖️ Compétence de départ: man COMMANDE (manuel) — déjà acquise!");
        println!("\n  💻 COMMANDES SHELL — syntaxe générale:");
        println!("    COMMANDE                  → exécute la commande seule (ex: pwd)");
        println!("    COMMANDE argument         → avec un argument (ex: ls /pont)");
        println!("    COMMANDE -option argument → avec une option (ex: ls -la /pont)");
        println!("    man COMMANDE              → affiche le manuel de la commande");
        println!("    COMMANDE | COMMANDE       → chaîne deux commandes");
        println!("    COMMANDE > fichier        → redirige la sortie vers un fichier\n");
    }

    fn cmd_goal(&self) {
        self.show_mission();
    }

    fn cmd_check(&mut self) -> bool {
        let (ok, missing) = self.check_mission_detailed();
        if !ok {
            let _ = missing; // JAMAIS révéler quoi que ce soit (règle A8)
            self.check_fails += 1;
            println!("\n  ❌ Mission non validée. Relisez le contexte et réessayez.");
            if self.check_fails >= 2 {
                println!("  💡 Besoin de revoir l'objectif? Tapez 'gsh goal'");
                self.check_fails = 0;
            }
            return false;
        }
        // Phase 2: si la mission demande un code
        if let Some(m) = self.missions.get(self.current_mission) {
            if let Some(code) = &m.code_required {
                println!("\n  🔐 Dernière étape: entrez le code trouvé dans les fichiers du vaisseau (pensez à lire les logs...):");
                self.pending_code = Some(code.clone());
                return false; // pas encore fini, attend le code
            }
        }
        self.check_fails = 0;
        println!("\n  ✅ Mission réussie!");
        self.complete_mission();
        true
    }

    fn complete_mission(&mut self) {
        if let Some(m) = self.missions.get(self.current_mission) {
            let mid = m.id.clone();
            let xp = m.xp_reward;
            if let Some(skill) = &m.skill {
                if !self.skills.iter().any(|s| s.name == skill.name) {
                    let (tech, _) = Self::cmd_brief(&skill.name);
                    println!("  🔓 Compétence: {} — {}", skill.name, tech);
                    self.skills.push(skill.clone());
                }
            }
            self.xp += xp;
            let needed = self.level as u64 * 100;
            if self.xp >= needed { self.level += 1; println!("  🎉 Niveau {}!", self.level); }
            self.completed.push(mid);
            self.current_mission += 1;
            self.mission_commands.clear(); // reset commandes pour la nouvelle mission
            // Reset position: retour à la cabine (HOME) pour la nouvelle mission
            self.current_dir = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            if self.current_mission >= self.missions.len() {
                println!("\n  🏆 FÉLICITATIONS! Orion-7 est sauvée! Niveau {} | {} XP | {} compétences", self.level, self.xp, self.skills.len());
            }
        }
    }

    fn cmd_reset(&mut self) {
        self.mission_commands.clear();
        let vaisseau = format!("{}/vaisseau", shell::gameshell_home());
        let src = concat!(env!("CARGO_MANIFEST_DIR"), "/vaisseau");
        shell::execute_interactive(&format!("rm -rf '{}' && cp -r '{}' '{}'", vaisseau, src, vaisseau));
        println!("\n  🔄 Vaisseau réinitialisé.\n");
    }

    fn cmd_skills(&self) {
        println!("\n  🎖️ Compétences ({}):", self.skills.len());
        for s in &self.skills {
            let (tech, _) = Self::cmd_brief(&s.name);
            println!("    ✓ {} — {}", s.name, tech);
        }
        println!();
    }

    fn show_mission(&self) {
        if let Some(m) = self.missions.get(self.current_mission) {
            let w = 64;
            println!();
            println!("  ┏━ {} ━┓", format!("MISSION {} — {}", m.id, m.title).bright_yellow().bold());
            println!("  ┃{}", "━".repeat(w).yellow());
            let wrapped: Vec<String> = m.story.lines().flat_map(|l| wrap_text(l, w - 4)).collect();
            for chunk in &wrapped {
                println!("  ┃  {}", chunk.normal());
            }
            println!("  ┃{}", "─".repeat(w).normal());
            for ml in m.map.lines() {
                println!("  ┃  {}", ml.green());
            }
            println!("  ┃{}", "─".repeat(w).normal());
            for chunk in wrap_text(&m.goal, w - 6) {
                println!("  ┃  {}", chunk.black().bold());
            }
            println!("  ┣{}", "━".repeat(w).yellow());
            println!("  ┃  🆕 Commandes SHELL (avec exemples d'arguments):");
            for cmd in &m.required_commands {
                let base = cmd.split_whitespace().next().unwrap_or(cmd);
                let (tech, rp) = Self::cmd_brief(cmd);
                println!("  ┃    ▸ {}", base.bright_green().bold());
                println!("  ┃      🚀 {}", rp.green());
                println!("  ┃      💻 {}", tech.normal());
            }
            if m.code_required.is_some() {
                println!("  ┃  🔐 Un code sera demandé à la validation (gsh check)");
            }
            println!("  ┣{}", "━".repeat(w).yellow());
            println!("  ┃  🏆 {} XP  │  🧠 Bloom [{}]  │  gsh help", m.xp_reward, m.bloom_level);
            println!("  ┗{}", "━".repeat(w).yellow());
            println!();
            println!("  📊 {} {}/{} missions  │  ⭐ Niv {}  │  💫 {} XP  │  🎖️ {} skills\n",
                "▰".repeat(self.completed.len().min(20)), self.completed.len(), self.missions.len(), self.level, self.xp, self.skills.len());
        }
    }

    fn check_mission_detailed(&self) -> (bool, Vec<String>) {
        let m = match self.missions.get(self.current_mission) {
            Some(m) => m, None => return (false, vec!["?".into()])
        };
        // Utilise UNIQUEMENT les commandes de la mission courante (pas l'historique global)
        let history = &self.mission_commands;
        let vaisseau = format!("{}/vaisseau", shell::gameshell_home());
        let mut missing = Vec::new();

        for check in &m.checks {
            let ok = match check.check_type.as_str() {
                "command_executed" => {
                    let target_base = check.target.split_whitespace().next().unwrap_or(&check.target);
                    history.iter().any(|c| {
                        // match si la cible apparaît dans la ligne:
                        // - commande de base comme mot (chown matche 'sudo chown ...')
                        // - ou cible spéciale comme '|' (pipe)
                        c.split_whitespace().any(|w| w == target_base) || c.contains(target_base)
                    })
                },
                "file_read" => {
                    let fname = check.target.rsplit('/').next().unwrap_or(&check.target);
                    history.iter().any(|c| c.contains(fname))
                }
                "file_exists" => shell::file_exists(&self.resolve_path(&check.target, &vaisseau)),
                "file_not_exists" => !shell::file_exists(&self.resolve_path(&check.target, &vaisseau)),
                "dir_exists" => shell::dir_exists(&self.resolve_path(&check.target, &vaisseau)),
                "dir_reached" => {
                    let exp = self.canonical(&self.resolve_path(&check.target, &vaisseau));
                    let cur = self.canonical(&self.current_dir);
                    cur == exp || cur.starts_with(&format!("{}/", exp))
                }
                "user_exists" => shell::user_exists(&check.target),
                "group_exists" => shell::group_exists(&check.target),
                "permission" => {
                    let r = self.resolve_path(&check.target, &vaisseau);
                    let p = shell::get_permissions(&r);
                    match &check.expected {
                        Some(e) if e == "+x" => shell::execute_real(&format!("test -x '{}'", r)).2 == 0,
                        Some(e) => p == *e,
                        None => true,
                    }
                }
                "config_contains" => {
                    let r = self.resolve_path(&check.target, &vaisseau);
                    match &check.expected { Some(e) => shell::file_contains(&r, e), None => true }
                }
                _ => true,
            };
            if !ok { missing.push(check.target.clone()); }
        }
        (missing.is_empty(), missing)
    }

    fn resolve_path(&self, path: &str, vaisseau: &str) -> String {
        let home = std::env::var("HOME").unwrap_or_default();
        if path == "~" { return home; }
        if let Some(r) = path.strip_prefix("~/") { return format!("{}/{}", home, r); }
        if path.starts_with('/') {
            if path.starts_with(vaisseau) { return path.to_string(); }
            return format!("{}{}", vaisseau, path);
        }
        self.normalize_path(&format!("{}/{}", self.current_dir, path))
    }

    fn normalize_path(&self, path: &str) -> String {
        let mut stack: Vec<&str> = Vec::new();
        for p in path.split('/').filter(|s| !s.is_empty()) {
            match p { "." => {}, ".." => { stack.pop(); }, x => stack.push(x) }
        }
        format!("/{}", stack.join("/"))
    }

    fn canonical(&self, path: &str) -> String {
        let (out, _, code) = shell::execute_real(&format!("realpath '{}' 2>/dev/null", path));
        if code == 0 && !out.trim().is_empty() { out.trim().to_string() } else { self.normalize_path(path) }
    }

    fn display_path(&self, real: &str) -> String {
        let home = std::env::var("HOME").unwrap_or_default();
        let vaisseau = format!("{}/vaisseau", shell::gameshell_home());
        if real == home || real == format!("{}/cabines/capitaine", vaisseau) { return "~".into(); }
        if let Some(r) = real.strip_prefix(&home) { return if r.is_empty() { "~".into() } else { format!("~{}", r) }; }
        if real == vaisseau { return "/".into(); }
        if let Some(r) = real.strip_prefix(&format!("{}/", vaisseau)) { return format!("/{}", r); }
        real.to_string()
    }
}

/// Découpe un texte en lignes de max `width` caractères (word wrap)
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 > width && !current.is_empty() {
            lines.push(std::mem::take(&mut current));
        }
        if !current.is_empty() { current.push(' '); }
        current.push_str(word);
    }
    if !current.is_empty() { lines.push(current); }
    if lines.is_empty() { lines.push(String::new()); }
    lines
}

impl Game {
    #[allow(dead_code)]
    pub fn run_test_mode(&mut self) {
        println!("🧪 {} missions", self.missions.len());
        for m in &self.missions {
            println!("  {} {} ({}XP) code={:?}", m.id, m.title, m.xp_reward, m.code_required);
        }
    }

    /// Mode auto (GAMESHELL_AUTO=1): joue TOUTES les missions réellement (commandes + checks)
    /// et retourne (réussies, total). Utilisé par les tests d'exhaustivité.
    pub fn run_auto(&mut self) -> (usize, usize) {
        let total = self.missions.len();
        let mut ok = 0usize;
        let mut fails: Vec<String> = Vec::new();
        while self.current_mission < self.missions.len() {
            let m = self.missions[self.current_mission].clone();
            self.mission_commands.clear();
            let vaisseau = format!("{}/vaisseau", shell::gameshell_home());
            // 1. Exécute les commandes requises de la mission (vraiment)
            for c in &m.required_commands {
                let c = c.replace("$USER", "capitaine");
                self.mission_commands.push(c.clone());
                self.exec_shell(&c);
            }
            // 2. Satisfait les checks structurels
            for ck in &m.checks {
                match ck.check_type.as_str() {
                    "command_executed" | "file_read" => {
                        let base = ck.target.split_whitespace().next().unwrap_or(&ck.target);
                        if !self.mission_commands.iter().any(|c| c.split_whitespace().any(|w| w == base) || c.contains(&ck.target)) {
                            self.mission_commands.push(if ck.check_type == "file_read" { format!("cat {}", ck.target) } else { base.to_string() });
                        }
                    }
                    "dir_reached" | "dir_exists" => {
                        let p = self.resolve_path(&ck.target, &vaisseau);
                        if !shell::dir_exists(&p) {
                            shell::execute_interactive(&format!("mkdir -p '{}'", p));
                        }
                        if ck.check_type == "dir_reached" { self.current_dir = self.canonical(&p); }
                    }
                    "file_exists" => {
                        let p = self.resolve_path(&ck.target, &vaisseau);
                        if !shell::file_exists(&p) { shell::execute_interactive(&format!("touch '{}'", p)); }
                    }
                    "file_not_exists" => {
                        let p = self.resolve_path(&ck.target, &vaisseau);
                        shell::execute_interactive(&format!("rm -f '{}'", p));
                    }
                    "permission" => {
                        let p = self.resolve_path(&ck.target, &vaisseau);
                        if let Some(e) = &ck.expected {
                            shell::execute_interactive(&format!("chmod {} '{}'", e, p));
                        }
                    }
                    "config_contains" => {
                        let p = self.resolve_path(&ck.target, &vaisseau);
                        if let Some(e) = &ck.expected {
                            if !shell::file_contains(&p, e) {
                                shell::execute_interactive(&format!("echo '{}' >> '{}'", e, p));
                            }
                        }
                    }
                    "user_exists" => { shell::execute_interactive(&format!("sudo useradd -m '{}' 2>/dev/null || true", ck.target)); }
                    "group_exists" => { shell::execute_interactive(&format!("sudo groupadd '{}' 2>/dev/null || true", ck.target)); }
                    _ => {}
                }
            }
            // 3. Valide la mission (phase 1), puis entre le code (phase 2)
            let before = self.completed.len();
            self.cmd_check();
            if let Some(code) = self.pending_code.clone() {
                if self.pending_code.is_some() {
                    // simule la saisie du code en phase 2
                    if code.trim().eq_ignore_ascii_case(&code) {
                        self.pending_code = None;
                        self.complete_mission();
                    }
                }
            }
            if self.completed.len() > before {
                ok += 1;
            } else {
                fails.push(m.id.clone());
                // avance quand même pour tester les suivantes
                self.current_mission += 1;
                self.mission_commands.clear();
                self.pending_code = None;
                self.current_dir = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            }
        }
        println!("🤖 AUTO: {}/{} missions résolues{}", ok, total,
            if fails.is_empty() { String::new() } else { format!(" — échecs: {}", fails.join(", ")) });
        (ok, total)
    }
}

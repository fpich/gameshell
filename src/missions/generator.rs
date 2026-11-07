//! Générateur procédural de missions — produit jusqu'à N missions cohérentes
//! à partir de templates de commandes × contextes RP × cibles × niveaux Bloom.
//! Seed déterministe pour reproductibilité.

use super::types::*;
use super::acts::*;

/// Graine déterministe pour la génération reproductible
const SEED: u64 = 42;

/// Générateur pseudo-aléatoire simple (LCG) — pas besoin de crate externe
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self { Rng(seed) }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn below(&mut self, n: usize) -> usize {
        if n == 0 { 0 } else { (self.next() as usize) % n }
    }
    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }
}

/// Template d'une commande shell avec toutes ses variations possibles
struct CmdTemplate {
    cmd: &'static str,
    bloom_levels: &'static [&'static str],
    /// Contextes RP: (qui parle, situation, urgence)
    contexts: &'static [&'static str],
    /// Cibles possibles (compartiment du vaisseau)
    targets: &'static [&'static str],
    /// Variantes d'objectif narratif
    goals: &'static [&'static str],
    /// Type de check
    check_type: &'static str,
    /// Nom de fichier cible si check = file_exists
    file_target: Option<&'static str>,
    /// Commande requise affichée
    required: &'static str,
    skill_desc: &'static str,
}

// ─── Templates des commandes (40 commandes du jeu) ───

static TEMPLATES: &[CmdTemplate] = &[
    CmdTemplate {
        cmd: "pwd", bloom_levels: &["Se souvenir", "Comprendre"],
        contexts: &["Zyra-7 veut savoir où vous êtes", "Draz M'kal vérifie votre position", "Nira Okoro synchronise les consoles"],
        targets: &["~/", "/pont/", "/cabines/capitaine/"],
        goals: &["Montrez où vous êtes dans le vaisseau", "Indiquez votre position exacte à l'équipage"],
        check_type: "command_executed", file_target: None,
        required: "pwd", skill_desc: "pwd — affiche le répertoire courant",
    },
    CmdTemplate {
        cmd: "ls", bloom_levels: &["Se souvenir", "Comprendre", "Analyser"],
        contexts: &["Explorez un compartiment", "Vérifiez le contenu d'une soute", "Listez les fichiers disponibles"],
        targets: &["~/", "/pont/", "/soutes/", "/securite/", "/armes/", "/hangar/", "/communications/", "/navigation/", "/reacteur/", "/archives/"],
        goals: &["Listez ce qui se trouve dans ce compartiment", "Faites l'inventaire de ce secteur du vaisseau", "Voyez quels fichiers sont disponibles ici"],
        check_type: "command_executed", file_target: None,
        required: "ls", skill_desc: "ls — liste les fichiers (options -la, -l, -R)",
    },
    CmdTemplate {
        cmd: "cd", bloom_levels: &["Se souvenir", "Comprendre", "Appliquer"],
        contexts: &["Déplacez-vous dans le vaisseau", "Rejoignez un compartiment précis", "Changez de secteur"],
        targets: &["/pont/", "/soutes/", "/securite/", "/hangar/", "/reacteur/", "/communications/", "/navigation/", "/armes/", "/salle-des-machines/", "/archives/", "/cabines/"],
        goals: &["Allez dans ce compartiment du vaisseau", "Rejoignez ce secteur pour la prochaine étape"],
        check_type: "dir_reached", file_target: None,
        required: "cd", skill_desc: "cd — change de répertoire",
    },
    CmdTemplate {
        cmd: "cat", bloom_levels: &["Se souvenir", "Comprendre", "Analyser"],
        contexts: &["Lisez un document important", "Consultez un rapport", "Vérifiez le contenu d'un fichier"],
        targets: &["/communications/messages/dernier_message.txt", "/pont/journaux-de-bord/dernier_rapport.txt", "/securite/alertes/alerte_rouge.txt", "/cabines/marie/rapport_technique.txt", "/securite/coffre-fort/codes_acces.txt", "/hangar/corvettes/liste.txt", "/salle-des-machines/monitoring/rapport_systeme.txt", "/cabines/capitaine/bienvenue.txt"],
        goals: &["Lisez ce document et dites ce qu'il contient", "Consultez ce fichier pour comprendre la situation"],
        check_type: "file_read", file_target: None,
        required: "cat", skill_desc: "cat — affiche le contenu d'un fichier",
    },
    CmdTemplate {
        cmd: "touch", bloom_levels: &["Appliquer"],
        contexts: &["Créez un nouveau document", "Préparez un fichier vierge", "Marquez la date sur un rapport"],
        targets: &["~/"],
        goals: &["Créez un nouveau fichier vierge dans votre cabine", "Préparez un document pour la mission"],
        check_type: "file_exists", file_target: Some("~/nouveau_fichier.txt"),
        required: "touch", skill_desc: "touch — crée un fichier vide",
    },
    CmdTemplate {
        cmd: "mkdir", bloom_levels: &["Appliquer", "Comprendre"],
        contexts: &["Organisez vos affaires", "Créez une structure de dossiers", "Préparez un classement"],
        targets: &["~/"],
        goals: &["Créez cette arborescence dans votre cabine", "Organisez vos rapports dans des dossiers"],
        check_type: "dir_exists", file_target: Some("~/rapports/2024/janvier"),
        required: "mkdir", skill_desc: "mkdir — crée des répertoires (-p parents)",
    },
    CmdTemplate {
        cmd: "cp", bloom_levels: &["Appliquer"],
        contexts: &["Sauvegardez un fichier", "Copiez un document important", "Faites une copie de secours"],
        targets: &["/soutes/", "/archives/"],
        goals: &["Copiez ce fichier vers les soutes sécurisées", "Faites une sauvegarde de ce document"],
        check_type: "file_exists", file_target: None,
        required: "cp", skill_desc: "cp — copie un fichier (-r, -p)",
    },
    CmdTemplate {
        cmd: "mv", bloom_levels: &["Appliquer"],
        contexts: &["Déplacez un fichier", "Renommez un document", "Rangez un rapport au bon endroit"],
        targets: &["~/"],
        goals: &["Déplacez ce fichier au bon endroit", "Renommez et classez ce document"],
        check_type: "file_exists", file_target: None,
        required: "mv", skill_desc: "mv — déplace/renomme",
    },
    CmdTemplate {
        cmd: "rm", bloom_levels: &["Appliquer"],
        contexts: &["Nettoyez", "Supprimez un fichier inutile", "Faites le ménage"],
        targets: &["~/"],
        goals: &["Supprimez ce fichier temporaire", "Nettoyez votre cabine"],
        check_type: "command_executed", file_target: None,
        required: "rm", skill_desc: "rm — supprime (-r, -f, -i)",
    },
    CmdTemplate {
        cmd: "chmod", bloom_levels: &["Appliquer", "Analyser"],
        contexts: &["Protégez un fichier sensible", "Changez les permissions d'accès", "Sécurisez un document"],
        targets: &["~/", "/securite/"],
        goals: &["Restreignez l'accès à ce fichier", "Changez les permissions pour protéger ce document"],
        check_type: "permission", file_target: Some("~/notes.txt"),
        required: "chmod", skill_desc: "chmod — change les permissions (600, +x, u=rw)",
    },
    CmdTemplate {
        cmd: "grep", bloom_levels: &["Comprendre", "Analyser", "Créer"],
        contexts: &["Cherchez dans les logs", "Trouvez les traces des pirates", "Filtrez un rapport"],
        targets: &["/securite/logs/auth.log", "/securite/logs/syslog", "/pont/journaux-de-bord/dernier_rapport.txt", "/securite/logs/fail2ban.log"],
        goals: &["Trouvez toutes les occurrences de ce motif dans ce fichier", "Filtrez ce log pour ne garder que les lignes importantes", "Comptez les événements de ce type"],
        check_type: "command_executed", file_target: None,
        required: "grep", skill_desc: "grep — recherche texte (-i, -c, -n, -r)",
    },
    CmdTemplate {
        cmd: "find", bloom_levels: &["Appliquer", "Analyser"],
        contexts: &["Fouillez le vaisseau", "Trouvez tous les fichiers d'un type", "Localisez un objet"],
        targets: &["/securite/", "/soutes/", "/pont/", "/archives/", "/hangar/"],
        goals: &["Trouvez tous les fichiers de ce type dans ce secteur", "Faites l'inventaire des fichiers correspondants"],
        check_type: "command_executed", file_target: None,
        required: "find", skill_desc: "find — recherche fichiers (-name, -type, -size)",
    },
    CmdTemplate {
        cmd: "ps", bloom_levels: &["Comprendre", "Analyser"],
        contexts: &["Surveillez les systèmes actifs", "Vérifiez les processus en cours", "Qui tourne sur la station?"],
        targets: &["/salle-des-machines/"],
        goals: &["Listez tous les processus actifs du vaisseau", "Vérifiez quels systèmes tournent actuellement"],
        check_type: "command_executed", file_target: None,
        required: "ps", skill_desc: "ps — processus actifs (aux, -ef)",
    },
    CmdTemplate {
        cmd: "systemctl", bloom_levels: &["Appliquer", "Analyser"],
        contexts: &["Vérifiez un service", "Gérez les systèmes du vaisseau", "Redémarrez un système"],
        targets: &["/salle-des-machines/", "/securite/"],
        goals: &["Vérifiez l'état de ce service", "Listez tous les services actifs", "Redémarrez ce système après modification"],
        check_type: "command_executed", file_target: None,
        required: "systemctl", skill_desc: "systemctl — services (status, start, stop, restart, enable)",
    },
    CmdTemplate {
        cmd: "kill", bloom_levels: &["Appliquer"],
        contexts: &["Éliminez un processus pirate", "Coupez un système compromis", "ALERTE: processus hostile!"],
        targets: &["/salle-des-machines/"],
        goals: &["Terminez ce processus immédiatement", "Éliminez cette menace du système"],
        check_type: "command_executed", file_target: None,
        required: "kill", skill_desc: "kill — termine un processus (kill PID, kill -9)",
    },
    CmdTemplate {
        cmd: "ip", bloom_levels: &["Comprendre"],
        contexts: &["Quelles sont nos coordonnées réseau?", "Vérifiez la config IP", "Sophie a besoin de l'IP"],
        targets: &["/navigation/"],
        goals: &["Affichez la configuration réseau du vaisseau", "Quelles sont nos coordonnées dans le secteur?"],
        check_type: "command_executed", file_target: None,
        required: "ip", skill_desc: "ip — config réseau (addr, route, link)",
    },
    CmdTemplate {
        cmd: "ufw", bloom_levels: &["Appliquer", "Créer"],
        contexts: &["Vérifiez les boucliers", "Activez le pare-feu", "Configurez les règles défensives"],
        targets: &["/securite/"],
        goals: &["Vérifiez si le pare-feu est actif", "Activez les boucliers déflecteurs", "Configurez une règle de pare-feu"],
        check_type: "command_executed", file_target: None,
        required: "ufw", skill_desc: "ufw — pare-feu (status, allow, deny, enable)",
    },
    CmdTemplate {
        cmd: "tar", bloom_levels: &["Appliquer"],
        contexts: &["Archivez les preuves", "Préparez une caisse de données", "Envoyez un colis au Galactica"],
        targets: &["/securite/logs/", "/archives/"],
        goals: &["Archivez ces fichiers dans une caisse compressée", "Préparez une archive pour l'envoi"],
        check_type: "file_exists", file_target: Some("~/archive.tar.gz"),
        required: "tar", skill_desc: "tar — archive compressée (czf, xzf, tvf)",
    },
    CmdTemplate {
        cmd: "df", bloom_levels: &["Comprendre"],
        contexts: &["Vérifiez les réserves", "Niveau des réservoirs?", "Espace disponible?"],
        targets: &["/soutes/"],
        goals: &["Vérifiez l'espace disponible dans les soutes", "Quel est le niveau de nos réserves?"],
        check_type: "command_executed", file_target: None,
        required: "df", skill_desc: "df — espace disque (-h, -i, -T)",
    },
    CmdTemplate {
        cmd: "apt", bloom_levels: &["Appliquer", "Créer"],
        contexts: &["Mettez à jour les systèmes", "Installez un nouvel équipement", "Nettoyez les vieilleries"],
        targets: &["/salle-des-machines/"],
        goals: &["Mettez à jour la liste des pièces disponibles", "Installez ce nouvel équipement", "Nettoyez les paquets inutiles"],
        check_type: "command_executed", file_target: None,
        required: "apt", skill_desc: "apt — paquets (update, install, remove, clean, autoremove)",
    },
    CmdTemplate {
        cmd: "useradd", bloom_levels: &["Appliquer"],
        contexts: &["Recrutez un nouveau membre", "Créez un badge d'accès", "Un nouvel arrivant monte à bord"],
        targets: &["/cabines/"],
        goals: &["Créez le badge d'accès de ce nouveau membre", "Enregistrez ce nouvel équipier dans le système"],
        check_type: "user_exists", file_target: Some("matelot"),
        required: "useradd", skill_desc: "useradd — crée un utilisateur (-m, -s)",
    },
    CmdTemplate {
        cmd: "sudo", bloom_levels: &["Appliquer"],
        contexts: &["Utilisez vos pouvoirs de capitaine", "Commande admin requise", "Accès restreint nécessaire"],
        targets: &["/pont/"],
        goals: &["Exécutez cette commande avec les droits du capitaine", "Montrez que vous avez les pouvoirs admin"],
        check_type: "command_executed", file_target: None,
        required: "sudo", skill_desc: "sudo — exécute en root (sudo cmd, sudo -l)",
    },
    CmdTemplate {
        cmd: "ssh", bloom_levels: &["Appliquer", "Analyser"],
        contexts: &["Établissez un canal sécurisé", "Connectez-vous à distance", "Testez la liaison cryptée"],
        targets: &["/securite/", "/communications/"],
        goals: &["Établissez une connexion sécurisée avec la station", "Testez le canal de communication crypté"],
        check_type: "command_executed", file_target: None,
        required: "ssh", skill_desc: "ssh — connexion distante sécurisée (user@host, -p)",
    },
    CmdTemplate {
        cmd: "journalctl", bloom_levels: &["Analyser"],
        contexts: &["Consultez le journal de bord automatique", "Qu'est-ce qui s'est passé récemment?", "Historique système"],
        targets: &["/salle-des-machines/"],
        goals: &["Consultez les dernières entrées du journal système", "Filtrez le journal pour un service précis"],
        check_type: "command_executed", file_target: None,
        required: "journalctl", skill_desc: "journalctl — logs systemd (-n, -u, -f, --since)",
    },
    CmdTemplate {
        cmd: "fail2ban", bloom_levels: &["Créer"],
        contexts: &["Installez le videur automatique", "Bannissez automatiquement les pirates", "Protection anti-intrusion"],
        targets: &["/securite/"],
        goals: &["Installez et activez la protection automatique anti-pirates", "Vérifiez quelles IPs ont été bannies"],
        check_type: "command_executed", file_target: None,
        required: "fail2ban", skill_desc: "fail2ban — bannissement auto (client status, jail.local)",
    },
    CmdTemplate {
        cmd: "chroot", bloom_levels: &["Créer"],
        contexts: &["Isolez un processus", "Créez une cellule d'isolement", "Enfermez dans une prison virtuelle"],
        targets: &["/soutes/"],
        goals: &["Créez une cellule d'isolement et enfermez-y un processus", "Isolez ce processus dans un environnement fermé"],
        check_type: "dir_exists", file_target: Some("/soutes/chroot"),
        required: "chroot", skill_desc: "chroot — environnement isolé (chroot /chemin cmd)",
    },
];

/// Génère N missions procédurales réparties sur les 5 actes
pub fn generate(count: usize) -> Vec<Mission> {
    let mut rng = Rng::new(SEED);
    let mut missions = Vec::with_capacity(count);
    let acts = acts_definition();

    for i in 0..count {
        let act_idx = (i * acts.len()) / count; // répartir équitablement sur les actes
        let act = &acts[act_idx.min(acts.len() - 1)];
        let tpl = rng.pick(TEMPLATES);
        let ctx: &str = tpl.contexts[rng.below(tpl.contexts.len())];
        let target: &str = tpl.targets[rng.below(tpl.targets.len())];
        let goal: &str = tpl.goals[rng.below(tpl.goals.len())];
        let bloom: &str = tpl.bloom_levels[rng.below(tpl.bloom_levels.len())];

        // ID unique global: acte.numéro_global (ex: 1.1, 3.1500)
        let id = format!("{}.{}", act_idx + 1, i + 1);

        // Story générée (20+ lignes via contexte + acte + cible)
        let story = generate_story(&mut rng, act, ctx, target, tpl.cmd);
        // Carte générée
        let map = generate_map(act, target);
        // Check
        let check = match tpl.check_type {
            "command_executed" => MissionCheck { check_type: "command_executed".into(), target: tpl.required.into(), expected: None },
            "file_exists" => MissionCheck { check_type: "file_exists".into(), target: tpl.file_target.unwrap_or("~/fichier.txt").into(), expected: None },
            "file_read" => MissionCheck { check_type: "file_read".into(), target: target.rsplit('/').next().unwrap_or(target).into(), expected: None },
            "dir_exists" => MissionCheck { check_type: "dir_exists".into(), target: tpl.file_target.unwrap_or("/soutes/").into(), expected: None },
            "dir_reached" => MissionCheck { check_type: "dir_reached".into(), target: target.into(), expected: None },
            "permission" => MissionCheck { check_type: "permission".into(), target: tpl.file_target.unwrap_or("~/notes.txt").into(), expected: Some("600".into()) },
            "user_exists" => MissionCheck { check_type: "user_exists".into(), target: tpl.file_target.unwrap_or("matelot").into(), expected: None },
            _ => MissionCheck { check_type: "command_executed".into(), target: tpl.required.into(), expected: None },
        };

        missions.push(Mission {
            id,
            chapter: (act_idx + 1) as u32,
            chapter_title: act.title.into(),
            act: (act_idx + 1) as u32,
            title: format!("{} — {}", tpl.cmd, ctx),
            description: story.clone(),
            story,
            goal: format!("{} (dans {})", goal, target),
            bloom_level: bloom.into(),
            bloom_question: format!("Comment utiliser '{}' pour {}?", tpl.cmd, goal.to_lowercase()),
            required_commands: vec![tpl.required.into()],
            checks: vec![check],
            hints: vec![],
            xp_reward: 50 + (act_idx as u64 * 25) + (rng.below(50) as u64),
            code_required: None,
            skill: Some(Skill {
                command: tpl.cmd.into(),
                name: tpl.cmd.into(),
                description: tpl.skill_desc.into(),
            }),
            map,
        });
    }
    missions
}

fn generate_story(rng: &mut Rng, act: &Act, ctx: &str, target: &str, cmd: &str) -> String {
    let pnjs = [
        "Zyra-7, l'IA de bord",
        "Commandant Vex Korshun",
        "Lieutenant Nira Okoro (communications)",
        "Ingénieur-chef Tael Rhen (machines)",
        "Officier sécurité Draz M'kal",
        "Amiral Xylia Vorn du Galactica",
        "Mécanicien Boz Tark",
        "Medic-IA Andromède",
    ];
    let pnj: &str = pnjs[rng.below(pnjs.len())];
    format!(
        "Acte {} — {}.\n\n{} vous contacte: « {} ». La situation à bord d'Orion-7 est tendue: {}\n\nLe compartiment concerné est {}. Les Corbeaux Noirs rôdent dans le secteur 7G\net chaque seconde compte. L'équipage de 5 personnes compte sur vous, Capitaine.\n\nPour avancer, vous devez utiliser vos outils de terminal. La commande adaptée à cette situation\nest disponible dans votre console. Consultez 'gsh help' si besoin, ou 'man {}' pour le manuel.\n\nN'oubliez pas: dans l'espace, la préparation et la précision font la différence entre\nla victoire et la défaite. Bonne chance, Capitaine.",
        act.num, act.title, pnj, ctx, act.situation, target, cmd
    )
}

fn generate_map(act: &Act, target: &str) -> String {
    format!(
        "VUE DE DESSUS — Acte {} ({}):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │SOUTES  │\n  └───┬────┘ └───┬────┘ └───┬────┘\n      │          ▼          │\n      │     [VOUS: cabine]  │\n      └──────────┬──────────┘\n            ▶ cible: {}",
        act.num, act.title, target
    )
}

//! Campagne 3 (Nexus Réseau, niveau DIFFICILE) — CHAPITRE 12 — Stockage et disques (Acte 3) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "12.1".into(), chapter: 12, chapter_title: "Stockage et disques".into(), act: 3,
            title: "Espace disque".into(),
            description: "Quelle place reste sur les serveurs?".into(),
            story: "Rosa: « Quelle place disque reste-t-il? Taille lisible. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa regarde l'espace: « Voilà la place restante, netrunner. Le stockage est sous contrôle. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez l'espace disque (df -h).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'espace disque?".into(),
            required_commands: vec!["df -h".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "df".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-h".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "df -h".into(), name: "df -h".into(), description: "df -h — espace disque lisible".into() }),
        },
        Mission {
            id: "12.2".into(), chapter: 12, chapter_title: "Stockage et disques".into(), act: 3,
            title: "Taille des logs par fichier".into(),
            description: "Taille de chaque log individuellement.".into(),
            story: "Karl: « Pesez chaque log UN PAR UN. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl pèse chaque log: « Voilà la taille de chaque fichier, netrunner. Nous saurons quoi archiver. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez la taille de chaque fichier de /logs (du -sh /logs/*).".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir la taille de chaque fichier?".into(),
            required_commands: vec!["du -sh /logs/*".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "du".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-sh".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "du -sh".into(), name: "du -sh".into(), description: "du -sh — taille d'un dossier".into() }),
        },
        Mission {
            id: "12.3".into(), chapter: 12, chapter_title: "Stockage et disques".into(), act: 3,
            title: "Disques montés".into(),
            description: "Quels disques sont montés?".into(),
            story: "Rosa: « Quels systèmes de fichiers sont montés sur le NOC? »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa vérifie les montages: « Les systèmes de fichiers sont listés, netrunner. Le NOC est bien monté. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez les disques montés (mount).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les disques montés?".into(),
            required_commands: vec!["mount".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "mount".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "mount".into(), name: "mount".into(), description: "mount — disques montés".into() }),
        },
        Mission {
            id: "12.4".into(), chapter: 12, chapter_title: "Stockage et disques".into(), act: 3,
            title: "Etat des baies".into(),
            description: "Lisez l'état des baies.".into(),
            story: "Karl: « Lisez l'état des baies: /baies/etat_baies.txt. La baie A3 a un disque à changer! »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl lit l'état des baies: « La baie A3 a bien un disque à changer, netrunner. Nous allons intervenir. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Lisez etat_baies.txt.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire l'état des baies?".into(),
            required_commands: vec!["cat /baies/etat_baies.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "etat_baies.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "12.5".into(), chapter: 12, chapter_title: "Stockage et disques".into(), act: 3,
            title: "Cluster".into(),
            description: "Quels serveurs forment le cluster?".into(),
            story: "Rosa: « Lisez la liste du cluster: /serveurs/cluster.txt. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa lit la liste: « Le cluster est répertorié, netrunner. Chaque serveur est identifié. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Lisez cluster.txt.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire la liste du cluster?".into(),
            required_commands: vec!["cat /serveurs/cluster.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "cluster.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
    ]
}

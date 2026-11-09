//! Campagne 3 (Nexus Réseau, niveau DIFFICILE) — CHAPITRE 14 — Fail2ban et services (Acte 4) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "14.1".into(), chapter: 14, chapter_title: "Fail2ban et services".into(), act: 4,
            title: "Statut fail2ban".into(),
            description: "Quelles IP sont bannies?".into(),
            story: "Karl: « Le système anti-bruteforce a banni quelles IP? Statut. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl consulte le statut: « Voilà les IP bannies par fail2ban, netrunner. L'anti-bruteforce travaille. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez l'état de la protection anti-intrusion du NOC.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les bannissements?".into(),
            required_commands: vec!["sudo fail2ban-client status".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "fail2ban-client".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "status".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "fail2ban-client status".into(), name: "fail2ban-client status".into(), description: "fail2ban-client status — bannissements".into() }),
        },
        Mission {
            id: "14.2".into(), chapter: 14, chapter_title: "Fail2ban et services".into(), act: 4,
            title: "Débannir le FAI".into(),
            description: "Le FAI (10.0.0.254) a été banni par erreur.".into(),
            story: "Rosa, paniquée: « Le fail2ban a banni le FAI 10.0.0.254 par erreur! Débanissez-le! »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa, soulagée: « Le FAI 10.0.0.254 est débanni, netrunner. La liaison est rétablie. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Débanissez 10.0.0.254 (sudo fail2ban-client unban 10.0.0.254).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment révoquer un bannissement?".into(),
            required_commands: vec!["sudo fail2ban-client unban 10.0.0.254".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "unban".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "fail2ban-client unban".into(), name: "fail2ban-client unban".into(), description: "fail2ban-client unban IP — révoque un ban".into() }),
        },
        Mission {
            id: "14.3".into(), chapter: 14, chapter_title: "Fail2ban et services".into(), act: 4,
            title: "État du service SSH".into(),
            description: "sshd tourne-t-il?".into(),
            story: "Karl: « Vérifiez que sshd tourne bien. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl vérifie: « sshd tourne bien, netrunner. Le service est en ordre. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez l'état du service ssh (systemctl status ssh).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'état d'un service?".into(),
            required_commands: vec!["systemctl status ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "systemctl".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "status".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "systemctl status".into(), name: "systemctl status".into(), description: "systemctl status — état d'un service".into() }),
        },
        Mission {
            id: "14.4".into(), chapter: 14, chapter_title: "Fail2ban et services".into(), act: 4,
            title: "Redémarrer SSH".into(),
            description: "Redémarrez ssh après les changements.".into(),
            story: "Karl: « Redémarrez ssh pour appliquer les réglages. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl confirme: « ssh est redémarré, netrunner. Les réglages sont appliqués. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Redémarrez le service SSH pour appliquer la nouvelle configuration.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment redémarrer un service?".into(),
            required_commands: vec!["sudo systemctl restart ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "restart".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "systemctl restart".into(), name: "systemctl restart".into(), description: "systemctl restart — redémarre un service".into() }),
        },
        Mission {
            id: "14.5".into(), chapter: 14, chapter_title: "Fail2ban et services".into(), act: 4,
            title: "Services critiques".into(),
            description: "Lisez la liste des services critiques.".into(),
            story: "Rosa: « Lisez la liste des services critiques: /configs/services/services_critiques.txt. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa lit la liste: « Les services critiques sont répertoriés, netrunner. Nous savons quoi protéger. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Lisez services_critiques.txt.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire la liste des services?".into(),
            required_commands: vec!["cat /configs/services/services_critiques.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "services_critiques.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
    ]
}

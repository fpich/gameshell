//! Campagne 3 (Nexus Réseau, niveau DIFFICILE) — CHAPITRE 1 — Arrivee au NOC (Acte 1) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "1.1".into(), chapter: 1, chapter_title: "Arrivee au NOC".into(), act: 1,
            title: "Premier contact avec Nexus".into(),
            description: "Vous prenez le poste netrunner au NOC Neon-9.".into(),
            story: "Le sas du NOC Neon-9 se referme. Vous êtes le nouveau netrunner, installé
à votre poste (indiqué par ~).

Rosa, netops, vous parle par le canal interne:
« Netrunner? Vous m'entendez? Dites-moi où vous êtes exactement dans le
datacenter, qu'on synchronise nos écrans de monitoring! »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Rosa confirme sur le canal: « Reçu, netrunner. Vous êtes bien au poste. Nos écrans de monitoring sont synchronisés. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Montrez à Rosa où vous êtes dans le NOC et ce qui vous entoure.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelle commande affiche le répertoire courant?".into(),
            required_commands: vec!["pwd".into(), "ls".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "pwd".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "ls".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            skill: Some(Skill { command: "pwd".into(), name: "pwd".into(), description: "pwd — affiche le répertoire courant".into() }),
        },
        Mission {
            id: "1.2".into(), chapter: 1, chapter_title: "Arrivee au NOC".into(), act: 1,
            title: "Le guide du NOC".into(),
            description: "Lisez bienvenue.txt à votre poste.".into(),
            story: "Rosa poursuit: « Lisez le guide bienvenue.txt de votre poste.
Il explique le fonctionnement du datacenter. »

Le fichier contient les bases: comment naviguer dans les salles,
qui contacter, les consignes de sécurité réseau.

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Rosa approuve: « Bien lu, netrunner. Vous connaissez les bases du datacenter. Un opérateur informé ne se laisse jamais surprendre. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Lisez le guide bienvenue.txt de votre poste.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier texte?".into(),
            required_commands: vec!["cat bienvenue.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "bienvenue.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: Some("NEXUS-ACCUEIL".into()),
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche le contenu d'un fichier".into() }),
        },
        Mission {
            id: "1.3".into(), chapter: 1, chapter_title: "Arrivee au NOC".into(), act: 1,
            title: "Config SSH du cluster".into(),
            description: "Lisez la config SSH dans /configs/sshd/.".into(),
            story: "Karl, sécurité: « Netrunner! Lisez la config SSH du cluster:
/configs/sshd/sshd_config. On doit la durcir. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl hoche la tête: « La config SSH est en vue, netrunner. On va pouvoir la durcir. Le cluster vous remercie. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Allez lire la config sshd_config dans /configs/sshd/.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment aller dans une salle et lire un fichier?".into(),
            required_commands: vec!["cd /configs/sshd".into(), "cat sshd_config".into()],
            checks: vec![MissionCheck { check_type: "dir_reached".into(), target: "/configs/sshd".into(), expected: None }, MissionCheck { check_type: "file_read".into(), target: "sshd_config".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: Some("NEXUS-SSH-22".into()),
            skill: Some(Skill { command: "cd".into(), name: "cd".into(), description: "cd — change de répertoire".into() }),
        },
        Mission {
            id: "1.4".into(), chapter: 1, chapter_title: "Arrivee au NOC".into(), act: 1,
            title: "Qui êtes-vous?".into(),
            description: "Prouvez votre identité au contrôle d'accès.".into(),
            story: "Le contrôle d'accès du NOC vérifie votre badge avant d'ouvrir la salle serveurs:
nom du NOC, votre user, vos identifiants complets (uid, gid, groupes).

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Le contrôle d'accès valide: « Badge confirmé, netrunner. Vous êtes bien du NOC. La salle serveurs vous est ouverte. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Prouvez votre identité au contrôle d'accès : nom du NOC, votre user et vos identifiants complets.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelles commandes donnent hostname/user/uid?".into(),
            required_commands: vec!["hostname".into(), "whoami".into(), "id".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "hostname".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "whoami".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "id".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "id".into(), name: "id".into(), description: "id — affiche uid, gid et groupes".into() }),
        },
        Mission {
            id: "1.5".into(), chapter: 1, chapter_title: "Arrivee au NOC".into(), act: 1,
            title: "Horodatage des incidents".into(),
            description: "Vérifiez l'horloge du NOC pour corréler les logs.".into(),
            story: "Rosa: « Netrunner, pour corréler les événements dans les logs, l'horloge
du NOC doit être exacte (UTC). Vérifiez l'heure système. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Rosa vérifie l'horloge: « L'heure est exacte, netrunner. Nous pourrons corréler les événements des logs sans erreur. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez la date et l'heure du système.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelle commande donne la date et l'heure?".into(),
            required_commands: vec!["date".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "date".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            skill: Some(Skill { command: "date".into(), name: "date".into(), description: "date — affiche la date et l'heure".into() }),
        },
    ]
}

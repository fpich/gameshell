//! Campagne 3 (Nexus Réseau, niveau DIFFICILE) — CHAPITRE 3 — Exploration du datacenter (Acte 1) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "3.1".into(), chapter: 3, chapter_title: "Exploration du datacenter".into(), act: 1,
            title: "Fichiers cachés du poste".into(),
            description: "Le token secret est caché (.secret).".into(),
            story: "Karl vous souffle: « Listez TOUT votre poste, fichiers cachés compris,
puis lisez .secret. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Karl hoche la tête: « Vous avez trouvé le fichier caché, netrunner. Rien n'échappe à un œil attentif. Le poste vous est transparent. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Listez votre poste en détail (cachés compris) et lisez .secret.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir aussi les fichiers cachés?".into(),
            required_commands: vec!["ls -la".into(), "cat .secret".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-la".into(), expected: None }, MissionCheck { check_type: "file_read".into(), target: ".secret".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ls -la".into(), name: "ls -la".into(), description: "ls -la — liste tout, cachés compris".into() }),
        },
        Mission {
            id: "3.2".into(), chapter: 3, chapter_title: "Exploration du datacenter".into(), act: 1,
            title: "Trouver un log".into(),
            description: "Trouvez firewall.log dans tout le NOC.".into(),
            story: "Rosa a perdu le chemin du log pare-feu: « Trouvez où se cache
firewall.log dans le datacenter. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Rosa est rassurée: « firewall.log est retrouvé, netrunner. Grâce à vous, le NOC ne perd plus aucune trace. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Trouvez le chemin complet de firewall.log.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment chercher un fichier par nom partout?".into(),
            required_commands: vec!["find / -name firewall.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "find".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-name".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "find".into(), name: "find".into(), description: "find -name — recherche par nom".into() }),
        },
        Mission {
            id: "3.3".into(), chapter: 3, chapter_title: "Exploration du datacenter".into(), act: 1,
            title: "Taille des logs".into(),
            description: "Quelle taille occupent les logs?".into(),
            story: "Karl: « Avant d'archiver, dites-moi combien occupent les logs au total. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl note: « Voilà la taille des logs, netrunner. Nous pourrons archiver en connaissance de cause. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Donnez la taille totale du dossier /logs.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment connaître la taille totale d'un dossier?".into(),
            required_commands: vec!["du -sh /logs".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "du".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-sh".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "du".into(), name: "du".into(), description: "du — taille des fichiers et dossiers".into() }),
        },
        Mission {
            id: "3.4".into(), chapter: 3, chapter_title: "Exploration du datacenter".into(), act: 1,
            title: "Salle des baies".into(),
            description: "Rejoignez la salle des baies.".into(),
            story: "Karl vous convoque dans la salle des baies: « Netrunner! Réunion
dans la salle BAIES immédiatement. Scan détecté. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Karl vous accueille dans la salle des baies: « Vous voilà, netrunner. Le point sur le scan peut commencer. Nous sommes prêts. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Rejoignez la salle des baies (/baies).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment aller à /baies en chemin absolu?".into(),
            required_commands: vec!["cd /baies".into()],
            checks: vec![MissionCheck { check_type: "dir_reached".into(), target: "/baies".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cd".into(), name: "cd".into(), description: "cd — change de répertoire".into() }),
        },
        Mission {
            id: "3.5".into(), chapter: 3, chapter_title: "Exploration du datacenter".into(), act: 1,
            title: "Note d'alerte".into(),
            description: "Écrivez l'alerte dans alerte.txt.".into(),
            story: "Rosa: « Notez dans alerte.txt à votre poste: 'Scan SYN depuis 10.0.0.66'. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa relit alerte.txt: « 'Scan SYN depuis 10.0.0.66'. C'est noté, netrunner. L'alerte est consignée. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Créez alerte.txt avec le texte demandé.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment écrire un texte directement dans un fichier?".into(),
            required_commands: vec!["echo 'Scan SYN depuis 10.0.0.66' > alerte.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "~/alerte.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "echo".into(), name: "echo".into(), description: "echo — affiche du texte à l'écran".into() }),
        },
    ]
}

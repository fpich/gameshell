//! Campagne 3 (Nexus Réseau, niveau DIFFICILE) — CHAPITRE 6 — Ports et connexions (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "6.1".into(), chapter: 6, chapter_title: "Ports et connexions".into(), act: 2,
            title: "Ports en écoute".into(),
            description: "Quels ports Nexus écoute?".into(),
            story: "Karl: « Quels ports sont OUVERTS sur le NOC? Connexions actives + écoute, avec processus. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl parcourt la liste: « Voilà les ports ouverts et leurs processus, netrunner. La surface d'attaque est visible. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez les ports en écoute avec processus (ss -tlnp).".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les ports en écoute avec les processus?".into(),
            required_commands: vec!["ss -tlnp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ss".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-tlnp".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ss -tlnp".into(), name: "ss -tlnp".into(), description: "ss -tlnp — ports en écoute + processus".into() }),
        },
        Mission {
            id: "6.2".into(), chapter: 6, chapter_title: "Ports et connexions".into(), act: 2,
            title: "Connexions établies".into(),
            description: "Qui est connecté au NOC en ce moment?".into(),
            story: "Karl: « Quelles connexions sont ÉTABLIES actuellement? »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl examine les connexions: « Voilà les sessions établies, netrunner. Rien de suspect pour l'instant. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez les connexions établies (ss -tn state established).".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment filtrer les connexions établies?".into(),
            required_commands: vec!["ss -tn state established".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ss".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "established".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ss -tn".into(), name: "ss -tn".into(), description: "ss -tn — connexions TCP".into() }),
        },
        Mission {
            id: "6.3".into(), chapter: 6, chapter_title: "Ports et connexions".into(), act: 2,
            title: "Ancien outil netstat".into(),
            description: "Vérifiez avec netstat aussi.".into(),
            story: "Rosa: « Certains vieux serveurs utilisent encore netstat. Listez les ports avec. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa vérifie avec netstat: « Les ports en écoute sont listés, netrunner. Les vieux serveurs sont couverts. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez les ports en écoute de votre poste avec le détail des processus.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les ports avec netstat?".into(),
            required_commands: vec!["netstat -tlnp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "netstat".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-tlnp".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "netstat -tlnp".into(), name: "netstat -tlnp".into(), description: "netstat -tlnp — ports en écoute (outil classique)".into() }),
        },
        Mission {
            id: "6.4".into(), chapter: 6, chapter_title: "Ports et connexions".into(), act: 2,
            title: "Scan d'un port précis".into(),
            description: "Le port 3306 (mysql) est-il ouvert sur l'attaquant?".into(),
            story: "Karl: « Scannez un seul port sur 10.0.0.66: le 3306 (mysql). »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl regarde le résultat du scan: « Le port 3306 de 10.0.0.66 est renseigné, netrunner. Nous savons ce qu'il expose. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Scannez le port 3306 de 10.0.0.66 (nmap -p 3306).".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment scanner un port précis?".into(),
            required_commands: vec!["nmap -p 3306 10.0.0.66".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "nmap".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-p".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "nmap -p".into(), name: "nmap -p".into(), description: "nmap -p PORT — scan d'un port précis".into() }),
        },
        Mission {
            id: "6.5".into(), chapter: 6, chapter_title: "Ports et connexions".into(), act: 2,
            title: "Chemin vers la cible".into(),
            description: "Par quels sauts passent les paquets vers 10.0.0.66?".into(),
            story: "Rosa: « Tracez la route des paquets vers l'attaquant 10.0.0.66. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa suit la route: « La trace vers l'attaquant est claire, netrunner. Nous savons par où il passe. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Tracez la route vers 10.0.0.66 (traceroute).".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment suivre le chemin des paquets?".into(),
            required_commands: vec!["traceroute 10.0.0.66".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "traceroute".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "traceroute".into(), name: "traceroute".into(), description: "traceroute — chemin des paquets (sauts)".into() }),
        },
    ]
}

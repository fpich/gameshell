//! Campagne 3 (Nexus Réseau, niveau DIFFICILE) — CHAPITRE 7 — DNS et HTTP (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "7.1".into(), chapter: 7, chapter_title: "DNS et HTTP".into(), act: 2,
            title: "Résolution DNS".into(),
            description: "Quelle IP derrière nexus.neon9.local?".into(),
            story: "Rosa: « Résolvez nexus.neon9.local en IP. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa confirme: « nexus.neon9.local est résolu, netrunner. Le DNS interne répond. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Résolvez nexus.neon9.local (dig +short).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment résoudre un nom de domaine?".into(),
            required_commands: vec!["dig +short nexus.neon9.local".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "dig".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "+short".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "dig +short".into(), name: "dig +short".into(), description: "dig +short — résolution DNS courte".into() }),
        },
        Mission {
            id: "7.2".into(), chapter: 7, chapter_title: "DNS et HTTP".into(), act: 2,
            title: "Lire les DNS du NOC".into(),
            description: "Lisez le fichier DNS du NOC.".into(),
            story: "Karl: « Lisez les enregistrements DNS internes: /captures/dns_nexus.txt. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl lit les enregistrements: « Les entrées DNS internes sont claires, netrunner. La zone est documentée. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Lisez le fichier dns_nexus.txt des captures.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire le fichier DNS?".into(),
            required_commands: vec!["cat /captures/dns_nexus.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "dns_nexus.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "7.3".into(), chapter: 7, chapter_title: "DNS et HTTP".into(), act: 2,
            title: "API interne du NOC".into(),
            description: "Interrogez l'API de statut du cluster.".into(),
            story: "Rosa: « Interrogez l'API interne de statut: curl -s http://10.0.0.1:8080/status. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa consulte la réponse: « L'API de statut répond, netrunner. Le service est en ligne. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Interrogez l'API de statut en HTTP (curl -s).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment interroger une API HTTP?".into(),
            required_commands: vec!["curl -s http://10.0.0.1:8080/status".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "curl".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-s".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "curl -s".into(), name: "curl -s".into(), description: "curl -s — requête HTTP silencieuse".into() }),
        },
        Mission {
            id: "7.4".into(), chapter: 7, chapter_title: "DNS et HTTP".into(), act: 2,
            title: "Capture du scan".into(),
            description: "Lisez la capture du scan de l'attaquant.".into(),
            story: "Karl: « Lisez la capture du scan de 10.0.0.66: /captures/scan_10.0.0.66.txt. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl lit la capture: « Le scan de 10.0.0.66 est documenté, netrunner. Nous avons la preuve de l'attaque. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Lisez la capture scan_10.0.0.66.txt.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment lire une capture réseau?".into(),
            required_commands: vec!["cat /captures/scan_10.0.0.66.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "scan_10.0.0.66.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "7.5".into(), chapter: 7, chapter_title: "DNS et HTTP".into(), act: 2,
            title: "Ports documentés".into(),
            description: "Quels ports sont documentés sur Nexus?".into(),
            story: "Rosa: « Lisez la doc des ports: /captures/ports_nexus.txt. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa lit la doc des ports: « Les ports du NOC sont répertoriés, netrunner. La référence est à jour. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Lisez ports_nexus.txt.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire la doc des ports?".into(),
            required_commands: vec!["cat /captures/ports_nexus.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "ports_nexus.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
    ]
}

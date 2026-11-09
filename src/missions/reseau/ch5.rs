//! Campagne 3 (Nexus Réseau, niveau DIFFICILE) — CHAPITRE 5 — Diagnostic reseau (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "5.1".into(), chapter: 5, chapter_title: "Diagnostic reseau".into(), act: 2,
            title: "Carte réseau du NOC".into(),
            description: "Configuration réseau complète de Nexus.".into(),
            story: "Rosa: « Quelles sont les interfaces et IP du NOC? Donnez la config complète. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa approuve: « Interfaces et IP sont claires, netrunner. La carte du réseau est dressée. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez la configuration réseau complète (ip addr).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les interfaces et IP?".into(),
            required_commands: vec!["ip addr".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ip".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "addr".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip addr".into(), name: "ip addr".into(), description: "ip addr — interfaces et IP".into() }),
        },
        Mission {
            id: "5.2".into(), chapter: 5, chapter_title: "Diagnostic reseau".into(), act: 2,
            title: "Table de routage".into(),
            description: "Par où passe le trafic vers le FAI?".into(),
            story: "Rosa: « Par quelle passerelle passe le trafic vers l'extérieur?
Montrez la table de routage. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Rosa vérifie la table de routage: « La passerelle est identifiée, netrunner. Le trafic sortant est cartographié. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez la table de routage (ip route).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les routes?".into(),
            required_commands: vec!["ip route".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "route".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip route".into(), name: "ip route".into(), description: "ip route — table de routage".into() }),
        },
        Mission {
            id: "5.3".into(), chapter: 5, chapter_title: "Diagnostic reseau".into(), act: 2,
            title: "Liens réseau".into(),
            description: "État des interfaces physiques.".into(),
            story: "Karl: « L'interface eth0 est-elle UP? Montrez l'état des liens. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl confirme: « eth0 est bien UP, netrunner. Les liens sont opérationnels. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez l'état des interfaces (ip link).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'état des interfaces?".into(),
            required_commands: vec!["ip link".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "link".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip link".into(), name: "ip link".into(), description: "ip link — état des interfaces".into() }),
        },
        Mission {
            id: "5.4".into(), chapter: 5, chapter_title: "Diagnostic reseau".into(), act: 2,
            title: "Voisins ARP".into(),
            description: "Quelles machines sont sur le réseau local?".into(),
            story: "Rosa: « Quelles IP sont directement joignables sur le LAN?
Montrez la table ARP (voisins). »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa examine la table ARP: « Les voisins directs sont listés, netrunner. Le LAN est cartographié. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez la table des voisins ARP (ip neigh).".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les voisins du réseau local?".into(),
            required_commands: vec!["ip neigh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "neigh".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ip neigh".into(), name: "ip neigh".into(), description: "ip neigh — table ARP des voisins".into() }),
        },
        Mission {
            id: "5.5".into(), chapter: 5, chapter_title: "Diagnostic reseau".into(), act: 2,
            title: "Ping vers le FAI".into(),
            description: "Le FAI (10.0.0.254) répond-il?".into(),
            story: "Rosa: « Le lien vers le FAI est instable. Envoyez 4 pings vers 10.0.0.254. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa regarde les pings: « Le lien vers le FAI répond, netrunner. La liaison est stable. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Envoyez 4 pings vers 10.0.0.254.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment tester la connectivité?".into(),
            required_commands: vec!["ping -c 4 10.0.0.254".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ping".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-c".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ping -c".into(), name: "ping -c".into(), description: "ping -c N — N paquets de test".into() }),
        },
    ]
}

//! Campagne 3 (Nexus Réseau, niveau DIFFICILE) — CHAPITRE 20 — Isolement total (Acte 5) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "20.1".into(), chapter: 20, chapter_title: "Isolement total".into(), act: 5,
            title: "Token du netrunner".into(),
            description: "Prouvez votre identité avec le token caché.".into(),
            story: "Karl: « Avant les renforts du SOC, prouvez que vous êtes le netrunner:
donnez le TOKEN (fichier caché .secret de votre poste). »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Karl vérifie le token: « Token confirmé, netrunner. Vous êtes bien le netrunner du NOC. Les renforts du SOC arrivent. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Trouvez le token dans .secret et validez avec ce code.".into(),
            bloom_level: "Évaluer".into(),
            bloom_question: "Où est le token secret?".into(),
            required_commands: vec!["cat .secret".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: ".secret".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: Some("NEON-77-KAMI".into()),
            skill: Some(Skill { command: "cat .secret".into(), name: "cat .secret".into(), description: "cat .secret — lit le token caché".into() }),
        },
        Mission {
            id: "20.2".into(), chapter: 20, chapter_title: "Isolement total".into(), act: 5,
            title: "Code du pare-feu".into(),
            description: "Validez avec le code du coffre-fort réseau.".into(),
            story: "Karl: « Le coffre-fort réseau est verrouillé. Donnez le CODE PARE-FEU
(/firewall/coffre/codes.txt). »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Karl vérifie le code: « Code pare-feu confirmé, netrunner. Le coffre réseau est ouvert. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Trouvez le code pare-feu et validez avec ce code.".into(),
            bloom_level: "Évaluer".into(),
            bloom_question: "Où est le code pare-feu?".into(),
            required_commands: vec!["cat /firewall/coffre/codes.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "codes.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: Some("BLACKOUT-42".into()),
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — lit les codes".into() }),
        },
        Mission {
            id: "20.3".into(), chapter: 20, chapter_title: "Isolement total".into(), act: 5,
            title: "Bloquer tout le trafic entrant".into(),
            description: "Dernière règle: tout bloquer sauf SSH.".into(),
            story: "Karl: « Règle finale: refusez TOUT le trafic entrant sauf le 22. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl approuve: « Tout le trafic entrant est bloqué sauf le 22, netrunner. Le NOC est verrouillé. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Bloquez tout sauf SSH (sudo ufw default deny incoming + allow 22).".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Comment faire une règle par défaut?".into(),
            required_commands: vec!["sudo ufw default deny incoming".into(), "sudo ufw allow 22/tcp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "default".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "deny".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "allow".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: "ufw default".into(), name: "ufw default".into(), description: "ufw default deny incoming — bloque tout par défaut".into() }),
        },
        Mission {
            id: "20.4".into(), chapter: 20, chapter_title: "Isolement total".into(), act: 5,
            title: "Dernier scan défensif".into(),
            description: "Scannez nos propres ports pour vérifier.".into(),
            story: "Rosa: « Scannez nos propres ports (10.0.0.1) pour vérifier la surface d'attaque. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa examine le scan: « La surface d'attaque de 10.0.0.1 est mesurée, netrunner. Nous savons ce qui est exposé. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Scannez 10.0.0.1 (nmap -sS 10.0.0.1).".into(),
            bloom_level: "Évaluer".into(),
            bloom_question: "Comment scanner sa propre machine?".into(),
            required_commands: vec!["nmap -sS 10.0.0.1".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "nmap".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-sS".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: "nmap -sS".into(), name: "nmap -sS".into(), description: "nmap -sS — scan SYN furtif".into() }),
        },
        Mission {
            id: "20.5".into(), chapter: 20, chapter_title: "Isolement total".into(), act: 5,
            title: "Protocole isolement total".into(),
            description: "Dernière épreuve: validez avec le protocole du coffre-fort.".into(),
            story: "Karl: « Dernière épreuve: donnez le PROTOCOLE D'ISOLEMENT TOTAL
(/firewall/coffre/codes.txt) pour lancer la contre-mesure décisive. »

Lisez le coffre, posez une caméra d'audit sur /archives, validez avec ce code.

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Karl, solennel: « Protocole d'isolement total confirmé, netrunner. La contre-mesure décisive est lancée. Le réseau Neon-9 est sauvé. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Lisez le coffre, audit sur /archives, validez avec le protocole.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Quel est le protocole d'isolement?".into(),
            required_commands: vec!["cat /firewall/coffre/codes.txt".into(), "sudo auditctl -w /archives -p wa -k final".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "codes.txt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "auditctl".into(), expected: None }],
            hints: vec![],
            xp_reward: 500,
            code_required: Some("SHADOW-9-REI".into()),
            skill: Some(Skill { command: "auditctl -w".into(), name: "auditctl -w".into(), description: "auditctl -w — caméra finale".into() }),
        },
    ]
}

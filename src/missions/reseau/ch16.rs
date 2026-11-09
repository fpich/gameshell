//! Campagne 3 (Nexus Réseau, niveau DIFFICILE) — CHAPITRE 16 — Permissions du NOC (Acte 4) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "16.1".into(), chapter: 16, chapter_title: "Permissions du NOC".into(), act: 4,
            title: "Verrouiller le plan d'incident".into(),
            description: "plan_incident.sh en 700.".into(),
            story: "Karl: « plan_incident.sh contient les procédures secrètes. Verrouillez-le: 700. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl approuve: « plan_incident.sh est verrouillé à 700, netrunner. Seul vous pouvez le lire. Les procédures sont en sûreté. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Mettez les permissions 700 sur plan_incident.sh.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment verrouiller un fichier?".into(),
            required_commands: vec!["chmod 700 plan_incident.sh".into()],
            checks: vec![MissionCheck { check_type: "permission".into(), target: "~/plan_incident.sh".into(), expected: Some("700".into()) }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chmod 700".into(), name: "chmod 700".into(), description: "chmod 700 — verrouillage total".into() }),
        },
        Mission {
            id: "16.2".into(), chapter: 16, chapter_title: "Permissions du NOC".into(), act: 4,
            title: "Rendre exécutable".into(),
            description: "Le plan d'incident doit être lançable.".into(),
            story: "Karl: « Ajoutez le droit d'exécution à plan_incident.sh. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl hoche la tête: « plan_incident.sh est exécutable, netrunner. En cas d'urgence, nous pourrons le lancer aussitôt. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Rendez plan_incident.sh exécutable (chmod +x).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment rendre exécutable?".into(),
            required_commands: vec!["chmod +x plan_incident.sh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "+x".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chmod +x".into(), name: "chmod +x".into(), description: "chmod +x — droit d'exécution".into() }),
        },
        Mission {
            id: "16.3".into(), chapter: 16, chapter_title: "Permissions du NOC".into(), act: 4,
            title: "Fiche du plan".into(),
            description: "Métadonnées de plan_incident.sh.".into(),
            story: "Rosa: « Fiche technique complète de plan_incident.sh. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa consulte la fiche: « Taille, permissions, dates... tout y est, netrunner. Le plan est parfaitement documenté. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Affichez les métadonnées (stat).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les métadonnées?".into(),
            required_commands: vec!["stat plan_incident.sh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "stat".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "stat".into(), name: "stat".into(), description: "stat — métadonnées".into() }),
        },
        Mission {
            id: "16.4".into(), chapter: 16, chapter_title: "Permissions du NOC".into(), act: 4,
            title: "Transférer la config".into(),
            description: "La config ufw vous appartient.".into(),
            story: "Karl: « Pour l'audit, /configs/ufw/regles.txt doit vous appartenir. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Karl confirme: « regles.txt vous appartient officiellement, netrunner. L'audit sera sans reproche. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Changez le propriétaire de regles.txt (sudo chown $USER).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment changer le propriétaire?".into(),
            required_commands: vec!["sudo chown $USER /configs/ufw/regles.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chown".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chown".into(), name: "chown".into(), description: "chown — change le propriétaire".into() }),
        },
        Mission {
            id: "16.5".into(), chapter: 16, chapter_title: "Permissions du NOC".into(), act: 4,
            title: "Vos pouvoirs".into(),
            description: "Vos droits sudo.".into(),
            story: "Rosa: « Pour l'audit, listez vos droits sudo. »

Une vague de scans SYN balaie le secteur depuis 10.0.0.66. Chaque seconde compte. L'équipe netops attend votre diagnostic.

Le NOC Nexus-9 est le cœur du réseau Neon-9. Cent serveurs comptent sur vous, netrunner.

N'oubliez pas: un réseau bien surveillé ne tombe jamais. La détection fait la différence entre un scan et une intrusion.

L'IDS a levé une alerte: bruteforce SSH détecté depuis 10.0.0.99.

Le pare-feu bloque déjà les ports sensibles. Sans configuration stricte, tout le cluster est vulnérable.

Chaque commande que vous apprenez ici sert à protéger le réseau et ses utilisateurs.

Rosa inspecte la liste: « Vos droits sudo sont clairs, netrunner. L'audit est complet. »

Le NOC Neon-9 poursuit sa veille. Les cent serveurs du réseau comptent sur vous, netrunner.".into(),
            goal: "Listez les commandes que vous êtes autorisé à exécuter avec les pouvoirs du netrunner.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment lister ses droits sudo?".into(),
            required_commands: vec!["sudo -l".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "sudo".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "sudo -l".into(), name: "sudo -l".into(), description: "sudo -l — droits sudo".into() }),
        },
    ]
}

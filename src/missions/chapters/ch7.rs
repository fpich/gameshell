//! CHAPITRE 7 - Sauvegarde
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "7.1".into(), chapter: 7, chapter_title: "Sauvegarde".into(), act: 4,
            title: "Archiver les logs".into(),
            description: "Archivez avant contre-attaque!".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Avant la contre-attaque, il faut archiver tous les logs comme preuves.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Archiver les logs est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Avant la contre-attaque, il faut archiver tous les logs comme preuves.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Avant la contre-attaque, archivez tous les logs comme preuves.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Creer une archive?".into(),
            required_commands: vec!["tar czf archive.tar.gz dossier/".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tar".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "~/logs.tar.gz".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            map: "VUE DE DESSUS — compartiment SÉCURITÉ (archivage logs):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶SECUR ◀│\n  └───┬────┘ └───┬────┘ └───┬────┘\n      └────► /securite/logs/".into(),
            skill: Some(Skill { command: "tar".into(), name: "tar".into(), description: "tar".into() }),
        },
        Mission {
            id: "7.2".into(), chapter: 7, chapter_title: "Sauvegarde".into(), act: 4,
            title: "ZIP".into(),
            description: "Le Galactica prefere le ZIP.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Le Galactica demande les rapports au format ZIP pour compatibilité.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Compresser en ZIP est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Le Galactica demande les rapports au format ZIP pour compatibilité.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Le Galactica demande vos rapports au format ZIP. Compressez-les.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Faire un ZIP?".into(),
            required_commands: vec!["zip -r archive.zip dossier/".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "zip".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "~/rapports.zip".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine (envoi ZIP au Galactica):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │▶CABINE◀│ │SOUTES  │\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "zip".into(), name: "zip".into(), description: "zip".into() }),
        },
        Mission {
            id: "7.3".into(), chapter: 7, chapter_title: "Sauvegarde".into(), act: 4,
            title: "Espace disque".into(),
            description: "Espace restant?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Avant d'archiver, vérifiez qu'il reste assez d'espace dans les soutes.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Vérifier l'espace disque est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Avant d'archiver, vérifiez qu'il reste assez d'espace dans les soutes.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Vérifiez qu'il reste assez de place dans les soutes avant d'archiver.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Voir l'espace disque?".into(),
            required_commands: vec!["df -h".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "df".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            map: "VUE DE DESSUS — soutes (jauges carburant/données):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶SOUTES◀│\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "df".into(), name: "df".into(), description: "df".into() }),
        },
        Mission {
            id: "7.4".into(), chapter: 7, chapter_title: "Sauvegarde".into(), act: 4,
            title: "Gros fichiers".into(),
            description: "Quel compartiment est gros?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Quelle soute prend le plus de place? Il faut le savoir pour optimiser le stockage.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Trouver les gros fichiers est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Quelle soute prend le plus de place? Il faut le savoir pour optimiser le stockage.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Quelle soute est la plus pleine? Pesmez-les.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Taille des dossiers?".into(),
            required_commands: vec!["du -sh dossier/".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "du".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            map: "VUE DE DESSUS — soutes (pesée des compartiments):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶SOUTES◀│\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "du".into(), name: "du".into(), description: "du".into() }),
        },
        Mission {
            id: "7.5".into(), chapter: 7, chapter_title: "Sauvegarde".into(), act: 4,
            title: "Disques".into(),
            description: "Quels disques branches?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Quels disques sont physiquement connectés à la station?

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Voir les disques montés est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Quels disques sont physiquement connectés à la station?

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Quels disques sont physiquement connectés à la station?".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Voir les partitions?".into(),
            required_commands: vec!["mount".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "mount".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            map: "VUE DE DESSUS — salle des machines (disques branchés):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶MACH ◀ │\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "mount".into(), name: "mount".into(), description: "mount".into() }),
        },
    ]
}

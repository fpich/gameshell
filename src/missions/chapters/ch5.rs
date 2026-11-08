//! CHAPITRE 5 - Enquete
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "5.1".into(), chapter: 5, chapter_title: "Enquete".into(), act: 3,
            title: "Compter les attaques".into(),
            description: "Combien de tentatives SSH echouees?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Vex Korshun veut le nombre exact de tentatives d'intrusion SSH pour évaluer la menace.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Compter les attaques est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Vex Korshun veut le nombre exact de tentatives d'intrusion SSH pour évaluer la menace.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Vex Korshun veut savoir combien de fois les pirates ont tenté de rentrer. Comptez dans les logs.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Compter les occurrences?".into(),
            required_commands: vec!["grep Failed auth.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "grep".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: Some("6".into()),
            map: "VUE DE DESSUS — compartiment SÉCURITÉ (logs auth):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶SECUR ◀│\n  └───┬────┘ └───┬────┘ └───┬────┘\n      └────► /securite/logs/auth.log".into(),
            skill: Some(Skill { command: "grep".into(), name: "grep".into(), description: "grep".into() }),
        },
        Mission {
            id: "5.2".into(), chapter: 5, chapter_title: "Enquete".into(), act: 3,
            title: "Localiser les logs".into(),
            description: "Quels fichiers .log?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Quels fichiers de logs existent sur le vaisseau? Où chercher les preuves?

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Localiser les logs est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Quels fichiers de logs existent sur le vaisseau? Où chercher les preuves?

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Où sont stockés tous les logs de la station? Faites l'inventaire.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Chercher par extension?".into(),
            required_commands: vec!["find /securite -name *.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "find".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            map: "VUE DE DESSUS — fouille de tout le vaisseau:\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶SECUR ◀│\n  └───┬────┘ └───┬────┘ └───┬────┘\n      └────► tous les *.log".into(),
            skill: Some(Skill { command: "find".into(), name: "find".into(), description: "find".into() }),
        },
        Mission {
            id: "5.3".into(), chapter: 5, chapter_title: "Enquete".into(), act: 3,
            title: "Chainer".into(),
            description: "Combinez ps|grep et cat|grep.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Il faut combiner plusieurs outils pour analyser rapidement la situation.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Chainer les analyses est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Il faut combiner plusieurs outils pour analyser rapidement la situation.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Combinez vos outils pour voir à la fois les processus SSH et les IPs bloquées.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Chainer des commandes?".into(),
            required_commands: vec!["ps aux | grep ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "|".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            map: "VUE DE DESSUS — salle des machines + logs:\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶MACH ◀ │\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "COMMANDE | COMMANDE".into(), name: "|".into(), description: "COMMANDE | COMMANDE — chaîne deux commandes".into() }),
        },
        Mission {
            id: "5.4".into(), chapter: 5, chapter_title: "Enquete".into(), act: 3,
            title: "Rapport".into(),
            description: "Sauvegardez les traces.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Toutes les traces des pirates doivent être consignées dans un rapport officiel.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Rédiger le rapport est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Toutes les traces des pirates doivent être consignées dans un rapport officiel.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Toutes les tentatives d'intrusion doivent être consignées dans un rapport.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Sauvegarder dans un fichier?".into(),
            required_commands: vec!["grep Failed auth.log > rapport.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "~/rapport.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine (rédaction du rapport):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │▶CABINE◀│ │SOUTES  │\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: ">".into(), name: ">".into(), description: ">".into() }),
        },
        Mission {
            id: "5.5".into(), chapter: 5, chapter_title: "Enquete".into(), act: 3,
            title: "Completer".into(),
            description: "Ajoutez les IPs UFW.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Le rapport doit être complété avec les IPs bloquées par le pare-feu.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Compléter le rapport est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Le rapport doit être complété avec les IPs bloquées par le pare-feu.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Ajoutez aussi les IPs que le pare-feu a bloquées au même rapport.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Ajouter a un fichier?".into(),
            required_commands: vec!["grep UFW syslog >> rapport.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine (compléter le rapport):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │▶CABINE◀│ │SOUTES  │\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: ">>".into(), name: ">>".into(), description: ">>".into() }),
        },
    ]
}

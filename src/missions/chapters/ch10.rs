//! CHAPITRE 6 - Reseau
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "10.1".into(), chapter: 10, chapter_title: "Reseau".into(), act: 3,
            title: "Coordonnees".into(),
            description: "IP du vaisseau?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Nira Okoro a besoin de l'adresse IP du vaisseau pour configurer les communications.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Coordonnées réseau est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Nira Okoro a besoin de l'adresse IP du vaisseau pour configurer les communications.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Nira Okoro a besoin de nos coordonnées réseau pour régler les communications.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Voir l'IP?".into(),
            required_commands: vec!["ip addr".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ip".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            skill: Some(Skill { command: "ip".into(), name: "ip".into(), description: "ip".into() }),
        },
        Mission {
            id: "10.2".into(), chapter: 10, chapter_title: "Reseau".into(), act: 3,
            title: "Liaison".into(),
            description: "Appeler la Terre?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Peut-on encore contacter la Terre et l'Andromède? La liaison est-elle intacte?

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Tester la liaison est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Peut-on encore contacter la Terre et l'Andromède? La liaison est-elle intacte?

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Vérifiez qu'on peut encore contacter l'Andromède.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Tester une connexion?".into(),
            required_commands: vec!["ping -c 4 127.0.0.1".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ping".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "ping".into(), name: "ping".into(), description: "ping".into() }),
        },
        Mission {
            id: "10.3".into(), chapter: 10, chapter_title: "Reseau".into(), act: 3,
            title: "Pare-feu".into(),
            description: "UFW actif?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Le pare-feu UFW semble inactif. Sans lui, les pirates entrent librement.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

État du pare-feu est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Le pare-feu UFW semble inactif. Sans lui, les pirates entrent librement.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Le pare-feu est-il vraiment actif? Vex Korshun en doute. Vérifiez.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Etat du pare-feu?".into(),
            required_commands: vec!["sudo ufw status".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ufw".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: "ufw".into(), name: "ufw".into(), description: "ufw".into() }),
        },
        Mission {
            id: "10.4".into(), chapter: 10, chapter_title: "Reseau".into(), act: 3,
            title: "SSH".into(),
            description: "Testez le canal SSH.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Il faut tester le canal SSH pour vérifier qu'il fonctionne avant de le durcir.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Connexion SSH est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Il faut tester le canal SSH pour vérifier qu'il fonctionne avant de le durcir.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Testez le canal sécurisé avec la station elle-même.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Se connecter en SSH?".into(),
            required_commands: vec!["ssh $USER@localhost".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ssh".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: "ssh".into(), name: "ssh".into(), description: "ssh".into() }),
        },
        Mission {
            id: "10.5".into(), chapter: 10, chapter_title: "Reseau".into(), act: 3,
            title: "Ports ouverts".into(),
            description: "Quels ports sont exposés?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Quels ports sont exposés? Les pirates scannent nos défenses réseau.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Scanner les ports est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Quels ports sont exposés? Les pirates scannent nos défenses réseau.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Quels ports de la station sont exposés? Les pirates scannent nos défenses.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Voir les ports ouverts?".into(),
            required_commands: vec!["ss -tlnp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ss".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: "ss".into(), name: "ss".into(), description: "ss".into() }),
        },
    ]
}

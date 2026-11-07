//! CHAPITRE 4 - Surveillance
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "4.1".into(), chapter: 4, chapter_title: "Surveillance".into(), act: 2,
            title: "Processus actifs".into(),
            description: "Quels processus tournent?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Pierre entend des bruits bizarres dans les coursives. Des processus inconnus tournent peut-être.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Voir les processus actifs est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Pierre entend des bruits bizarres dans les coursives. Des processus inconnus tournent peut-être.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "ps aux.".into(),
            bloom_level: "Understand".into(),
            bloom_question: "Voir tous les processus?".into(),
            required_commands: vec!["ps".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ps".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ps".into(), name: "ps".into(), description: "ps".into() }),
        },
        Mission {
            id: "4.2".into(), chapter: 4, chapter_title: "Surveillance".into(), act: 2,
            title: "Ressources".into(),
            description: "Surveillez CPU/RAM.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. La station ralentit. Les boucliers consomment trop de ressources.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Surveiller les ressources est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: La station ralentit. Les boucliers consomment trop de ressources.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "top -bn1.".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Surveiller les ressources?".into(),
            required_commands: vec!["top".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "top".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "top".into(), name: "top".into(), description: "top".into() }),
        },
        Mission {
            id: "4.3".into(), chapter: 4, chapter_title: "Surveillance".into(), act: 2,
            title: "Services".into(),
            description: "Verifiez ssh et les services.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Jean veut vérifier que tous les services de sécurité sont bien actifs.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Vérifier les services est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Jean veut vérifier que tous les services de sécurité sont bien actifs.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "systemctl status ssh + list-units.".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Gerer les services?".into(),
            required_commands: vec!["systemctl".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "systemctl".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            skill: Some(Skill { command: "systemctl".into(), name: "systemctl".into(), description: "systemctl".into() }),
        },
        Mission {
            id: "4.4".into(), chapter: 4, chapter_title: "Surveillance".into(), act: 2,
            title: "Tuer un pirate".into(),
            description: "PID 9999 backdoor!".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. ALERTE ROUGE! Un processus backdoor (PID 9999) a été détecté. Il faut l'éliminer immédiatement.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Tuer le processus pirate est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: ALERTE ROUGE! Un processus backdoor (PID 9999) a été détecté. Il faut l'éliminer immédiatement.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "kill 9999.".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Tuer un processus?".into(),
            required_commands: vec!["kill".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "kill".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            skill: Some(Skill { command: "kill".into(), name: "kill".into(), description: "kill".into() }),
        },
        Mission {
            id: "4.5".into(), chapter: 4, chapter_title: "Surveillance".into(), act: 2,
            title: "Priorites".into(),
            description: "Scan prioritaire.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Le scan de sécurité doit être prioritaire face à l'attaque imminente.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Gérer les priorités est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Le scan de sécurité doit être prioritaire face à l'attaque imminente.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "nice -n 10 sleep 100 & + ps -l.".into(),
            bloom_level: "Understand".into(),
            bloom_question: "Changer la priorite?".into(),
            required_commands: vec!["nice".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "nice".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "nice".into(), name: "nice".into(), description: "nice".into() }),
        },
    ]
}

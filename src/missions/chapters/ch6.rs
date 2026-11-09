//! CHAPITRE 4 - Surveillance
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "6.1".into(), chapter: 6, chapter_title: "Surveillance".into(), act: 2,
            title: "Processus actifs".into(),
            description: "Quels processus tournent?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Tael Rhen entend des bruits bizarres dans les coursives. Des processus inconnus tournent peut-être.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Voir les processus actifs est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Tael Rhen entend des bruits bizarres dans les coursives. Des processus inconnus tournent peut-être.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

La liste des systèmes en cours d'exécution s'affiche. Tael Rhen l'examine: « Tout semble normal, Capitaine. Merci. »

Vous comprenez que surveiller les processus actifs, c'est garder un œil sur l'activité du vaisseau.".into(),
            goal: "Tael Rhen entend des bruits bizarres. Vérifiez quels systèmes tournent actuellement.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Voir tous les processus?".into(),
            required_commands: vec!["ps aux".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ps".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ps".into(), name: "ps".into(), description: "ps".into() }),
        },
        Mission {
            id: "6.2".into(), chapter: 6, chapter_title: "Surveillance".into(), act: 2,
            title: "Ressources".into(),
            description: "Surveillez CPU/RAM.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. La station ralentit. Les boucliers consomment trop de ressources.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Surveiller les ressources est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: La station ralentit. Les boucliers consomment trop de ressources.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Les jauges de ressources s'affichent: CPU, mémoire, charge. Vous repérez un processus qui consomme trop.

« Voilà le coupable », note Tael Rhen. « Surveiller les ressources, c'est diagnostiquer les ralentissements. »".into(),
            goal: "La station ralentit. Surveillez les ressources pour comprendre pourquoi.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Surveiller les ressources?".into(),
            required_commands: vec!["top -bn1".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "top".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "top".into(), name: "top".into(), description: "top".into() }),
        },
        Mission {
            id: "6.3".into(), chapter: 6, chapter_title: "Surveillance".into(), act: 2,
            title: "Services".into(),
            description: "Verifiez ssh et les services.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Vex Korshun veut vérifier que tous les services de sécurité sont bien actifs.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Vérifier les services est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Vex Korshun veut vérifier que tous les services de sécurité sont bien actifs.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

La liste des services de sécurité s'affiche. Vex Korshun vérifie que chacun est bien actif.

« Bien », approuve-t-il. « Savoir quels services tournent, c'est s'assurer que nos défenses sont en place. »".into(),
            goal: "Vex Korshun veut savoir si tous les services de sécurité sont bien en marche.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Gerer les services?".into(),
            required_commands: vec!["systemctl status ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "systemctl".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            skill: Some(Skill { command: "systemctl".into(), name: "systemctl".into(), description: "systemctl".into() }),
        },
        Mission {
            id: "6.4".into(), chapter: 6, chapter_title: "Surveillance".into(), act: 2,
            title: "Tuer un pirate".into(),
            description: "PID 9999 backdoor!".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. ALERTE ROUGE! Un processus backdoor (PID 9999) a été détecté. Il faut l'éliminer immédiatement.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Tuer le processus pirate est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: ALERTE ROUGE! Un processus backdoor (PID 9999) a été détecté. Il faut l'éliminer immédiatement.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Le processus pirate est éliminé. Draz M'kal pousse un soupir de soulagement.

« Bien joué, Capitaine. Un processus malveillant neutralisé, c'est une menace de moins à bord. »".into(),
            goal: "Un système pirate a pris le contrôle! Éliminez-le immédiatement!".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Tuer un processus?".into(),
            required_commands: vec!["kill 9999".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "kill".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            skill: Some(Skill { command: "kill".into(), name: "kill".into(), description: "kill".into() }),
        },
        Mission {
            id: "6.5".into(), chapter: 6, chapter_title: "Surveillance".into(), act: 2,
            title: "Priorites".into(),
            description: "Scan prioritaire.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Le scan de sécurité doit être prioritaire face à l'attaque imminente.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Gérer les priorités est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Le scan de sécurité doit être prioritaire face à l'attaque imminente.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Le scan de sécurité se lance en priorité. Les autres tâches attendent leur tour.

« Priorité haute, c'est de la gestion d'urgence », explique Tael Rhen. « Les tâches critiques passent d'abord. »".into(),
            goal: "Le scan de sécurité est prioritaire en ce moment. Lancez-le en priorité.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Changer la priorite?".into(),
            required_commands: vec!["nice -n 10 commande".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "nice".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "nice".into(), name: "nice".into(), description: "nice".into() }),
        },
    ]
}

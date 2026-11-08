//! CHAPITRE 3 - Securiser les acces
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "5.1".into(), chapter: 5, chapter_title: "Securiser les acces".into(), act: 2,
            title: "Comprendre les permissions".into(),
            description: "Vex Korshun explique: ls -l montre rwx.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Vex Korshun vous explique que chaque fichier du vaisseau a des permissions. Sans comprendre ça, impossible de protéger quoi que ce soit.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Comprendre les permissions est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Vex Korshun vous explique que chaque fichier du vaisseau a des permissions. Sans comprendre ça, impossible de protéger quoi que ce soit.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Vex Korshun vous explique que chaque fichier a des permissions. Regardez celles de votre cabine.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Que signifient rwx?".into(),
            required_commands: vec!["ls -l".into(), "ls -la".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ls -l".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "ls -la".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ls -l".into(), name: "ls -l".into(), description: "ls -l — liste détaillée: permissions, taille, date".into() }),
        },
        Mission {
            id: "5.2".into(), chapter: 5, chapter_title: "Securiser les acces".into(), act: 2,
            title: "Proteger le code secret".into(),
            description: "Votre .secret contient FALCON-9. Protegez-le.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Votre code personnel FALCON-9 est dans un fichier lisible par tous. Un pirate pourrait le voler.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Protéger le code secret est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Votre code personnel FALCON-9 est dans un fichier lisible par tous. Un pirate pourrait le voler.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Votre code personnel ne doit être lisible que par vous. Protégez ce fichier.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Restreindre au proprio seul?".into(),
            required_commands: vec!["chmod 600 fichier".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chmod".into(), expected: None }, MissionCheck { check_type: "permission".into(), target: "~/notes.txt".into(), expected: Some("600".into()) }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "chmod".into(), name: "chmod".into(), description: "chmod — change les permissions rwx d'un fichier (600, +x)".into() }),
        },
        Mission {
            id: "5.3".into(), chapter: 5, chapter_title: "Securiser les acces".into(), act: 2,
            title: "Rendre executable".into(),
            description: "Le plan de defense doit s'executer.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Le plan de défense plan_defense.sh ne peut pas s'exécuter. Il lui manque le droit d'exécution.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Rendre le plan exécutable est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Le plan de défense plan_defense.sh ne peut pas s'exécuter. Il lui manque le droit d'exécution.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Le plan de défense ne peut pas se lancer. Il lui manque quelque chose. Corrigez.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter +x?".into(),
            required_commands: vec!["chmod 600 fichier".into()],
            checks: vec![MissionCheck { check_type: "permission".into(), target: "~/plan_defense.sh".into(), expected: Some("+x".into()) }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chmod".into(), name: "chmod +x".into(), description: "chmod +x — rend un fichier exécutable (script de défense)".into() }),
        },
        Mission {
            id: "5.4".into(), chapter: 5, chapter_title: "Securiser les acces".into(), act: 2,
            title: "Changer le proprio".into(),
            description: "Transferez le fichier equipage.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Un fichier appartient à root. Vous devez en prendre la responsabilité pour le modifier.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Changer le propriétaire est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Un fichier appartient à root. Vous devez en prendre la responsabilité pour le modifier.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Un fichier appartient encore à l'ancien admin. Transférez-le à votre nom.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment changer le proprio?".into(),
            required_commands: vec!["sudo chown $USER fichier".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chown".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "chown".into(), name: "chown".into(), description: "chown — change le propriétaire d'un fichier (user:group)".into() }),
        },
        Mission {
            id: "5.5".into(), chapter: 5, chapter_title: "Securiser les acces".into(), act: 2,
            title: "Metadonnees".into(),
            description: "Comparez les dates de modification.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Pour détecter une intrusion, comparez les dates de modification des fichiers sensibles.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Analyser les métadonnées est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Pour détecter une intrusion, comparez les dates de modification des fichiers sensibles.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Comparez les dates de modification des fichiers de sécurité pour détecter une intrusion.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "stat vs ls -l?".into(),
            required_commands: vec!["stat fichier".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "stat".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "stat".into(), name: "stat".into(), description: "stat — affiche les métadonnées: taille, permissions, dates, inode".into() }),
        },
    ]
}

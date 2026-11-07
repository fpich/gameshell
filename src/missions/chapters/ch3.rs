//! CHAPITRE 3 - Securiser les acces
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "3.1".into(), chapter: 3, chapter_title: "Securiser les acces".into(), act: 2,
            title: "Comprendre les permissions".into(),
            description: "Jean explique: ls -l montre rwx.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Jean vous explique que chaque fichier du vaisseau a des permissions. Sans comprendre ça, impossible de protéger quoi que ce soit.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Comprendre les permissions est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Jean vous explique que chaque fichier du vaisseau a des permissions. Sans comprendre ça, impossible de protéger quoi que ce soit.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Affichez les permissions de votre cabine (ls -l, ls -la).".into(),
            bloom_level: "Understand".into(),
            bloom_question: "Que signifient rwx?".into(),
            required_commands: vec!["ls -l".into(), "ls -la".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ls -l".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "ls -la".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine (permissions des portes):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │▶CABINE◀│ │SOUTES  │\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "chmod".into(), name: "chmod".into(), description: "chmod".into() }),
        },
        Mission {
            id: "3.2".into(), chapter: 3, chapter_title: "Securiser les acces".into(), act: 2,
            title: "Proteger le code secret".into(),
            description: "Votre .secret contient FALCON-9. Protegez-le.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Votre code personnel FALCON-9 est dans un fichier lisible par tous. Un pirate pourrait le voler.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Protéger le code secret est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Votre code personnel FALCON-9 est dans un fichier lisible par tous. Un pirate pourrait le voler.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Mettez ~/notes.txt en 600 (chmod 600).".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Restreindre au proprio seul?".into(),
            required_commands: vec!["chmod 600 fichier".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chmod".into(), expected: None }, MissionCheck { check_type: "permission".into(), target: "~/notes.txt".into(), expected: Some("600".into()) }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine (coffre personnel):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │▶CABINE◀│ │SOUTES  │\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "chown".into(), name: "chown".into(), description: "chown".into() }),
        },
        Mission {
            id: "3.3".into(), chapter: 3, chapter_title: "Securiser les acces".into(), act: 2,
            title: "Rendre executable".into(),
            description: "Le plan de defense doit s'executer.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Le plan de défense plan_defense.sh ne peut pas s'exécuter. Il lui manque le droit d'exécution.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Rendre le plan exécutable est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Le plan de défense plan_defense.sh ne peut pas s'exécuter. Il lui manque le droit d'exécution.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Rendez ~/plan_defense.sh executable (chmod +x).".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Comment ajouter +x?".into(),
            required_commands: vec!["chmod 600 fichier".into()],
            checks: vec![MissionCheck { check_type: "permission".into(), target: "~/plan_defense.sh".into(), expected: Some("+x".into()) }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine (plan de défense):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │▶CABINE◀│ │SOUTES  │\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "ls -l".into(), name: "ls -l".into(), description: "ls -l".into() }),
        },
        Mission {
            id: "3.4".into(), chapter: 3, chapter_title: "Securiser les acces".into(), act: 2,
            title: "Changer le proprio".into(),
            description: "Transferez le fichier equipage.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Un fichier appartient à root. Vous devez en prendre la responsabilité pour le modifier.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Changer le propriétaire est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Un fichier appartient à root. Vous devez en prendre la responsabilité pour le modifier.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "sudo chown capitaine sur un fichier.".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Comment changer le proprio?".into(),
            required_commands: vec!["chown capitaine fichier".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chown".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            map: "VUE DE DESSUS — compartiment SÉCURITÉ:\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶SECUR ◀│\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "stat".into(), name: "stat".into(), description: "stat".into() }),
        },
        Mission {
            id: "3.5".into(), chapter: 3, chapter_title: "Securiser les acces".into(), act: 2,
            title: "Metadonnees".into(),
            description: "Comparez les dates de modification.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Pour détecter une intrusion, comparez les dates de modification des fichiers sensibles.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Analyser les métadonnées est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Pour détecter une intrusion, comparez les dates de modification des fichiers sensibles.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "stat sur /securite/logs/auth.log.".into(),
            bloom_level: "Analyze".into(),
            bloom_question: "stat vs ls -l?".into(),
            required_commands: vec!["stat fichier".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "stat".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            map: "VUE DE DESSUS — compartiment SÉCURITÉ (logs):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶SECUR ◀│\n  └───┬────┘ └───┬────┘ └───┬────┘\n      └────► /securite/logs/".into(),
            skill: Some(Skill { command: "getfacl".into(), name: "getfacl".into(), description: "getfacl".into() }),
        },
    ]
}

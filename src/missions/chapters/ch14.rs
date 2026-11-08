//! CHAPITRE 8 - Equipage
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "14.1".into(), chapter: 14, chapter_title: "Equipage".into(), act: 4,
            title: "Recruter".into(),
            description: "Creez le compte matelot.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Un nouvel arrivant rejoint l'équipage. Il lui faut un compte sur le système.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Recruter un matelot est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Un nouvel arrivant rejoint l'équipage. Il lui faut un compte sur le système.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Un nouvel arrivant monte à bord. Créez-lui son badge d'accès.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Creer un user?".into(),
            required_commands: vec!["sudo useradd -m matelot".into(), "id".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "useradd".into(), expected: None }, MissionCheck { check_type: "user_exists".into(), target: "matelot".into(), expected: None }],
            hints: vec![],
            xp_reward: 225,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine (recrutement matelot):\n  ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓\n  ┃    PONT    ┃ ┃  ▶CABINE◀  ┃ ┃   SOUTES   ┃\n  ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛\n       ┻━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┻┛".into(),
            skill: Some(Skill { command: "useradd".into(), name: "useradd".into(), description: "useradd".into() }),
        },
        Mission {
            id: "14.2".into(), chapter: 14, chapter_title: "Equipage".into(), act: 4,
            title: "Equipe".into(),
            description: "Groupe equipage + matelot.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Pour gérer les accès collectifs, il faut créer un groupe d'équipage.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Créer l'équipe equipage est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Pour gérer les accès collectifs, il faut créer un groupe d'équipage.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Créez une équipe 'equipage' et mettez le nouveau dedans.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Creer un groupe?".into(),
            required_commands: vec!["sudo groupadd equipage".into(), "usermod".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "groupadd".into(), expected: None }, MissionCheck { check_type: "group_exists".into(), target: "equipage".into(), expected: None }],
            hints: vec![],
            xp_reward: 225,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine (création équipe):\n  ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓\n  ┃    PONT    ┃ ┃  ▶CABINE◀  ┃ ┃   SOUTES   ┃\n  ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛\n       ┻━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┻┛".into(),
            skill: Some(Skill { command: "groupadd".into(), name: "groupadd".into(), description: "groupadd".into() }),
        },
        Mission {
            id: "14.3".into(), chapter: 14, chapter_title: "Equipage".into(), act: 4,
            title: "Mot de passe".into(),
            description: "Securisez matelot.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Le compte matelot n'a pas de mot de passe. C'est une faille de sécurité.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Changer le mot de passe est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Le compte matelot n'a pas de mot de passe. C'est une faille de sécurité.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Le badge du nouveau n'a pas de code. Sécurisez-le.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Changer un MDP?".into(),
            required_commands: vec!["sudo passwd matelot".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "passwd".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine (sécuriser le compte):\n  ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓\n  ┃    PONT    ┃ ┃  ▶CABINE◀  ┃ ┃   SOUTES   ┃\n  ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛\n       ┻━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┻┛".into(),
            skill: Some(Skill { command: "passwd".into(), name: "passwd".into(), description: "passwd".into() }),
        },
        Mission {
            id: "14.4".into(), chapter: 14, chapter_title: "Equipage".into(), act: 4,
            title: "Changer d'user".into(),
            description: "Testez le compte matelot.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Il faut tester que le compte matelot fonctionne correctement.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Tester le compte matelot est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Il faut tester que le compte matelot fonctionne correctement.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Testez que le nouveau badge fonctionne bien.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Changer d'user?".into(),
            required_commands: vec!["su - matelot -c whoami".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "su".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine (test compte matelot):\n  ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓\n  ┃    PONT    ┃ ┃  ▶CABINE◀  ┃ ┃   SOUTES   ┃\n  ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛\n       ┻━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┻┛".into(),
            skill: Some(Skill { command: "su".into(), name: "su".into(), description: "su".into() }),
        },
        Mission {
            id: "14.5".into(), chapter: 14, chapter_title: "Equipage".into(), act: 4,
            title: "Sudo".into(),
            description: "Prouvez vos droits admin.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. En tant que capitaine, vous devez connaître vos pouvoirs sudo.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Utiliser sudo est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: En tant que capitaine, vous devez connaître vos pouvoirs sudo.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Montrez que vous avez bien les pouvoirs du capitaine.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Executer en root?".into(),
            required_commands: vec!["sudo whoami".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "sudo".into(), expected: None }],
            hints: vec![],
            xp_reward: 225,
            code_required: None,
            map: "VUE DE DESSUS — pont de commandement (pouvoirs capitaine):\n  ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓\n  ┃   ▶PONT◀   ┃ ┃  CABINES   ┃ ┃   SOUTES   ┃\n  ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛\n       ┻━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┻┛".into(),
            skill: Some(Skill { command: "sudo".into(), name: "sudo".into(), description: "sudo".into() }),
        },
    ]
}

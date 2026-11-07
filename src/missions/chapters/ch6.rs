//! CHAPITRE 6 - Reseau
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "6.1".into(), chapter: 6, chapter_title: "Reseau".into(), act: 3,
            title: "Coordonnees".into(),
            description: "IP du vaisseau?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Sophie a besoin de l'adresse IP du vaisseau pour configurer les communications.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Coordonnées réseau est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Sophie a besoin de l'adresse IP du vaisseau pour configurer les communications.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "ip addr + ip route.".into(),
            bloom_level: "Understand".into(),
            bloom_question: "Voir l'IP?".into(),
            required_commands: vec!["ip".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ip".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            map: "VUE DE DESSUS — salle de navigation (coordonnées):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶ NAV ◀ │\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "ip".into(), name: "ip".into(), description: "ip".into() }),
        },
        Mission {
            id: "6.2".into(), chapter: 6, chapter_title: "Reseau".into(), act: 3,
            title: "Liaison".into(),
            description: "Appeler la Terre?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Peut-on encore contacter la Terre et le Galactica? La liaison est-elle intacte?

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Tester la liaison est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Peut-on encore contacter la Terre et le Galactica? La liaison est-elle intacte?

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "ping -c 4 127.0.0.1.".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Tester une connexion?".into(),
            required_commands: vec!["ping".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ping".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            map: "VUE DE DESSUS — antenne de communication:\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶ COMMS◀│\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "ping".into(), name: "ping".into(), description: "ping".into() }),
        },
        Mission {
            id: "6.3".into(), chapter: 6, chapter_title: "Reseau".into(), act: 3,
            title: "Pare-feu".into(),
            description: "UFW actif?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Le pare-feu UFW semble inactif. Sans lui, les pirates entrent librement.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

État du pare-feu est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Le pare-feu UFW semble inactif. Sans lui, les pirates entrent librement.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "sudo ufw status.".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Etat du pare-feu?".into(),
            required_commands: vec!["ufw".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ufw".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            map: "VUE DE DESSUS — compartiment SÉCURITÉ (pare-feu):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶SECUR ◀│\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "ufw".into(), name: "ufw".into(), description: "ufw".into() }),
        },
        Mission {
            id: "6.4".into(), chapter: 6, chapter_title: "Reseau".into(), act: 3,
            title: "SSH".into(),
            description: "Testez le canal SSH.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Il faut tester le canal SSH pour vérifier qu'il fonctionne avant de le durcir.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Connexion SSH est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Il faut tester le canal SSH pour vérifier qu'il fonctionne avant de le durcir.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "ssh capitaine@localhost hostname.".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Se connecter en SSH?".into(),
            required_commands: vec!["ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ssh".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            map: "VUE DE DESSUS — compartiment SÉCURITÉ (canal SSH):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶SECUR ◀│\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "ssh".into(), name: "ssh".into(), description: "ssh".into() }),
        },
        Mission {
            id: "6.5".into(), chapter: 6, chapter_title: "Reseau".into(), act: 3,
            title: "Ports ouverts".into(),
            description: "Quels ports sont exposés?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Quels ports sont exposés? Les pirates scannent nos défenses réseau.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Scanner les ports est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Quels ports sont exposés? Les pirates scannent nos défenses réseau.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "ss -tlnp.".into(),
            bloom_level: "Analyze".into(),
            bloom_question: "Voir les ports ouverts?".into(),
            required_commands: vec!["ss".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ss".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            map: "VUE DE DESSUS — salle de navigation (radar ports):\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶ NAV ◀ │\n  └───┬────┘ └───┬────┘ └───┬────┘".into(),
            skill: Some(Skill { command: "ss".into(), name: "ss".into(), description: "ss".into() }),
        },
    ]
}

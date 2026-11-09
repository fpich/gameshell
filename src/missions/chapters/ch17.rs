//! CHAPITRE 9 - Maintenance
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "17.1".into(), chapter: 17, chapter_title: "Maintenance".into(), act: 5,
            title: "Mise a jour".into(),
            description: "Mettez a jour les paquets.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Les paquets de sécurité doivent être à jour contre les dernières vulnérabilités.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Mettre à jour les paquets est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Les paquets de sécurité doivent être à jour contre les dernières vulnérabilités.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

La liste des pièces détachées est mise à jour. Tael Rhen la consulte.

« Parfait, Capitaine. Je saurai maintenant ce qui est disponible pour les réparations. »".into(),
            goal: "Mettez à jour la liste des pièces détachées disponibles.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Rafraichir les paquets?".into(),
            required_commands: vec!["sudo apt update".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "apt update".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: "apt".into(), name: "apt".into(), description: "apt — gestionnaire de paquets (install, remove, update)".into() }),
        },
        Mission {
            id: "17.2".into(), chapter: 17, chapter_title: "Maintenance".into(), act: 5,
            title: "Installer".into(),
            description: "Installez cowsay.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Un outil de test est nécessaire pour vérifier le bon fonctionnement des annonces.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Installer cowsay est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Un outil de test est nécessaire pour vérifier le bon fonctionnement des annonces.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

L'outil de test des annonces est installé. Tael Rhen le vérifie.

« Bien », approuve-t-il. « Un outil installé, c'est une capacité de plus à bord. »".into(),
            goal: "Installez un outil pour tester les annonces de la station.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Installer un logiciel?".into(),
            required_commands: vec!["sudo apt install -y cowsay".into(), "dpkg -s cowsay".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "apt install".into(), expected: None }],
            hints: vec![],
            xp_reward: 225,
            code_required: None,
            skill: Some(Skill { command: "dpkg".into(), name: "dpkg".into(), description: "dpkg — gestionnaire de paquets Debian de bas niveau".into() }),
        },
        Mission {
            id: "17.3".into(), chapter: 17, chapter_title: "Maintenance".into(), act: 5,
            title: "Redemarrer SSH".into(),
            description: "Apres modif config.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Après modification de la config SSH, le service doit être redémarré.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Redémarrer SSH est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Après modification de la config SSH, le service doit être redémarré.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Le service SSH est rechargé avec la nouvelle configuration.

« Redémarrer, c'est appliquer les changements », explique Draz M'kal. « Le service repart avec les réglages à jour. »".into(),
            goal: "La config SSH a changé, il faut recharger le service.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Redemarrer un service?".into(),
            required_commands: vec!["sudo systemctl restart ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "systemctl restart".into(), expected: None }],
            hints: vec![],
            xp_reward: 225,
            code_required: None,
            skill: Some(Skill { command: "systemctl restart".into(), name: "systemctl restart".into(), description: "systemctl restart — redémarre un service système".into() }),
        },
        Mission {
            id: "17.4".into(), chapter: 17, chapter_title: "Maintenance".into(), act: 5,
            title: "Journal".into(),
            description: "Historique recent.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. L'historique complet de la station est dans le journal systemd. Consultez-le.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Consulter le journal est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: L'historique complet de la station est dans le journal systemd. Consultez-le.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Le journal récent de la station s'affiche. Vous y lisez les derniers événements.

« Le journal raconte l'activité du vaisseau », note Draz M'kal. « C'est là qu'on repère les anomalies. »".into(),
            goal: "Qu'est-ce qui s'est passé récemment sur la station? Consultez le journal.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Voir les logs systemd?".into(),
            required_commands: vec!["sudo journalctl -n 20".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "journalctl".into(), expected: None }],
            hints: vec![],
            xp_reward: 250,
            code_required: None,
            skill: Some(Skill { command: "journalctl".into(), name: "journalctl".into(), description: "journalctl — consulte les journaux système (logs)".into() }),
        },
        Mission {
            id: "17.5".into(), chapter: 17, chapter_title: "Maintenance".into(), act: 5,
            title: "Nettoyer".into(),
            description: "Liberez de l'espace.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Les soutes sont pleines de paquets inutiles. Nettoyez pour libérer de l'espace.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Nettoyer le système est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Les soutes sont pleines de paquets inutiles. Nettoyez pour libérer de l'espace.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Les vieilleries sont nettoyées du système. L'espace est libéré.

« Bien joué », approuve Tael Rhen. « Un système propre est un système sain. Nettoyer, c'est entretenir. »".into(),
            goal: "Les soutes sont pleines de vieilleries. Nettoyez le système.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Nettoyer le systeme?".into(),
            required_commands: vec!["sudo apt autoremove -y".into(), "apt clean".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "apt autoremove".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "apt clean".into(), expected: None }],
            hints: vec![],
            xp_reward: 225,
            code_required: None,
            skill: Some(Skill { command: "apt update".into(), name: "apt update".into(), description: "apt update — met à jour la liste des paquets disponibles".into() }),
        },
    ]
}

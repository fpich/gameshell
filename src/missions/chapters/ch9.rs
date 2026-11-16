//! CHAPITRE 5 - Enquete
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "9.1".into(), chapter: 9, chapter_title: "Enquete".into(), act: 3,
            title: "Compter les attaques".into(),
            description: "Combien de tentatives SSH echouees?".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Vex Korshun veut le nombre exact de tentatives d'intrusion SSH. Chaque échec est noté « Failed » dans /securite/logs/auth.log: affichez ces lignes et comptez-les. Ce nombre sera votre code de validation.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Compter les attaques est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Vex Korshun veut le nombre exact de tentatives d'intrusion SSH. Chaque échec est noté « Failed » dans /securite/logs/auth.log: affichez ces lignes et comptez-les. Ce nombre sera votre code de validation.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Le compteur affiche le nombre de tentatives d'intrusion. Le chiffre est alarmant.

« Autant d'attaques en si peu de temps », murmure Draz M'kal. « Nous devons renforcer nos défenses. »".into(),
            goal: "Vex Korshun veut savoir combien de fois les pirates ont tenté d'entrer: retrouvez les lignes « Failed » de /securite/logs/auth.log et comptez-les.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Compter les occurrences?".into(),
            required_commands: vec!["grep Failed /securite/logs/auth.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "grep".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: Some("6".into()),
            skill: Some(Skill { command: "grep".into(), name: "grep".into(), description: "grep — recherche un motif dans un fichier ou une sortie".into() }),
        },
        Mission {
            id: "9.2".into(), chapter: 9, chapter_title: "Enquete".into(), act: 3,
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
la station et son équipage de 5 personnes.

L'inventaire des logs s'affiche. Vous savez désormais où sont stockées toutes les traces du vaisseau.

« Les logs sont la mémoire de la station », explique Draz M'kal. « Savoir où ils sont, c'est pouvoir les consulter. »".into(),
            goal: "Où sont stockés tous les logs de la station? Faites l'inventaire.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Chercher par extension?".into(),
            required_commands: vec!["find /securite -name *.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "find".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            skill: Some(Skill { command: "find".into(), name: "find".into(), description: "find — recherche des fichiers selon des critères (nom, type, taille)".into() }),
        },
        Mission {
            id: "9.3".into(), chapter: 9, chapter_title: "Enquete".into(), act: 3,
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
la station et son équipage de 5 personnes.

Vous combinez vos outils: les processus SSH et les IPs bloquées s'affichent côte à côte.

« Bien vu », approuve Draz M'kal. « Combiner les sources, c'est croiser les indices pour trouver la vérité. »".into(),
            goal: "Combinez vos outils pour voir à la fois les processus SSH et les IPs bloquées.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Chainer des commandes?".into(),
            required_commands: vec!["ps aux | grep ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "|".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: "COMMANDE | COMMANDE".into(), name: "|".into(), description: "COMMANDE | COMMANDE — chaîne deux commandes".into() }),
        },
        Mission {
            id: "9.4".into(), chapter: 9, chapter_title: "Enquete".into(), act: 3,
            title: "Rapport".into(),
            description: "Sauvegardez les traces.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Toutes les lignes « Failed » de /securite/logs/auth.log doivent être consignées dans un rapport officiel: le fichier rapport.txt de votre cabine.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Rédiger le rapport est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Toutes les lignes « Failed » de /securite/logs/auth.log doivent être consignées dans un rapport officiel: le fichier rapport.txt de votre cabine.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Le rapport des tentatives d'intrusion est consigné. Chaque attaque y est répertoriée.

« Un rapport clair, c'est une preuve exploitable », note Draz M'kal. « La documentation est notre arme. »".into(),
            goal: "Consignez toutes les lignes « Failed » de /securite/logs/auth.log dans le fichier rapport.txt de votre cabine.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Sauvegarder dans un fichier?".into(),
            required_commands: vec!["grep Failed /securite/logs/auth.log > rapport.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "~/rapport.txt".into(), expected: None }, MissionCheck { check_type: "config_contains".into(), target: "~/rapport.txt".into(), expected: Some("Failed".into()) }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: ">".into(), name: ">".into(), description: "> — redirige la sortie d'une commande vers un fichier (écrase)".into() }),
        },
        Mission {
            id: "9.5".into(), chapter: 9, chapter_title: "Enquete".into(), act: 3,
            title: "Completer".into(),
            description: "Ajoutez les IPs UFW.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Le rapport rapport.txt doit être complété, SANS effacer son contenu, avec les lignes « UFW » du journal /securite/logs/syslog: ce sont les IPs bloquées par le pare-feu.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Compléter le rapport est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Le rapport rapport.txt doit être complété, SANS effacer son contenu, avec les lignes « UFW » du journal /securite/logs/syslog: ce sont les IPs bloquées par le pare-feu.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Les IPs bloquées par le pare-feu s'ajoutent au rapport. Le dossier est complet.

« Excellent », conclut Draz M'kal. « Toutes les traces sont réunies. Nous avons maintenant une vision complète de l'attaque. »".into(),
            goal: "Ajoutez à la fin de rapport.txt, sans l'écraser, les lignes « UFW » de /securite/logs/syslog.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Ajouter a un fichier?".into(),
            required_commands: vec!["grep UFW /securite/logs/syslog >> rapport.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }, MissionCheck { check_type: "config_contains".into(), target: "~/rapport.txt".into(), expected: Some("Failed".into()) }, MissionCheck { check_type: "config_contains".into(), target: "~/rapport.txt".into(), expected: Some("UFW".into()) }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: ">>".into(), name: ">>".into(), description: ">> — redirige la sortie d'une commande vers un fichier (ajoute)".into() }),
        },
    ]
}

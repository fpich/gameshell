//! CHAPITRE 15 - Archives et sauvegardes (Acte 4) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "15.1".into(), chapter: 15, chapter_title: "Archives et sauvegardes".into(), act: 4,
            title: "Archiver les rapports".into(),
            description: "Tael Rhen veut une archive compressée des rapports dans les soutes.".into(),
            story: "Tael Rhen: « Capitaine, avant l'attaque, archivons tous les rapports
dans une caisse hermétique compressée, dans les soutes sécurisées. »

Il vous demande de créer une archive .tar.gz de tout le dossier rapports/
de votre cabine, et de la déposer dans /soutes/.

« Créez /soutes/backup_rapports.tar.gz contenant rapports/, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

L'archive compressée est créée dans les soutes. Les rapports sont désormais en sécurité.

« Une sauvegarde, c'est une assurance », explique Tael Rhen. « Si les originaux sont perdus, la copie nous sauve. »".into(),
            goal: "Créez une archive compressée /soutes/backup_rapports.tar.gz contenant le dossier rapports/.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer une archive compressée d'un dossier?".into(),
            required_commands: vec!["tar czf /soutes/backup_rapports.tar.gz rapports/".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tar".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "/soutes/backup_rapports.tar.gz".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "tar czf".into(), name: "tar".into(), description: "tar czf — crée une archive compressée .tar.gz".into() }),
        },
        Mission {
            id: "15.2".into(), chapter: 15, chapter_title: "Archives et sauvegardes".into(), act: 4,
            title: "Vérifier le contenu de l'archive".into(),
            description: "Qu'y a-t-il dans backup_rapports.tar.gz?".into(),
            story: "Tael Rhen: « Capitaine, avant d'envoyer l'archive à l'Andromède,
vérifiez ce qu'elle contient. Listez son contenu sans l'extraire. »

« Quels fichiers contient /soutes/backup_rapports.tar.gz? »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

Le contenu de l'archive s'affiche sans l'extraire. Vous vérifiez que tous les rapports sont bien là.

« Vérifier avant d'envoyer, c'est éviter les mauvaises surprises », note Tael Rhen. « Un coup d'œil suffit. »".into(),
            goal: "Listez le contenu de l'archive /soutes/backup_rapports.tar.gz sans l'extraire.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lister le contenu d'une archive sans l'extraire?".into(),
            required_commands: vec!["tar tvf /soutes/backup_rapports.tar.gz".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tvf".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "tar tvf".into(), name: "tar tvf".into(), description: "tar tvf — liste le contenu d'une archive sans l'extraire".into() }),
        },
        Mission {
            id: "15.3".into(), chapter: 15, chapter_title: "Archives et sauvegardes".into(), act: 4,
            title: "Extraire l'archive".into(),
            description: "Extrayez l'archive dans le compartiment temporaire /tmp.".into(),
            story: "Tael Rhen: Â« Capitaine, extrayez l'archive backup_rapports.tar.gz dans
le compartiment temporaire /tmp pour vÃ©rification, sans toucher aux soutes. Â»

Â« DÃ©compressez l'archive dans /tmp, Capitaine. Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

N'oubliez pas, Capitaine: dans l'espace, la prÃ©paration et la prÃ©cision font la diffÃ©rence entre la victoire et la dÃ©faite.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

L'archive est extraite dans /tmp. Les fichiers sont prêts à être examinés.

« Extraire dans un espace de travail, c'est plus sûr », explique Tael Rhen. « On ne touche pas aux originaux. »".into(),
            goal: "Extrayez l'archive /soutes/backup_rapports.tar.gz dans le répertoire /tmp.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment extraire une archive dans un dossier précis?".into(),
            required_commands: vec!["tar xzf /soutes/backup_rapports.tar.gz -C /tmp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "xzf".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "tar xzf".into(), name: "tar xzf".into(), description: "tar xzf — extrait une archive compressée".into() }),
        },
        Mission {
            id: "15.4".into(), chapter: 15, chapter_title: "Archives et sauvegardes".into(), act: 4,
            title: "Lire une archive ZIP".into(),
            description: "L'Andromède préfère le ZIP: listez rapports.zip.".into(),
            story: "Nira Okoro: Â« Capitaine, l'AndromÃ¨de prÃ©fÃ¨re recevoir les rapports au format
ZIP. Vous avez crÃ©Ã© rapports.zip dans votre cabine. Montrez-moi son contenu. Â»

Â« Listez le contenu de rapports.zip, Capitaine. Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

N'oubliez pas, Capitaine: dans l'espace, la prÃ©paration et la prÃ©cision font la diffÃ©rence entre la victoire et la dÃ©faite.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

Le contenu du fichier ZIP s'affiche. L'Andromède pourra lire ces rapports.

« Le format ZIP est universel », note Nira Okoro. « C'est le langage commun des vaisseaux. »".into(),
            goal: "Listez le contenu de l'archive rapports.zip de votre cabine.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lister le contenu d'une archive ZIP?".into(),
            required_commands: vec!["unzip -l rapports.zip".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "unzip".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "unzip -l".into(), name: "unzip".into(), description: "unzip -l — liste le contenu d'une archive ZIP".into() }),
        },
        Mission {
            id: "15.5".into(), chapter: 15, chapter_title: "Archives et sauvegardes".into(), act: 4,
            title: "Etat des reservoirs".into(),
            description: "Tael Rhen veut la jauge des réservoirs de données du vaisseau.".into(),
            story: "Tael Rhen: « Capitaine, combien de carburant numérique reste-t-il dans
les réservoirs de données? Donnez-moi l'état de toutes les partitions,
en taille lisible. »

« Quelle place disque restante sur le vaisseau? »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

L'espace disque s'affiche en taille lisible. Les soutes de données ont encore de la place.

« Bien », approuve Tael Rhen. « Nous savons combien de données nous pouvons encore stocker avant saturation. »".into(),
            goal: "Affichez l'espace disque utilisé et disponible de toutes les partitions, en taille lisible.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'espace disque restant?".into(),
            required_commands: vec!["df -h".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "df".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "df -h".into(), name: "df".into(), description: "df -h — espace disque utilisé/disponible en taille lisible".into() }),
        },
    ]
}

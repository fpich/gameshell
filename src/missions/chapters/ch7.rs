//! CHAPITRE 7 - Permissions et acces (Acte 2) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "7.1".into(), chapter: 7, chapter_title: "Permissions et acces".into(), act: 2,
            title: "Verrouiller le plan de defense".into(),
            description: "Vex Korshun: le plan de défense ne doit être lisible que par vous.".into(),
            story: "Vex Korshun, officier sécurité, frappe à votre porte:
« Capitaine, le fichier plan_defense.sh sur votre bureau contient les codes
de déploiement des armes. En ce moment, tout l'équipage peut le lire! »

Il vous demande de verrouiller le fichier: seul le capitaine (vous) doit
pouvoir le lire et l'écrire. Les autres: aucun accès.

« Mettez les permissions 700 sur plan_defense.sh, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Vex Korshun vérifie les permissions: « Désormais, seul vous pouvez lire le plan de défense. C'est une question de sécurité vitale. »

Le plan est protégé. Vous comprenez que restreindre l'accès aux documents sensibles est la première ligne de défense contre les espions.".into(),
            goal: "Restreignez plan_defense.sh pour que seul vous puissiez le lire (permissions 700).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment interdire l'accès à un fichier à tout le monde sauf soi?".into(),
            required_commands: vec!["chmod 700 plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "permission".into(), target: "~/plan_defense.sh".into(), expected: Some("700".into()) }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "chmod 700".into(), name: "chmod".into(), description: "chmod — change les permissions rwx d'un fichier (700, 600, +x)".into() }),
        },
        Mission {
            id: "7.2".into(), chapter: 7, chapter_title: "Permissions et acces".into(), act: 2,
            title: "Rendre le script executable".into(),
            description: "Le plan de défense doit pouvoir être lancé comme un programme.".into(),
            story: "Vex Korshun revient: Â« Capitaine, plan_defense.sh est un script de dÃ©fense.
Pour le lancer en urgence, il doit Ãªtre exÃ©cutable. Â»

Un fichier texte n'est pas exÃ©cutable par dÃ©faut: il faut lui ajouter
le droit d'exÃ©cution.

Â« Ajoutez le droit d'exÃ©cution Ã  plan_defense.sh, Capitaine. Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

Le plan de défense est maintenant exécutable. Vex Korshun hoche la tête: « En cas d'urgence, vous pourrez le lancer d'une simple commande. »

Un fichier protégé mais inutilisable ne sert à rien. Savoir ajuster les droits d'exécution, c'est rendre un outil réellement opérationnel.".into(),
            goal: "Rendez le fichier plan_defense.sh exécutable.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment rendre un fichier exécutable?".into(),
            required_commands: vec!["chmod +x plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "+x".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "chmod +x".into(), name: "chmod +x".into(), description: "chmod +x — rend un fichier exécutable (script)".into() }),
        },
        Mission {
            id: "7.3".into(), chapter: 7, chapter_title: "Permissions et acces".into(), act: 2,
            title: "Lire le coffre-fort".into(),
            description: "Les codes d'accès de la station sont dans le coffre-fort.".into(),
            story: "Vex Korshun vous confie: « Capitaine, les codes d'accès de la station
sont dans le coffre-fort du compartiment sécurité:
/securite/coffre-fort/codes_acces.txt. »

L'Amiral Xylia Vorn en aura besoin pour la contre-attaque.

« Lisez le contenu du coffre-fort et dites-moi ce qu'il contient. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

Vous lisez les codes d'accès du coffre-fort de la sécurité. Chaque code ouvre une porte, chaque porte protège un secret.

Draz M'kal vous observe: « Ces codes sont la clé du vaisseau. Gardez-les en mémoire, Capitaine. »".into(),
            goal: "Lisez le fichier codes_acces.txt du coffre-fort de la sécurité.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier dans un autre compartiment?".into(),
            required_commands: vec!["cat /securite/coffre-fort/codes_acces.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "codes_acces.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche le contenu complet d'un fichier texte".into() }),
        },
        Mission {
            id: "7.4".into(), chapter: 7, chapter_title: "Permissions et acces".into(), act: 2,
            title: "Fiche technique du plan".into(),
            description: "Vérifiez les métadonnées de plan_defense.sh.".into(),
            story: "Tael Rhen veut la fiche technique complète de plan_defense.sh:
sa taille, ses permissions actuelles, sa date de modification.

« Capitaine, donnez-moi toutes les métadonnées du fichier plan_defense.sh
avant qu'on le modifie. »

Il vous faut la commande qui affiche la fiche technique complète d'un fichier.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

La fiche technique de plan_defense.sh s'affiche: taille, permissions, dates de modification.

« Ces métadonnées racontent l'histoire du fichier », explique Tael Rhen. « Une date anormale peut révéler une intrusion. »".into(),
            goal: "Affichez les métadonnées complètes (taille, permissions, dates) de plan_defense.sh.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir toutes les métadonnées d'un fichier?".into(),
            required_commands: vec!["stat plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "stat".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "stat".into(), name: "stat".into(), description: "stat — affiche les métadonnées: taille, permissions, dates, inode".into() }),
        },
        Mission {
            id: "7.5".into(), chapter: 7, chapter_title: "Permissions et acces".into(), act: 2,
            title: "Transférer la responsabilité".into(),
            description: "Le plan de défense relève officiellement du capitaine.".into(),
            story: "Vex Korshun: « Capitaine, pour les audits, plan_defense.sh doit officiellement
vous appartenir. Actuellement il appartient à un autre membre d'équipage. »

Il vous faut changer le propriétaire du fichier pour vous-même.
Utilisez votre propre nom d'utilisateur système.

« Transférez la propriété de plan_defense.sh à votre compte, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Le plan de défense vous appartient désormais officiellement. Vex Korshun approuve: « Bien. La propriété d'un fichier doit refléter la réalité des responsabilités. »

Vous comprenez que chaque fichier a un propriétaire, et que cette attribution est essentielle pour l'audit et la sécurité.".into(),
            goal: "Changez le propriétaire de plan_defense.sh pour votre compte utilisateur.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment changer le propriétaire d'un fichier?".into(),
            required_commands: vec!["sudo chown $USER plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chown".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chown".into(), name: "chown".into(), description: "chown — change le propriétaire d'un fichier (user:group)".into() }),
        },
    ]
}

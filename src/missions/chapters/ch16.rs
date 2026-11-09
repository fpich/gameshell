//! CHAPITRE 16 - Gestion de l'equipage (Acte 4) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "16.1".into(), chapter: 16, chapter_title: "Gestion de l'equipage".into(), act: 4,
            title: "Vos equipes".into(),
            description: "Vex Korshun veut savoir à quelles équipes appartient le capitaine.".into(),
            story: "Vex Korshun, officier sÃ©curitÃ©: Â« Capitaine, pour le contrÃ´le d'accÃ¨s du pont,
je dois vÃ©rifier Ã  quelles Ã©quipes (groupes) appartient votre badge. Â»

La commande qui affiche les groupes de l'utilisateur courant.

Â« Ã quelles Ã©quipes appartenez-vous, Capitaine? Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La liste de vos groupes s'affiche. Vous appartenez à plusieurs équipes du vaisseau.

« Chaque groupe définit vos droits », explique Vex Korshun. « Savoir à quelles équipes vous appartenez, c'est connaître votre place à bord. »".into(),
            goal: "Affichez la liste des groupes (équipes) auxquels appartient votre compte.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Comment voir ses propres groupes?".into(),
            required_commands: vec!["groups".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "groups".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "groups".into(), name: "groups".into(), description: "groups — affiche les groupes de l'utilisateur courant".into() }),
        },
        Mission {
            id: "16.2".into(), chapter: 16, chapter_title: "Gestion de l'equipage".into(), act: 4,
            title: "Fiche du matelot".into(),
            description: "Vérifiez le badge complet du nouveau matelot.".into(),
            story: "Vex Korshun: « Capitaine, le matelot vient d'être recruté. Vérifiez son
badge complet: uid, gid et tous ses groupes. »

La commande qui affiche l'identité complète d'un utilisateur précis.

« Donnez-moi la fiche complète du matelot, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

La fiche complète du compte matelot s'affiche: uid, gid, groupes.

« Chaque membre a une identité numérique unique », explique Vex Korshun. « C'est elle qui détermine ses accès. »".into(),
            goal: "Affichez l'identité complète (uid, gid, groupes) du compte matelot.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'identité complète d'un autre utilisateur?".into(),
            required_commands: vec!["id matelot".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "id".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "id user".into(), name: "id".into(), description: "id — affiche uid, gid et groupes d'un utilisateur".into() }),
        },
        Mission {
            id: "16.3".into(), chapter: 16, chapter_title: "Gestion de l'equipage".into(), act: 4,
            title: "Ajouter le matelot a l'equipe".into(),
            description: "Le matelot rejoint l'équipe 'equipage' du vaisseau.".into(),
            story: "Vex Korshun: Â« Capitaine, le matelot doit intÃ©grer l'Ã©quipe 'equipage'
pour accÃ©der aux coursives communes. Ajoutez-le Ã  ce groupe. Â»

La commande de modification d'utilisateur permet d'ajouter un membre
Ã  un groupe existant, sans retirer les autres.

Â« Ajoutez matelot au groupe equipage, Capitaine. Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

Le compte matelot rejoint l'équipe equipage. Ses droits sont mis à jour.

« Bien », approuve Vex Korshun. « Le matelot peut maintenant accéder aux ressources de son équipe. »".into(),
            goal: "Ajoutez le compte matelot au groupe equipage.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter un utilisateur à un groupe?".into(),
            required_commands: vec!["sudo usermod -aG equipage matelot".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "usermod".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "usermod -aG".into(), name: "usermod".into(), description: "usermod -aG — ajoute un utilisateur à un groupe".into() }),
        },
        Mission {
            id: "16.4".into(), chapter: 16, chapter_title: "Gestion de l'equipage".into(), act: 4,
            title: "Prendre le fauteuil du matelot".into(),
            description: "Vérifiez que le compte matelot fonctionne en prenant sa session.".into(),
            story: "Vex Korshun: Â« Capitaine, testez le compte du matelot: prenez sa session
et dites-moi qui il est. Â»

La commande de changement d'utilisateur ouvre une session temporaire
dans le compte d'un autre membre d'Ã©quipage pour exÃ©cuter une commande.

Â« Prenez la session du matelot et affichez qui il est. Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

Vous ouvrez une session temporaire du compte matelot. Le nom affiché confirme son identité.

« Le compte fonctionne », note Vex Korshun. « Tester une session, c'est vérifier que tout est en ordre. »".into(),
            goal: "Ouvrez une session temporaire du compte matelot et affichez son nom d'utilisateur.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment exécuter une commande en tant qu'autre utilisateur?".into(),
            required_commands: vec!["su - matelot -c whoami".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "su".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "su - user -c cmd".into(), name: "su".into(), description: "su - user -c cmd — exécute une commande en tant qu'autre utilisateur".into() }),
        },
        Mission {
            id: "16.5".into(), chapter: 16, chapter_title: "Gestion de l'equipage".into(), act: 4,
            title: "Vos pouvoirs de capitaine".into(),
            description: "Quelles commandes sudo le capitaine peut-il lancer?".into(),
            story: "Vex Korshun: « Capitaine, pour l'audit de sécurité, listez les commandes
que votre clé maître (sudo) vous autorise à lancer. »

La commande sudo avec l'option de liste affiche les droits du capitaine.

« Quels sont vos pouvoirs sudo, Capitaine? »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

La liste de vos droits sudo s'affiche. Vous pouvez exécuter certaines commandes en tant que capitaine.

« Ces pouvoirs sont une responsabilité », rappelle Vex Korshun. « Utilisez-les avec discernement. »".into(),
            goal: "Montrez à Vex Korshun les commandes que vous êtes autorisé à exécuter avec les pouvoirs du capitaine.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment lister ses propres droits sudo?".into(),
            required_commands: vec!["sudo -l".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "sudo".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "sudo".into(), name: "sudo".into(), description: "sudo — exécute une commande en tant que root (clé maître du capitaine)".into() }),
        },
    ]
}

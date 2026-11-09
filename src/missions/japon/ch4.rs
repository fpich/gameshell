//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 4 — Registres du domaine (Acte 1) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "4.1".into(), chapter: 4, chapter_title: "Registres du domaine".into(), act: 1,
            title: "Lire vos notes".into(),
            description: "Relisez notes.txt.".into(),
            story: "Sur votre bureau: notes.txt, votre carnet. « Seigneur, relisez vos notes:
elle contient des points à ne pas oublier. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Vous relisez vos notes, Seigneur. Chaque point y est consigné. Un seigneur qui se souvient de tout ne se laisse jamais surprendre.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Lisez notes.txt dans vos quartiers.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier?".into(),
            required_commands: vec!["cat notes.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "notes.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche le contenu".into() }),
        },
        Mission {
            id: "4.2".into(), chapter: 4, chapter_title: "Registres du domaine".into(), act: 1,
            title: "Ajouter au registre".into(),
            description: "Ajoutez une ligne à la fin de notes.txt.".into(),
            story: "Dame Kaguya: « Ajoutez à la FIN de notes.txt, sans écraser le reste:
'Reparer le rempart est'. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya vérifie notes.txt: « 'Reparer le rempart est' est bien ajouté, sans rien écraser. Le registre est complet, Seigneur. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Ajoutez une ligne à la fin de notes.txt sans l'écraser.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter à la fin d'un fichier?".into(),
            required_commands: vec!["echo 'Reparer le rempart est' >> notes.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: ">>".into(), name: ">>".into(), description: "COMMANDE >> fichier — ajoute à la fin".into() }),
        },
        Mission {
            id: "4.3".into(), chapter: 4, chapter_title: "Registres du domaine".into(), act: 1,
            title: "Compter les lignes du kaisho".into(),
            description: "Combien de lignes fait kaisho.txt?".into(),
            story: "Dame Kaguya: « Pour le registre, dites-moi combien de lignes fait
kaisho.txt. Juste le nombre. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya note le nombre: « Voilà la longueur du kaisho, Seigneur. Le registre est à jour. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Comptez les lignes de kaisho.txt.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment compter les lignes?".into(),
            required_commands: vec!["wc -l kaisho.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "wc".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "wc -l".into(), name: "wc -l".into(), description: "wc -l — compte les lignes".into() }),
        },
        Mission {
            id: "4.4".into(), chapter: 4, chapter_title: "Registres du domaine".into(), act: 1,
            title: "Début du journal des gardes".into(),
            description: "Les 5 premières lignes du journal des gardes.".into(),
            story: "Au donjon, le capitaine des gardes consulte journal_gardes.txt:
« Seigneur, montrez-moi le DÉBUT du journal des gardes. 5 premières lignes. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Le capitaine des gardes lit le début du journal: « Les premières rondes sont consignées, Seigneur. La garde fait son devoir. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez les 5 premières lignes du journal des gardes.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment n'afficher que le début?".into(),
            required_commands: vec!["head -5 /donjon/gardes/journal_gardes.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "head".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-5".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "head".into(), name: "head".into(), description: "head -n N — N premières lignes".into() }),
        },
        Mission {
            id: "4.5".into(), chapter: 4, chapter_title: "Registres du domaine".into(), act: 1,
            title: "Fin du journal système".into(),
            description: "Les 3 dernières lignes du syslog.".into(),
            story: "Le moine Tetsuo: « Seigneur, quelles sont les DERNIÈRES nouvelles du
château? Montrez-moi la fin du syslog. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le moine Tetsuo lit la fin du syslog: « Les dernières nouvelles du château sont claires, Seigneur. Rien d'inquiétant pour l'instant. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez les 3 dernières lignes du syslog.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment n'afficher que la fin?".into(),
            required_commands: vec!["tail -3 /scribes/logs/syslog".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tail".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-3".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "tail".into(), name: "tail".into(), description: "tail -n N — N dernières lignes".into() }),
        },
    ]
}

//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 16 — Inventaire complet (Acte 4) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "16.1".into(), chapter: 16, chapter_title: "Inventaire complet".into(), act: 4,
            title: "Racine du château".into(),
            description: "Listez la racine du château.".into(),
            story: "Dame Kaguya: « Seigneur, listez tous les bâtiments du château (racine /). »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya parcourt la liste: « Tous les bâtiments du château sont là, Seigneur. Le domaine est bien organisé. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Listez la racine / du château.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Comment lister la racine?".into(),
            required_commands: vec!["ls /".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ls".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "ls /".into(), name: "ls /".into(), description: "ls / — liste la racine".into() }),
        },
        Mission {
            id: "16.2".into(), chapter: 16, chapter_title: "Inventaire complet".into(), act: 4,
            title: "Tous les fichiers cachés".into(),
            description: "Liste détaillée de la racine, cachés compris.".into(),
            story: "Le moine Tetsuo: « Seigneur, listez la racine EN DÉTAIL, y compris les
fichiers cachés. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le moine Tetsuo inspecte la liste: « Même les fichiers cachés sont visibles, Seigneur. Rien ne vous échappe. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Listez la racine en détail avec cachés (ls -la /).".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment tout lister à la racine?".into(),
            required_commands: vec!["ls -la /".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-la".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ls -la".into(), name: "ls -la".into(), description: "ls -la — tout lister".into() }),
        },
        Mission {
            id: "16.3".into(), chapter: 16, chapter_title: "Inventaire complet".into(), act: 4,
            title: "Tous les .txt".into(),
            description: "Listez tous les fichiers .txt du château.".into(),
            story: "Dame Kaguya: « Seigneur, combien de documents .txt contient le château?
Trouvez-les tous. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya recense les documents: « Voilà tous les .txt du château, Seigneur. L'inventaire est complet. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Trouvez tous les fichiers .txt (find / -name '*.txt').".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment chercher tous les .txt?".into(),
            required_commands: vec!["find / -name '*.txt'".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "find".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-name".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "find".into(), name: "find".into(), description: "find -name '*.txt' — tous les documents".into() }),
        },
        Mission {
            id: "16.4".into(), chapter: 16, chapter_title: "Inventaire complet".into(), act: 4,
            title: "Etat des remparts".into(),
            description: "Lisez l'état des remparts.".into(),
            story: "Le capitaine des gardes: « Seigneur, lisez l'état des remparts:
/remparts/etat_remparts.txt. Le rempart est est fissuré! »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes lit l'état: « Le rempart est est fissuré, Seigneur. Nous savons quoi réparer en priorité. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Lisez le fichier etat_remparts.txt.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire l'état des remparts?".into(),
            required_commands: vec!["cat /remparts/etat_remparts.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "etat_remparts.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "16.5".into(), chapter: 16, chapter_title: "Inventaire complet".into(), act: 4,
            title: "Stock de riz".into(),
            description: "Lisez le registre du riz.".into(),
            story: "Le marchand Genji: « Seigneur, consultez le stock de riz:
/greniers/riz/registre_riz.txt. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le marchand Genji consulte le stock: « Voilà le riz disponible, Seigneur. Le commerce peut continuer. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Lisez le registre du riz des greniers.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire le registre du riz?".into(),
            required_commands: vec!["cat /greniers/riz/registre_riz.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "registre_riz.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
    ]
}

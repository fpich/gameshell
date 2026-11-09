//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 8 — Navigation et disques (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "8.1".into(), chapter: 8, chapter_title: "Navigation et disques".into(), act: 2,
            title: "Priorité basse".into(),
            description: "La surveillance des greniers ne doit pas gêner la forge.".into(),
            story: "Le forgeron: « Seigneur, lancez le pesage des greniers en tâche de fond,
priorité FAIBLE. La forge d'abord! »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le forgeron approuve: « Le pesage tourne en tâche de fond, à basse priorité, Seigneur. La forge garde toute sa puissance. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Lancez le pesage des greniers avec priorité basse (nice -n 19).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment lancer une commande en priorité basse?".into(),
            required_commands: vec!["nice -n 19 du -sh /greniers".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "nice".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-n".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "nice -n".into(), name: "nice -n".into(), description: "nice -n 19 — priorité basse".into() }),
        },
        Mission {
            id: "8.2".into(), chapter: 8, chapter_title: "Navigation et disques".into(), act: 2,
            title: "Espace des réserves".into(),
            description: "Quelle place reste sur les réserves?".into(),
            story: "Dame Kaguya: « Seigneur, quelle place disque reste-t-il sur les
réserves du château? Taille lisible. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya consulte l'espace: « Voilà la place restante, Seigneur. Les réserves du château sont saines. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez l'espace disque restant (df -h).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'espace disque restant?".into(),
            required_commands: vec!["df -h".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "df".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-h".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "df -h".into(), name: "df -h".into(), description: "df -h — espace disque lisible".into() }),
        },
        Mission {
            id: "8.3".into(), chapter: 8, chapter_title: "Navigation et disques".into(), act: 2,
            title: "Taille par grenier".into(),
            description: "Taille de chaque grenier.".into(),
            story: "Le marchand Genji: « Seigneur, pesez chaque grenier UN PAR UN:
riz, blé, saké, miso. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le marchand Genji pèse chaque grenier: « Riz, blé, saké, miso... chaque réserve est mesurée, Seigneur. Le commerce sera juste. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez la taille de chaque sous-dossier de /greniers.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir la taille de chaque sous-dossier?".into(),
            required_commands: vec!["du -sh /greniers/*".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "du".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-sh".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "du".into(), name: "du".into(), description: "du — taille des fichiers et dossiers".into() }),
        },
        Mission {
            id: "8.4".into(), chapter: 8, chapter_title: "Navigation et disques".into(), act: 2,
            title: "Remonter d'un niveau".into(),
            description: "Depuis /archives/lettres, revenez à la racine.".into(),
            story: "Dame Kaguya: « Seigneur, vous êtes dans /archives/lettres. Revenez à
la racine du château avec un chemin relatif. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya approuve: « Vous voilà à la racine du château, Seigneur. Un chemin relatif bien maîtrisé. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Depuis /archives/lettres, rejoignez la racine / du château.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment remonter dans l'arborescence?".into(),
            required_commands: vec!["cd /archives/lettres".into(), "cd ../..".into(), "pwd".into()],
            checks: vec![MissionCheck { check_type: "dir_reached".into(), target: "/".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "cd ..".into(), name: "cd ..".into(), description: "cd .. — remonte d'un niveau".into() }),
        },
        Mission {
            id: "8.5".into(), chapter: 8, chapter_title: "Navigation et disques".into(), act: 2,
            title: "Le home du seigneur".into(),
            description: "Revenez dans vos quartiers avec ~.".into(),
            story: "Après toutes ces courses, Dame Kaguya vous rappelle: « Seigneur,
revenez dans VOS quartiers avec le raccourci ~. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya sourit: « Vous êtes de retour dans vos quartiers, Seigneur. Le raccourci ~ est bien pratique. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Revenez dans vos quartiers (~) et affichez votre position.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Comment revenir directement dans son home?".into(),
            required_commands: vec!["cd ~".into(), "pwd".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "~".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cd ~".into(), name: "cd ~".into(), description: "cd ~ — retour aux quartiers".into() }),
        },
    ]
}

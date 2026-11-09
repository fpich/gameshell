//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 5 — Permissions et sceaux (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "5.1".into(), chapter: 5, chapter_title: "Permissions et sceaux".into(), act: 2,
            title: "Verrouiller le plan de défense".into(),
            description: "plan_defense.sh ne doit être lisible que par vous: 700.".into(),
            story: "Le capitaine des gardes: « Seigneur, plan_defense.sh contient les ordres
de bataille. Tout le monde peut le lire actuellement! Verrouillez-le:
seul vous (700). »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Le capitaine des gardes approuve: « plan_defense.sh est verrouillé, Seigneur. Seul vous pouvez le lire. Les ordres de bataille sont en sûreté. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Mettez les permissions 700 sur plan_defense.sh.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment restreindre un fichier à soi seul?".into(),
            required_commands: vec!["chmod 700 plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "permission".into(), target: "~/plan_defense.sh".into(), expected: Some("700".into()) }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chmod".into(), name: "chmod".into(), description: "chmod — change les permissions".into() }),
        },
        Mission {
            id: "5.2".into(), chapter: 5, chapter_title: "Permissions et sceaux".into(), act: 2,
            title: "Rendre le plan exécutable".into(),
            description: "Le plan de défense doit être lançable: +x.".into(),
            story: "Le capitaine des gardes: « Seigneur, pour lancer le plan de défense en
urgence, il doit être exécutable. Ajoutez-lui le droit d'exécution. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Le capitaine des gardes hoche la tête: « Le plan de défense est exécutable, Seigneur. En cas d'urgence, nous pourrons le lancer aussitôt. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Rendez plan_defense.sh exécutable.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment rendre un script exécutable?".into(),
            required_commands: vec!["chmod +x plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "+x".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chmod +x".into(), name: "chmod +x".into(), description: "chmod +x — droit d'exécution".into() }),
        },
        Mission {
            id: "5.3".into(), chapter: 5, chapter_title: "Permissions et sceaux".into(), act: 2,
            title: "Fiche du plan de défense".into(),
            description: "Métadonnées complètes de plan_defense.sh.".into(),
            story: "Dame Kaguya: « Seigneur, donnez-moi la fiche technique complète de
plan_defense.sh: taille, permissions, dates. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya consulte la fiche: « Taille, permissions, dates... tout y est, Seigneur. Le plan de défense est parfaitement documenté. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez les métadonnées de plan_defense.sh.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir toutes les métadonnées?".into(),
            required_commands: vec!["stat plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "stat".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "stat".into(), name: "stat".into(), description: "stat — métadonnées complètes".into() }),
        },
        Mission {
            id: "5.4".into(), chapter: 5, chapter_title: "Permissions et sceaux".into(), act: 2,
            title: "Transférer le sceau".into(),
            description: "Le plan appartient officiellement au seigneur.".into(),
            story: "Pour l'audit, plan_defense.sh doit vous appartenir officiellement.
« Transférez la propriété à votre compte, Seigneur. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya confirme: « plan_defense.sh vous appartient officiellement, Seigneur. L'audit sera sans reproche. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Changez le propriétaire de plan_defense.sh pour votre compte.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment changer le propriétaire?".into(),
            required_commands: vec!["sudo chown $USER plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chown".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chown".into(), name: "chown".into(), description: "chown — change le propriétaire".into() }),
        },
        Mission {
            id: "5.5".into(), chapter: 5, chapter_title: "Permissions et sceaux".into(), act: 2,
            title: "Détail des quartiers".into(),
            description: "Liste détaillée avec permissions.".into(),
            story: "Dame Kaguya veut contrôler les permissions de vos quartiers:
« Seigneur, listez vos quartiers EN DÉTAIL: permissions, tailles, dates. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya inspecte la liste: « Permissions, tailles, dates... tout est en ordre dans vos quartiers, Seigneur. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Listez vos quartiers avec le détail des permissions (ls -l).".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les permissions de chaque fichier?".into(),
            required_commands: vec!["ls -l".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ls -l".into(), name: "ls -l".into(), description: "ls -l — liste détaillée avec permissions".into() }),
        },
    ]
}

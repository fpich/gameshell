//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 6 — Surveiller le château (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "6.1".into(), chapter: 6, chapter_title: "Surveiller le chateau".into(), act: 2,
            title: "Tous les processus".into(),
            description: "Quels systèmes tournent dans le château?".into(),
            story: "Le forgeron: « Seigneur, quels systèmes tournent en ce moment dans le
château? Donnez-moi la liste complète des processus. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le forgeron parcourt la liste: « Voilà tous les systèmes qui tournent, Seigneur. Le château est bien vivant. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Listez tous les processus (ps aux).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment lister tous les processus?".into(),
            required_commands: vec!["ps aux".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ps".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "aux".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ps aux".into(), name: "ps aux".into(), description: "ps aux — tous les processus".into() }),
        },
        Mission {
            id: "6.2".into(), chapter: 6, chapter_title: "Surveiller le chateau".into(), act: 2,
            title: "Filtrer les gardes".into(),
            description: "Quels processus concernent les gardes?".into(),
            story: "Le capitaine des gardes: « La liste est longue. Je veux seulement les
processus qui parlent de 'garde'. Filtrez le résultat. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes examine le filtre: « Seuls les processus de garde, Seigneur. La surveillance est ciblée. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez uniquement les processus contenant 'garde'.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment filtrer la sortie d'une commande?".into(),
            required_commands: vec!["ps aux | grep garde".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "|".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ps".into(), name: "ps".into(), description: "ps — liste les processus en cours".into() }),
        },
        Mission {
            id: "6.3".into(), chapter: 6, chapter_title: "Surveiller le chateau".into(), act: 2,
            title: "Processus par nom".into(),
            description: "Y a-t-il des processus 'espion'?".into(),
            story: "Le ninja Hanzō prévient qu'un processus suspect tourne.
« Seigneur, listez les processus dont le nom contient 'espion'. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le ninja Hanzō examine la liste: « Le processus espion est repéré, Seigneur. Nous savons où il se cache. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Listez les processus dont le nom contient 'espion'.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment lister des processus par nom?".into(),
            required_commands: vec!["pgrep -a espion".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "pgrep".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-a".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "pgrep -a".into(), name: "pgrep -a".into(), description: "pgrep -a — processus par nom".into() }),
        },
        Mission {
            id: "6.4".into(), chapter: 6, chapter_title: "Surveiller le chateau".into(), act: 2,
            title: "Jauges du château".into(),
            description: "Aperçu instantané des ressources.".into(),
            story: "Le forgeron: « Seigneur, un aperçu des jauges du château: mémoire, CPU,
processus gourmands. Un cliché suffit. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le forgeron regarde le cliché: « Mémoire, CPU, processus gourmands... le château tient bon, Seigneur. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez un aperçu des ressources (top -bn1).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment surveiller les ressources?".into(),
            required_commands: vec!["top -bn1".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "top".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-bn1".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "top -bn1".into(), name: "top -bn1".into(), description: "top -bn1 — aperçu CPU/mémoire".into() }),
        },
        Mission {
            id: "6.5".into(), chapter: 6, chapter_title: "Surveiller le chateau".into(), act: 2,
            title: "Éliminer l'intrus".into(),
            description: "Le processus espion PID 6666 tourne encore.".into(),
            story: "Hanzō: « Seigneur! Le processus espion, PID 6666, vole nos registres.
Coupez son alimentation, de force. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Hanzō confirme: « Le processus espion est neutralisé, Seigneur. Nos registres sont en sûreté. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Tuez le processus 6666 (kill -9).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment tuer un processus par PID?".into(),
            required_commands: vec!["kill -9 6666".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "kill".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-9".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "kill -9".into(), name: "kill -9".into(), description: "kill -9 — tue de force".into() }),
        },
    ]
}

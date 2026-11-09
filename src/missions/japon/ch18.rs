//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 18 — Services du château (Acte 5) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "18.1".into(), chapter: 18, chapter_title: "Services du chateau".into(), act: 5,
            title: "État du service SSH".into(),
            description: "Le service SSH tourne-t-il?".into(),
            story: "Hanzō: « Seigneur, avant le siège, vérifiez que le service SSH tourne
bien: systemctl status ssh. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Hanzō vérifie: « Le service SSH tourne bien, Seigneur. Le portail est ouvert et fonctionnel. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez l'état du service ssh.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'état d'un service?".into(),
            required_commands: vec!["systemctl status ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "systemctl".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "status".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "systemctl status".into(), name: "systemctl status".into(), description: "systemctl status — état d'un service".into() }),
        },
        Mission {
            id: "18.2".into(), chapter: 18, chapter_title: "Services du chateau".into(), act: 5,
            title: "Redémarrer SSH".into(),
            description: "Redémarrez SSH après les réglages.".into(),
            story: "Hanzō: « Seigneur, redémarrez SSH pour appliquer les réglages. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Hanzō confirme: « SSH est redémarré, Seigneur. Les réglages sont appliqués. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Redémarrez le service ssh (sudo systemctl restart ssh).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment redémarrer un service?".into(),
            required_commands: vec!["sudo systemctl restart ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "restart".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "systemctl restart".into(), name: "systemctl restart".into(), description: "systemctl restart — redémarre un service".into() }),
        },
        Mission {
            id: "18.3".into(), chapter: 18, chapter_title: "Services du chateau".into(), act: 5,
            title: "Logs du service SSH".into(),
            description: "Les 10 dernières lignes du journal de ssh.".into(),
            story: "Le capitaine des gardes: « Seigneur, vérifiez le journal du service SSH:
10 dernières lignes. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes lit le journal: « Les dernières lignes de SSH sont claires, Seigneur. Le service est sain. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez les 10 dernières lignes du journal de ssh.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les logs d'un service?".into(),
            required_commands: vec!["journalctl -u ssh -n 10".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "journalctl".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-u".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-n".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "journalctl".into(), name: "journalctl".into(), description: "journalctl -u ssh -n 10 — logs d'un service".into() }),
        },
        Mission {
            id: "18.4".into(), chapter: 18, chapter_title: "Services du chateau".into(), act: 5,
            title: "Bannissements fail2ban".into(),
            description: "Quels espions sont bannis?".into(),
            story: "Le capitaine des gardes: « Seigneur, le videur automatique (fail2ban)
a banni des IP. Montrez le statut. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes consulte le statut: « Le videur a banni des IP, Seigneur. Les intrus sont tenus à distance. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez l'état de la protection anti-intrusion du château.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les bannissements?".into(),
            required_commands: vec!["sudo fail2ban-client status".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "fail2ban-client".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "status".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "fail2ban-client status".into(), name: "fail2ban-client status".into(), description: "fail2ban-client status — bannissements".into() }),
        },
        Mission {
            id: "18.5".into(), chapter: 18, chapter_title: "Services du chateau".into(), act: 5,
            title: "Débannir l'allié".into(),
            description: "Le clan Hattori (10.0.0.99) a été banni par erreur.".into(),
            story: "Hanzō, paniqué: « Seigneur! Le videur a banni par erreur le clan Hattori
(10.0.0.99)! Débanissez-le immédiatement! »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Hanzō, soulagé: « Le clan Hattori est débanni, Seigneur. Nos alliés peuvent revenir. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Débanissez 10.0.0.99 (sudo fail2ban-client unban 10.0.0.99).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment révoquer un bannissement?".into(),
            required_commands: vec!["sudo fail2ban-client unban 10.0.0.99".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "unban".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "fail2ban-client unban".into(), name: "fail2ban-client unban".into(), description: "fail2ban-client unban IP — révoque un ban".into() }),
        },
    ]
}

//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 7 — Lire les registres (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "7.1".into(), chapter: 7, chapter_title: "Lire les registres".into(), act: 2,
            title: "Échecs de connexion".into(),
            description: "Quelles tentatives ont échoué dans auth.log?".into(),
            story: "Le capitaine des gardes ouvre auth.log: « Seigneur, montrez-moi toutes les
tentatives de connexion ÉCHOUÉES ('Failed'). »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes lit les échecs: « Toutes les tentatives échouées sont là, Seigneur. Quelqu'un cherche à entrer. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez les lignes 'Failed' de /scribes/logs/auth.log.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment chercher un mot dans un fichier?".into(),
            required_commands: vec!["grep Failed /scribes/logs/auth.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "grep".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "grep".into(), name: "grep".into(), description: "grep — recherche un motif".into() }),
        },
        Mission {
            id: "7.2".into(), chapter: 7, chapter_title: "Lire les registres".into(), act: 2,
            title: "Combien d'attaques?".into(),
            description: "Comptez les 'Failed' dans auth.log.".into(),
            story: "Le capitaine des gardes: « Juste le NOMBRE d'échecs, Seigneur. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes note le nombre: « Voilà le compte des échecs, Seigneur. L'ampleur de l'attaque est claire. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Comptez les lignes 'Failed' de auth.log.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment compter les résultats d'une recherche?".into(),
            required_commands: vec!["grep -c Failed /scribes/logs/auth.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-c".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "grep -c".into(), name: "grep -c".into(), description: "grep -c — compte les lignes trouvées".into() }),
        },
        Mission {
            id: "7.3".into(), chapter: 7, chapter_title: "Lire les registres".into(), act: 2,
            title: "L'espion avec numéros de ligne".into(),
            description: "Lignes de 10.0.0.66 avec numéros.".into(),
            story: "Hanzō: « Seigneur, dans auth.log, quelles lignes parlent de l'IP espion
10.0.0.66? Avec leur NUMÉRO de ligne pour le procès-verbal. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Hanzō relève les numéros de ligne: « Les preuves contre l'IP espion sont consignées, Seigneur. Le procès-verbal sera solide. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez les lignes de 10.0.0.66 avec leur numéro.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment afficher les numéros de ligne?".into(),
            required_commands: vec!["grep -n 10.0.0.66 /scribes/logs/auth.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-n".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "grep -n".into(), name: "grep -n".into(), description: "grep -n — numéros de ligne".into() }),
        },
        Mission {
            id: "7.4".into(), chapter: 7, chapter_title: "Lire les registres".into(), act: 2,
            title: "Chercher 'ronin' sans la casse".into(),
            description: "Trouvez 'ronin' quelle que soit la casse.".into(),
            story: "Le capitaine des gardes: « Cherchez 'ronin' dans auth.log, peu importe
majuscules ou minuscules (Ronin, RONIN...). »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes approuve: « Toutes les mentions de 'ronin' sont trouvées, Seigneur, quelle que soit la casse. Rien ne nous échappe. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Cherchez 'ronin' dans auth.log insensible à la casse.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment chercher sans tenir compte des majuscules?".into(),
            required_commands: vec!["grep -i ronin /scribes/logs/auth.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-i".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "grep -i".into(), name: "grep -i".into(), description: "grep -i — insensible à la casse".into() }),
        },
        Mission {
            id: "7.5".into(), chapter: 7, chapter_title: "Lire les registres".into(), act: 2,
            title: "Deux registres en une commande".into(),
            description: "Comptez les lignes de auth.log ET syslog.".into(),
            story: "Dame Kaguya: « Seigneur, comptez les lignes de auth.log ET de syslog
en une seule commande. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya note les totaux: « Les deux registres sont comptés, Seigneur. Le domaine est sous contrôle. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Comptez les lignes des deux fichiers de logs.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment donner deux fichiers à une commande?".into(),
            required_commands: vec!["wc -l /scribes/logs/auth.log /scribes/logs/syslog".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "wc".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "wc".into(), name: "wc".into(), description: "wc — compte lignes, mots et caractères".into() }),
        },
    ]
}

//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 9 — Enquête: les espions (Acte 3) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "9.1".into(), chapter: 9, chapter_title: "Enquete: les espions".into(), act: 3,
            title: "Lire l'alerte".into(),
            description: "Une alerte ronin a été levée.".into(),
            story: "Le capitaine des gardes: « Seigneur, une alerte a été rédigée aux scribes.
Lisez /scribes/alertes/alerte_ronin.txt. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes lit l'alerte: « L'alerte ronin est confirmée, Seigneur. Nous savons à quoi nous attendre. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Lisez le fichier alerte_ronin.txt.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier d'alerte?".into(),
            required_commands: vec!["cat /scribes/alertes/alerte_ronin.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "alerte_ronin.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "9.2".into(), chapter: 9, chapter_title: "Enquete: les espions".into(), act: 3,
            title: "Tous les registres".into(),
            description: "Listez tous les fichiers 'registre*' du château.".into(),
            story: "Dame Kaguya: « Seigneur, trouvez TOUS les registres du château
(fichiers commençant par 'registre'). »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya recense les registres: « Tous les registres du château sont retrouvés, Seigneur. Rien n'a été oublié. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Listez tous les fichiers dont le nom commence par 'registre'.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment chercher avec un motif?".into(),
            required_commands: vec!["find / -name 'registre*'".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "find".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-name".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "find".into(), name: "find".into(), description: "find -name — motif".into() }),
        },
        Mission {
            id: "9.3".into(), chapter: 9, chapter_title: "Enquete: les espions".into(), act: 3,
            title: "Journal des gardes complet".into(),
            description: "Lisez tout le journal des gardes.".into(),
            story: "Au donjon: « Seigneur, lisez le journal des gardes EN ENTIER:
/donjon/gardes/journal_gardes.txt. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Au donjon, le journal est lu en entier: « Chaque ronde est consignée, Seigneur. La garde n'a rien caché. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Lisez tout le journal des gardes.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier entier?".into(),
            required_commands: vec!["cat /donjon/gardes/journal_gardes.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "journal_gardes.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "9.4".into(), chapter: 9, chapter_title: "Enquete: les espions".into(), act: 3,
            title: "Registre des visiteurs".into(),
            description: "Qui est entré au château aujourd'hui?".into(),
            story: "Au portail: « Seigneur, lisez le registre des visiteurs:
/portail/registre_visiteurs.txt. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Au portail, le registre est consulté: « Tous les visiteurs sont notés, Seigneur. Nous saurons qui est entré. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Lisez le registre des visiteurs du portail.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire le registre du portail?".into(),
            required_commands: vec!["cat /portail/registre_visiteurs.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "registre_visiteurs.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "9.5".into(), chapter: 9, chapter_title: "Enquete: les espions".into(), act: 3,
            title: "Taille du journal".into(),
            description: "Combien de lignes dans auth.log au total?".into(),
            story: "Dame Kaguya: « Seigneur, combien de lignes fait auth.log au total? »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya note le total: « Voilà la taille d'auth.log, Seigneur. L'enquête avance. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Comptez toutes les lignes de auth.log.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment compter toutes les lignes?".into(),
            required_commands: vec!["wc -l /scribes/logs/auth.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "wc".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "wc -l".into(), name: "wc -l".into(), description: "wc -l — total des lignes".into() }),
        },
    ]
}

//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 14 — Comptes et accès (Acte 4) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "14.1".into(), chapter: 14, chapter_title: "Comptes et acces".into(), act: 4,
            title: "Mot de passe du samouraï".into(),
            description: "Changez le mot de passe de samourai2.".into(),
            story: "Seigneur Hattori: « Seigneur, changez le mot de passe de samourai2
pour sécuriser son compte. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Seigneur Hattori approuve: « Le mot de passe de samourai2 est changé, Seigneur. Son compte est sécurisé. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Changez le mot de passe de samourai2 (passwd).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment changer un mot de passe?".into(),
            required_commands: vec!["sudo passwd samourai2".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "passwd".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "passwd".into(), name: "passwd".into(), description: "passwd — change un mot de passe".into() }),
        },
        Mission {
            id: "14.2".into(), chapter: 14, chapter_title: "Comptes et acces".into(), act: 4,
            title: "Session du samouraï".into(),
            description: "Testez le compte samourai2.".into(),
            story: "Le capitaine des gardes: « Seigneur, prenez la session de samourai2
et dites qui il est. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes confirme: « C'est bien samourai2, Seigneur. La session est ouverte et vérifiée. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Ouvrez une session samourai2 et affichez qui il est (su - ... -c whoami).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment exécuter une commande en tant qu'autre utilisateur?".into(),
            required_commands: vec!["su - samourai2 -c whoami".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "su".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-c".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "su -c".into(), name: "su -c".into(), description: "su - user -c cmd — session temporaire".into() }),
        },
        Mission {
            id: "14.3".into(), chapter: 14, chapter_title: "Comptes et acces".into(), act: 4,
            title: "Vos pouvoirs".into(),
            description: "Quelles commandes sudo pouvez-vous lancer?".into(),
            story: "Dame Kaguya: « Seigneur, pour l'audit, listez vos droits sudo. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya consulte vos droits: « Vos privilèges sudo sont listés, Seigneur. L'audit est en ordre. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Listez les commandes que vous êtes autorisé à exécuter avec les pouvoirs du seigneur.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment lister ses droits sudo?".into(),
            required_commands: vec!["sudo -l".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "sudo".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "sudo -l".into(), name: "sudo -l".into(), description: "sudo -l — droits sudo".into() }),
        },
        Mission {
            id: "14.4".into(), chapter: 14, chapter_title: "Comptes et acces".into(), act: 4,
            title: "Clé maître".into(),
            description: "Prouvez que sudo vous reconnaît comme seigneur.".into(),
            story: "Le capitaine des gardes: « Seigneur, prouvez que vous avez la clé maître:
exécutez whoami en sudo. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes salue: « La clé maître fonctionne, Seigneur. Vous avez bien les pleins pouvoirs. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Prouvez que vous disposez des pouvoirs du seigneur en exécutant une commande d'identité.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment exécuter une commande en root?".into(),
            required_commands: vec!["sudo whoami".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "sudo".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "sudo".into(), name: "sudo".into(), description: "sudo — exécute en root".into() }),
        },
        Mission {
            id: "14.5".into(), chapter: 14, chapter_title: "Comptes et acces".into(), act: 4,
            title: "Transférer un registre".into(),
            description: "Le registre des impôts appartient au seigneur.".into(),
            story: "Pour l'audit, le registre des impôts doit vous appartenir.
« Transférez /salle-du-trone/registres/registre_impots.txt à votre compte. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya confirme: « Le registre des impôts vous appartient, Seigneur. L'audit sera sans reproche. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Changez le propriétaire du registre des impôts (chown).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment changer le propriétaire d'un fichier?".into(),
            required_commands: vec!["sudo chown $USER /salle-du-trone/registres/registre_impots.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chown".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chown".into(), name: "chown".into(), description: "chown — change le propriétaire".into() }),
        },
    ]
}

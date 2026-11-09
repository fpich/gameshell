//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 19 — Durcissement final (Acte 5) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "19.1".into(), chapter: 19, chapter_title: "Durcissement final".into(), act: 5,
            title: "Config SSH du château".into(),
            description: "Lisez la configuration SSH des scribes.".into(),
            story: "Hanzō: « Seigneur, avant de durcir, lisez la config SSH du château:
/scribes/registres/sshd_config. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Hanzō lit la config: « Voilà la configuration SSH du château, Seigneur. Nous savons quoi durcir. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Lisez le fichier sshd_config des scribes.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire la config SSH?".into(),
            required_commands: vec!["cat /scribes/registres/sshd_config".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "sshd_config".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "19.2".into(), chapter: 19, chapter_title: "Durcissement final".into(), act: 5,
            title: "Interdire root en SSH".into(),
            description: "Ajoutez 'PermitRootLogin no' à sshd_config.".into(),
            story: "Hanzō: « Seigneur, la config autorise le login root: c'est une faille.
Ajoutez 'PermitRootLogin no' à la FIN de sshd_config. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Hanzō approuve: « PermitRootLogin no est ajouté, Seigneur. La faille est colmatée. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Ajoutez 'PermitRootLogin no' à la fin de sshd_config.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter une ligne de config?".into(),
            required_commands: vec!["echo 'PermitRootLogin no' >> /scribes/registres/sshd_config".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }, MissionCheck { check_type: "config_contains".into(), target: "/scribes/registres/sshd_config".into(), expected: Some("PermitRootLogin no".into()) }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: ">>".into(), name: ">>".into(), description: ">> — ajoute la config".into() }),
        },
        Mission {
            id: "19.3".into(), chapter: 19, chapter_title: "Durcissement final".into(), act: 5,
            title: "Caméra sur le coffre".into(),
            description: "Surveillez les accès au coffre-fort.".into(),
            story: "Le capitaine des gardes: « Seigneur, installez une caméra d'audit sur
les écritures du coffre-fort: /donjon/coffre-fort. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes confirme: « La caméra d'audit surveille le coffre-fort, Seigneur. Chaque écriture sera tracée. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Installez une règle d'audit sur /donjon/coffre-fort (auditctl -w).".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Comment surveiller un dossier sensible?".into(),
            required_commands: vec!["sudo auditctl -w /donjon/coffre-fort -p wa -k sceaux".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "auditctl".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-w".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            skill: Some(Skill { command: "auditctl -w".into(), name: "auditctl -w".into(), description: "auditctl -w — caméra d'audit".into() }),
        },
        Mission {
            id: "19.4".into(), chapter: 19, chapter_title: "Durcissement final".into(), act: 5,
            title: "Isoler un prisonnier".into(),
            description: "Enfermez un processus dans une cellule (chroot).".into(),
            story: "Hanzō: « Seigneur, isolez le prisonnier dans une cellule d'isolement:
chroot /donjon/prison /bin/bash. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Hanzō approuve: « Le prisonnier est isolé dans sa cellule, Seigneur. Il ne peut plus nuire. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Isolez un prisonnier dans la cellule du donjon en lançant un environnement confiné.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Comment isoler un processus dans un chroot?".into(),
            required_commands: vec!["sudo chroot /donjon/prison /bin/bash".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chroot".into(), expected: None }, MissionCheck { check_type: "dir_exists".into(), target: "/donjon/prison".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            skill: Some(Skill { command: "chroot".into(), name: "chroot".into(), description: "chroot — isole dans une cellule".into() }),
        },
        Mission {
            id: "19.5".into(), chapter: 19, chapter_title: "Durcissement final".into(), act: 5,
            title: "Mise à jour du domaine".into(),
            description: "Mettez à jour les paquets du château (apt update).".into(),
            story: "Dame Kaguya: « Seigneur, avant le siège, mettons à jour les paquets
du domaine: sudo apt update. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya confirme: « Les paquets du domaine sont à jour, Seigneur. Le château est prêt pour le siège. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Mettez à jour la liste des paquets (sudo apt update).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment mettre à jour les paquets?".into(),
            required_commands: vec!["sudo apt update".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "apt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "update".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "apt update".into(), name: "apt update".into(), description: "apt update — met à jour les paquets".into() }),
        },
    ]
}

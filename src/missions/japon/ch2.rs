//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 2 — Organisation des quartiers (Acte 1) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "2.1".into(), chapter: 2, chapter_title: "Organisation des quartiers".into(), act: 1,
            title: "Nouveau sceau".into(),
            description: "Créez un fichier vierge pour un nouveau sceau.".into(),
            story: "Dame Kaguya: « Seigneur, créez un fichier sceau_du_soir.txt sur votre
bureau pour consigner les ordres de ce soir. C'est la première étape
du registre quotidien. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Dame Kaguya approuve: « Le sceau du soir est créé, Seigneur. Le registre quotidien commence. Chaque ordre sera consigné. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Créez le fichier sceau_du_soir.txt dans vos quartiers.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer un fichier vide?".into(),
            required_commands: vec!["touch sceau_du_soir.txt".into(), "ls -la".into()],
            checks: vec![MissionCheck { check_type: "file_exists".into(), target: "~/sceau_du_soir.txt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-la".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "touch".into(), name: "touch".into(), description: "touch — crée un fichier vide".into() }),
        },
        Mission {
            id: "2.2".into(), chapter: 2, chapter_title: "Organisation des quartiers".into(), act: 1,
            title: "Classer les archives".into(),
            description: "Créez l'arborescence archives/2024/lettres.".into(),
            story: "Dame Kaguya veut un classement strict: archives/AAAA/type/.
« Créez archives/2024/lettres dans vos quartiers, Seigneur. Une seule
commande avec l'option qui crée tous les parents. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Dame Kaguya vérifie l'arborescence: « archives/2024/lettres est en place, Seigneur. Un classement strict, c'est un domaine bien tenu. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Créez l'arborescence archives/2024/lettres dans vos quartiers.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer une arborescence complète en une commande?".into(),
            required_commands: vec!["mkdir -p archives/2024/lettres".into()],
            checks: vec![MissionCheck { check_type: "dir_exists".into(), target: "~/archives/2024/lettres".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-p".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "mkdir -p".into(), name: "mkdir -p".into(), description: "mkdir -p — crée toute l'arborescence".into() }),
        },
        Mission {
            id: "2.3".into(), chapter: 2, chapter_title: "Organisation des quartiers".into(), act: 1,
            title: "Copier la lettre aux archives".into(),
            description: "Gardez une copie de la lettre Hattori dans les archives.".into(),
            story: "Dame Kaguya: « Seigneur, copiez la lettre du clan Hattori dans
/archives/2024/lettres/ pour la conserver sûrement. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya confirme: « La lettre du clan Hattori est en sûreté dans les archives, Seigneur. Rien ne se perdra. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Copiez lettre_clan_hattori.txt vers /archives/2024/lettres/.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment copier un fichier vers un autre dossier?".into(),
            required_commands: vec!["mkdir -p /archives/2024/lettres".into(), "cp /archives/lettres/lettre_clan_hattori.txt /archives/2024/lettres/".into()],
            checks: vec![MissionCheck { check_type: "file_exists".into(), target: "/archives/2024/lettres/lettre_clan_hattori.txt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "cp".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "cp".into(), name: "cp".into(), description: "cp — copie un fichier".into() }),
        },
        Mission {
            id: "2.4".into(), chapter: 2, chapter_title: "Organisation des quartiers".into(), act: 1,
            title: "Renommer un document".into(),
            description: "Renommez sceau_du_soir.txt en ordre_du_soir.txt.".into(),
            story: "Dame Kaguya: « Seigneur, le fichier sceau_du_soir.txt s'appellera plutôt
ordre_du_soir.txt. Renommez-le. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya approuve: « ordre_du_soir.txt, c'est plus clair, Seigneur. Vos quartiers sont bien tenus. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Renommez sceau_du_soir.txt en ordre_du_soir.txt.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment renommer un fichier?".into(),
            required_commands: vec!["mv sceau_du_soir.txt ordre_du_soir.txt".into()],
            checks: vec![MissionCheck { check_type: "file_exists".into(), target: "~/ordre_du_soir.txt".into(), expected: None }, MissionCheck { check_type: "file_not_exists".into(), target: "~/sceau_du_soir.txt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "mv".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "mv".into(), name: "mv".into(), description: "mv — déplace ou renomme".into() }),
        },
        Mission {
            id: "2.5".into(), chapter: 2, chapter_title: "Organisation des quartiers".into(), act: 1,
            title: "Faire le ménage".into(),
            description: "Supprimez les brouillons inutiles.".into(),
            story: "Dame Kaguya: « Seigneur, vos quartiers sont en désordre. Supprimez
les brouillons: brouillon1.txt, brouillon2.txt. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya sourit: « Vos quartiers sont enfin ordonnés, Seigneur. Un seigneur rangé est un seigneur qui voit clair. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Supprimez les fichiers brouillon de vos quartiers.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment supprimer un fichier?".into(),
            required_commands: vec!["touch brouillon1.txt brouillon2.txt".into(), "rm brouillon1.txt brouillon2.txt".into()],
            checks: vec![MissionCheck { check_type: "file_not_exists".into(), target: "~/brouillon1.txt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "rm".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "rm".into(), name: "rm".into(), description: "rm — supprime un fichier".into() }),
        },
    ]
}

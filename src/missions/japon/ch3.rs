//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 3 — Exploration du domaine (Acte 1) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "3.1".into(), chapter: 3, chapter_title: "Exploration du domaine".into(), act: 1,
            title: "Fichiers cachés des quartiers".into(),
            description: "Le sceau secret est un fichier caché.".into(),
            story: "Le moine Tetsuo vous souffle: « Seigneur, certains documents sont cachés
(d'un point). Listez TOUT vos quartiers, fichiers cachés compris, puis lisez
le fichier .secret. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Le moine Tetsuo hoche la tête: « Vous avez trouvé le fichier caché, Seigneur. Rien n'échappe à un œil attentif. Le domaine vous est transparent. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Listez tous vos quartiers (fichiers cachés compris) et lisez .secret.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir aussi les fichiers cachés?".into(),
            required_commands: vec!["ls -la".into(), "cat .secret".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-la".into(), expected: None }, MissionCheck { check_type: "file_read".into(), target: ".secret".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ls".into(), name: "ls".into(), description: "ls — liste les fichiers et dossiers".into() }),
        },
        Mission {
            id: "3.2".into(), chapter: 3, chapter_title: "Exploration du domaine".into(), act: 1,
            title: "Trouver un registre".into(),
            description: "Trouvez registre_riz.txt dans tout le château.".into(),
            story: "Dame Kaguya a égaré le registre du riz: « Seigneur, trouvez où se cache
registre_riz.txt dans le château. Fouillez partout. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya est rassurée: « Le registre du riz est retrouvé, Seigneur. Grâce à vous, le domaine ne perd plus rien. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Trouvez le chemin complet de registre_riz.txt.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment chercher un fichier par nom partout?".into(),
            required_commands: vec!["find / -name registre_riz.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "find".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-name".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "find".into(), name: "find".into(), description: "find -name — recherche par nom".into() }),
        },
        Mission {
            id: "3.3".into(), chapter: 3, chapter_title: "Exploration du domaine".into(), act: 1,
            title: "Poids des greniers".into(),
            description: "Quelle taille occupent les greniers?".into(),
            story: "Le marchand Genji: « Seigneur, avant de vendre le riz, dites-moi combien
occupent les greniers au total. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le marchand Genji note: « Voilà la taille des greniers, Seigneur. Nous pourrons négocier le riz en connaissance de cause. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Donnez la taille totale du bâtiment /greniers.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment connaître la taille totale d'un dossier?".into(),
            required_commands: vec!["du -sh /greniers".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "du".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-sh".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "du -sh".into(), name: "du -sh".into(), description: "du -sh — taille totale lisible".into() }),
        },
        Mission {
            id: "3.4".into(), chapter: 3, chapter_title: "Exploration du domaine".into(), act: 1,
            title: "Monter au donjon".into(),
            description: "Rejoignez le donjon pour le conseil de guerre.".into(),
            story: "Seigneur Hattori vous convoque au donjon: « Seigneur! Conseil de guerre
au DONJON immédiatement. Les espions du clan rival approchent. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Seigneur Hattori vous accueille au donjon: « Vous voilà, Seigneur. Le conseil de guerre peut commencer. Les espions approchent, mais nous sommes prêts. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Rejoignez le donjon (/donjon).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment aller au donjon en chemin absolu?".into(),
            required_commands: vec!["cd /donjon".into()],
            checks: vec![MissionCheck { check_type: "dir_reached".into(), target: "/donjon".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cd".into(), name: "cd".into(), description: "cd — change de répertoire".into() }),
        },
        Mission {
            id: "3.5".into(), chapter: 3, chapter_title: "Exploration du domaine".into(), act: 1,
            title: "Ordre du soir".into(),
            description: "Écrivez l'ordre du soir dans un fichier.".into(),
            story: "Dame Kaguya: « Seigneur, notez dans ordre.txt sur votre bureau:
'Fermer le portail à la nuit tombée'. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya relit ordre.txt: « 'Fermer le portail à la nuit tombée'. C'est noté, Seigneur. Vos ordres seront exécutés. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Créez ordre.txt dans vos quartiers avec le texte demandé.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment écrire un texte directement dans un fichier?".into(),
            required_commands: vec!["echo 'Fermer le portail a la nuit tombee' > ordre.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "~/ordre.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "echo".into(), name: "echo".into(), description: "echo — affiche du texte à l'écran".into() }),
        },
    ]
}

//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 1 — Arrivee au chateau (Acte 1) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "1.1".into(), chapter: 1, chapter_title: "Arrivee au chateau".into(), act: 1,
            title: "Premier contact avec Shiro".into(),
            description: "Vous arrivez au château Shiro. Dame Kaguya confirme votre position.".into(),
            story: "Le portail du château Shiro se referme derrière vous avec un grincement.
Vous êtes le nouveau seigneur du domaine, installé dans vos quartiers (indiqués
par le symbole ~).

Dame Kaguya, votre intendante, vous parle par le porte-voix:
« Seigneur? Vous m'entendez? Dites-moi où vous êtes exactement dans le
château, qu'on synchronise nos registres! »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

Dame Kaguya hoche la tête, soulagée: « Parfait, Seigneur. Vos quartiers sont bien enregistrés. Les registres sont synchronisés. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Montrez à Dame Kaguya où vous êtes dans le château et ce qui vous entoure.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelle commande affiche le répertoire courant?".into(),
            required_commands: vec!["pwd".into(), "ls".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "pwd".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "ls".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            skill: Some(Skill { command: "pwd".into(), name: "pwd".into(), description: "pwd — affiche le répertoire courant".into() }),
        },
        Mission {
            id: "1.2".into(), chapter: 1, chapter_title: "Arrivee au chateau".into(), act: 1,
            title: "Le kaisho des quartiers".into(),
            description: "Lisez le kaisho (guide) posé dans vos quartiers.".into(),
            story: "Dame Kaguya poursuit: « Seigneur, prenez le temps de lire le kaisho
de vos quartiers. Il explique le fonctionnement du domaine. »

Sur votre bureau, un document: kaisho.txt, le manuel rédigé par le
seigneur précédent. Il contient les bases: comment naviguer dans le
château, qui contacter, les consignes de sécurité.

« Lisez-le attentivement. Le château a déjà connu des sièges. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

Dame Kaguya approuve: « Bien lu, Seigneur. Vous connaissez désormais les bases du domaine. Le château a déjà connu des sièges, mais un seigneur informé ne tombe jamais. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Lisez le guide kaisho.txt de vos quartiers.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier texte?".into(),
            required_commands: vec!["cat kaisho.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "kaisho.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: Some("SHIRO-ACCUEIL".into()),
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche le contenu d'un fichier".into() }),
        },
        Mission {
            id: "1.3".into(), chapter: 1, chapter_title: "Arrivee au chateau".into(), act: 1,
            title: "Lettre du clan Hattori".into(),
            description: "Le clan Hattori a déposé une lettre dans les archives.".into(),
            story: "Le marchand Genji signale: « Seigneur! Une lettre du clan Hattori
vient d'arriver aux archives! Nos alliés nous proposent des renforts. »

Le document se trouve dans /archives/lettres/lettre_clan_hattori.txt.

« Allez la lire, Seigneur. Nos défenses en dépendent. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le marchand Genji sourit: « Excellente nouvelle, Seigneur! Le clan Hattori nous propose des renforts. Nos défenses se renforcent. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Allez lire la lettre du clan Hattori dans /archives/lettres/.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment aller dans un autre bâtiment et lire un fichier?".into(),
            required_commands: vec!["cd /archives/lettres".into(), "cat lettre_clan_hattori.txt".into()],
            checks: vec![MissionCheck { check_type: "dir_reached".into(), target: "/archives/lettres".into(), expected: None }, MissionCheck { check_type: "file_read".into(), target: "lettre_clan_hattori.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: Some("HATTORI-20".into()),
            skill: Some(Skill { command: "cd".into(), name: "cd".into(), description: "cd — change de répertoire".into() }),
        },
        Mission {
            id: "1.4".into(), chapter: 1, chapter_title: "Arrivee au chateau".into(), act: 1,
            title: "Qui êtes-vous, Seigneur?".into(),
            description: "Le garde en chef vérifie votre identité.".into(),
            story: "Le garde en chef, au portail, vous interpelle: « Seigneur! Avant que je
vous ouvre le donjon, je dois vérifier votre identité sur le système. »

Le château utilise un système d'authentification: chaque membre a un
badge (uid) et appartient à des groupes qui déterminent ses droits.

Prouvez qui vous êtes: nom du château, votre nom d'utilisateur,
vos identifiants complets (uid, gid, groupes).

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

Le garde en chef salue: « Identité confirmée, Seigneur. Le donjon vous est ouvert. Bienvenue dans votre domaine. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Prouvez votre identité au garde en chef : nom du château, votre nom d'utilisateur et vos identifiants complets.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelles commandes donnent hostname/user/uid?".into(),
            required_commands: vec!["hostname".into(), "whoami".into(), "id".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "hostname".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "whoami".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "id".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "id".into(), name: "id".into(), description: "id — affiche uid, gid et groupes".into() }),
        },
        Mission {
            id: "1.5".into(), chapter: 1, chapter_title: "Arrivee au chateau".into(), act: 1,
            title: "L'heure du château".into(),
            description: "Synchronisez l'horloge du beffroi pour les registres.".into(),
            story: "Dame Kaguya: « Seigneur, une dernière vérification. Pour corréler les
événements dans les registres de garde, l'horloge du beffroi doit être
exacte. Vérifiez qu'elle est bien réglée. »

« Donnez-moi l'heure système, Seigneur. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Dame Kaguya vérifie l'horloge: « L'heure est exacte, Seigneur. Les registres de garde seront désormais fiables. Le domaine est prêt. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez la date et l'heure du système du château.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelle commande donne la date et l'heure?".into(),
            required_commands: vec!["date".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "date".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            skill: Some(Skill { command: "date".into(), name: "date".into(), description: "date — affiche la date et l'heure".into() }),
        },
    ]
}

//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 10 — Reseau du domaine (Acte 3) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "10.1".into(), chapter: 10, chapter_title: "Reseau du domaine".into(), act: 3,
            title: "Coordonnées du château".into(),
            description: "Configuration réseau complète de Shiro.".into(),
            story: "Hanzō: « Seigneur, quelles sont les coordonnées réseau du château?
Toutes les interfaces, toutes les IP. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Hanzō examine les interfaces: « Toutes les coordonnées réseau du château sont là, Seigneur. Nous savons où nous sommes. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez la configuration réseau complète (ip addr).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les adresses IP?".into(),
            required_commands: vec!["ip addr".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ip".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "addr".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip addr".into(), name: "ip addr".into(), description: "ip addr — interfaces et IP".into() }),
        },
        Mission {
            id: "10.2".into(), chapter: 10, chapter_title: "Reseau du domaine".into(), act: 3,
            title: "Routes du domaine".into(),
            description: "Par où passe le trafic vers le clan Hattori?".into(),
            story: "Hanzō: « Seigneur, par quelle route passe le trafic vers nos alliés?
Montrez la table de routage. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Hanzō suit la route: « Voilà le chemin vers nos alliés, Seigneur. Le trafic est bien orienté. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez la table de routage (ip route).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les routes réseau?".into(),
            required_commands: vec!["ip route".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "route".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip route".into(), name: "ip route".into(), description: "ip route — table de routage".into() }),
        },
        Mission {
            id: "10.3".into(), chapter: 10, chapter_title: "Reseau du domaine".into(), act: 3,
            title: "Ports ouverts".into(),
            description: "Quels ports du château sont en écoute?".into(),
            story: "Le capitaine des gardes: « Seigneur, quels ports sont OUVERTS sur le
château? Connexions actives et ports en écoute. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes inspecte les ports: « Voilà les portes ouvertes du château, Seigneur. Nous saurons quoi surveiller. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez les ports en écoute (ss -tlnp).".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les ports en écoute?".into(),
            required_commands: vec!["ss -tlnp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ss".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-tlnp".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ss -tlnp".into(), name: "ss -tlnp".into(), description: "ss -tlnp — ports en écoute".into() }),
        },
        Mission {
            id: "10.4".into(), chapter: 10, chapter_title: "Reseau du domaine".into(), act: 3,
            title: "L'IP espion répond-il?".into(),
            description: "Pingez l'IP 10.0.0.66.".into(),
            story: "Hanzō: « Seigneur, l'IP espion 10.0.0.66 a été détectée. Envoyez-lui
4 pings pour voir s'il répond. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Hanzō observe les réponses: « L'IP espion répond aux pings, Seigneur. Elle est bien là. Nous la tenons. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Envoyez 4 pings vers 10.0.0.66.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment tester la connectivité vers une IP?".into(),
            required_commands: vec!["ping -c 4 10.0.0.66".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ping".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-c".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ping -c".into(), name: "ping -c".into(), description: "ping -c N — N paquets".into() }),
        },
        Mission {
            id: "10.5".into(), chapter: 10, chapter_title: "Reseau du domaine".into(), act: 3,
            title: "Offrandes de la pagode".into(),
            description: "Lisez le fichier de la pagode.".into(),
            story: "Le moine Tetsuo: « Seigneur, pour le rapport, lisez les offrandes du
jour à la pagode: /pagode/autel/offrandes.txt. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le moine Tetsuo lit les offrandes: « Les offrandes du jour sont consignées, Seigneur. La pagode veille sur le domaine. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Lisez le fichier offrandes.txt de la pagode.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier de la pagode?".into(),
            required_commands: vec!["cat /pagode/autel/offrandes.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "offrandes.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
    ]
}

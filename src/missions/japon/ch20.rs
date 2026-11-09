//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 20 — Le siège final (Acte 5) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "20.1".into(), chapter: 20, chapter_title: "Le siege final".into(), act: 5,
            title: "Sceau du seigneur".into(),
            description: "Prouvez que vous connaissez le sceau secret (fichier caché .secret).".into(),
            story: "Seigneur Hattori: « Seigneur, avant de nous envoyer 20 samouraïs, prouvez
que vous êtes le seigneur de Shiro: donnez-moi le SCEAU SECRET (fichier
caché .secret de vos quartiers). »

Trouvez le sceau, puis validez la mission avec ce code.

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Seigneur Hattori vérifie le sceau: « Le sceau secret est authentique, Seigneur. Vous êtes bien le seigneur de Shiro. Les 20 samouraïs arrivent. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Trouvez le sceau secret dans .secret et validez avec ce code.".into(),
            bloom_level: "Évaluer".into(),
            bloom_question: "Où se trouve le sceau secret?".into(),
            required_commands: vec!["cat .secret".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: ".secret".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: Some("LACUS-7-ONI".into()),
            skill: Some(Skill { command: "cat .secret".into(), name: "cat .secret".into(), description: "cat .secret — lit le sceau caché".into() }),
        },
        Mission {
            id: "20.2".into(), chapter: 20, chapter_title: "Le siege final".into(), act: 5,
            title: "Code du donjon".into(),
            description: "Validez avec le code du coffre-fort.".into(),
            story: "Le capitaine des gardes: « Seigneur, le donjon est verrouillé. Entrez
le CODE DU COFFRE-FORT (dans /donjon/coffre-fort/sceaux.txt et
/archives/secret/code_coffre.txt) pour ouvrir la salle de guerre. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Le capitaine des gardes ouvre la salle de guerre: « Le code est correct, Seigneur. La salle de guerre est ouverte. Le conseil peut siéger. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Trouvez le code du coffre-fort et validez avec ce code.".into(),
            bloom_level: "Évaluer".into(),
            bloom_question: "Où se trouve le code du coffre-fort?".into(),
            required_commands: vec!["cat /donjon/coffre-fort/sceaux.txt".into(), "cat /archives/secret/code_coffre.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "sceaux.txt".into(), expected: None }, MissionCheck { check_type: "file_read".into(), target: "code_coffre.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: Some("KAMI-442-TORA".into()),
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — lit les codes".into() }),
        },
        Mission {
            id: "20.3".into(), chapter: 20, chapter_title: "Le siege final".into(), act: 5,
            title: "Rapport de siège".into(),
            description: "Rédigez le rapport de siège complet.".into(),
            story: "Dame Kaguya: « Seigneur, rédigez le rapport de siège dans rapport_siege.txt:
les attaques 'Failed' du jour, puis l'état des remparts, à la suite. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Dame Kaguya lit rapport_siege.txt: « Les attaques puis l'état des remparts, à la suite, Seigneur. Le rapport de siège est complet. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Créez rapport_siege.txt avec les attaques puis l'état des remparts.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Comment composer un rapport multi-sources?".into(),
            required_commands: vec!["grep Failed /scribes/logs/auth.log > rapport_siege.txt".into(), "cat /remparts/etat_rembarts.txt >> rapport_siege.txt".into()],
            checks: vec![MissionCheck { check_type: "file_exists".into(), target: "~/rapport_siege.txt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: ">".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: ">".into(), name: ">".into(), description: "> — compose le rapport".into() }),
        },
        Mission {
            id: "20.4".into(), chapter: 20, chapter_title: "Le siege final".into(), act: 5,
            title: "Archiver les preuves".into(),
            description: "Archivez tous les logs dans les archives.".into(),
            story: "Dame Kaguya: « Seigneur, archivez TOUS les logs des scribes dans
/archives/preuves_siege.tar.gz pour le procès des espions. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya approuve: « Tous les logs sont archivés, Seigneur. Les preuves du siège sont en sûreté pour le procès. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Créez /archives/preuves_siege.tar.gz contenant /scribes/logs/.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Comment archiver toutes les preuves?".into(),
            required_commands: vec!["tar czf /archives/preuves_siege.tar.gz /scribes/logs/".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tar".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "/archives/preuves_siege.tar.gz".into(), expected: None }],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: "tar".into(), name: "tar".into(), description: "tar — crée et lit des archives".into() }),
        },
        Mission {
            id: "20.5".into(), chapter: 20, chapter_title: "Le siege final".into(), act: 5,
            title: "Protocole de siège final".into(),
            description: "Dernière épreuve: validez avec le protocole de siège du coffre-fort.".into(),
            story: "Seigneur Hattori: « Seigneur, dernière épreuve: donnez-moi le PROTOCOLE
DE SIÈGE du coffre-fort (/donjon/coffre-fort/sceaux.txt) pour lancer la
contre-attaque décisive. »

Lisez les sceaux, installez une caméra sur les archives secrètes, puis validez avec ce code.

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Seigneur Hattori salue: « Le protocole de siège est confirmé, Seigneur. La contre-attaque décisive peut commencer. Shiro ne tombera pas! »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Lisez les sceaux, installez une caméra d'audit sur /archives/secret, validez avec le protocole de siège.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Quel est le protocole de siège?".into(),
            required_commands: vec!["cat /donjon/coffre-fort/sceaux.txt".into(), "sudo auditctl -w /archives/secret -p wa -k sceau_final".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "sceaux.txt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "auditctl".into(), expected: None }],
            hints: vec![],
            xp_reward: 500,
            code_required: Some("TORA-442-KAMI".into()),
            skill: Some(Skill { command: "auditctl -w".into(), name: "auditctl -w".into(), description: "auditctl -w — caméra finale".into() }),
        },
    ]
}

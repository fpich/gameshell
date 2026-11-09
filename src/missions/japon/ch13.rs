//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 13 — Recrutement (Acte 4) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "13.1".into(), chapter: 13, chapter_title: "Recrutement".into(), act: 4,
            title: "Recruter un samouraï".into(),
            description: "Créez le compte du nouveau samouraï 'samourai2'.".into(),
            story: "Seigneur Hattori envoie un renfort: « Seigneur, créez le compte du
samouraï 'samourai2' avec son quartier (-m). »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Seigneur Hattori approuve: « Le compte de samourai2 est créé, avec son quartier, Seigneur. Le renfort est prêt. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Créez le compte samourai2 (useradd -m) et affichez sa fiche.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer un utilisateur avec son home?".into(),
            required_commands: vec!["sudo useradd -m samourai2".into(), "id samourai2".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "useradd".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-m".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "id".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "useradd -m".into(), name: "useradd -m".into(), description: "useradd -m — crée un utilisateur avec home".into() }),
        },
        Mission {
            id: "13.2".into(), chapter: 13, chapter_title: "Recrutement".into(), act: 4,
            title: "Créer une garnison".into(),
            description: "Créez le groupe 'garnison'.".into(),
            story: "Le capitaine des gardes: « Seigneur, créez le groupe 'garnison' pour
les nouvelles recrues. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes confirme: « Le groupe garnison est créé, Seigneur. Les recrues ont un foyer. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Créez le groupe garnison (groupadd).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer un groupe?".into(),
            required_commands: vec!["sudo groupadd garnison".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "groupadd".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "groupadd".into(), name: "groupadd".into(), description: "groupadd — crée un groupe".into() }),
        },
        Mission {
            id: "13.3".into(), chapter: 13, chapter_title: "Recrutement".into(), act: 4,
            title: "Affecter le samouraï".into(),
            description: "Ajoutez samourai2 au groupe garnison.".into(),
            story: "Le capitaine des gardes: « Seigneur, ajoutez samourai2 à la garnison,
sans retirer ses autres groupes. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes vérifie: « samourai2 est dans la garnison, sans perdre ses autres groupes, Seigneur. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Ajoutez samourai2 à garnison (usermod -aG).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter un utilisateur à un groupe?".into(),
            required_commands: vec!["sudo usermod -aG garnison samourai2".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "usermod".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-aG".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "usermod -aG".into(), name: "usermod -aG".into(), description: "usermod -aG — ajoute à un groupe".into() }),
        },
        Mission {
            id: "13.4".into(), chapter: 13, chapter_title: "Recrutement".into(), act: 4,
            title: "Vos groupes".into(),
            description: "À quels groupes appartenez-vous?".into(),
            story: "Dame Kaguya: « Seigneur, pour le contrôle d'accès du donjon, vérifiez
vos propres groupes. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Dame Kaguya note vos groupes: « Vos accès au donjon sont confirmés, Seigneur. Le contrôle d'accès est en ordre. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez vos groupes (groups).".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Comment voir ses groupes?".into(),
            required_commands: vec!["groups".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "groups".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "groups".into(), name: "groups".into(), description: "groups — groupes de l'utilisateur".into() }),
        },
        Mission {
            id: "13.5".into(), chapter: 13, chapter_title: "Recrutement".into(), act: 4,
            title: "Fiche du samouraï".into(),
            description: "Identité complète de samourai2.".into(),
            story: "Le capitaine des gardes: « Seigneur, montrez la fiche complète de
samourai2: uid, gid, groupes. »

Les espions du clan rival rôdent autour du château Shiro. Chaque seconde compte. Le seigneur Hattori attend votre rapport.

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.

N'oubliez pas: un domaine bien administré ne tombe jamais. La préparation fait la différence entre la victoire et le siège.

Le moine Tetsuo médite à la pagode et prie pour la paix du domaine.

Les gardes surveillent le portail jour et nuit. Un ronin a été aperçu près des remparts est.

Le capitaine des gardes consulte la fiche: « uid, gid, groupes... tout est là, Seigneur. samourai2 est bien enregistré. »

Le château Shiro est perché sur sa colline, entouré de remparts. Cent âmes vivent dans le domaine et comptent sur vous, Seigneur.".into(),
            goal: "Affichez l'identité complète de samourai2 (id samourai2).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'identité d'un autre utilisateur?".into(),
            required_commands: vec!["id samourai2".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "id".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "id".into(), name: "id".into(), description: "id — identité d'un utilisateur".into() }),
        },
    ]
}

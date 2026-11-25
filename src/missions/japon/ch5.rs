//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 5 — Permissions et sceaux (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "5.1".into(), chapter: 5, chapter_title: "Permissions et sceaux".into(), act: 2,
            title: "Verrouiller le plan de défense".into(),
            description: "plan_defense.sh ne doit être lisible que par vous: 700.".into(),
            story: "Le capitaine des gardes entre dans vos quartiers, de la poussière sur l'épaule: « Seigneur, les ordres de bataille! »

Il pose vos notes sur le bureau: « Le plan de défense, plan_defense.sh, contient la marche à suivre en cas d'attaque: fermer le portail, sonner le gong, placer les archers. Si un espion le lit, il connaîtra nos faiblesses avant la bataille. »

Il désigne le fichier: « Pour l'instant, tout le monde peut le lire dans le domaine. Verrouillez-le: vous seul devez pouvoir le lire et le modifier. Passez le fichier à 700. »

Vous songez aux coffres du donjon: les documents les plus précieux n'ont qu'une seule clé, celle du seigneur.

Vous prenez le plan de défense et resserrez ses permissions jusqu'à ce que le fichier 700 ne parle plus qu'à vous.

Le capitaine vérifie d'un geste: « plan_defense.sh est verrouillé, Seigneur. Seul vous pouvez le lire. Les ordres de bataille sont en sûreté. »".into(),
            goal: "Mettez les permissions 700 sur plan_defense.sh.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment restreindre un fichier à soi seul?".into(),
            required_commands: vec!["chmod 700 plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "permission".into(), target: "~/plan_defense.sh".into(), expected: Some("700".into()) }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chmod".into(), name: "chmod".into(), description: "chmod — change les permissions".into() }),
        },
        Mission {
            id: "5.2".into(), chapter: 5, chapter_title: "Permissions et sceaux".into(), act: 2,
            title: "Rendre le plan exécutable".into(),
            description: "Le plan de défense doit être lançable: +x.".into(),
            story: "Le capitaine des gardes revient, plus pressé que la veille: « Seigneur, bonnes nouvelles: les archers sont prêts, les remparts sont solides. Il ne manque qu'un détail au plan de défense. »

Il montre plan_defense.sh: « Pour lancer le plan en pleine alerte, il doit pouvoir être exécuté. Donnez-lui le droit d'exécution, pour qu'on puisse le déclencher d'un geste quand le gong sonnera. »

Vous songez aux ordres du conseil: un plan qui ne peut pas être lancé ne sert à rien le jour où l'ennemi attaque.

Vous ajoutez le droit d'exécution au plan de défense, afin que le capitaine puisse le déclencher aussitôt.

Le capitaine hoche la tête: « Le plan de défense est exécutable, Seigneur. En cas d'urgence, nous pourrons le lancer sans attendre. »".into(),
            goal: "Rendez plan_defense.sh exécutable.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment rendre un script exécutable?".into(),
            required_commands: vec!["chmod +x plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "+x".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chmod +x".into(), name: "chmod +x".into(), description: "chmod +x — droit d'exécution".into() }),
        },
        Mission {
            id: "5.3".into(), chapter: 5, chapter_title: "Permissions et sceaux".into(), act: 2,
            title: "Fiche du plan de défense".into(),
            description: "Métadonnées complètes de plan_defense.sh.".into(),
            story: "Dame Kaguya vous attend au pupitre des archives, un registre ouvert: « Seigneur, l'audit du domaine commence demain. Pour chaque document sensible, je dois consigner sa fiche technique: la taille, les permissions, les dates de modification. »

Elle désigne vos quartiers: « Commencez par le plan de défense, plan_defense.sh. Montrez-moi la fiche complète de ce fichier: toutes ses métadonnées, sans rien omettre. »

Vous savez que derrière un document se cache une fiche plus discrète: ce qu'il pèse, qui peut le toucher, quand il a été modifié.

Vous allez chercher le plan de défense et en déroulez la fiche technique complète pour la scribe.

Dame Kaguya consulte la fiche: « Taille, permissions, dates... tout y est, Seigneur. Le plan de défense est parfaitement documenté. »".into(),
            goal: "Affichez les métadonnées de plan_defense.sh.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir toutes les métadonnées?".into(),
            required_commands: vec!["stat plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "stat".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "stat".into(), name: "stat".into(), description: "stat — métadonnées complètes".into() }),
        },
        Mission {
            id: "5.4".into(), chapter: 5, chapter_title: "Permissions et sceaux".into(), act: 2,
            title: "Transférer le sceau".into(),
            description: "Le plan appartient officiellement au seigneur.".into(),
            story: "Un messager du donjon apporte un rouleau scellé: l'audit du domaine.

Dame Kaguya le déroule et fronce les sourcils: « Seigneur, le plan de défense, plan_defense.sh, porte encore la marque de son ancien ordre: celle de la garde. D'après l'auditeur, un document de bataille doit appartenir au seigneur lui-même. »

Elle repose le rouleau: « Transférez la propriété du plan de défense à votre compte, pour que l'audit soit sans reproche. »

Vous songez aux sceaux des édits: un acte n'a de valeur que s'il porte le sceau du bon maître. Le plan de bataille doit porter le vôtre.

Vous prenez la propriété du plan de défense et la remettez en votre nom, officiellement.

Dame Kaguya relit le rapport et sourit: « plan_defense.sh vous appartient officiellement, Seigneur. L'audit sera sans reproche. »".into(),
            goal: "Changez le propriétaire de plan_defense.sh pour votre compte.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment changer le propriétaire?".into(),
            required_commands: vec!["sudo chown $USER plan_defense.sh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chown".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chown".into(), name: "chown".into(), description: "chown — change le propriétaire".into() }),
        },
        Mission {
            id: "5.5".into(), chapter: 5, chapter_title: "Permissions et sceaux".into(), act: 2,
            title: "Détail des quartiers".into(),
            description: "Liste détaillée avec permissions.".into(),
            story: "Dame Kaguya se tient sur le seuil de vos quartiers, une tablette à la main: « Seigneur, pour clore l'audit, je dois vérifier l'état de vos quartiers: qui possède chaque document, qui peut le lire, sa taille, sa date. »

Elle parcourt la pièce du regard: « Montrez-moi le contenu de vos quartiers en détail: pour chaque fichier, les permissions, le propriétaire, la taille et la date. Une liste complète, pas un simple aperçu. »

Vous songez aux inventaires du domaine: on ne contrôle bien une salle qu'en regardant chaque objet un par un.

Vous parcourez vos quartiers, examinez chaque document et chaque dossier, puis présentez la liste détaillée à Dame Kaguya.

Dame Kaguya inspecte la liste: « Permissions, tailles, dates... tout est en ordre dans vos quartiers, Seigneur. L'audit est clos. »".into(),
            goal: "Montrez à Dame Kaguya le contenu de vos quartiers en format détaillé: permissions, propriétaire, taille et date de chaque élément.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les permissions de chaque fichier?".into(),
            required_commands: vec!["ls -l".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ls -l".into(), name: "ls -l".into(), description: "ls -l — liste détaillée avec permissions".into() }),
        },
    ]
}

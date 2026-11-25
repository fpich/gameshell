//! CHAPITRE 16 - Gestion de l'equipage (Acte 4) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "16.1".into(), chapter: 16, chapter_title: "Gestion de l'equipage".into(), act: 4,
            title: "Vos equipes".into(),
            description: "Vex Korshun veut savoir à quelles équipes appartient le capitaine.".into(),
            story: "Vex Korshun, l'officier de sécurité, contrôle les accès du pont :
« Capitaine, avant d'ouvrir les coursives au nouvel arrivant, je dois
vérifier à quelles équipes de bord votre badge est rattaché. »

Il pointe sa console : « Chaque équipe regroupe des membres qui ont les
mêmes droits. Montrez-moi la liste des équipes auxquelles votre badge
appartient. Je dois savoir où vous êtes inscrit. »

Vous consultez votre badge : la liste de vos équipes s'affiche. Vous
êtes membre de plusieurs équipes du vaisseau.

« Chaque groupe définit vos droits », explique Vex Korshun. « Savoir à
quelles équipes vous appartenez, c'est connaître votre place à bord. »".into(),
            goal: "Affichez la liste des groupes (équipes) auxquels appartient votre compte.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Comment voir ses propres groupes?".into(),
            required_commands: vec!["groups".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "groups".into(), expected: None }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "groups".into(), name: "groups".into(), description: "groups — affiche les groupes de l'utilisateur courant".into() }),
        },
        Mission {
            id: "16.2".into(), chapter: 16, chapter_title: "Gestion de l'equipage".into(), act: 4,
            title: "Fiche du matelot".into(),
            description: "Vérifiez le badge complet du nouveau matelot.".into(),
            story: "Vex Korshun consulte la liste des nouveaux arrivants :
« Capitaine, le compte du matelot a été créé. Avant de lui confier un
badge d'accès, vérifions sa fiche complète : son numéro d'identification
(uid), son groupe principal (gid) et toutes les équipes dont il est
membre. »

Il se tourne vers vous : « Affichez la fiche complète du compte matelot.
Je dois voir son identité numérique exacte pour valider son badge. »

Vous affichez la fiche : l'uid, le gid et les groupes du compte matelot
s'y trouvent, prêts à être contrôlés par l'officier.

« Chaque membre a une identité numérique unique », explique Vex Korshun.
« C'est elle qui détermine ses accès. »".into(),
            goal: "Affichez l'identité complète (uid, gid, groupes) du compte matelot.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'identité complète d'un autre utilisateur?".into(),
            required_commands: vec!["id matelot".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "id".into(), expected: None }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "id user".into(), name: "id".into(), description: "id — affiche uid, gid et groupes d'un utilisateur".into() }),
        },
        Mission {
            id: "16.3".into(), chapter: 16, chapter_title: "Gestion de l'equipage".into(), act: 4,
            title: "Ajouter le matelot a l'equipe".into(),
            description: "Le matelot rejoint l'équipe 'equipage' du vaisseau.".into(),
            story: "Vex Korshun vous montre un plan des coursives communes :
« Capitaine, le matelot est inscrit à bord, mais il n'appartient encore
à aucune équipe. Sans équipe, il n'aura pas accès aux coursives et au
mess partagés par l'équipage. »

Il désigne le plan : « Ajoutez le compte matelot à l'équipe equipage.
Vous pouvez l'ajouter sans toucher aux équipes qu'il pourrait avoir
déjà. »

Votre commande terminée, le compte matelot est inscrit dans l'équipe
equipage. Ses droits d'accès aux espaces communs sont mis à jour.

« Bien », approuve Vex Korshun. « Le matelot peut maintenant accéder aux
ressources de son équipe. »".into(),
            goal: "Ajoutez le compte matelot au groupe equipage.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter un utilisateur à un groupe?".into(),
            required_commands: vec!["sudo usermod -aG equipage matelot".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "usermod".into(), expected: None }, MissionCheck { check_type: "user_in_group".into(), target: "matelot".into(), expected: Some("equipage".into()) }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "usermod -aG".into(), name: "usermod".into(), description: "usermod -aG — ajoute un utilisateur à un groupe".into() }),
        },
        Mission {
            id: "16.4".into(), chapter: 16, chapter_title: "Gestion de l'equipage".into(), act: 4,
            title: "Prendre le fauteuil du matelot".into(),
            description: "Vérifiez que le compte matelot fonctionne en prenant sa session.".into(),
            story: "Vex Korshun revient vers vous, un badge de test à la main :
« Capitaine, le compte du matelot est en ordre sur le papier. Il reste à
prouver qu'il fonctionne vraiment : qu'une session peut s'ouvrir avec
cette identité. »

Il vous tend le badge : « Ouvrez une session temporaire du compte
matelot et affichez le nom d'utilisateur qu'elle reconnaît. Si la
réponse est matelot, tout est bon. »

Vous ouvrez la session : l'identité affichée confirme le compte matelot.
Le badge fonctionne comme prévu.

« Le compte fonctionne », note Vex Korshun. « Tester une session, c'est
vérifier que tout est en ordre. »".into(),
            goal: "Ouvrez une session temporaire du compte matelot et affichez son nom d'utilisateur.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment exécuter une commande en tant qu'autre utilisateur?".into(),
            required_commands: vec!["su - matelot -c whoami".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "su".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "su - user -c cmd".into(), name: "su".into(), description: "su - user -c cmd — exécute une commande en tant qu'autre utilisateur".into() }),
        },
        Mission {
            id: "16.5".into(), chapter: 16, chapter_title: "Gestion de l'equipage".into(), act: 4,
            title: "Vos pouvoirs de capitaine".into(),
            description: "Quelles commandes sudo le capitaine peut-il lancer?".into(),
            story: "Vex Korshun ferme la porte du poste de sécurité derrière lui :
« Capitaine, nous faisons l'audit des pouvoirs à bord. Votre badge de
capitaine donne accès à la clé maîtresse (sudo) : je dois savoir
exactement ce qu'elle vous autorise à lancer. »

Il pose son bloc : « Montrez-moi la liste des pouvoirs que votre clé
maîtresse vous permet d'exercer. Cette liste nous dit ce qu'un
administrateur peut faire sur le vaisseau. »

La liste de vos pouvoirs s'affiche : les commandes que vous êtes
autorisé à exécuter avec la clé maîtresse y figurent, prêtes pour
l'audit.

« Ces pouvoirs sont une responsabilité », rappelle Vex Korshun.
« Utilisez-les avec discernement. »".into(),
            goal: "Montrez à Vex Korshun les commandes que vous êtes autorisé à exécuter avec les pouvoirs du capitaine.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment lister ses propres droits sudo?".into(),
            required_commands: vec!["sudo -l".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "sudo".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "sudo".into(), name: "sudo".into(), description: "sudo — exécute une commande en tant que root (clé maître du capitaine)".into() }),
        },
    ]
}

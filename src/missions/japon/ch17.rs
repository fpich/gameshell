//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 17 — Pare-feu du château (Acte 5) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "17.1".into(), chapter: 17, chapter_title: "Pare-feu du chateau".into(), act: 5,
            title: "État des déflecteurs".into(),
            description: "État du pare-feu (ufw) en détaillé.".into(),
            story: "Le capitaine des gardes: « Seigneur, avant d'engager la
chasse aux espions, je veux savoir où en est le pare-feu
du château: est-il en service? Quelle conduite tient-il
face aux entrées du domaine? Montrez-moi son état complet,
avec tout le détail. »

Le pare-feu est le premier rempart du domaine: chaque
règle posée hier décide de ce qui entre demain. Les
sentinelles ont signalé des allées et venues suspectes
sur les routes qui mènent au château; avant de lever les
ponts, il faut lire l'état des déflecteurs.

Le capitaine ne demande pas un simple avis: il veut le
détail des réglages, la politique appliquée aux
connexions entrantes, ce qui est consigné dans les
registres de veille. Un pare-feu dont on ignore le détail
est un mur dont on ignore les brèches.

Vérifiez l'état complet et détaillé du pare-feu, avec
les droits d'administrateur. Quand le capitaine aura le
tableau exact sous les yeux, il décidera des prochaines
règles en connaissance de cause.".into(),
            goal: "Avant de bloquer les espions, vérifiez avec les droits d'administrateur l'état complet et détaillé du pare-feu du château.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'état du pare-feu?".into(),
            required_commands: vec!["sudo ufw status verbose".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ufw".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "status".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "verbose".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ufw status".into(), name: "ufw status".into(), description: "ufw status — état du pare-feu".into() }),
        },
        Mission {
            id: "17.2".into(), chapter: 17, chapter_title: "Pare-feu du chateau".into(), act: 5,
            title: "Bloquer l'IP espion".into(),
            description: "Bloquez 10.0.0.66 au pare-feu.".into(),
            story: "Le capitaine des gardes: « Seigneur, une adresse hostile,
10.0.0.66, martèle notre porte sans relâche. Tant qu'elle
peut frapper, notre canal d'échange est en danger.
Fermez-lui la porte au pare-feu: qu'aucun trafic venu
d'elle ne soit plus jamais admis. »

Les éclaireurs ont reconnu la signature: c'est l'espion
qui guette les échanges du château depuis des jours. Il
sonde, il attend, il tente d'entrer par le passage
chiffré. Chaque tentative renseigne celui qui cherche nos
faiblesses.

Le pare-feu peut refuser net tout ce qui vient de cette
adresse: aucune entrée, aucun échange, aucun détour
possible. La règle doit être ferme et durable, pas une
mesure d'un soir.

Interdisez définitivement à 10.0.0.66 tout accès au
château. Quand le pare-feu la repoussera, le capitaine
pourra dire aux gardes que la porte est enfin close.".into(),
            goal: "Interdisez définitivement au pare-feu tout trafic en provenance de l'espion 10.0.0.66.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment bloquer une IP au pare-feu?".into(),
            required_commands: vec!["sudo ufw deny from 10.0.0.66".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ufw".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "deny".into(), expected: None }, MissionCheck { check_type: "sim_contains".into(), target: "ufw.rules".into(), expected: Some("DENY&&from 10.0.0.66".into()) }],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "ufw deny from".into(), name: "ufw deny from".into(), description: "ufw deny from IP — bloque une IP".into() }),
        },
        Mission {
            id: "17.3".into(), chapter: 17, chapter_title: "Pare-feu du chateau".into(), act: 5,
            title: "Règles du noyau".into(),
            description: "Listez les règles iptables.".into(),
            story: "Le capitaine des gardes: « Seigneur, le pare-feu n'est
pas seul en jeu: sous lui, le noyau du système filtre
chaque trame qui entre ou sort. Montrez-moi ces règles
de fond, telles qu'elles sont écrites, sans habillage.
J'ai besoin de voir chaque adresse à l'état brut. »

Le capitaine lit deux lignes de défense: les déflecteurs
du pare-feu, que vous venez de vérifier, puis les tables
du noyau, plus anciennes, où chaque passage est jugé.
Si une règle de fond interdit ou laisse passer par
erreur, elle décide avant toutes les autres.

Il ne veut pas de noms de machines complaisants sur la
liste: l'adresse qui frappe doit apparaître telle quelle,
chiffre après chiffre, pour qu'il la reconnaisse sans
ambiguïté quand elle se présentera.

Listez les règles de filtrage du noyau, à l'état brut.
Quand la liste sera déroulée devant lui, le capitaine
comparera chaque ligne à ce qu'il attend et achèvera la
mise en défense du domaine.".into(),
            goal: "Listez les règles de filtrage du pare-feu du château.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment lister les règles du noyau?".into(),
            required_commands: vec!["sudo iptables -L -n".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "iptables".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-L".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-n".into(), expected: None }],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "iptables -L".into(), name: "iptables -L".into(), description: "iptables -L -n — règles du noyau".into() }),
        },
        Mission {
            id: "17.4".into(), chapter: 17, chapter_title: "Pare-feu du chateau".into(), act: 5,
            title: "Autoriser le SSH".into(),
            description: "Autorisez le port 22 au pare-feu.".into(),
            story: "Dame Kaguya: « Seigneur, les déflecteurs repoussent
l'espion, mais un allié doit pouvoir nous joindre: le
clan Hattori envoie ses messagers par le passage chiffré.
Ouvrez-lui la porte 22 du pare-feu, pour que sa parole
arrive jusqu'à nous sans être écoutée en chemin. »

Les messagers du clan allié portent des nouvelles que
l'ennemi guette: accords, mouvements, dates. Le passage
chiffré garantit que seul le château comprend le message,
mais encore faut-il que le pare-feu l'accueille.

Fermer l'entrée à l'espion ne doit pas fermer le domaine
aux amis: la défense n'a de sens que si les alliés
peuvent entrer par la porte qu'on leur ouvre.

Chaque porte du pare-feu se décide: fermer celle-ci
reviendrait à laisser les messagers à la merci des
guetteurs. Le chiffre protège la parole, le pare-feu
protège le passage: les deux doivent s'accorder.

Autorisez le passage chiffré sur le port 22 du pare-feu.
Quand la porte sera ouverte, les messagers du clan Hattori
pourront approcher le château sans risque.".into(),
            goal: "Ouvrez le port 22 au pare-feu pour que le clan Hattori puisse joindre le château par le canal chiffré.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment autoriser un port au pare-feu?".into(),
            required_commands: vec!["sudo ufw allow 22".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "allow".into(), expected: None }, MissionCheck { check_type: "sim_contains".into(), target: "ufw.rules".into(), expected: Some("ALLOW IN 22".into()) }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ufw allow".into(), name: "ufw allow".into(), description: "ufw allow 22 — autorise le SSH".into() }),
        },
        Mission {
            id: "17.5".into(), chapter: 17, chapter_title: "Pare-feu du chateau".into(), act: 5,
            title: "Activer le pare-feu".into(),
            description: "Activez le pare-feu du château.".into(),
            story: "Le capitaine des gardes: « Seigneur, toutes les règles
sont posées: l'espion repoussé, la porte des alliés
ouverte. Mais tant que le pare-feu n'est pas mis en
marche, ces règles restent de l'encre sur un registre:
rien n'est encore appliqué. Mettez-le en service,
maintenant. »

Vous l'avez vu au premier regard: le pare-feu n'était
pas actif. Les règles, si justes soient-elles, n'agissent
que lorsque la garde est réveillée et prend son poste.
L'heure est venue de lever la consigne.

Une fois le pare-feu en marche, chaque entrée sera jugée
dès la première trame: l'espion repoussé avant d'atteindre
le château, l'allié accueilli sur son passage.

Mettez le pare-feu du château en service, avec les droits
d'administrateur. Quand il sera actif, les gardes pourront
veiller en paix: le domaine est clos, et ses portes
connues.".into(),
            goal: "Les règles sont prêtes: mettez le pare-feu du château en service, avec les droits d'administrateur.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment activer le pare-feu?".into(),
            required_commands: vec!["sudo ufw enable".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "enable".into(), expected: None }, MissionCheck { check_type: "sim_contains".into(), target: "ufw.conf".into(), expected: Some("enabled=yes".into()) }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ufw enable".into(), name: "ufw enable".into(), description: "ufw enable — active le pare-feu".into() }),
        },
    ]
}

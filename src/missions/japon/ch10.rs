//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 10 — Reseau du domaine (Acte 3) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "10.1".into(), chapter: 10, chapter_title: "Reseau du domaine".into(), act: 3,
            title: "Coordonnées du château".into(),
            description: "Configuration réseau complète de Shiro.".into(),
            story: "Hanzō vous attend au quartier des renseignements, le visage fermé: « Seigneur, le château échange des messages avec d'autres machines par un réseau invisible. Pour savoir qui est connecté et comment, il me faut les coordonnées complètes du château: toutes les interfaces réseau et toutes les adresses qui y sont rattachées. »

Il déroule un parchemin vierge: « Le conseil veut un état précis avant la nuit. Montrez-moi rien que la configuration réseau de Shiro, dans son intégralité. »

Vous pensez aux remparts: comme les portes d'entrée d'une forteresse, chaque interface est une ouverture vers l'extérieur. Il faut toutes les voir.

Le moine Tetsuo, qui descendait l'escalier, s'arrête: « Connaître ses propres portes, Seigneur, c'est la première défense contre ceux qui les cherchent. »

Hanzō pose son pinceau. Il attend votre rapport: quelles interfaces le château possède-t-il, et quelles adresses portent-elles?".into(),
            goal: "Affichez la configuration réseau complète du château: interfaces et adresses IP.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les adresses IP?".into(),
            required_commands: vec!["ip addr".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ip".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "addr".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip addr".into(), name: "ip addr".into(), description: "ip addr — interfaces et IP".into() }),
        },
        Mission {
            id: "10.2".into(), chapter: 10, chapter_title: "Reseau du domaine".into(), act: 3,
            title: "Routes du domaine".into(),
            description: "Par où passe le trafic vers le clan Hattori?".into(),
            story: "Hanzō revient vers vous, un message du clan Hattori à la main: « Seigneur, nos alliés nous ont écrit. Le rouleau a voyagé à travers plusieurs machines avant d'arriver. Pour comprendre par où passent nos messages, je dois voir la table de routage du château. »

Il trace une carte sommaire: « Il y a le domaine local, et au-delà la passerelle qui mène vers les autres clans. Montrez-moi comment Shiro choisit ses routes: vers quoi part le trafic, et par quelle voie. »

Vous songez aux sentiers qui quittent le château: certains mènent aux champs voisins, d'autres rejoignent la grand-route. La table de routage est cette carte, en caractères précis.

Le moine Tetsuo vous croise dans le couloir: « Un messager qui connaît ses chemins ne se perd jamais, Seigneur. »

Hanzō aligne ses pinceaux. Il veut voir, d'un coup d'œil, toutes les routes que le château connaît: montrez-les-lui.".into(),
            goal: "Affichez la table de routage du château: par où sortent les messages vers les autres clans?".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les routes réseau?".into(),
            required_commands: vec!["ip route".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "route".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip route".into(), name: "ip route".into(), description: "ip route — table de routage".into() }),
        },
        Mission {
            id: "10.3".into(), chapter: 10, chapter_title: "Reseau du domaine".into(), act: 3,
            title: "Ports ouverts".into(),
            description: "Quels ports du château sont en écoute?".into(),
            story: "Le capitaine des gardes entre dans la salle du conseil, les sourcils froncés: « Seigneur, le château a des portes que personne n'a ordonné d'ouvrir: des services qui écoutent en permanence, prêts à recevoir des visiteurs. »

Il pose la pointe de sa lance au sol: « Avant de verrouiller quoi que ce soit, il faut savoir lesquelles sont ouvertes. Dressez-moi la liste des services du château qui écoutent, avec un affichage numérique pour que je puisse compter, et le nom de la sentinelle qui monte la garde sur chacun. »

Vous sentez le danger: chaque service en écoute est une porte qui peut s'ouvrir de l'extérieur. Et parmi elles, une seule devrait suffire à alerter.

Dame Kaguya, qui observait depuis la porte, murmure: « Nous avons peut-être ouvert une porte à quelqu'un qui n'aurait pas dû entrer, Seigneur. »

Le capitaine attend, sa lance prête. Il faut lui montrer les portes ouvertes du château, en chiffres, avec leur gardien.".into(),
            goal: "Dressez pour le capitaine la liste des ports TCP en écoute sur le château, en affichage numérique, avec le processus qui tient chacun.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les ports en écoute?".into(),
            required_commands: vec!["ss -tlnp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ss".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-tlnp".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ss -tlnp".into(), name: "ss -tlnp".into(), description: "ss -tlnp — ports en écoute".into() }),
        },
        Mission {
            id: "10.4".into(), chapter: 10, chapter_title: "Reseau du domaine".into(), act: 3,
            title: "L'IP espion répond-il?".into(),
            description: "Pingez l'IP 10.0.0.66.".into(),
            story: "Hanzō surgit dans vos quartiers, un parchemin déchiré à la main: « Seigneur! Les scribes ont relevé des messages venant de l'adresse 10.0.0.66, et cette machine n'est jamais annoncée dans aucun registre du domaine. »

Il le déplie devant vous: « C'est une adresse étrangère, peut-être une machine espionne postée près de nos remparts électroniques. Avant de lancer quoi que ce soit contre elle, il faut savoir si elle est vivante, si elle écoute, si elle répond. »

Vous comprenez l'enjeu: une machine muette peut n'être qu'un fantôme dans les registres; une machine qui répond est une présence réelle qui vous observe.

Le moine Tetsuo, assis dans un coin, laisse tomber: « On ne craint que ce qui répond, Seigneur. Un silence n'a jamais reparlé. »

Hanzō vous tend un rouleau: « Interrogez-la, mais avec mesure: envoyez-lui quatre salutations, pas une de plus. Quatre, et quatre seulement. Puis dites-moi ce qu'elle a répondu. »".into(),
            goal: "Envoyez 4 pings vers 10.0.0.66.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment tester la connectivité vers une IP?".into(),
            required_commands: vec!["ping -c 4 10.0.0.66".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ping".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-c".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ping -c".into(), name: "ping -c".into(), description: "ping -c N — N paquets".into() }),
        },
        Mission {
            id: "10.5".into(), chapter: 10, chapter_title: "Reseau du domaine".into(), act: 3,
            title: "Offrandes de la pagode".into(),
            description: "Lisez le fichier de la pagode.".into(),
            story: "Le moine Tetsuo vous accueille au seuil de la pagode, les mains jointes: « Seigneur, le conseil prépare le rapport de la journée, et la pagode doit y figurer. Les offrandes déposées ce matin sur l'autel sont consignées dans le fichier /pagode/autel/offrandes.txt. »

Il incline la tête: « Pendant que la lune monte, je voudrais que vous lisiez ce qui y est écrit, pour que personne ne prétende que la pagode a été oubliée dans le rapport. »

Vous contemplez l'autel: les bols de saké, les fruits, l'encens. Chaque détail de la journée y est consigné, comme une mémoire de pierre.

Dame Kaguya, qui vous a suivi, ajoute doucement: « Même les plus petits gestes du domaine méritent d'être rapportés, Seigneur. »

Tetsuo se retire en silence. La réponse est dans le parchemin de la pagode: lisez-le, et le rapport sera complet.".into(),
            goal: "Lisez le fichier offrandes.txt de la pagode.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier de la pagode?".into(),
            required_commands: vec!["cat /pagode/autel/offrandes.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "offrandes.txt".into(), expected: None }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
    ]
}

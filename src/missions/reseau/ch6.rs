//! Campagne 3 (Nexus Réseau, niveau DIFFICILE) — CHAPITRE 6 — Ports et connexions (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "6.1".into(), chapter: 6, chapter_title: "Ports et connexions".into(), act: 2,
            title: "Ports en écoute".into(),
            description: "Quels ports Nexus écoute?".into(),
            story: "Karl: « Cette nuit, une machine du secteur a tenté de forcer notre serveur SSH. Pour savoir ce qu'elle pouvait viser, il faut d'abord connaître la surface d'attaque du NOC: tous les services qui acceptent une connexion de l'extérieur.

Un port en écoute, c'est une porte ouverte sur cette machine. Chaque porte ouverte est une cible possible. Sans cette liste, nous travaillons à l'aveugle.

L'affichage numérique évite la résolution des noms: chaque adresse reste lisible telle quelle, ce qui la rend comparable aux logs de l'IDS. Sans les numéros de ports, la liste serait inutilisable pour recouper les journaux de la nuit.

Interrogez les sockets du poste: montrez les ports TCP en écoute, en affichage numérique, et surtout le processus qui tient chaque port. La passerelle, le service de courrier et les services web du NOC doivent y figurer.

Nous avons besoin de savoir si l'attaquant a déjà pu atteindre l'un de ces services pendant sa tentative. »".into(),
            goal: "Donnez à Karl la liste des ports TCP en écoute sur le NOC, en affichage numérique, avec le processus qui tient chacun.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les ports en écoute avec les processus?".into(),
            required_commands: vec!["ss -tlnp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ss".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-tlnp".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ss -tlnp".into(), name: "ss -tlnp".into(), description: "ss -tlnp — ports en écoute + processus".into() }),
        },
        Mission {
            id: "6.2".into(), chapter: 6, chapter_title: "Ports et connexions".into(), act: 2,
            title: "Connexions établies".into(),
            description: "Qui est connecté au NOC en ce moment?".into(),
            story: "Karl: « La liste des ports est précisée: rien n'écoute hors de nos services connus. Reste la question immédiate: qui tient actuellement une session ouverte sur le NOC?

Les connexions établies sont des canaux actifs, en cours. Une session ouverte vers une machine que nous ne connaissons pas est la signature typique d'une intrusion en cours: l'attaquant garde un accès pendant que ses outils tournent.

L'équipe netops a relevé 10.0.0.66 dans les logs de la nuit. Je veux savoir si cette machine tient encore une session avec notre poste, maintenant, à l'instant où vous lisez ces lignes.

La table des connexions distingue les états: écoute, établie, en attente. On ne regarde que les sessions réellement ouvertes, pas celles qui attendent une nouvelle connexion. Si une adresse du secteur y apparaît, c'est qu'un dialogue est en cours avec nous: c'est le point à noter.

Affichez uniquement les connexions TCP établies sur le NOC, en affichage numérique, et dites-moi quelles machines dialoguent avec nous. »".into(),
            goal: "Montrez à Karl uniquement les connexions TCP actuellement établies sur le NOC, en affichage numérique.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment filtrer les connexions établies?".into(),
            required_commands: vec!["ss -tn state established".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ss".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "established".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ss -tn".into(), name: "ss -tn".into(), description: "ss -tn — connexions TCP".into() }),
        },
        Mission {
            id: "6.3".into(), chapter: 6, chapter_title: "Ports et connexions".into(), act: 2,
            title: "Ancien outil netstat".into(),
            description: "Vérifiez avec netstat aussi.".into(),
            story: "Rosa: « Les procédures de réponse rédigées l'an dernier utilisent toutes l'ancien outil netstat. Les serveurs de secours ne sont pas encore migrés: seul netstat y est disponible.

Une même liste de ports doit pouvoir être vérifiée par les deux outils, sinon les équipes de nuit ne sauront pas quoi contrôler sur les machines qu'ils n'ont pas mises à jour.

Refaites la liste des ports en écoute de ce poste avec netstat, en affichage numérique et avec le détail des processus. Comparez-la à celle fournie tout à l'heure: si vous voyez un port absent de la première liste, vous saurez que les deux outils ne lisent pas les mêmes sockets et nous devrons creuser.

Affichez les ports en écoute du poste via netstat, avec les processus concernés. »".into(),
            goal: "Affichez les ports en écoute de votre poste avec le détail des processus.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les ports avec netstat?".into(),
            required_commands: vec!["netstat -tlnp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "netstat".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-tlnp".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "netstat -tlnp".into(), name: "netstat -tlnp".into(), description: "netstat -tlnp — ports en écoute (outil classique)".into() }),
        },
        Mission {
            id: "6.4".into(), chapter: 6, chapter_title: "Ports et connexions".into(), act: 2,
            title: "Scan d'un port précis".into(),
            description: "Le port 3306 (mysql) est-il ouvert sur l'attaquant?".into(),
            story: "Karl: « La machine 10.0.0.66 est revenue taper sur notre SSH pendant des heures. Nos sondes passives la montrent comme un serveur ancien: je parie qu'elle expose des services obsolètes, d'une autre époque.

Un service vieillissant, c'est une porte ouverte pour nous: base de données sans correctif, protocole désuet... Si elle abrite une base de données accessible, nous tenons peut-être de quoi comprendre ce qu'elle prépare contre le cluster.

Sondez la machine 10.0.0.66 sur un seul point: son port 3306, celui des bases de données. Je ne veux pas d'un scan général qui attirerait son attention, juste ce port précis: est-il ouvert et quel service répond?

L'outil de scan nomme le port cible et limite son analyse à lui seul. Avec le nom du service qui répond, nous saurons si c'est un serveur de bases de données exposé, et de quel logiciel: de quoi préparer la suite du diagnostic sans alerter la cible.

Rapportez-moi l'état de ce port: ouvert, fermé, et le service annoncé. »".into(),
            goal: "Sondez uniquement le port 3306 de l'attaquant 10.0.0.66 pour savoir si sa base de données est exposée.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment scanner un port précis?".into(),
            required_commands: vec!["nmap -p 3306 10.0.0.66".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "nmap".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-p".into(), expected: None }],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "nmap -p".into(), name: "nmap -p".into(), description: "nmap -p PORT — scan d'un port précis".into() }),
        },
        Mission {
            id: "6.5".into(), chapter: 6, chapter_title: "Ports et connexions".into(), act: 2,
            title: "Chemin vers la cible".into(),
            description: "Par quels sauts passent les paquets vers 10.0.0.66?".into(),
            story: "Rosa: « Le port 3306 est ouvert sur 10.0.0.66, avec un vieux serveur de bases de données qui n'a rien à faire exposé. Voilà une piste.

Pour préparer notre réponse, il faut connaître le chemin que prennent nos paquets pour la rejoindre: quels routeurs sont traversés entre le NOC et cette machine. Chaque routeur entre nous et elle est un point où nous pourrons la couper, ou un point qu'elle peut utiliser contre nous.

Le premier saut doit être notre passerelle, vers le FAI. Ensuite, le trafic emprunte des relais dont nous ignorons encore l'existence: c'est exactement ce que la mission d'aujourd'hui doit révéler.

La trace liste les routeurs un à un, du plus proche au plus lointain, avec le temps de réponse de chacun. si un saut ne répond pas mais que le suivant répond, c'est lui qui fait transiter nos paquets sans se faire voir: un comportement de relais qui vaut d'être noté.

Tracez le chemin des paquets jusqu'à 10.0.0.66 et listez les routeurs traversés. »".into(),
            goal: "Révélez à Rosa chaque routeur traversé par les paquets jusqu'à l'attaquant 10.0.0.66.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment suivre le chemin des paquets?".into(),
            required_commands: vec!["traceroute 10.0.0.66".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "traceroute".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "traceroute".into(), name: "traceroute".into(), description: "traceroute — chemin des paquets (sauts)".into() }),
        },
    ]
}

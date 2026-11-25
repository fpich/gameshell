//! Campagne 3 (Nexus Réseau, niveau DIFFICILE) — CHAPITRE 5 — Diagnostic reseau (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "5.1".into(), chapter: 5, chapter_title: "Diagnostic reseau".into(), act: 2,
            title: "Carte réseau du NOC".into(),
            description: "Configuration réseau complète de Nexus.".into(),
            story: "Rosa: « Netrunner, la nuit a été agitée: des connexions suspectes ont tenté de forcer le serveur SSH du NOC. Tant que nous n'aurons pas une carte exacte du réseau, nous ne saurons pas ce qui a été touché ni par où.

Le NOC est le cœur de Neon-9: quatre baies de serveurs, une passerelle vers le FAI et des postes qui dialoguent sur le réseau local. Avant de relancer la surveillance, je veux l'état complet des interfaces de ce poste: le nom de chacune et l'adresse qu'elle porte.

Ce poste est relié au cluster par sa carte filaire, que le réseau connaît sous le nom d'eth0. La boucle locale, lo, est elle aussi une interface: elle sert aux services internes de la machine.

Affichez le détail des interfaces du NOC avec leurs adresses IP, comme on lit le plan d'un bâtiment avant de le réparer. Une fois la carte dressée, nous verrons comment le trafic sort vers l'extérieur. »".into(),
            goal: "Affichez la configuration réseau complète du NOC: interfaces et adresses IP.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les interfaces et IP?".into(),
            required_commands: vec!["ip addr".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ip".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "addr".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip addr".into(), name: "ip addr".into(), description: "ip addr — interfaces et IP".into() }),
        },
        Mission {
            id: "5.2".into(), chapter: 5, chapter_title: "Diagnostic reseau".into(), act: 2,
            title: "Table de routage".into(),
            description: "Par où passe le trafic vers le FAI?".into(),
            story: "Karl: « La carte des interfaces est bonne. Question suivante: quand un paquet doit quitter le NOC, à quelle machine est-il confié en premier?

Tout le trafic du cluster vers l'extérieur passe par une passerelle unique. Si cette route par défaut est erronée, nos serveurs répondent à des machines qu'on ne contrôle pas, et le FAI ne reçoit plus rien de nous.

La table de routage distingue deux lignes: celle qui mène vers l'extérieur (la route par défaut) et celle qui décrit le réseau local du NOC. Quand vous l'afficherez, dites-moi quelle adresse apparaît comme passerelle de la route par défaut: c'est la machine qui relaie notre trafic vers le FAI.

Rosa veut le chemin utilisé pour sortir du réseau: l'adresse de la passerelle par défaut. Affichez la table qui décrit le routage des paquets, celle où figurent la route par défaut du NOC et le réseau local qui lui est associé.

Quand nous saurons par où part le trafic, nous vérifierons que nos interfaces sont réellement opérationnelles. »".into(),
            goal: "Affichez la table de routage du NOC: par quelle passerelle sort le trafic?".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les routes?".into(),
            required_commands: vec!["ip route".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "route".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip route".into(), name: "ip route".into(), description: "ip route — table de routage".into() }),
        },
        Mission {
            id: "5.3".into(), chapter: 5, chapter_title: "Diagnostic reseau".into(), act: 2,
            title: "Liens réseau".into(),
            description: "État des interfaces physiques.".into(),
            story: "Karl: « La passerelle 10.0.0.1, cohérent avec nos plans. Mais une adresse ne prouve rien: il faut savoir si les interfaces du NOC sont vraiment actives.

La nuit dernière, le commutateur a signalé un lien instable. Si l'interface principale de ce poste était descendue, le cluster serait isolé sans que personne ne le remarque.

La sortie montre pour chaque interface son état (UP ou DOWN), sa vitesse et son adresse matérielle. L'adresse matérielle est inscrite sur la carte elle-même: c'est elle que le commutateur voit, pas l'adresse IP. Si l'une des interfaces était DOWN, les services du NOC deviendraient inaccessibles sans erreur visible sur les applications.

Vérifiez l'état physique des interfaces: la boucle locale et la carte filaire. Est-ce que eth0 est bien active? Affichez l'état de chaque lien réseau, avec son adresse matérielle, pour que je voie la carte répondre ou non.

Si tout est UP, on s'intéressera aux machines vues sur le réseau local pendant l'incident. »".into(),
            goal: "Affichez l'état de chaque interface réseau du NOC (activée ou non, adresse matérielle).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'état des interfaces?".into(),
            required_commands: vec!["ip link".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "link".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip link".into(), name: "ip link".into(), description: "ip link — état des interfaces".into() }),
        },
        Mission {
            id: "5.4".into(), chapter: 5, chapter_title: "Diagnostic reseau".into(), act: 2,
            title: "Voisins ARP".into(),
            description: "Quelles machines sont sur le réseau local?".into(),
            story: "Rosa: « L'interface est active, parfait. Reste une question essentielle: quelles machines ont dialogué avec le NOC sur le réseau local pendant l'attaque?

Un diagnostic ne s'arrête pas aux interfaces: il faut savoir qui est joignable en direct, sans passer par la passerelle. Le poste garde en mémoire les machines qui ont répondu récemment sur le segment local, avec leur adresse.

Chaque ligne de la table correspond à une machine qui a échangé avec nous: son adresse IP, son adresse matérielle et l'état du contact (atteignable ou non). La passerelle 10.0.0.1 doit y figurer, tout comme le poste voisin nexus-02. Toute machine présente en plus est un suspect à noter: l'attaquant a peut-être laissé une adresse dans cette liste.

Vous verrez peut-être aussi 10.0.0.66 et 10.0.0.254, déjà repérés par l'IDS la nuit dernière. Notez-les: elles resserviront pour la suite du diagnostic.

Affichez la table des voisins du réseau local: les adresses IP vues et l'adresse matérielle de chacune. Vous y verrez la passerelle, le poste voisin nexus-02 et d'autres machines qui ont échangé avec nous au moment critique.

Avec cette liste, nous saurons qui était présent sur le LAN pendant la tentative de forçage du serveur SSH. »".into(),
            goal: "Affichez la table des voisins ARP du NOC: quelles machines du réseau local ont été vues?".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les voisins du réseau local?".into(),
            required_commands: vec!["ip neigh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "neigh".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ip neigh".into(), name: "ip neigh".into(), description: "ip neigh — table ARP des voisins".into() }),
        },
        Mission {
            id: "5.5".into(), chapter: 5, chapter_title: "Diagnostic reseau".into(), act: 2,
            title: "Test de liaison vers le FAI".into(),
            description: "Le FAI (10.0.0.254) répond-il?".into(),
            story: "Rosa: « Toutes les briques du diagnostic sont en place. Il reste le lien le plus important: la liaison vers le FAI.

Le fournisseur nous signale des paquets perdus dans la nuit. Son routeur est joignable sur notre réseau à l'adresse 10.0.0.254. Avant de le rappeler, je veux une preuve que ce lien répond, et qu'il répond de façon régulière.

La durée de réponse n'est pas le seul critère: c'est le nombre de réponses qui compte. Si la commande est utilisée sans limite, elle continue indéfiniment et monopolise le poste; c'est pour cela qu'on précise le nombre de paquets à envoyer. Quatre suffisent pour juger la stabilité du lien.

Envoyez exactement 4 paquets de test vers 10.0.0.254. Le compte rendu doit montrer combien de réponses arrivent en retour: si un seul paquet manque, la liaison est instable et le fournisseur est en cause.

Choisissez la commande qui teste la connectivité d'une machine distante, en limitant l'envoi à 4 paquets, et montrez-moi le résultat des réponses. »".into(),
            goal: "Vérifiez que la liaison vers le FAI répond, en envoyant exactement 4 paquets de test vers 10.0.0.254.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment tester la connectivité?".into(),
            required_commands: vec!["ping -c 4 10.0.0.254".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ping".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-c".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ping -c".into(), name: "ping -c".into(), description: "ping -c N — N paquets de test".into() }),
        },
    ]
}

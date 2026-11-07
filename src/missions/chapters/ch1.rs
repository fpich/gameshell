//! CHAPITRE 1 - Arrivee a bord
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "1.1".into(), chapter: 1, chapter_title: "Arrivee a bord".into(), act: 1,
            title: "Premier contact".into(),
            description: "Vous amarrez a Orion-7. Vous êtes dans VOTRE cabine, indiquée par le symbole (~). Marie a l'intercom: Confirmez votre position!".into(),
            story: "Le sas d'amarrage de votre navette se referme avec un grondement métallique.
Vous êtes seul dans votre cabine à bord de la station Orion-7, un vaisseau de classe éclaireur
posté dans le secteur 7G pour surveiller les mouvements de la flotte pirate des Corbeaux Noirs.

La cabine est petite mais fonctionnelle: un hublot donnant sur l'espace étoilé, un bureau avec
quelques documents, et les liens vers les principaux compartiments du vaisseau.

Marie, l'ingénieur système, vous contacte par l'intercom depuis la salle des machines:
« Capitaine? Vous m'entendez? Le transfert depuis le Galactica s'est bien passé?
Dites-moi où vous êtes exactement dans le vaisseau, qu'on puisse synchroniser nos consoles! »

Elle a besoin de savoir votre position exacte pour calibrer les écrans du pont de commandement.
Sans cette information, elle ne peut pas vous donner accès aux systèmes de défense.".into(),
            goal: "Affichez votre position (pwd) et listez les compartiments de votre cabine (ls). Validez avec gsh check.".into(),
            bloom_level: "Remember".into(),
            bloom_question: "Quelle commande affiche le repertoire courant?".into(),
            required_commands: vec!["pwd".into(), "ls -la".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "pwd".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "ls -la".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            map: "VUE DE DESSUS — vous êtes dans votre cabine:\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │▶CABINE◀│ │SOUTES  │\n  └───┬────┘ └───┬────┘ └───┬────┘\n      └──────────┴──────────┘".into(),
            skill: Some(Skill { command: "pwd".into(), name: "pwd".into(), description: "pwd".into() }),
        },
        Mission {
            id: "1.2".into(), chapter: 1, chapter_title: "Arrivee a bord".into(), act: 1,
            title: "Lire le guide".into(),
            description: "Marie: Lisez bienvenue.txt dans votre cabine.".into(),
            story: "Marie poursuit à l'intercom: « Bien reçu pour la position, Capitaine.
Maintenant, prenez le temps de lire le guide de survie de la cabine.
Il explique le fonctionnement de base de la station. »

Sur votre bureau, un fichier texte attire votre attention: bienvenue.txt.
C'est le manuel d'accueil rédigé par le précédent capitaine.

Il contient les informations essentielles: comment naviguer dans les compartiments,
qui contacter en cas de problème, et les consignes de sécurité de base.

Marie insiste: « Lisez-le attentivement, Capitaine. La station a déjà subi des attaques.
Chaque membre d'équipage doit connaître les bases. »

Vous ouvrez le fichier pour en lire le contenu.".into(),
            goal: "Lisez bienvenue.txt de votre cabine.".into(),
            bloom_level: "Understand".into(),
            bloom_question: "Comment lire un fichier?".into(),
            required_commands: vec!["cat bienvenue.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "bienvenue.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine:\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │▶CABINE◀│ │SOUTES  │\n  └───┬────┘ └───┬────┘ └───┬────┘\n      └────► bienvenue.txt sur votre bureau".into(),
            skill: Some(Skill { command: "ls -la".into(), name: "ls -la".into(), description: "ls -la".into() }),
        },
        Mission {
            id: "1.3".into(), chapter: 1, chapter_title: "Arrivee a bord".into(), act: 1,
            title: "Message de l'Amiral".into(),
            description: "Sophie: L'Amiral a envoye un message dans /communications/messages/!".into(),
            story: "Sophie, l'officier des communications, coupe la parole sur l'intercom:
« Capitaine! Urgence! L'Amiral Kowalski du vaisseau-mère Galactica vient de nous envoyer
un message prioritaire! »

Elle a intercepté la transmission sur la fréquence d'urgence 121.5 MHz.
Le message provient du vaisseau amiral, actuellement positionné dans le quadrant voisin.

« Il faut le lire immédiatement, Capitaine. Nos capteurs détectent déjà des signatures hostiles
qui s'approchent du secteur 7G. Les Corbeaux Noirs sont en route. »

Sophie vous indique le chemin: le message est stocké dans le compartiment communications,
sous-compartiment messages, fichier dernier_message.txt.

« Dépêchez-vous, Capitaine. Chaque seconde compte. »".into(),
            goal: "Allez dans /communications/messages/ et lisez dernier_message.txt.".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Comment aller dans un dossier precis?".into(),
            required_commands: vec!["cd /pont".into(), "cat bienvenue.txt".into()],
            checks: vec![MissionCheck { check_type: "dir_reached".into(), target: "/communications/messages".into(), expected: None }, MissionCheck { check_type: "file_read".into(), target: "dernier_message.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            map: "VUE DE DESSUS — objectif: compartiment COMMUNICATIONS:\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶COMMS ◀│\n  └───┬────┘ └───┬────┘ └───┬────┘\n      └─────────┴──► /communications/messages/".into(),
            skill: Some(Skill { command: "cat bienvenue.txt".into(), name: "cat bienvenue.txt".into(), description: "cat bienvenue.txt".into() }),
        },
        Mission {
            id: "1.4".into(), chapter: 1, chapter_title: "Arrivee a bord".into(), act: 1,
            title: "Qui etes-vous?".into(),
            description: "Jean verifie votre identite.".into(),
            story: "Jean, l'officier sécurité, vous interpelle depuis le poste de garde:
« Capitaine? Ici Jean. Avant que je vous ouvre les accès sécurisés du pont,
je dois vérifier votre identité sur le système. »

La station Orion-7 utilise un système d'authentification strict.
Chaque membre d'équipage a un badge numérique (uid) et appartient à des groupes
qui déterminent ses droits d'accès.

Jean vous demande de prouver qui vous êtes:
- Le nom de la station sur laquelle vous travaillez
- Votre nom d'utilisateur sur le système
- Vos identifiants complets (uid, gid, groupes)

« Sans ça, je ne peux pas désactiver les verrouillages des portes du pont de commandement,
confirme Jean. Et avec les pirates qui approchent, on n'a pas de temps à perdre. »".into(),
            goal: "Affichez hostname, whoami, id.".into(),
            bloom_level: "Remember".into(),
            bloom_question: "Quelles commandes donnent hostname/user/groups?".into(),
            required_commands: vec!["hostname".into(), "whoami".into(), "id".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "hostname".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "whoami".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "id".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            map: "VUE DE DESSUS — vous êtes le capitaine:\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │▶ PONT ◀│ │CABINES │ │SOUTES  │\n  └───┬────┘ └───┬────┘ └───┬────┘\n      └──── fauteuil du capitaine".into(),
            skill: Some(Skill { command: "cd /pont".into(), name: "cd /pont".into(), description: "cd /pont".into() }),
        },
        Mission {
            id: "1.5".into(), chapter: 1, chapter_title: "Arrivee a bord".into(), act: 1,
            title: "Heure du vaisseau".into(),
            description: "Verifiez l'heure pour correler les logs.".into(),
            story: "Marie revient à l'intercom: « Capitaine, une dernière vérification avant l'alerte.
Pour corréler les événements dans les journaux de bord, l'horloge de la station doit être exacte.
Vérifiez qu'elle est bien synchronisée. »

Dans l'espace, la synchronisation temporelle est critique.
Si l'horloge de bord est décalée, les logs d'attaque des pirates seront impossibles à corréler
avec les événements réels. L'Amiral Kowalski exige des horodatages fiables pour l'enquête.

« Vérifiez l'heure système, Capitaine. Dites-moi si elle est correcte. »

Vous consultez l'horloge du système de la station.".into(),
            goal: "Affichez date et heure (date).".into(),
            bloom_level: "Remember".into(),
            bloom_question: "Quelle commande donne date/heure?".into(),
            required_commands: vec!["date".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "date".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            map: "VUE DE DESSUS — horloge de la salle de navigation:\n  ┌────────┐ ┌────────┐ ┌────────┐\n  │  PONT  │ │CABINES │ │▶ NAV ◀ │\n  └───┬────┘ └───┬────┘ └───┬────┘\n      └─────────┴──► coordonnees/".into(),
            skill: Some(Skill { command: "hostname".into(), name: "hostname".into(), description: "hostname".into() }),
        },
    ]
}

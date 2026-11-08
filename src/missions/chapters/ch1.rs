//! CHAPITRE 1 - Arrivee a bord
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "1.1".into(), chapter: 1, chapter_title: "Arrivee a bord".into(), act: 1,
            title: "Premier contact".into(),
            description: "Vous amarrez a Orion-7. Vous êtes dans VOTRE cabine, indiquée par le symbole (~). Zyra-7 a l'intercom: Confirmez votre position!".into(),
            story: "Le sas d'amarrage de votre navette se referme avec un grondement métallique.
Vous êtes seul dans votre cabine à bord de la station Orion-7, un vaisseau de classe éclaireur
posté dans le secteur 7G pour surveiller les mouvements de la flotte pirate des Corbeaux Noirs.

La cabine est petite mais fonctionnelle: un hublot donnant sur l'espace étoilé, un bureau avec
quelques documents, et les liens vers les principaux compartiments du vaisseau.

Zyra-7, l'ingénieur système, vous contacte par l'intercom depuis la salle des machines:
« Capitaine? Vous m'entendez? Le transfert depuis l'Andromède s'est bien passé?
Dites-moi où vous êtes exactement dans le vaisseau, qu'on puisse synchroniser nos consoles! »

Elle a besoin de savoir votre position exacte pour calibrer les écrans du pont de commandement.
Sans cette information, elle ne peut pas vous donner accès aux systèmes de défense.".into(),
            goal: "Zyra-7 attend votre réponse à l'intercom. Montrez-lui où vous êtes et ce qui vous entoure.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelle commande affiche le repertoire courant?".into(),
            required_commands: vec!["pwd".into(), "ls -la".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "pwd".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "ls -la".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            map: "VUE DE DESSUS — vous êtes dans votre cabine:\n  ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓\n  ┃    PONT    ┃ ┃  ▶CABINE◀  ┃ ┃   SOUTES   ┃\n  ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛\n       ┻━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┻┛".into(),
            skill: Some(Skill { command: "pwd".into(), name: "pwd".into(), description: "pwd — affiche le répertoire courant".into() }),
        },
        Mission {
            id: "1.2".into(), chapter: 1, chapter_title: "Arrivee a bord".into(), act: 1,
            title: "Lire le guide".into(),
            description: "Zyra-7: Lisez bienvenue.txt dans votre cabine.".into(),
            story: "Zyra-7 poursuit à l'intercom: « Bien reçu pour la position, Capitaine.
Maintenant, prenez le temps de lire le guide de survie de la cabine.
Il explique le fonctionnement de base de la station. »

Sur votre bureau, un fichier texte attire votre attention: bienvenue.txt.
C'est le manuel d'accueil rédigé par le précédent capitaine.

Il contient les informations essentielles: comment naviguer dans les compartiments,
qui contacter en cas de problème, et les consignes de sécurité de base.

Zyra-7 insiste: « Lisez-le attentivement, Capitaine. La station a déjà subi des attaques.
Chaque membre d'équipage doit connaître les bases. »

Vous ouvrez le fichier pour en lire le contenu.".into(),
            goal: "Zyra-7 vous demande de lire le guide de survie posé sur votre bureau avant toute chose.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier?".into(),
            required_commands: vec!["cat bienvenue.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "bienvenue.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            map: "VUE DE DESSUS — votre cabine:\n  ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓\n  ┃    PONT    ┃ ┃  ▶CABINE◀  ┃ ┃   SOUTES   ┃\n  ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛\n       ┻━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┻┛\n       └────► bienvenue.txt sur votre bureau".into(),
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche le contenu complet d'un fichier texte".into() }),
        },
        Mission {
            id: "1.3".into(), chapter: 1, chapter_title: "Arrivee a bord".into(), act: 1,
            title: "Message de l'Amiral".into(),
            description: "Nira Okoro: L'Amiral a envoye un message dans /communications/messages/!".into(),
            story: "Nira Okoro, l'officier des communications, coupe la parole sur l'intercom:
« Capitaine! Urgence! L'Amiral Xylia Vorn du vaisseau-mère Andromède vient de nous envoyer
un message prioritaire! »

Elle a intercepté la transmission sur la fréquence d'urgence 121.5 MHz.
Le message provient du vaisseau amiral, actuellement positionné dans le quadrant voisin.

« Il faut le lire immédiatement, Capitaine. Nos capteurs détectent déjà des signatures hostiles
qui s'approchent du secteur 7G. Les Corbeaux Noirs sont en route. »

Nira Okoro vous indique le chemin: le message est stocké dans le compartiment communications,
sous-compartiment messages, fichier dernier_message.txt.

« Dépêchez-vous, Capitaine. Chaque seconde compte. »".into(),
            goal: "Nira Okoro vous a dit où trouver le message de l'Amiral. Allez le lire et dites-nous ce qu'il contient.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment aller dans un dossier precis?".into(),
            required_commands: vec!["cd /communications/messages".into(), "cat dernier_message.txt".into()],
            checks: vec![MissionCheck { check_type: "dir_reached".into(), target: "/communications/messages".into(), expected: None }, MissionCheck { check_type: "file_read".into(), target: "dernier_message.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            map: "VUE DE DESSUS — objectif: compartiment COMMUNICATIONS:\n  ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓\n  ┃    PONT    ┃ ┃  CABINES   ┃ ┃  ▶COMMS◀   ┃\n  ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛\n       ┻━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┻┛\n       └────► /communications/messages/".into(),
            skill: Some(Skill { command: "cd".into(), name: "cd".into(), description: "cd — change de répertoire (cd /pont, cd .., cd ~)".into() }),
        },
        Mission {
            id: "1.4".into(), chapter: 1, chapter_title: "Arrivee a bord".into(), act: 1,
            title: "Qui etes-vous?".into(),
            description: "Vex Korshun verifie votre identite.".into(),
            story: "Vex Korshun, l'officier sécurité, vous interpelle depuis le poste de garde:
« Capitaine? Ici Vex Korshun. Avant que je vous ouvre les accès sécurisés du pont,
je dois vérifier votre identité sur le système. »

La station Orion-7 utilise un système d'authentification strict.
Chaque membre d'équipage a un badge numérique (uid) et appartient à des groupes
qui déterminent ses droits d'accès.

Vex Korshun vous demande de prouver qui vous êtes:
- Le nom de la station sur laquelle vous travaillez
- Votre nom d'utilisateur sur le système
- Vos identifiants complets (uid, gid, groupes)

« Sans ça, je ne peux pas désactiver les verrouillages des portes du pont de commandement,
confirme Vex Korshun. Et avec les pirates qui approchent, on n'a pas de temps à perdre. »".into(),
            goal: "Vex Korshun ne vous laissera pas monter au pont sans vérifier qui vous êtes. Prouvez votre identité.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelles commandes donnent hostname/user/groups?".into(),
            required_commands: vec!["hostname".into(), "whoami".into(), "id".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "hostname".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "whoami".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "id".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            map: "VUE DE DESSUS — vous êtes le capitaine:\n  ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓\n  ┃   ▶PONT◀   ┃ ┃  CABINES   ┃ ┃   SOUTES   ┃\n  ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛\n       ┻━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┻┛".into(),
            skill: Some(Skill { command: "id".into(), name: "id".into(), description: "id — affiche uid, gid et groupes de l'utilisateur courant".into() }),
        },
        Mission {
            id: "1.5".into(), chapter: 1, chapter_title: "Arrivee a bord".into(), act: 1,
            title: "Heure du vaisseau".into(),
            description: "Verifiez l'heure pour correler les logs.".into(),
            story: "Zyra-7 revient Ã  l'intercom: Â« Capitaine, une derniÃ¨re vÃ©rification avant l'alerte.\nPour corrÃ©ler les Ã©vÃ©nements dans les journaux de bord, l'horloge de la station doit Ãªtre exacte.\nVÃ©rifiez qu'elle est bien synchronisÃ©e. Â»\n\nDans l'espace, la synchronisation temporelle est critique.\nSi l'horloge de bord est dÃ©calÃ©e, les logs d'attaque des pirates seront impossibles Ã  corrÃ©ler\navec les Ã©vÃ©nements rÃ©els. L'Amiral Xylia Vorn exige des horodatages fiables pour l'enquÃªte.\n\nÂ« VÃ©rifiez l'heure systÃ¨me, Capitaine. Dites-moi si elle est correcte. Â»\n\nVous consultez l'horloge du systÃ¨me de la station.\n\nLes Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.".into(),
            goal: "Vérifiez que l'horloge de bord est bien réglée avant de commencer l'enquête.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelle commande donne date/heure?".into(),
            required_commands: vec!["date".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "date".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            map: "VUE DE DESSUS — horloge de la salle de navigation:\n  ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━┓\n  ┃    PONT    ┃ ┃  CABINES   ┃ ┃   ▶NAV◀    ┃\n  ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛ ┗━━━━┳━━━━━━━┛\n       ┻━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━┻┛\n       └────► coordonnees/".into(),
            skill: Some(Skill { command: "date".into(), name: "date".into(), description: "date — affiche la date et l'heure du système (+format)".into() }),
        },
    ]
}

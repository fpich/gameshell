//! CHAPITRE 20 - Boucliers et reaction (Acte 5) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "20.1".into(), chapter: 20, chapter_title: "Boucliers et reaction".into(), act: 5,
            title: "Etat des boucliers".into(),
            description: "Vex Korshun vérifie l'état du pare-feu du vaisseau.".into(),
            story: "Vex Korshun, le commandant de la station, vous accueille d'un geste brusque sur la passerelle. « Capitaine, les Corbeaux Noirs sont massés dans le secteur 7G. Avant de réagir, je veux un état précis de nos boucliers déflecteurs, le pare-feu du vaisseau. »

Il se penche sur votre console: « J'ai besoin de la version détaillée: état général, politique par défaut, journalisation, règles en vigueur. Pas une simple ligne: le rapport complet. »

Draz M'kal, chef de la sécurité, ajoute depuis son pupitre: « Si les boucliers sont éteints, chaque seconde d'attente est une porte ouverte. Montrez-nous ce que voit le système. »

« Affichez l'état du pare-feu, Capitaine, en version détaillée. Nous devons savoir sur quoi nous pouvons compter avant de riposter. »

La réponse s'affiche: statut, politique par défaut, journalisation. Vex Korshun lit le résultat attentivement et note les règles en place.

« Voilà où nous en sommes », dit-il. « Savoir l'état exact des boucliers, c'est savoir ce qu'il nous reste à défendre. »".into(),
            goal: "Affichez l'état détaillé du pare-feu du vaisseau.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'état du pare-feu?".into(),
            required_commands: vec!["sudo ufw status verbose".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ufw".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ufw status".into(), name: "ufw".into(), description: "ufw — pare-feu du vaisseau (status, allow, deny, enable)".into() }),
        },
        Mission {
            id: "20.2".into(), chapter: 20, chapter_title: "Boucliers et reaction".into(), act: 5,
            title: "Bloquer le pirate 10.0.0.66".into(),
            description: "Bloquez toutes les connexions venant de l'IP pirate.".into(),
            story: "Draz M'kal surgit sur le pont, le visage fermé. « Capitaine, cette nuit, les journaux de la station ont enregistré des tentatives d'intrusion répétées sur le port SSH. J'ai retrouvé la source: l'adresse 10.0.0.66. »

Il affiche les traces: « Que des échecs de connexion, en rafales, toujours depuis la même adresse. Ce n'est pas un curieux: c'est une sonde des Corbeaux Noirs qui teste nos défenses. »

« Refusez tout le trafic venant d'elle, Capitaine. Qu'elle tente le port SSH, la navigation, n'importe quoi: chaque paquet doit être rejeté, sans exception. »

Vex Korshun approuve: « Une adresse hostile, c'est une porte que l'on verrouille. Bloquez-la au niveau du pare-feu. »

La règle est enregistrée: le pare-feu refuse désormais tout trafic en provenance de 10.0.0.66. L'attaquant frappe contre un mur.

« Elle peut marteler autant qu'elle veut », conclut Draz M'kal. « Une adresse bloquée, c'est une menace neutralisée. »".into(),
            goal: "Bloquez au pare-feu tout le trafic provenant de l'adresse 10.0.0.66.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment bloquer une adresse IP au pare-feu?".into(),
            required_commands: vec!["sudo ufw deny from 10.0.0.66".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "deny".into(), expected: None }, MissionCheck { check_type: "sim_contains".into(), target: "ufw.rules".into(), expected: Some("DENY&&from 10.0.0.66".into()) }],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "ufw deny from".into(), name: "ufw deny from".into(), description: "ufw deny from IP — bloque tout le trafic d'une IP pirate".into() }),
        },
        Mission {
            id: "20.3".into(), chapter: 20, chapter_title: "Boucliers et reaction".into(), act: 5,
            title: "Regles du noyau".into(),
            description: "Vérifiez les règles de filtrage au niveau noyau (iptables).".into(),
            story: "Draz M'kal vous retrouve dans la salle des machines, devant les coffrets du réseau. « Le pare-feu est verrouillé, Capitaine, mais ce n'est pas la seule barrière: sous lui, le noyau du vaisseau filtre chaque paquet qui entre et sort. »

Il désigne un voyant: « Je veux voir ces règles de filtrage telles qu'elles sont réellement actives. Avec les adresses et les ports en clair, sans traduction de noms: j'ai besoin de lire les chaînes exactes qui s'appliquent. »

« Affichez-moi la liste des règles de filtrage du noyau, Capitaine. Nous devons vérifier que la politique de la station est conforme à ce que nous avons décidé ensemble. »

La liste s'affiche: chaînes, politiques par défaut, exceptions. Draz M'kal parcourt chaque ligne et compare avec la configuration attendue.

« Tout est en ordre », dit-il. « Au-delà du pare-feu, le noyau reste notre dernière barrière. Tant que ces règles sont en place, chaque paquet hostile est contrôlé. »".into(),
            goal: "Listez les règles de filtrage du pare-feu actuellement actives.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment lister les règles de filtrage du noyau?".into(),
            required_commands: vec!["sudo iptables -L -n".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "iptables".into(), expected: None }],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "iptables -L".into(), name: "iptables".into(), description: "iptables -L -n — liste les règles de filtrage du noyau".into() }),
        },
        Mission {
            id: "20.4".into(), chapter: 20, chapter_title: "Boucliers et reaction".into(), act: 5,
            title: "Revoquer un bannissement".into(),
            description: "L'Andromède (10.0.0.99) a été banni par erreur: débannissez-le.".into(),
            story: "Nira Okoro coupe la liaison avant même d'avoir salué. « Capitaine, problème grave: le videur automatique, notre surveillance des connexions, vient de bannir l'Andromède. L'adresse 10.0.0.99. »

Elle se reprend: « C'est le vaisseau-mère, celui de l'Amiral Xylia Vorn! Le système a confondu une synchronisation de routine avec une tentative d'intrusion. »

« Sans l'Andromède, nous perdons le soutien de la flotte au pire moment. Il faut révoquer ce bannissement immédiatement, avant qu'il ne coupe toute communication avec le QG. »

Vex Korshun, l'air tendu, confirme: « Annulez le bannissement de 10.0.0.99, Capitaine. Le videur peut se tromper; nous devons pouvoir corriger son erreur. »

Le bannissement est levé: l'adresse de l'Andromède peut à nouveau dialoguer avec la station. Nira Okoro relance la liaison, qui répond sans délai.

« C'était une erreur », soupire-t-elle. « Savoir débannir une adresse, c'est réparer ce que l'automatisme a cassé. »".into(),
            goal: "Révoquez le bannissement de l'adresse 10.0.0.99.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment révoquer un bannissement fail2ban?".into(),
            required_commands: vec!["sudo fail2ban-client unban 10.0.0.99".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "fail2ban-client".into(), expected: None }, MissionCheck { check_type: "sim_not_contains".into(), target: "fail2ban.bans".into(), expected: Some("10.0.0.99".into()) }],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "fail2ban-client".into(), name: "fail2ban-client".into(), description: "fail2ban-client unban IP — révoque un bannissement".into() }),
        },
        Mission {
            id: "20.5".into(), chapter: 20, chapter_title: "Boucliers et reaction".into(), act: 5,
            title: "Surveiller le coffre-fort + code final".into(),
            description: "Dernière barrière: surveillez les accès au coffre-fort, puis entrez le protocole d'autodestruction.".into(),
            story: "Amiral Xylia Vorn apparaît sur l'écran de la passerelle, l'image légèrement brouillée par la distance. « Capitaine, dernière barrière avant la contre-attaque: les Corbeaux Noirs visent nos secrets. Nous devons savoir qui touche au coffre. »

Elle précise: « Installez une surveillance sur /soutes/secrets, le compartiment où sont entreposées les données classifiées. Chaque accès en écriture doit être tracé, et chaque événement étiqueté “corbeau” pour que nos analystes puissent le retrouver dans les journaux. »

« Quand la surveillance sera en place, ouvrez le coffre-fort de la sécurité, /securite/coffre-fort/codes_acces.txt. Le protocole d'auto-destruction y est consigné. Donnez-le-moi pour déclencher la phase finale. »

Vex Korshun murmure: « Ce protocole est notre dernier recours, Capitaine. Ne le lisons qu'une fois la caméra branchée: chaque étape doit rester tracée. »

La règle d'audit est installée. Vous ouvrez le coffre-fort, lisez le protocole consigné dans codes_acces.txt et le transmettez à l'Amiral.

« Parfait », dit Xylia Vorn. « La surveillance est en place, le protocole est connu. La station Orion-7 est prête pour la phase finale. »".into(),
            goal: "Installez une règle d'audit sur /soutes/secrets, puis validez avec le protocole d'auto-destruction du coffre-fort.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Comment surveiller les accès à un compartiment sensible?".into(),
            required_commands: vec!["sudo auditctl -w /soutes/secrets -p wa -k corbeau".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "auditctl".into(), expected: None }, MissionCheck { check_type: "sim_contains".into(), target: "audit.rules".into(), expected: Some("-w /soutes/secrets".into()) }],
            xp_reward: 500,
            code_required: Some("OMEGA-9981-DELTA".into()),
            skill: Some(Skill { command: "auditctl -w".into(), name: "auditctl".into(), description: "auditctl -w — installe une caméra d'audit sur un fichier sensible".into() }),
        },
    ]
}

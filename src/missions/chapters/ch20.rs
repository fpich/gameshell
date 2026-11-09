//! CHAPITRE 20 - Boucliers et reaction (Acte 5) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "20.1".into(), chapter: 20, chapter_title: "Boucliers et reaction".into(), act: 5,
            title: "Etat des boucliers".into(),
            description: "Vex Korshun vérifie l'état du pare-feu du vaisseau.".into(),
            story: "Vex Korshun: « Capitaine, avant de bloquer les pirates, vérifions l'état
des boucliers déflecteurs (le pare-feu ufw). »

La commande du pare-feu affiche son état, en version détaillée.

« Les boucliers sont-ils actifs? Montrez-moi leur état, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

L'état détaillé du pare-feu s'affiche. Les règles sont en place.

« Le pare-feu est notre bouclier », explique Draz M'kal. « Savoir s'il est actif, c'est savoir si nous sommes protégés. »".into(),
            goal: "Affichez l'état détaillé du pare-feu du vaisseau.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'état du pare-feu?".into(),
            required_commands: vec!["sudo ufw status verbose".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ufw".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ufw status".into(), name: "ufw".into(), description: "ufw — pare-feu du vaisseau (status, allow, deny, enable)".into() }),
        },
        Mission {
            id: "20.2".into(), chapter: 20, chapter_title: "Boucliers et reaction".into(), act: 5,
            title: "Bloquer le pirate 10.0.0.66".into(),
            description: "Bloquez toutes les connexions venant de l'IP pirate.".into(),
            story: "Vex Korshun, alarmÃ©: Â« Capitaine! L'IP 10.0.0.66 martÃ¨le notre port SSH.
Bloquez-la au niveau des boucliers: refusez TOUT trafic venant d'elle. Â»

Â« Refusez 10.0.0.66, Capitaine. DÃ©finitivement. Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

N'oubliez pas, Capitaine: dans l'espace, la prÃ©paration et la prÃ©cision font la diffÃ©rence entre la victoire et la dÃ©faite.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

L'adresse 10.0.0.66 est bloquée au pare-feu. L'attaquant ne peut plus entrer.

« Bien joué, Capitaine », approuve Draz M'kal. « Une adresse bloquée, c'est une menace neutralisée. »".into(),
            goal: "Bloquez au pare-feu tout le trafic provenant de l'adresse 10.0.0.66.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment bloquer une adresse IP au pare-feu?".into(),
            required_commands: vec!["sudo ufw deny from 10.0.0.66".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "deny".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "ufw deny from".into(), name: "ufw deny from".into(), description: "ufw deny from IP — bloque tout le trafic d'une IP pirate".into() }),
        },
        Mission {
            id: "20.3".into(), chapter: 20, chapter_title: "Boucliers et reaction".into(), act: 5,
            title: "Regles du noyau".into(),
            description: "Vérifiez les règles de filtrage au niveau noyau (iptables).".into(),
            story: "Vex Korshun: Â« Capitaine, au-delÃ  des boucliers ufw, vÃ©rifions les rÃ¨gles
de filtrage au niveau du noyau du vaisseau (iptables). Listez-les. Â»

Â« Quelles sont les rÃ¨gles iptables actives? Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

N'oubliez pas, Capitaine: dans l'espace, la prÃ©paration et la prÃ©cision font la diffÃ©rence entre la victoire et la dÃ©faite.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

Les règles iptables s'affichent. Vous voyez le filtrage du noyau en action.

« Au-delà du pare-feu, le noyau filtre chaque paquet », explique Draz M'kal. « Ces règles sont la dernière barrière. »".into(),
            goal: "Listez les règles de filtrage du pare-feu actuellement actives.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment lister les règles de filtrage du noyau?".into(),
            required_commands: vec!["sudo iptables -L -n".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "iptables".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "iptables -L".into(), name: "iptables".into(), description: "iptables -L -n — liste les règles de filtrage du noyau".into() }),
        },
        Mission {
            id: "20.4".into(), chapter: 20, chapter_title: "Boucliers et reaction".into(), act: 5,
            title: "Revoquer un bannissement".into(),
            description: "L'Andromède (10.0.0.99) a été banni par erreur: débannissez-le.".into(),
            story: "Nira Okoro, paniquÃ©e: Â« Capitaine! Le videur automatique a banni par erreur
le vaisseau-mÃ¨re AndromÃ¨de (10.0.0.99)! Sans lui, plus de soutien! Â»

Il faut rÃ©voquer le bannissement de 10.0.0.99 via le client fail2ban.

Â« DÃ©banissez 10.0.0.99 immÃ©diatement, Capitaine. Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

Le bannissement de 10.0.0.99 est révoqué. L'adresse peut à nouveau communiquer.

« C'était une erreur », admet Draz M'kal. « Parfois, le videur automatique se trompe. Savoir débannir, c'est réparer une injustice. »".into(),
            goal: "Révoquez le bannissement de l'adresse 10.0.0.99.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment révoquer un bannissement fail2ban?".into(),
            required_commands: vec!["sudo fail2ban-client unban 10.0.0.99".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "fail2ban-client".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "fail2ban-client".into(), name: "fail2ban-client".into(), description: "fail2ban-client unban IP — révoque un bannissement".into() }),
        },
        Mission {
            id: "20.5".into(), chapter: 20, chapter_title: "Boucliers et reaction".into(), act: 5,
            title: "Surveiller le coffre-fort + code final".into(),
            description: "Dernière barrière: surveillez les accès au coffre-fort, puis entrez le protocole d'autodestruction.".into(),
            story: "Amiral Xylia Vorn, depuis l'AndromÃ¨de: Â« Capitaine, derniÃ¨re barriÃ¨re avant
la contre-attaque: installez une camÃ©ra de surveillance sur les accÃ¨s en
Ã©criture au coffre-fort des secrets: /soutes/secrets. Â»

Puis, pour valider la mission finale, entrez le PROTOCOLE D'AUTO-DESTRUCTION
que vous avez trouvÃ© dans le coffre-fort (/securite/coffre-fort/codes_acces.txt).

Â« Surveillez /soutes/secrets, puis donnez-moi le protocole, Capitaine. Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La règle d'audit est installée sur /soutes/secrets. Chaque accès sera désormais surveillé.

« La surveillance est notre meilleure alliée », conclut Draz M'kal. « Chaque accès au coffre sera tracé. »".into(),
            goal: "Installez une règle d'audit sur /soutes/secrets, puis validez avec le protocole d'auto-destruction du coffre-fort.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Comment surveiller les accès à un compartiment sensible?".into(),
            required_commands: vec!["sudo auditctl -w /soutes/secrets -p wa -k corbeau".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "auditctl".into(), expected: None }],
            hints: vec![],
            xp_reward: 500,
            code_required: Some("OMEGA-9981-DELTA".into()),
            skill: Some(Skill { command: "auditctl -w".into(), name: "auditctl".into(), description: "auditctl -w — installe une caméra d'audit sur un fichier sensible".into() }),
        },
    ]
}

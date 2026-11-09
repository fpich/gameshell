//! CHAPITRE 11 - Logs de securite (Acte 3) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "11.1".into(), chapter: 11, chapter_title: "Logs de securite".into(), act: 3,
            title: "Tentatives de connexion pirates".into(),
            description: "Draz M'kal analyse auth.log: quelles attaques?".into(),
            story: "Draz M'kal, l'officier de sécurité, ouvre le journal d'authentification de la station:
/securite/logs/auth.log.

« Capitaine, ce fichier enregistre toutes les tentatives de connexion.
Je veux voir seulement les lignes qui parlent d'ÉCHEC ('Failed'). »

Il vous faut chercher un mot précis dans ce fichier.

« Montrez-moi les tentatives échouées, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Les lignes 'Failed' défilent: des dizaines de tentatives de connexion échouées.

Draz M'kal analyse: « Chaque échec est une tentative d'intrusion repoussée. Ces traces sont notre première preuve. »".into(),
            goal: "Affichez uniquement les lignes contenant 'Failed' dans le fichier auth.log.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment chercher un mot précis dans un fichier de logs?".into(),
            required_commands: vec!["grep Failed /securite/logs/auth.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "grep".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "grep".into(), name: "grep".into(), description: "grep — recherche un motif dans un fichier".into() }),
        },
        Mission {
            id: "11.2".into(), chapter: 11, chapter_title: "Logs de securite".into(), act: 3,
            title: "Combien d'attaques?".into(),
            description: "Comptez les tentatives échouées dans auth.log.".into(),
            story: "Vex Korshun: « Capitaine, au lieu des lignes entières, je veux juste
SAVOIR COMBIEN de tentatives ont échoué. Le nombre suffit. »

La recherche dans le fichier existe, mais avec une option qui ne compte
que les lignes trouvées.

« Donnez-moi le nombre d'échecs, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

Le compteur affiche le nombre total de tentatives échouées. Le chiffre est alarmant.

« Autant d'attaques en si peu de temps », murmure Draz M'kal. « Nous devons renforcer nos défenses. »".into(),
            goal: "Comptez combien de lignes contiennent 'Failed' dans auth.log.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment compter les lignes trouvées par une recherche?".into(),
            required_commands: vec!["grep -c Failed /securite/logs/auth.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-c".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "grep -c".into(), name: "grep -c".into(), description: "grep -c — compte les lignes trouvées".into() }),
        },
        Mission {
            id: "11.3".into(), chapter: 11, chapter_title: "Logs de securite".into(), act: 3,
            title: "L'IP du pirate avec les numeros de ligne".into(),
            description: "Nira Okoro veut les lignes parlant de 10.0.0.66, avec leur numéro.".into(),
            story: "Nira Okoro, officier communications: « Capitaine, dans le fichier des
fréquences ennemies, je veux les lignes qui parlent de l'IP 10.0.0.66.
Et j'ai besoin de leur NUMÉRO de ligne pour citer le rapport. »

Fichier concerné: /communications/frequences/frequences_ennemies.txt.

« Lignes de 10.0.0.66 avec numéros, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Les lignes mentionnant l'adresse 10.0.0.66 s'affichent avec leur numéro. Cette IP revient sans cesse.

« C'est elle », affirme Draz M'kal. « L'adresse de l'attaquant. Chaque ligne numérotée est une pièce du dossier. »".into(),
            goal: "Affichez les lignes contenant 10.0.0.66 avec leur numéro de ligne.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment afficher les numéros de ligne d'une recherche?".into(),
            required_commands: vec!["grep -n 10.0.0.66 /communications/frequences/frequences_ennemies.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-n".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "grep -n".into(), name: "grep -n".into(), description: "grep -n — affiche les numéros de ligne des résultats".into() }),
        },
        Mission {
            id: "11.4".into(), chapter: 11, chapter_title: "Logs de securite".into(), act: 3,
            title: "Qui a ete banni?".into(),
            description: "Le log fail2ban enregistre les bannissements de pirates.".into(),
            story: "Vex Korshun consulte le journal du videur automatique:
/securite/logs/fail2ban.log.

« Capitaine, je veux voir toutes les lignes qui parlent de BANNISSEMENT,
sans tenir compte de la casse (Ban, ban, BAN...). »

La recherche doit ignorer la différence majuscules/minuscules.

« Quels pirates ont été bannis? »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Les lignes de bannissement s'affichent, insensibles à la casse. Plusieurs adresses ont été bloquées.

« Le videur automatique fait son travail », note Draz M'kal. « Mais il faut vérifier qu'il ne bannit pas des alliés par erreur. »".into(),
            goal: "Affichez les lignes parlant de bannissement dans fail2ban.log, insensible à la casse.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment chercher un mot sans tenir compte des majuscules?".into(),
            required_commands: vec!["grep -i ban /securite/logs/fail2ban.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-i".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "grep -i".into(), name: "grep -i".into(), description: "grep -i — recherche insensible aux majuscules/minuscules".into() }),
        },
        Mission {
            id: "11.5".into(), chapter: 11, chapter_title: "Logs de securite".into(), act: 3,
            title: "Les dernieres alertes systeme".into(),
            description: "Les 5 dernières lignes du syslog de la station.".into(),
            story: "Tael Rhen a besoin des événements les plus récents du vaisseau:
« Capitaine, le syslog de la station est interminable. Montrez-moi seulement
les 5 DERNIÈRES lignes: ce qui vient de se passer. »

Fichier: /securite/logs/syslog.

« Les 5 dernières lignes du syslog, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

Les cinq dernières lignes du syslog s'affichent. Vous y lisez les derniers événements du vaisseau.

« La fin du journal, c'est l'instant présent », explique Zyra-7. « C'est là que se joue la situation actuelle. »".into(),
            goal: "Affichez les 5 dernières lignes du fichier syslog.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir seulement la fin d'un fichier de logs?".into(),
            required_commands: vec!["tail -5 /securite/logs/syslog".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-5".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "tail -n N".into(), name: "tail -n N".into(), description: "tail -n N — affiche les N dernières lignes d'un fichier".into() }),
        },
    ]
}

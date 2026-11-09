//! CHAPITRE 19 - Durcissement SSH (Acte 5) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "19.1".into(), chapter: 19, chapter_title: "Durcissement SSH".into(), act: 5,
            title: "Lire la config SSH".into(),
            description: "Vex Korshun audite la configuration SSH de la station.".into(),
            story: "Vex Korshun: Â« Capitaine, avant de durcir la station, lisons la
configuration actuelle du service SSH: /securite/config/sshd_config. Â»

Ce fichier contient les rÃ©glages de sÃ©curitÃ© du canal de communication
sÃ©curisÃ©. Lisez-le entiÃ¨rement.

Â« Montrez-moi la config SSH actuelle, Capitaine. Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

Vous lisez la configuration SSH du vaisseau. Certaines options sont dangereuses.

« Le login root est autorisé », constate Draz M'kal. « C'est une faille que les pirates pourraient exploiter. »".into(),
            goal: "Lisez le contenu du fichier de configuration /securite/config/sshd_config.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier de configuration?".into(),
            required_commands: vec!["cat /securite/config/sshd_config".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "sshd_config".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche le contenu complet d'un fichier texte".into() }),
        },
        Mission {
            id: "19.2".into(), chapter: 19, chapter_title: "Durcissement SSH".into(), act: 5,
            title: "Interdire le login root".into(),
            description: "Ajoutez 'PermitRootLogin no' à la config SSH.".into(),
            story: "Vex Korshun: « Capitaine, la config actuelle autorise le login root
(PermitRootLogin yes). C'est une faille: les pirates essaient le compte root
en premier. »

Il faut ajouter la ligne 'PermitRootLogin no' à la fin du fichier
sshd_config, sans effacer le reste.

« Ajoutez 'PermitRootLogin no' à la fin de sshd_config, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

La ligne 'PermitRootLogin no' est ajoutée à la fin du fichier. Le login root est désormais interdit.

« Bien », approuve Draz M'kal. « Une faille de moins. Chaque ligne durcie renforce nos défenses. »".into(),
            goal: "Ajoutez la ligne 'PermitRootLogin no' à la fin du fichier sshd_config.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter une ligne de configuration à la fin d'un fichier?".into(),
            required_commands: vec!["echo 'PermitRootLogin no' >> /securite/config/sshd_config".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }, MissionCheck { check_type: "config_contains".into(), target: "/securite/config/sshd_config".into(), expected: Some("PermitRootLogin no".into()) }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "cmd >> fichier".into(), name: ">>".into(), description: "COMMANDE >> fichier — ajoute la sortie à la fin du fichier".into() }),
        },
        Mission {
            id: "19.3".into(), chapter: 19, chapter_title: "Durcissement SSH".into(), act: 5,
            title: "Etat du service SSH".into(),
            description: "Le service SSH tourne-t-il encore sur la station?".into(),
            story: "Tael Rhen, depuis la salle des machines: « Capitaine, avant de redémarrer
SSH après la modification, vérifiez qu'il tourne bien actuellement. »

La commande de gestion des services affiche l'état d'un service.

« Le service ssh est-il actif, Capitaine? »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

L'état du service ssh s'affiche. Le service tourne correctement.

« Le service est actif », confirme Draz M'kal. « Vérifier son état, c'est s'assurer que la porte reste ouverte aux bons visiteurs. »".into(),
            goal: "Affichez l'état actuel du service ssh.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'état d'un service systemd?".into(),
            required_commands: vec!["systemctl status ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "systemctl".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "systemctl status".into(), name: "systemctl status".into(), description: "systemctl status — affiche l'état d'un service".into() }),
        },
        Mission {
            id: "19.4".into(), chapter: 19, chapter_title: "Durcissement SSH".into(), act: 5,
            title: "Redemarrer SSH".into(),
            description: "Appliquez la nouvelle configuration en redémarrant SSH.".into(),
            story: "Tael Rhen: « Capitaine, la config est modifiée. Redémarrez le service SSH
pour appliquer le durcissement immédiatement. »

« Redémarrez ssh maintenant, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

Zyra-7, l'IA de bord, surveille les capteurs en permanence et vous alerte dès qu'une signature hostile approche le secteur.

Le service ssh redémarre. La nouvelle configuration est appliquée.

« Redémarrer, c'est appliquer les changements », explique Draz M'kal. « Le service repart avec les réglages durcis. »".into(),
            goal: "Redémarrez le service ssh pour appliquer la nouvelle configuration.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment redémarrer un service après modification de sa config?".into(),
            required_commands: vec!["sudo systemctl restart ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "restart".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "systemctl restart".into(), name: "systemctl restart".into(), description: "systemctl restart — redémarre un service pour appliquer sa config".into() }),
        },
        Mission {
            id: "19.5".into(), chapter: 19, chapter_title: "Durcissement SSH".into(), act: 5,
            title: "Logs du service SSH".into(),
            description: "Consultez les 10 dernières lignes du journal du service SSH.".into(),
            story: "Vex Korshun: « Capitaine, pour vérifier que le redémarrage s'est bien passé,
consultez les dernières entrées du journal du service SSH. »

Le journal journalisé permet de filtrer par service et de limiter le nombre
de lignes affichées.

« Montrez-moi les 10 dernières lignes du journal de ssh. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

Les dix dernières lignes du journal de ssh s'affichent. Vous y voyez les connexions récentes.

« Le journal raconte l'activité du service », note Draz M'kal. « C'est là qu'on repère les connexions suspectes. »".into(),
            goal: "Affichez les 10 dernières lignes du journal du service ssh.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment consulter les logs d'un service précis?".into(),
            required_commands: vec!["journalctl -u ssh -n 10".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "journalctl".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "journalctl".into(), name: "journalctl".into(), description: "journalctl -u service -n N — logs d'un service (N dernières lignes)".into() }),
        },
    ]
}

//! CHAPITRE 12 - Diagnostic reseau (Acte 3) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "12.1".into(), chapter: 12, chapter_title: "Diagnostic reseau".into(), act: 3,
            title: "Coordonnees radio du vaisseau".into(),
            description: "Nira Okoro veut la configuration réseau complète d'Orion-7.".into(),
            story: "Nira Okoro, depuis la salle des communications: « Capitaine, quels sont
les coordonnées radio exactes du vaisseau? Toutes les interfaces, toutes
les adresses IP. »

Il vous faut la commande qui affiche la configuration réseau complète:
interfaces, adresses IP, état des liens.

« Donnez-moi la config réseau complète, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

La configuration réseau complète du vaisseau s'affiche: interfaces, adresses IP.

Nira Okoro note les coordonnées: « Parfait. Je pourrai enfin régler les communications avec l'Andromède. »".into(),
            goal: "Affichez la configuration réseau complète du vaisseau (interfaces et IP).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les adresses IP du vaisseau?".into(),
            required_commands: vec!["ip addr".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ip".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip addr".into(), name: "ip".into(), description: "ip addr — affiche les interfaces et adresses IP".into() }),
        },
        Mission {
            id: "12.2".into(), chapter: 12, chapter_title: "Diagnostic reseau".into(), act: 3,
            title: "Par où passe le trafic?".into(),
            description: "Nira Okoro veut la table de routage du vaisseau.".into(),
            story: "Nira Okoro: « Capitaine, par quelle route passe le trafic vers l'Andromède?
Montrez-moi la table de routage du vaisseau: les routes et la passerelle. »

La commande réseau affiche aussi les routes.

« Quelles sont les routes du vaisseau? »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

La table de routage s'affiche. Vous voyez par où passe le trafic vers l'extérieur.

« Sans cette route, aucun message ne sortirait du vaisseau », explique Nira Okoro. « C'est notre lien avec le monde. »".into(),
            goal: "Affichez la table de routage du vaisseau.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les routes réseau du vaisseau?".into(),
            required_commands: vec!["ip route".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "route".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ip route".into(), name: "ip route".into(), description: "ip route — affiche la table de routage".into() }),
        },
        Mission {
            id: "12.3".into(), chapter: 12, chapter_title: "Diagnostic reseau".into(), act: 3,
            title: "Le pirate repond-il?".into(),
            description: "Testez la connectivité vers l'IP pirate 10.0.0.66.".into(),
            story: "Nira Okoro: « Capitaine, les capteurs ont détecté un vaisseau pirate à
l'adresse 10.0.0.66. Envoyez-lui 4 signaux 'ping' pour voir s'il répond. »

La commande ping envoie des paquets et mesure la réponse.

« 4 pings vers 10.0.0.66, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

Les quatre paquets partent vers 10.0.0.66. Les réponses reviennent: la cible est bien là.

« Elle répond », confirme Draz M'kal. « L'attaquant est actif. Nous savons maintenant qu'il est joignable. »".into(),
            goal: "Testez la connectivité vers l'adresse 10.0.0.66 en envoyant 4 paquets.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment tester si une adresse réseau répond?".into(),
            required_commands: vec!["ping -c 4 10.0.0.66".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ping".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ping -c 4".into(), name: "ping".into(), description: "ping -c N — envoie N paquets pour tester la connectivité".into() }),
        },
        Mission {
            id: "12.4".into(), chapter: 12, chapter_title: "Diagnostic reseau".into(), act: 3,
            title: "Quels vaisseaux sont connectes?".into(),
            description: "Quels ports du vaisseau sont en écoute?".into(),
            story: "Vex Korshun: Â« Capitaine, quels vaisseaux sont actuellement connectÃ©s
Ã  nos ports? Je veux la liste des connexions actives et des ports en Ã©coute. Â»

Il vous faut la commande qui affiche les connexions rÃ©seau actives
et les ports en Ã©coute, avec les processus associÃ©s.

Â« Quels ports sont ouverts sur la station? Â»

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

Les connexions actives et les ports en écoute s'affichent. Plusieurs ports sont ouverts.

« Chaque port ouvert est une porte d'entrée potentielle », avertit Draz M'kal. « Il faut savoir lesquels sont nécessaires. »".into(),
            goal: "Affichez les connexions réseau actives et les ports en écoute du vaisseau.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les ports en écoute du vaisseau?".into(),
            required_commands: vec!["ss -tlnp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ss".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ss -tlnp".into(), name: "ss".into(), description: "ss -tlnp — affiche les connexions actives et ports en écoute".into() }),
        },
        Mission {
            id: "12.5".into(), chapter: 12, chapter_title: "Diagnostic reseau".into(), act: 3,
            title: "Coordonnees d'Orion-7".into(),
            description: "Lisez le fichier des coordonnées de navigation.".into(),
            story: "Nira Okoro: « Capitaine, pour le rapport à l'Andromède, j'ai besoin des
coordonnées officielles de la station. Elles sont dans la salle de navigation:
/navigation/coordonnees/orion7.txt. »

« Lisez ce fichier et dites-moi où nous sommes. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

N'oubliez pas, Capitaine: dans l'espace, la préparation et la précision font la différence entre la victoire et la défaite.

Vous lisez les coordonnées de navigation d'Orion-7. Le vaisseau est positionné dans le secteur 7G.

« Ces coordonnées sont notre adresse dans l'espace », explique Nira Okoro. « Sans elles, personne ne saurait où nous trouver. »".into(),
            goal: "Lisez le fichier orion7.txt des coordonnées de navigation.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier de coordonnées dans un autre compartiment?".into(),
            required_commands: vec!["cat /navigation/coordonnees/orion7.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "orion7.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche le contenu complet d'un fichier texte".into() }),
        },
    ]
}

//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 11 — Combiner les commandes (Acte 3) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "11.1".into(), chapter: 11, chapter_title: "Combiner les commandes".into(), act: 3,
            title: "Attaques filtrées".into(),
            description: "Combien de 'Failed' venant de 10.0.0.66?".into(),
            story: "Hanzō pousse la porte de la salle des rapports, un rouleau noir à la main. Il le déroule devant vous: « Le château tient un journal de chaque tentative d'entrée, Seigneur. On y lit 'Failed' quand un inconnu a frappé à une porte sans présenter le bon sceau. »

Il pose un doigt sur une ligne: « Hier soir, cette machine étrangère, 10.0.0.66, a frappé plusieurs fois. Je veux savoir combien de fois exactement. Ne me rapportez pas toute la liste: dans /scribes/logs/auth.log, ne gardez d'abord que les échecs, puis seulement les lignes de 10.0.0.66, puis donnez-m'en le compte. Trois gestes enchaînés, un seul verdict. »

Vous songez aux sentinelles qui filtrent les visiteurs au portail: chacune ne laisse passer que ce qu'on lui demande.

Le moine Tetsuo, qui préparait son encens, murmure: « Mesurer la menace, c'est déjà la moitié de la victoire, Seigneur. »

Hanzō attend, pinceau levé. Combien d'échecs viennent de 10.0.0.66?

".into(),
            goal: "Comptez pour Hanzō les lignes 'Failed' venant de 10.0.0.66 dans /scribes/logs/auth.log, en enchaînant trois filtres: les échecs, puis cette IP, puis le décompte des lignes.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment enchaîner trois commandes?".into(),
            required_commands: vec!["grep Failed /scribes/logs/auth.log | grep 10.0.0.66 | wc -l".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "|".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "wc".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "|".into(), name: "|".into(), description: "| — chaîne de commandes".into() }),
        },
        Mission {
            id: "11.2".into(), chapter: 11, chapter_title: "Combiner les commandes".into(), act: 3,
            title: "Rapport d'attaques".into(),
            description: "Écrivez les attaques dans rapport.txt.".into(),
            story: "Dame Kaguya vous rejoint dans vos quartiers, un plateau de thé à la main. Elle le dépose, puis sort un rouleau vide: « Le conseil veut garder une trace écrite des attaques de la nuit, Seigneur. Un rapport que l'on pourra relire dans un an, quand tout sera apaisé. »

Elle désigne la porte des scribes: « Les échecs inscrits dans /scribes/logs/auth.log doivent être consignés dans un nouveau parchemin, rapport.txt, posé sur votre bureau. Toutes les lignes 'Failed', sans en omettre une seule. »

« Je n'ai pas besoin de les lire ici, Seigneur: c'est la trace qui compte. Écrivez-les dans le parchemin, et prévenez-moi quand ce sera fait. »

Vous pensez aux scribes qui copient chaque décision du conseil sur du papier épais: la mémoire du domaine vit dans ses registres.

Le moine Tetsuo, qui venait saluer Kaguya, ajoute: « Ce qui n'est pas écrit s'évapore, Seigneur. Ce qui est écrit survit. »

Kaguya s'installe pour attendre. Le parchemin rapport.txt doit voir le jour.

".into(),
            goal: "Enregistrez les lignes 'Failed' de /scribes/logs/auth.log dans un nouveau parchemin rapport.txt.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment enregistrer un résultat dans un fichier?".into(),
            required_commands: vec!["grep Failed /scribes/logs/auth.log > rapport.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "~/rapport.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: ">".into(), name: ">".into(), description: "> — redirige vers un fichier".into() }),
        },
        Mission {
            id: "11.3".into(), chapter: 11, chapter_title: "Combiner les commandes".into(), act: 3,
            title: "Ajouter au rapport".into(),
            description: "Ajoutez la fin du syslog au rapport.".into(),
            story: "Dame Kaguya revient, un second rouleau dans les mains. Elle le déroule: « Voici le syslog du château, Seigneur: la mémoire des services et des rondes. Le conseil veut sa conclusion dans le rapport de la journée. »

« Prenez les trois dernières lignes de /scribes/logs/syslog, rien de plus, rien de moins, et ajoutez-les tout à la fin de rapport.txt. »

Elle insiste, le doigt levé: « Sans effacer ce qui s'y trouve déjà, Seigneur. Les échecs de la nuit doivent rester, et cette fin les suivra. Rien ne doit être perdu. »

Vous pensez aux rouleaux du château: on ne rature jamais une page déjà écrite, on la prolonge.

Le moine Tetsuo, qui remonte de la pagode, approuve: « Toute parole a sa suite, Seigneur, mais jamais au prix de celle qui précède. »

Kaguya attend près de la porte. Les trois dernières lignes doivent rejoindre la fin du parchemin, sans rien perdre.

".into(),
            goal: "Ajoutez les 3 dernières lignes de /scribes/logs/syslog à la fin de rapport.txt, sans effacer ce qu'il contient.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter à la fin d'un fichier?".into(),
            required_commands: vec!["tail -3 /scribes/logs/syslog >> rapport.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: ">>".into(), name: ">>".into(), description: ">> — ajoute à la fin".into() }),
        },
        Mission {
            id: "11.4".into(), chapter: 11, chapter_title: "Combiner les commandes".into(), act: 3,
            title: "Compter les processus filtrés".into(),
            description: "Combien de processus 'garde'?".into(),
            story: "Le capitaine des gardes entre, la lance plantée dans le sol. « Seigneur, chaque ronde du château laisse une trace dans la liste des tâches en cours: garde_ronde, garde_veilleur, et d'autres. Je voudrais savoir combien de ses sentinelles veillent en ce moment. »

Il croise les bras: « Je ne veux pas déchiffrer toute la liste, Seigneur. Tenez-la complète, ne gardez que les lignes qui parlent de la garde, puis donnez-m'en le compte. Trois gestes, un nombre, une réponse claire. »

Vous pensez au tableau des rondes accroché au poste de garde: chaque nom y est épinglé, chaque ligne y figure.

Le moine Tetsuo, consulté sur la veille, répond: « On ne compte bien que ce que l'on a d'abord isolé, Seigneur. »

Le capitaine impose le silence. Combien de tâches de garde tournent dans le château?

".into(),
            goal: "Comptez pour le capitaine les lignes de la liste complète des processus du château qui contiennent 'garde', en enchaînant liste, filtre et décompte.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment compter un résultat filtré?".into(),
            required_commands: vec!["ps aux | grep garde | wc -l".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "|".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "wc".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "|".into(), name: "|".into(), description: "| — chaîne".into() }),
        },
        Mission {
            id: "11.5".into(), chapter: 11, chapter_title: "Combiner les commandes".into(), act: 3,
            title: "Rapport complet".into(),
            description: "Rapport final: attaques + visiteurs.".into(),
            story: "Dame Kaguya rassemble les rouleaux de la journée sur la table du conseil. « Tout est prêt, Seigneur, mais il manque la pièce maîtresse: le rapport final que le conseil lira ce soir. »

Elle range deux parchemins devant vous: « Dans un nouveau parchemin, rapport_final.txt, je veux d'abord les attaques de la nuit, les échecs 'Failed' de /scribes/logs/auth.log, puis, tout à la suite, le registre des visiteurs du jour, /portail/registre_visiteurs.txt. »

« Dans cet ordre, Seigneur: les menaces d'abord, puis ceux qui sont entrés. Rien ne doit manquer, rien ne doit se mélanger. »

Vous pensez aux chroniques du château: les grands événements, puis les menus faits, chacun à sa place dans la même page.

Le moine Tetsuo observe la table: « L'ordre du récit fait le récit, Seigneur. »

Kaguya s'assoit, pinceau en main, prête à relire. Le rapport final doit réunir les deux sources, dans l'ordre.
".into(),
            goal: "Créez rapport_final.txt avec les attaques puis les visiteurs.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment composer un rapport multi-sources?".into(),
            required_commands: vec!["grep Failed /scribes/logs/auth.log > rapport_final.txt".into(), "cat /portail/registre_visiteurs.txt >> rapport_final.txt".into()],
            checks: vec![MissionCheck { check_type: "file_exists".into(), target: "~/rapport_final.txt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: ">".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "grep".into(), name: "grep".into(), description: "grep — recherche du texte dans un fichier".into() }),
        },
    ]
}

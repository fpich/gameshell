//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 8 — Navigation et disques (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "8.1".into(), chapter: 8, chapter_title: "Navigation et disques".into(), act: 2,
            title: "Priorité basse".into(),
            description: "La surveillance des greniers ne doit pas gêner la forge.".into(),
            story: "Le forgeron vous cherche dans la cour, le souffle court: « Seigneur, les températures du four montent et je ne peux pas m'arrêter: la lame du capitaine doit être trempée avant la tombée de la nuit. Mais il faut savoir ce que pèsent nos réserves. »

Il plante son marteau dans le sol: « Comptez-moi ce que pèsent les greniers, à /greniers sur la racine du domaine, d'un seul coup, et en taille lisible. Le château doit savoir ce qui lui reste. »

Il ajoute, l'air embarrassé: « Seulement voilà: le pesage ne doit pas ralentir la forge. Je veux que le calcul parte en tâche de fond, avec la priorité la plus basse possible, le niveau 19, pour que mon marteau garde toute sa force. »

Vous passez devant la forge où résonnent les coups. Le four ronfle, le métal rougit: tout doit continuer de tourner pendant que vous mesurez.

Le forgeron rit en croisant vos bras chargés de résultats: « Les greniers sont pesés et le feu n'a pas faibli une seconde. La lame sera trempée à temps, Seigneur. »

Le pesage a tourné en douceur, sans gêner la forge: le château connaît ses réserves, et l'acier chante toujours.".into(),
            goal: "Pesez /greniers au total, en taille lisible, en lançant le calcul avec la priorité la plus faible possible, 19, pour que la forge passe d'abord.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment lancer une commande en priorité basse?".into(),
            required_commands: vec!["nice -n 19 du -sh /greniers".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "nice".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-n".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "nice -n".into(), name: "nice -n".into(), description: "nice -n 19 — priorité basse".into() }),
        },
        Mission {
            id: "8.2".into(), chapter: 8, chapter_title: "Navigation et disques".into(), act: 2,
            title: "Espace des réserves".into(),
            description: "Quelle place reste sur les réserves?".into(),
            story: "Dame Kaguya vous fait venir dans la salle des cartes, un rouleau de riz sous le bras: « Seigneur, les pluies approchent. Avant qu'elles ne coupent les routes, je dois savoir si les réserves du château tiendront l'hiver. »

Elle déroule une carte du domaine: « Encore une fois, tout se joue sur la place disponible. Dites-moi ce qui reste sur nos réserves, celles du château entier: la place globale, pas un dossier particulier. »

Elle précise, en tapotant la table: « Et rendez-moi ça lisible pour un humain: je ne veux pas calculer des blocs moi-même, des tailles compréhensibles, sans effort. »

Vous traversez la cour vers les bâtiments de stockage: silos, celliers et entrepôts alignés au pied des remparts. La pluie menace à l'horizon.

Dame Kaguya lit le résultat, soulagée: « Voilà qui est clair. Nous avons de quoi passer l'hiver, Seigneur, et je saurai quoi rationner si besoin. »

La place restante est désormais connue: le château peut préparer la saison froide en connaissance de cause.".into(),
            goal: "Indiquez à Dame Kaguya l'espace disque restant sur les réserves du château, avec des tailles lisibles par un humain.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'espace disque restant?".into(),
            required_commands: vec!["df -h".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "df".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-h".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "df -h".into(), name: "df -h".into(), description: "df -h — espace disque lisible".into() }),
        },
        Mission {
            id: "8.3".into(), chapter: 8, chapter_title: "Navigation et disques".into(), act: 2,
            title: "Taille par grenier".into(),
            description: "Taille de chaque grenier.".into(),
            story: "Le marchand Genji vous attend au pied des greniers, balances et tablettes à la main: « Seigneur, le clan du sud veut acheter séparément chacune de nos réserves: le riz, le blé, le saké et le miso. »

Il désigne les bâtiments: « J'ai besoin d'un poids pour CHACUN des greniers, pas un total. Une valeur par réserve, pour que chaque marchandise soit négociée pour elle-même. »

Il insiste: « Les greniers sont à la racine du domaine, dans /greniers: mesurez chaque stock un par un, sans les mélanger. En taille lisible. »

Vous faites le tour des quatre bâtiments de stockage: riz, blé, saké, miso. Chacun a sa récolte, chacun a son carnet de comptes.

Genji coche chaque valeur sur sa tablette: « Riz, blé, saké, miso: chaque poids est noté. Nous vendrons chaque réserve à son juste prix, Seigneur. »

Chaque grenier a maintenant son poids inscrit dans les comptes du commerce, prêt pour la négociation.".into(),
            goal: "Affichez la taille de chaque sous-dossier de /greniers.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir la taille de chaque sous-dossier?".into(),
            required_commands: vec!["du -sh /greniers/*".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "du".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-sh".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "du".into(), name: "du".into(), description: "du — taille des fichiers et dossiers".into() }),
        },
        Mission {
            id: "8.4".into(), chapter: 8, chapter_title: "Navigation et disques".into(), act: 2,
            title: "Remonter d'un niveau".into(),
            description: "Depuis /archives/lettres, revenez à la racine.".into(),
            story: "Dame Kaguya vous croise dans la cour, un registre à la main: « Seigneur, le registre des lettres du clan est rangé dans la salle des archives: /archives/lettres. Allez l'y chercher. »

Elle poursuit: « Et une fois là-bas, revenez-moi au point de départ du domaine, à la racine, là d'où partent tous les chemins du château. Je vous y attends. »

Elle ajoute: « Pas besoin de refaire le chemin à l'œil: dans les archives, remontez les niveaux vers le haut, un à un, jusqu'à la racine. Affirmez-moi votre position quand vous y serez. »

Autour de vous, les étagères des archives ploient sous les rouleaux. Plus haut dans l'arborescence, la cour du château vous attend.

Dame Kaguya vous salue depuis la cour: « Vous voilà à la racine du domaine, Seigneur. Vous savez désormais redescendre et remonter sans vous perdre. »

La racine du château est retrouvée: les archives sont derrière vous, et tous les chemins du domaine repartent d'ici.".into(),
            goal: "Depuis /archives/lettres, rejoignez la racine / du château.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment remonter dans l'arborescence?".into(),
            required_commands: vec!["cd /archives/lettres".into(), "cd ../..".into(), "pwd".into()],
            checks: vec![MissionCheck { check_type: "dir_reached".into(), target: "/".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "pwd".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "cd ..".into(), name: "cd ..".into(), description: "cd .. — remonte d'un niveau".into() }),
        },
        Mission {
            id: "8.5".into(), chapter: 8, chapter_title: "Navigation et disques".into(), act: 2,
            title: "Le home du seigneur".into(),
            description: "Revenez dans vos quartiers avec ~.".into(),
            story: "La journée touche à sa fin et Dame Kaguya vous raccompagne dans la cour: « Seigneur, vous avez parcouru la moitié du domaine aujourd'hui: archives, greniers, salles des cartes. »

Elle désigne les toits au-delà des remparts: « Vos quartiers personnels vous attendent, là-haut, avec vos rouleaux et votre repos. Rejoignez-les directement, sans retraverser toutes les salles. »

Elle précise: « Il existe un raccourci vers votre home: le chemin personnel du seigneur. Prenez-le, puis affirmez où vous êtes. »

Les torches s'allument le long des remparts: la nuit tombe, et le domaine se prépare doucement au repos.

Dame Kaguya vous salue depuis le seuil de vos quartiers: « Vous voilà rentré, Seigneur. Votre home vous reconnaît: tout est à sa place. »

Le raccourci vous a ramené chez vous: vos quartiers personnels, vos rouleaux et votre repos vous attendent.".into(),
            goal: "Revenez dans vos quartiers personnels par le chemin le plus court, puis affichez votre position.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Comment revenir directement dans son home?".into(),
            required_commands: vec!["cd ~".into(), "pwd".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "~".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "pwd".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cd ~".into(), name: "cd ~".into(), description: "cd ~ — retour aux quartiers".into() }),
        },
    ]
}

//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 16 — Inventaire complet (Acte 4) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "16.1".into(), chapter: 16, chapter_title: "Inventaire complet".into(), act: 4,
            title: "Racine du château".into(),
            description: "Listez la racine du château.".into(),
            story: "Dame Kaguya: « Seigneur, avant le conseil de guerre, je
veux la liste complète des lieux du château: chaque
quartier, chaque bâtiment, depuis la racine du domaine.
Montrez-moi tout ce qui compose le château Shiro, rien
de plus. »

L'inventaire commence par les lieux eux-mêmes: savoir
où sont les remparts, les greniers, les archives, c'est
savoir ce que le domaine peut défendre. La carte du
château se lit depuis son origine, depuis la première
porte par laquelle tout entre.

Kaguya prépare la table du conseil: chaque nom de lieu
que vous lui montrerez deviendra une ligne de son
rapport. Un lieu oublié, c'est une défense en moins le
jour où le clan rival frappera.

Montrez-lui la liste des lieux du château, depuis la
racine du domaine. Quand elle aura tous les bâtiments
sous les yeux, elle pourra commencer l'inventaire et
répartir les gardes en connaissance de cause.".into(),
            goal: "Listez la racine / du château.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Comment lister la racine?".into(),
            required_commands: vec!["ls /".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ls".into(), expected: None }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "ls /".into(), name: "ls /".into(), description: "ls / — liste la racine".into() }),
        },
        Mission {
            id: "16.2".into(), chapter: 16, chapter_title: "Inventaire complet".into(), act: 4,
            title: "Tous les fichiers cachés".into(),
            description: "Liste détaillée de la racine, cachés compris.".into(),
            story: "Le moine Tetsuo: « Seigneur, l'inventaire ne sera
complet que si rien ne reste dans l'ombre. Montrez-moi
le contenu de la racine du château dans son entier:
chaque fichier, chaque détail, et surtout ce qui se
cache derrière les noms qui commencent par un point. »

Tetsuo inspecte les registres du temple comme les
greniers: ce qui est visible n'est pas tout le domaine.
Un fichier discret, un nom à peine tracé, peut révéler
un accord oublié ou une dette cachée. L'inventaire doit
porter sur tout, sans exception.

Il attend de la liste qu'elle montre davantage que les
noms: la taille de chaque document, les droits marqués
dessus, qui peut le lire. C'est ce détail qui dira
l'importance de chaque pièce.

Montrez-lui la racine du château avec tout son détail,
même ce qui semble caché. Quand rien ne manquera à la
liste, Tetsuo jugera l'inventaire digne de la table du
conseil.".into(),
            goal: "Montrez au moine Tetsuo le contenu de la racine / en format détaillé, fichiers cachés compris.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment tout lister à la racine?".into(),
            required_commands: vec!["ls -la /".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-la".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ls -la".into(), name: "ls -la".into(), description: "ls -la — tout lister".into() }),
        },
        Mission {
            id: "16.3".into(), chapter: 16, chapter_title: "Inventaire complet".into(), act: 4,
            title: "Tous les .txt".into(),
            description: "Listez tous les fichiers .txt du château.".into(),
            story: "Dame Kaguya: « Seigneur, il me faut le compte exact de
tous les documents du domaine dont le nom se termine
par .txt. Certains reposent dans les archives, d'autres
dans les greniers ou chez les scribes. N'en oubliez
aucun: fouillez le château entier, depuis la racine. »

Chaque document .txt est une parole du domaine: un
registre, un ordre, une dette. Pour clore l'inventaire,
Kaguya doit savoir combien de parchemins existent et où
ils reposent. Un document oublié pourrait reparaître
sous une forme déformée devant le conseil.

Ne cherchez pas pièce par pièce: partez de la racine du
château et remontez chaque recoin, en ne retenant que
les documents dont le nom se termine exactement par
.txt. Leurs noms suffisent, pas leur contenu.

Quand la liste sera posée devant elle, Kaguya comptera
les documents et saura si le domaine a perdu une parole
en route.".into(),
            goal: "Recherchez dans tout le château, depuis la racine, chaque document dont le nom se termine par .txt.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment chercher tous les .txt?".into(),
            required_commands: vec!["find / -name '*.txt'".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "find".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-name".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "find".into(), name: "find".into(), description: "find -name '*.txt' — tous les documents".into() }),
        },
        Mission {
            id: "16.4".into(), chapter: 16, chapter_title: "Inventaire complet".into(), act: 4,
            title: "Etat des remparts".into(),
            description: "Lisez l'état des remparts.".into(),
            story: "Le capitaine des gardes: « Seigneur, une ronde a repéré
des fissures sur le rempart est. Avant de mobiliser les
équipes, je dois connaître l'état exact du mur: ce qui
est consigné dans /remparts/etat_remparts.txt. Lisez-le
pour moi. »

Le rempart est la première ligne du domaine: s'il cède,
les greniers et les quartiers sont à nu. Les pierres
parlent mal, mais les scribes ont consigné chaque
réparation et chaque faiblesse dans ce rapport.

Le capitaine doit prioriser: ce qui menace la base ne
se répare pas comme ce qui fend la crête du mur. Lisez-
lui le rapport tel qu'il est écrit, pour qu'il décide
avec les faits sous les yeux.

La ronde est déjà partie inspecter l'aile ouest. Dès
que vous lui aurez lu l'état consigné, le capitaine
saura quelles équipes envoyer au rempart est avant que
la fissure ne s'élargisse pendant la nuit.".into(),
            goal: "Lisez le fichier etat_remparts.txt.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire l'état des remparts?".into(),
            required_commands: vec!["cat /remparts/etat_remparts.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "etat_remparts.txt".into(), expected: None }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "16.5".into(), chapter: 16, chapter_title: "Inventaire complet".into(), act: 4,
            title: "Stock de riz".into(),
            description: "Lisez le registre du riz.".into(),
            story: "Le marchand Genji: « Seigneur, les greniers du château,
voilà ce qui décide de l'hiver. Avant de conclure mes
achats auprès des villages, je dois connaître le stock
exact de riz consigné dans /greniers/riz/registre_riz.txt.
Lisez-le-moi. »

Le riz est la monnaie du domaine: chaque semis promis,
chaque dette due se mesure en koku. Genji doit savoir
ce que le château possède déjà avant de promettre au
dehors ce qu'il n'a peut-être pas, sous peine de voir
les greniers vides avant les semailles.

Le registre des greniers note chaque entrée et chaque
sortie au fil des saisons: ce qui entre des villages,
ce qui part vers les cuisines et le port. Lisez-lui
son contenu tel qu'il est écrit, sans ajouter ni
retrancher un seul chiffre.

Les marchands du port attendent sa réponse avant la
marée. Avec le stock sous les yeux, Genji pourra dire
ce que le château peut engager sans risquer la disette
et l'hiver gardera sa part.".into(),
            goal: "Lisez le registre du riz des greniers.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire le registre du riz?".into(),
            required_commands: vec!["cat /greniers/riz/registre_riz.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "registre_riz.txt".into(), expected: None }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
    ]
}

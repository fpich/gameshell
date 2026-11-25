//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 3 — Exploration du domaine (Acte 1) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "3.1".into(),
            chapter: 3,
            chapter_title: "Exploration du domaine".into(),
            act: 1,
            title: "Fichiers cachés des quartiers".into(),
            description: "Le sceau secret est un fichier caché.".into(),
            story: "Le moine Tetsuo vous croise dans le couloir des scribes, l'air
préoccupé: « Seigneur, le sceau secret du domaine est conservé dans
vos quartiers. Son nom commence par un point: une liste ordinaire
ne le montre pas, et un œil distrait pourrait croire la pièce
vide. » Il vous remet un rouleau: « Montrez-moi tout ce qui se
trouve dans vos quartiers, y compris les documents cachés, puis
ouvrez le fichier .secret pour vérifier que le sceau y est bien
consigné. »

Vous songez aux coffres dissimulés derrière les panneaux
coulissants du château: comme eux, certains fichiers se voilent
tant qu'on ne demande pas à les voir.

Tetsuo insiste: « Le sceau est la signature de vos ordres. S'il
était perdu, n'importe quel faux édit passerait pour authentique
et les gardes lui obéiraient. »

Vous parcourez le bureau, examinez chaque recoin, puis lisez ce
que renferme le fichier .secret.

Le moine hoche la tête: « Rien n'échappe à un œil attentif,
Seigneur. Le domaine vous est transparent. »"
                .into(),
            goal: "Listez tous vos quartiers (fichiers cachés compris) et lisez .secret.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir aussi les fichiers cachés?".into(),
            required_commands: vec!["ls -la".into(), "cat .secret".into()],
            checks: vec![
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "-la".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "file_read".into(),
                    target: ".secret".into(),
                    expected: None,
                },
            ],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "ls".into(),
                name: "ls".into(),
                description: "ls — liste les fichiers et dossiers".into(),
            }),
        },
        Mission {
            id: "3.2".into(),
            chapter: 3,
            chapter_title: "Exploration du domaine".into(),
            act: 1,
            title: "Trouver un registre".into(),
            description: "Trouvez registre_riz.txt dans tout le château.".into(),
            story: "Dame Kaguya feuillette fébrilement son pupitre: « Seigneur, le
registre du riz a disparu! Le fichier registre_riz.txt était là ce
matin, et maintenant plus rien. Sans lui, impossible de dire ce
que renferment les greniers. »

Elle se tourne vers vous: « Il a pu être rangé n'importe où dans
le château: un scribe empressé, une corvée d'inventaire... Je n'ai
pas le temps de fouiller chaque salle une par une. »

Vous pensez aux étagères interminables des archives, aux coffres
du donjon, aux réserves des greniers: un document égaré au milieu
de tout cela ne se retrouve qu'en le cherchant par son nom, depuis
la racine du domaine, sans laisser un recoin de côté.

Kaguya ajoute: « Cherchez dans tout le château et rapportez-moi le
chemin complet où se trouve registre_riz.txt, pour que je le
replace dans la salle des registres. »

Vous parcourez le domaine du bout des doigts, et le nom du fichier
finit par vous répondre.

Dame Kaguya est rassurée: « Le registre du riz est retrouvé,
Seigneur. Grâce à vous, le domaine ne perd plus rien. »"
                .into(),
            goal: "Trouvez le chemin complet de registre_riz.txt.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment chercher un fichier par nom partout?".into(),
            required_commands: vec!["find / -name registre_riz.txt".into()],
            checks: vec![
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "find".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "-name".into(),
                    expected: None,
                },
            ],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "find".into(),
                name: "find".into(),
                description: "find -name — recherche par nom".into(),
            }),
        },
        Mission {
            id: "3.3".into(),
            chapter: 3,
            chapter_title: "Exploration du domaine".into(),
            act: 1,
            title: "Poids des greniers".into(),
            description: "Quelle taille occupent les greniers?".into(),
            story: "Le marchand Genji vous reçoit devant les greniers, tablettes à la
main: « Seigneur, les cours du riz montent et le clan du sud veut
acheter notre récolte. »

Il désigne les bâtiments derrière lui: « Avant de négocier, j'ai
besoin de savoir combien pèsent les greniers au total, à /greniers
sur la racine du domaine: tous les stocks, sacs, jarres et
réserves, chiffrés en une seule valeur lisible. Pas une liste
dossier par dossier: un total d'ensemble, que je puisse annoncer
au marchand du sud. »

Vous regardez les quatre réserves: riz, blé, saké, miso. Le
terrain est vaste; il faut mesurer l'ensemble d'un seul coup
d'œil, sans additionner chaque recoin à la main.

Genji précise: « Les greniers sont à la racine du domaine. C'est
là que s'entassent les récoltes de l'année, et c'est ce que le
clan du sud veut estimer. »

Vous examinez les réserves et déterminez enfin l'espace occupé par
l'ensemble.

Le marchand note la taille sur sa tablette: « Voilà, Seigneur.
Nous pourrons négocier le riz en connaissance de cause. »"
                .into(),
            goal: "Donnez la taille totale du bâtiment /greniers.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment connaître la taille totale d'un dossier?".into(),
            required_commands: vec!["du -sh /greniers".into()],
            checks: vec![
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "du".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "-sh".into(),
                    expected: None,
                },
            ],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "du -sh".into(),
                name: "du -sh".into(),
                description: "du -sh — taille totale lisible".into(),
            }),
        },
        Mission {
            id: "3.4".into(),
            chapter: 3,
            chapter_title: "Exploration du domaine".into(),
            act: 1,
            title: "Monter au donjon".into(),
            description: "Rejoignez le donjon pour le conseil de guerre.".into(),
            story: "Un garde essoufflé surgit dans vos quartiers: « Seigneur Hattori
convoque immédiatement les chefs au conseil de guerre! Les espions
du clan rival ont été vus près du portail est. »

Le conseil se tient au donjon, la tour centrale du château, à la
racine du domaine: /donjon. C'est là que le seigneur reçoit les
rapports et décide de la défense.

Le garde insiste: « Le seigneur Hattori attend tout le monde avant
la troisième veille. Vous seul pouvez représenter vos quartiers. »

Vous songez à la hauteur de la tour, à ses murs épais, au drapeau
du clan qui flotte au sommet: c'est de là que l'on voit approcher
l'ennemi de loin.

Vous traversez la cour, longez la pagode, puis gravissez les
marches du donjon.

Seigneur Hattori vous accueille au sommet: « Vous voilà, Seigneur.
Le conseil de guerre peut commencer. Les espions approchent, mais
nous sommes prêts. »"
                .into(),
            goal: "Rejoignez le donjon (/donjon).".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment aller au donjon en chemin absolu?".into(),
            required_commands: vec!["cd /donjon".into()],
            checks: vec![MissionCheck {
                check_type: "dir_reached".into(),
                target: "/donjon".into(),
                expected: None,
            }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill {
                command: "cd".into(),
                name: "cd".into(),
                description: "cd — change de répertoire".into(),
            }),
        },
        Mission {
            id: "3.5".into(),
            chapter: 3,
            chapter_title: "Exploration du domaine".into(),
            act: 1,
            title: "Ordre du soir".into(),
            description: "Écrivez l'ordre du soir dans un fichier.".into(),
            story: "Dame Kaguya arrive dans vos quartiers, un rouleau à la main: «
Seigneur, avant la nuit, chaque service doit recevoir ses ordres.
Le portail doit être fermé à la nuit tombée: c'est la consigne que
les gardes attendent. »

Elle vous tend le texte: « Écrivez cette consigne dans le fichier
ordre.txt, sur votre bureau, dans vos quartiers: \"Fermer le
portail à la nuit tombée\". »

Vous pensez aux soirs précédents: quand le portail reste entrouvert
trop tard, les rôdeurs s'approchent des remparts et les gardes
fatiguent. Une consigne écrite vaut mieux qu'un ordre crié dans la
cour.

Le bureau du seigneur est dans vos quartiers: c'est là que dorment
les édits du soir. Posez la consigne à cet endroit, telle que Dame
Kaguya l'a dictée.

Vous ouvrez le fichier ordre.txt et y inscrivez la phrase.

Dame Kaguya relit le fichier: « \"Fermer le portail à la nuit
tombée\". C'est noté, Seigneur. Vos ordres seront exécutés. »"
                .into(),
            goal: "Créez ordre.txt dans vos quartiers avec le texte demandé.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment écrire un texte directement dans un fichier?".into(),
            required_commands: vec!["echo 'Fermer le portail a la nuit tombee' > ordre.txt".into()],
            checks: vec![
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: ">".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "file_exists".into(),
                    target: "~/ordre.txt".into(),
                    expected: None,
                },
            ],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "echo".into(),
                name: "echo".into(),
                description: "echo — affiche du texte à l'écran".into(),
            }),
        },
    ]
}

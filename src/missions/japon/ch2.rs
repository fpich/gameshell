//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 2 — Organisation des quartiers (Acte 1) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "2.1".into(),
            chapter: 2,
            chapter_title: "Organisation des quartiers".into(),
            act: 1,
            title: "Nouveau sceau".into(),
            description: "Créez un fichier vierge pour un nouveau sceau.".into(),
            story: "Dame Kaguya entre dans vos quartiers, un rouleau d'ordres à la
main:
« Seigneur, les consignes de ce soir doivent être consignées par
écrit avant le coucher du soleil. Les scribes noteront chaque
ordre que vous donnerez dans la nuit. »

Elle pose un doigt sur votre bureau:
« Commençons le registre quotidien. Créez un fichier vierge
nommé sceau_du_soir.txt dans vos quartiers. Il recevra les
ordres de la soirée. »

Le registre du domaine exige une trace écrite: un seigneur qui
ordonne sans laisser de sceau laisse ses vassaux sans
instructions claires.

Créez le fichier, puis montrez à Dame Kaguya qu'il est bien là,
en affichant le contenu de vos quartiers avec le détail complet
(tailles, permissions, dates).

Dame Kaguya vérifie le fichier:
« Le sceau du soir est créé, Seigneur. Le registre quotidien
commence. Chaque ordre sera consigné. »"
                .into(),
            goal: "Créez le fichier sceau_du_soir.txt dans vos quartiers.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer un fichier vide?".into(),
            required_commands: vec!["touch sceau_du_soir.txt".into(), "ls -la".into()],
            checks: vec![
                MissionCheck {
                    check_type: "file_exists".into(),
                    target: "~/sceau_du_soir.txt".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "-la".into(),
                    expected: None,
                },
            ],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill {
                command: "touch".into(),
                name: "touch".into(),
                description: "touch — crée un fichier vide".into(),
            }),
        },
        Mission {
            id: "2.2".into(),
            chapter: 2,
            chapter_title: "Organisation des quartiers".into(),
            act: 1,
            title: "Classer les archives".into(),
            description: "Créez l'arborescence archives/2024/lettres.".into(),
            story: "Dame Kaguya déroule un plan sur votre bureau:
« Seigneur, les archives du domaine suivent un classement
strict: d'abord l'année, puis le type de document. Aujourd'hui
nous ouvrons l'année 2024. »

Elle trace un trait sur le plan:
« Créez l'arborescence archives/2024/lettres dans vos
quartiers. Ainsi, chaque lettre entrante aura sa place
définitive. »

Elle précise, en vous regardant:
« Une seule commande suffit, si vous utilisez l'option qui crée
tous les dossiers intermédiaires au passage. »

Vos quartiers accueilleront bientôt copies et courriers; un
classement impeccable évite de chercher un document au milieu
de la nuit.

Dame Kaguya vérifie l'arborescence:
« archives/2024/lettres est en place, Seigneur. Un classement
strict, c'est un domaine bien tenu. »"
                .into(),
            goal: "Créez l'arborescence archives/2024/lettres dans vos quartiers.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer une arborescence complète en une commande?".into(),
            required_commands: vec!["mkdir -p archives/2024/lettres".into()],
            checks: vec![
                MissionCheck {
                    check_type: "dir_exists".into(),
                    target: "~/archives/2024/lettres".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "-p".into(),
                    expected: None,
                },
            ],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "mkdir -p".into(),
                name: "mkdir -p".into(),
                description: "mkdir -p — crée toute l'arborescence".into(),
            }),
        },
        Mission {
            id: "2.3".into(),
            chapter: 2,
            chapter_title: "Organisation des quartiers".into(),
            act: 1,
            title: "Copier la lettre aux archives".into(),
            description: "Gardez une copie de la lettre Hattori dans les archives.".into(),
            story: "Dame Kaguya vous croise dans le couloir des archives:
« Seigneur, la lettre du clan Hattori est trop précieuse pour
rester dans le seul dossier des lettres entrantes. Les archives
du château gardent désormais une copie classée par année. »

Elle désigne la grande salle:
« Copiez /archives/lettres/lettre_clan_hattori.txt dans le
dossier /archives/2024/lettres/, en conservant l'original en
place. »

Les archives centrales sont organisées par année depuis la
fondation du château: un document rangé là ne se perd jamais,
même si le dossier des entrées est remanié.

Copiez le document, sans le déplacer. Dame Kaguya attend de
voir la copie enregistrée avant de clore le registre.

Dame Kaguya confirme:
« La lettre du clan Hattori est en sûreté dans les archives,
Seigneur. Rien ne se perdra. »"
                .into(),
            goal: "Copiez lettre_clan_hattori.txt vers /archives/2024/lettres/.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment copier un fichier vers un autre dossier?".into(),
            required_commands: vec![
                "mkdir -p /archives/2024/lettres".into(),
                "cp /archives/lettres/lettre_clan_hattori.txt /archives/2024/lettres/".into(),
            ],
            checks: vec![
                MissionCheck {
                    check_type: "file_exists".into(),
                    target: "/archives/2024/lettres/lettre_clan_hattori.txt".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "cp".into(),
                    expected: None,
                },
            ],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "cp".into(),
                name: "cp".into(),
                description: "cp — copie un fichier".into(),
            }),
        },
        Mission {
            id: "2.4".into(),
            chapter: 2,
            chapter_title: "Organisation des quartiers".into(),
            act: 1,
            title: "Renommer un document".into(),
            description: "Renommez sceau_du_soir.txt en ordre_du_soir.txt.".into(),
            story: "Dame Kaguya revient vers vous, un sceau à la main:
« Seigneur, réflexion faite, le fichier sceau_du_soir.txt porte
un nom trompeur. Il consigne des ordres, pas un sceau. »

Elle hoche la tête:
« Renommez-le ordre_du_soir.txt. Ainsi, les scribes sauront
immédiatement ce qu'il contient. »

Les noms des documents du domaine doivent être explicites: un
messager pressé ne lit pas le contenu, il se fie au nom pour
acheminer le rouleau.

Renommez le fichier dans vos quartiers. L'ancien nom ne doit
plus subsister nulle part dans le registre.

Dame Kaguya approuve:
« ordre_du_soir.txt, c'est plus clair, Seigneur. Vos quartiers
sont bien tenus. »"
                .into(),
            goal: "Renommez sceau_du_soir.txt en ordre_du_soir.txt.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment renommer un fichier?".into(),
            required_commands: vec!["mv sceau_du_soir.txt ordre_du_soir.txt".into()],
            checks: vec![
                MissionCheck {
                    check_type: "file_exists".into(),
                    target: "~/ordre_du_soir.txt".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "file_not_exists".into(),
                    target: "~/sceau_du_soir.txt".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "mv".into(),
                    expected: None,
                },
            ],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "mv".into(),
                name: "mv".into(),
                description: "mv — déplace ou renomme".into(),
            }),
        },
        Mission {
            id: "2.5".into(),
            chapter: 2,
            chapter_title: "Organisation des quartiers".into(),
            act: 1,
            title: "Faire le ménage".into(),
            description: "Supprimez les brouillons inutiles.".into(),
            story: "Dame Kaguya contemple vos quartiers d'un œil sévère:
« Seigneur, ces brouillons qui traînent sur votre bureau
prêtent à confusion. Un serviteur pourrait les prendre pour des
ordres et les exécuter. »

Elle désigne les deux feuillets:
« brouillon1.txt et brouillon2.txt sont des notes à jeter. Le
registre du domaine ne doit contenir que des documents
officiels. Supprimez-les. »

Un domaine bien tenu ne garde pas d'écrits non datés et non
signés: ils n'ont aucune valeur d'archives et encombrent le
classement.

Supprimez les deux brouillons de vos quartiers, puis montrez
que votre bureau est net.

Dame Kaguya sourit:
« Vos quartiers sont enfin ordonnés, Seigneur. Un seigneur
rangé est un seigneur qui voit clair. »"
                .into(),
            goal: "Supprimez les fichiers brouillon de vos quartiers.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment supprimer un fichier?".into(),
            required_commands: vec!["rm brouillon1.txt brouillon2.txt".into()],
            checks: vec![
                MissionCheck {
                    check_type: "file_not_exists".into(),
                    target: "~/brouillon1.txt".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "file_not_exists".into(),
                    target: "~/brouillon2.txt".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "rm".into(),
                    expected: None,
                },
            ],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "rm".into(),
                name: "rm".into(),
                description: "rm — supprime un fichier".into(),
            }),
        },
    ]
}

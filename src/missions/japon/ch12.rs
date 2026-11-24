//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 12 — Archives du domaine (Acte 3) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "12.1".into(), chapter: 12, chapter_title: "Archives du domaine".into(), act: 3,
            title: "Archiver les registres".into(),
            description: "Archive compressée des logs vers les archives.".into(),
            story: "Dame Kaguya: « Seigneur, le convoi vers la capitale part
avant l'aube. Avant de sceller les caisses, mettez tous
les logs des scribes en sûreté dans une seule caisse
compressée: /archives/backup_logs.tar.gz. »

Les scribes consignent chaque nuit les allées et venues
au portail dans /scribes/logs/auth.log et l'activité du
château dans /scribes/logs/syslog. Si une seule de ces
pages se perd, l'enquête sur les espions perd ses
preuves. Tout le dossier /scribes/logs/ doit entrer dans
la caisse, sans exception.

Le moine Tetsuo a signalé des traces fraîches près des
remparts est. Le seigneur Hattori a ordonné de préparer
le départ avant l'aube. Vous descendez aux archives, une
lanterne à la main. Une seule caisse, bien scellée,
compressée pour traverser la route: le domaine ne doit
rien laisser traîner derrière lui.

Quand la caisse sera prête, Dame Kaguya la fera partir
avec le premier convoi.
".into(),
            goal: "Créez /archives/backup_logs.tar.gz contenant /scribes/logs/.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer une archive compressée?".into(),
            required_commands: vec!["tar czf /archives/backup_logs.tar.gz /scribes/logs/".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tar".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "czf".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "/archives/backup_logs.tar.gz".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "tar czf".into(), name: "tar czf".into(), description: "tar czf — archive compressée".into() }),
        },
        Mission {
            id: "12.2".into(), chapter: 12, chapter_title: "Archives du domaine".into(), act: 3,
            title: "Vérifier l'archive".into(),
            description: "Contenu de l'archive sans l'extraire.".into(),
            story: "Dame Kaguya: « La caisse part au lever du soleil. Avant
qu'elle ne quitte le château, montrez-moi ce qu'elle
contient, en détail, sans l'ouvrir. »

Vous la descendez de l'étagère des archives et vous vous
apprêtez à rendre compte. La dame veut voir chaque
fichier de /archives/backup_logs.tar.gz: les noms, les
tailles, tout. Rien ne doit rester caché dans la caisse.

Le scribe en chef attend la liste pour signer son
registre de départ. S'il manque une page, l'archive ne
sera pas scellée et le convoi partira incomplet. Il a
consigné l'inventaire exact lors de l'écriture: c'est
votre liste qui doit le confirmer.

Vous soulevez le couvercle de la caisse avec précaution:
la vapeur des herbes à l'intérieur. Non, ce n'est que le
parfum des archives. Voilà: la liste s'affiche, fichier
par fichier. Les pages sont toutes là.
".into(),
            goal: "Montrez à Dame Kaguya le contenu détaillé de l'archive /archives/backup_logs.tar.gz, sans l'extraire.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lister une archive sans l'extraire?".into(),
            required_commands: vec!["tar tvf /archives/backup_logs.tar.gz".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tvf".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "tar tvf".into(), name: "tar tvf".into(), description: "tar tvf — liste une archive".into() }),
        },
        Mission {
            id: "12.3".into(), chapter: 12, chapter_title: "Archives du domaine".into(), act: 3,
            title: "Extraire l'archive".into(),
            description: "Extrayez l'archive dans /tmp.".into(),
            story: "Le scribe en chef: « Seigneur, la caisse est arrivée
saine et sauve sur les étagères. Dame Kaguya veut
contrôler les pages une à une avant de clore l'inventaire
des espoirs. Décompressez l'archive dans /tmp, pour
qu'elle puisse vérifier sur place. »

Vous ne devez pas toucher au contenu original: l'archive
doit rester intacte dans /archives/backup_logs.tar.gz.
Seule la zone de travail /tmp recevra les fichiers
sortis de la caisse. Ainsi, si une page est déchirée,
l'original reste à l'abri et la comparaison est possible.

Le scribe pose la lanterne sur la table et vous tend le
rouleau récapitulatif. Pendant que vous travaillez, il
murmure les noms des fichiers, comme s'il les comptait.
Au bout du couloir, les gardes changent de poste. La
vérification peut commencer dès que les fichiers sont
posés dans /tmp.

Le scribe aligne ses outils sur la table: rien ne
doit manquer d'ici la fin de la corvée de nuit.
".into(),
            goal: "Décompressez et extrayez l'archive /archives/backup_logs.tar.gz dans le répertoire /tmp pour que Dame Kaguya la vérifie.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment extraire dans un dossier précis?".into(),
            required_commands: vec!["tar xzf /archives/backup_logs.tar.gz -C /tmp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "xzf".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-C".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "tar xzf".into(), name: "tar xzf".into(), description: "tar xzf — extrait une archive".into() }),
        },
        Mission {
            id: "12.4".into(), chapter: 12, chapter_title: "Archives du domaine".into(), act: 3,
            title: "Archiver en ZIP".into(),
            description: "Le clan Hattori préfère le ZIP.".into(),
            story: "Genji le marchand: « Seigneur, le clan Hattori de la
capitale utilise des colis au format ZIP pour ses
registres. Ils ont demandé celui des impôts de la salle
du trône. Créez registres.zip avec le dossier
/salle-du-trone/registres/, et préparez-le dans vos
quartiers: je viendrai le prendre au matin. »

La salle du trône abrite le registre_impots.txt, la
liste de toutes les redevances du domaine. Le clan
allié veut pouvoir l'ouvrir avec ses propres outils,
d'où le format ZIP, la formule qu'ils emploient pour
toutes leurs transmissions.

Vous rentrez dans vos quartiers, refermez la porte et
vous mettez à l'ouvrage. Le fichier doit tenir dans un
seul colis compact, comme le marchand l'a demandé.
L'écho des pas de Genji s'atténue dans le couloir:
vous avez jusqu'au matin.

Les comptes du domaine reposent sur ce registre:
une page perdue, et le clan de la capitale perd
confiance dans notre parole.
".into(),
            goal: "Créez registres.zip contenant /salle-du-trone/registres/.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer une archive ZIP?".into(),
            required_commands: vec!["zip -r registres.zip /salle-du-trone/registres/".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "zip".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-r".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "~/registres.zip".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "zip -r".into(), name: "zip -r".into(), description: "zip -r — archive ZIP".into() }),
        },
        Mission {
            id: "12.5".into(), chapter: 12, chapter_title: "Archives du domaine".into(), act: 3,
            title: "Lire le ZIP".into(),
            description: "Contenu de registres.zip.".into(),
            story: "Genji se présente à l'aube, le sac de cuir à l'épaule:
« Seigneur, je n'embarque pas un colis dont je ne
connais pas le contenu. Montrez-moi la liste des
fichiers de registres.zip, sans rien décompresser.
Je vérifie, je signe, et le corbeau messager s'envole. »

Le format pourra attendre: ce que le marchand veut
voir, c'est la preuve que le registre des impôts est
bien dans la caisse. Ouvrir le colis, le déballer,
serait inutile et risqué: il suffit de faire défiler
la liste devant lui, fichier par fichier.

Vous prenez registres.zip dans vos quartiers et vous
lui montrez. Genji hoche la tête à chaque ligne, l'œil
vif. Au bout du compte, il appose son sceau sur le
parchemin et glisse le colis dans son sac. Le corbeau
attend sur le toit de la pagode.

La lune pâlit derrière les remparts. Le temps des
vérifications touche à sa fin: une fois la liste
officialisée, le corbeau filera vers la capitale.
".into(),
            goal: "Montrez à Genji la liste des fichiers contenus dans registres.zip, sans rien décompresser.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lister un ZIP?".into(),
            required_commands: vec!["unzip -l registres.zip".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "unzip".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "unzip -l".into(), name: "unzip -l".into(), description: "unzip -l — liste un ZIP".into() }),
        },
    ]
}

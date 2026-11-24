//! CHAPITRE 15 - Archives et sauvegardes (Acte 4) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "15.1".into(), chapter: 15, chapter_title: "Archives et sauvegardes".into(), act: 4,
            title: "Archiver les rapports".into(),
            description: "Tael Rhen veut une archive compressée des rapports dans les soutes.".into(),
            story: "Tael Rhen vous rejoint sur le pont, un casier de transport à la main :
« Capitaine, l'Andromède veut une copie de nos rapports avant la prochaine
relève. Ne partons pas avec seulement les originaux dans la cabine. »

Il pose le casier : « Créez une archive compressée de tout le dossier
rapports/ de votre cabine, et déposez-la dans les soutes : la caisse
s'appellera backup_rapports.tar.gz. Sous-dossiers compris. »

Vous vérifiez votre cabine : le dossier rapports/ est bien là, rangé par
année et par mois, avec le rapport incident_15jan.txt et le contrôle
moteur.

Votre commande terminée, la caisse backup_rapports.tar.gz est dans les
soutes, à côté des autres réserves. Les rapports y sont en sécurité.

« Une sauvegarde, c'est une assurance », explique Tael Rhen. « Si les
originaux se perdent, la copie nous sauve. »".into(),
            goal: "Créez une archive compressée /soutes/backup_rapports.tar.gz contenant le dossier rapports/.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer une archive compressée d'un dossier?".into(),
            required_commands: vec!["tar czf /soutes/backup_rapports.tar.gz rapports/".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tar".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "/soutes/backup_rapports.tar.gz".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "tar czf".into(), name: "tar".into(), description: "tar czf — crée une archive compressée .tar.gz".into() }),
        },
        Mission {
            id: "15.2".into(), chapter: 15, chapter_title: "Archives et sauvegardes".into(), act: 4,
            title: "Vérifier le contenu de l'archive".into(),
            description: "Qu'y a-t-il dans backup_rapports.tar.gz?".into(),
            story: "Tael Rhen examine la caisse qui vient d'arriver dans les soutes :
« Capitaine, avant d'envoyer cette archive à l'Andromède, vérifions ce
qu'elle contient réellement. Une caisse envoyée à vide, ce serait une
déception pour l'Amiral. »

Il se tourne vers vous : « Listez le contenu de la caisse
backup_rapports.tar.gz, sans en sortir les fichiers. Vous verrez d'un
coup d'œil si tous nos rapports sont dedans. »

Vous passez en revue le contenu affiché : le dossier rapports/, son
sous-dossier 2024/janvier, et les fichiers qu'il contient sont bien
repris.

« Vérifier avant d'envoyer, c'est éviter les mauvaises surprises »,
note Tael Rhen. « Un coup d'œil suffit. »".into(),
            goal: "Listez le contenu de l'archive /soutes/backup_rapports.tar.gz sans l'extraire.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lister le contenu d'une archive sans l'extraire?".into(),
            required_commands: vec!["tar tvf /soutes/backup_rapports.tar.gz".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tvf".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "tar tvf".into(), name: "tar tvf".into(), description: "tar tvf — liste le contenu d'une archive sans l'extraire".into() }),
        },
        Mission {
            id: "15.3".into(), chapter: 15, chapter_title: "Archives et sauvegardes".into(), act: 4,
            title: "Extraire l'archive".into(),
            description: "Extrayez l'archive dans le compartiment temporaire /tmp.".into(),
            story: "Tael Rhen déplie un plan du compartiment temporaire sur la table :
« Capitaine, la caisse backup_rapports.tar.gz est dans les soutes, mais
nous allons devoir examiner ses fichiers de près. N'ouvrez pas la caisse
sur place : extrayez son contenu dans le compartiment temporaire, le
dossier /tmp, sans toucher aux soutes. »

Il ajoute : « Le dossier temporaire est fait pour ce genre de travail.
Les originaux restent rangés dans les soutes pendant qu'on inspecte la
copie. »

Votre commande terminée, les fichiers de la caisse sont déposés dans
/tmp, prêts à être consultés. La caisse elle-même reste intacte dans les
soutes.

« Extraire dans un espace de travail, c'est plus sûr », explique
Tael Rhen. « On ne touche pas aux originaux. »".into(),
            goal: "Extrayez l'archive /soutes/backup_rapports.tar.gz dans le répertoire /tmp.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment extraire une archive dans un dossier précis?".into(),
            required_commands: vec!["tar xzf /soutes/backup_rapports.tar.gz -C /tmp".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "xzf".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "tar xzf".into(), name: "tar xzf".into(), description: "tar xzf — extrait une archive compressée".into() }),
        },
        Mission {
            id: "15.4".into(), chapter: 15, chapter_title: "Archives et sauvegardes".into(), act: 4,
            title: "Lire une archive ZIP".into(),
            description: "L'Andromède préfère le ZIP: listez rapports.zip.".into(),
            story: "Nira Okoro, la responsable des communications, vous appelle depuis la
salle des messages, un écran à la main :
« Capitaine, l'Andromède préfère recevoir nos échanges au format ZIP.
Vous avez déjà préparé rapports.zip dans votre cabine. »

Elle désigne l'écran : « Montrez-moi ce que contient ce fichier
rapports.zip, sans le déballer : je dois vérifier que tous nos rapports
y sont avant de l'envoyer au vaisseau-mère. »

Vous listez l'archive depuis votre cabine : le dossier rapports/, ses
sous-dossiers et les fichiers du mois de janvier y figurent bien.

« Le format ZIP est universel », note Nira Okoro. « C'est le langage
commun des vaisseaux. »".into(),
            goal: "Listez le contenu de l'archive rapports.zip de votre cabine.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lister le contenu d'une archive ZIP?".into(),
            required_commands: vec!["unzip -l rapports.zip".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "unzip".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "unzip -l".into(), name: "unzip".into(), description: "unzip -l — liste le contenu d'une archive ZIP".into() }),
        },
        Mission {
            id: "15.5".into(), chapter: 15, chapter_title: "Archives et sauvegardes".into(), act: 4,
            title: "Etat des reservoirs".into(),
            description: "Tael Rhen veut la jauge des réservoirs de données du vaisseau.".into(),
            story: "Tael Rhen vous rejoint à la passerelle, un bloc-notes à la main :
« Capitaine, avant de charger de nouvelles données, il faut savoir où
nous en sommes. Chaque compartiment de stockage a sa jauge : total,
utilisé et restant. »

Il tend le bloc : « Donnez-moi la jauge de toutes les partitions du
vaisseau, avec les tailles en format lisible. Je dois savoir si nous
pouvons encore charger les cartes de la prochaine mission. »

Vous affichez la jauge : la partition racine, les soutes et les archives
montrent leur capacité, ce qui est occupé et ce qui reste, en taille
humaine.

« Bien », approuve Tael Rhen. « Nous savons exactement combien de
données nous pouvons encore stocker avant saturation. »".into(),
            goal: "Affichez l'espace disque utilisé et disponible de toutes les partitions, en taille lisible.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'espace disque restant?".into(),
            required_commands: vec!["df -h".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "df".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "df -h".into(), name: "df".into(), description: "df -h — espace disque utilisé/disponible en taille lisible".into() }),
        },
    ]
}

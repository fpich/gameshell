//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 4 — Registres du domaine (Acte 1) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "4.1".into(), chapter: 4, chapter_title: "Registres du domaine".into(), act: 1,
            title: "Lire vos notes".into(),
            description: "Relisez notes.txt.".into(),
            story: "Dame Kaguya pousse la porte de vos quartiers, un plateau de thé à la main: « Seigneur, avant de recevoir les rapports de la journée, vous devriez relire vos notes. »

Elle désigne le bureau: « Le carnet notes.txt est resté ouvert toute la nuit. Vous y avez consigné les affaires en cours: le riz des greniers, les lettres du clan Hattori, le code du coffre. Un détail oublié peut coûter cher. »

Vous vous souvenez du soir précédent: les scribes passaient les listes deux à deux, et vous notiez chaque tâche pour ne rien laisser échapper. Vos notes sont la mémoire du seigneur.

Vous prenez le carnet, l'ouvrez, et relisez notes.txt ligne après ligne.

Dame Kaguya observe, puis hoche la tête: « Voilà, Seigneur. Tout est en tête. Le registre de vos obligations est à jour. »".into(),
            goal: "Lisez notes.txt dans vos quartiers.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier?".into(),
            required_commands: vec!["cat notes.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "notes.txt".into(), expected: None }],
            xp_reward: 50,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche le contenu".into() }),
        },
        Mission {
            id: "4.2".into(), chapter: 4, chapter_title: "Registres du domaine".into(), act: 1,
            title: "Ajouter au registre".into(),
            description: "Ajoutez une ligne à la fin de notes.txt.".into(),
            story: "Dame Kaguya revient de la cour, un rouleau froissé à la main: « Seigneur, un messager du rempart est vient de déposer ceci: le mur de l'est s'affaisse par endroits. Il faut le réparer avant l'hiver. »

Elle fait glisser vos notes sur le bureau: « Ajoutez cette tâche à la fin de notes.txt, sans effacer ce qui est déjà écrit. La consigne exacte des charpentiers est: Reparer le rempart est. »

Vous songez aux édits du domaine: chaque tâche doit être consignée pour être exécutée. Vos notes ne doivent rien perdre des affaires en cours.

Vous prenez le carnet, ajoutez la ligne à la suite des précédentes, et replacez notes.txt sur le bureau.

Dame Kaguya relit le carnet: « Reparer le rempart est est bien consigné, après tout le reste, sans rien écraser. Le registre est complet, Seigneur. »".into(),
            goal: "Ajoutez une ligne à la fin de notes.txt sans l'écraser.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter à la fin d'un fichier?".into(),
            required_commands: vec!["echo 'Reparer le rempart est' >> notes.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: ">>".into(), name: ">>".into(), description: "COMMANDE >> fichier — ajoute à la fin".into() }),
        },
        Mission {
            id: "4.3".into(), chapter: 4, chapter_title: "Registres du domaine".into(), act: 1,
            title: "Compter les lignes du kaisho".into(),
            description: "Combien de lignes fait kaisho.txt?".into(),
            story: "Le scribe en chef vous attend au pupitre des registres, un pinceau levé: « Seigneur, j'établis le registre des visiteurs du château. J'ai besoin du compte exact des lignes du kaisho. »

Il désigne le registre d'accueil: « Le fichier kaisho.txt, dans vos quartiers, accueille chaque invité et chaque ordre du jour. Pour le relier, il me faut sa longueur: combien de lignes le composent? Juste le nombre. »

Vous vous souvenez des rouleaux des scribes: avant de relier un registre, on compte ses lignes pour vérifier qu'aucune page ne manque.

Vous allez chercher le kaisho, comptez ses lignes une à une, et revenez annoncer le nombre au scribe.

Le scribe note le chiffre sur sa tablette: « Voilà la longueur du kaisho, Seigneur. Le registre des visiteurs est à jour. »".into(),
            goal: "Comptez les lignes de kaisho.txt.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment compter les lignes?".into(),
            required_commands: vec!["wc -l kaisho.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "wc".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "wc -l".into(), name: "wc -l".into(), description: "wc -l — compte les lignes".into() }),
        },
        Mission {
            id: "4.4".into(), chapter: 4, chapter_title: "Registres du domaine".into(), act: 1,
            title: "Début du journal des gardes".into(),
            description: "Les 5 premières lignes du journal des gardes.".into(),
            story: "Le capitaine des gardes vous accueille en haut des marches du donjon, un livre épais sous le bras: « Seigneur, je dois rendre compte de la nuit à Hattori. Montrez-moi ce que dit le registre des rondes. »

Il ouvre le registre des rondes: « Le journal des gardes, journal_gardes.txt, rangé dans /donjon/gardes, consigne chaque tour de veille depuis minuit. Les cinq premières lignes suffisent pour le rapport du matin: les rondes du nord, le bruit du portail est, la relève. »

Vous savez que le début d'un registre montre l'ordre des événements: ce qui s'est passé en premier éclaire le reste.

Vous vous penchez sur le registre, lisez les premières consignes, et présentez au capitaine le début du journal.

Le capitaine lit par-dessus votre épaule: « Les premières rondes sont consignées, Seigneur. La garde a fait son devoir toute la nuit. »".into(),
            goal: "Affichez les 5 premières lignes du journal des gardes.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment n'afficher que le début?".into(),
            required_commands: vec!["head -5 /donjon/gardes/journal_gardes.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "head".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-5".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "head".into(), name: "head".into(), description: "head -n N — N premières lignes".into() }),
        },
        Mission {
            id: "4.5".into(), chapter: 4, chapter_title: "Registres du domaine".into(), act: 1,
            title: "Fin du journal système".into(),
            description: "Les 3 dernières lignes du syslog.".into(),
            story: "Le moine Tetsuo vous croise dans la cour de la pagode, un parchemin roulé sous le bras: « Seigneur, les scribes tiennent le journal des événements du château, le syslog, rangé dans /scribes/logs. Les dernières heures y sont consignées minute par minute. »

Il le déroule à demi: « Montrez-moi les trois dernières consignes. Ce sont les plus récentes nouvelles du domaine: la ronde de l'est, la cuisine, les dernières entrées. »

Vous vous souvenez que la fin d'un journal dit ce qui vient de se passer, tandis que le début raconte le passé lointain.

Vous allez aux archives des scribes, lisez la fin du syslog, et revenez rapporter les dernières consignes au moine.

Tetsuo écoute, puis hoche la tête: « Les dernières nouvelles du château sont claires, Seigneur. Rien d'inquiétant pour l'instant. Le domaine veille. »".into(),
            goal: "Affichez les 3 dernières lignes du syslog.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment n'afficher que la fin?".into(),
            required_commands: vec!["tail -3 /scribes/logs/syslog".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tail".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-3".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "tail".into(), name: "tail".into(), description: "tail -n N — N dernières lignes".into() }),
        },
    ]
}

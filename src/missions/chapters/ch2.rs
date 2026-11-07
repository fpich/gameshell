//! CHAPITRE 2 - Organisation de la cabine
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "2.1".into(), chapter: 2, chapter_title: "Organisation de la cabine".into(), act: 1,
            title: "Creer un rapport".into(),
            description: "Jean a besoin d'un fichier vierge.".into(),
            story: "Jean, l'officier sécurité, passe la tête par l'embrasure de la porte de votre cabine.
« Capitaine, on démarre l'enquête sur les attaques. J'ai besoin d'un rapport vierge
pour consigner toutes nos observations. »

Il vous explique la procédure: chaque incident doit être documenté dans un fichier dédié,
avec la date, l'heure, les systèmes concernés et les actions entreprises.

« Créez un fichier incident_15jan.txt sur votre bureau, Capitaine.
C'est la première étape de la chaîne de preuves. Sans rapport, l'Amiral ne peut pas
justifier l'envoi de renforts. »

Il ajoute: « Et n'oubliez pas, Capitaine: dans l'espace, la paperasse sauve des vies.
Un bon rapport peut faire la différence entre une station sauvée et une station tombée. »

Vous devez créer ce fichier vide pour commencer le rapport officiel.".into(),
            goal: "Creez incident_15jan.txt (touch) et verifiez (ls).".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Comment creer un fichier vide?".into(),
            required_commands: vec!["touch".into(), "ls".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "touch".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "~/incident_15jan.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "touch".into(), name: "touch".into(), description: "touch".into() }),
        },
        Mission {
            id: "2.2".into(), chapter: 2, chapter_title: "Organisation de la cabine".into(), act: 1,
            title: "Classer les rapports".into(),
            description: "Creez une arborescence mensuelle.".into(),
            story: "Marie, depuis la salle des machines, vous fait remarquer que votre cabine ressemble
à un champ de bataille après une bataille spatiale.
« Capitaine, on ne retrouve plus rien dans vos affaires!
Il nous faut une vraie arborescence pour classer les rapports. »

Elle vous propose un standard de classement utilisé sur toute la flotte:
rapports/AAAA/mois/ pour chaque rapport mensuel.

« Comme ça, dit-elle, quand l'Amiral demande le rapport de janvier 2024,
vous savez exactement où le trouver en 3 secondes. »

Elle insiste sur l'option -p qui crée toute l'arborescence d'un coup,
évitant de créer chaque dossier un par un.

« Créez rapports/2024/janvier dans votre cabine, Capitaine. Une seule commande suffit. »".into(),
            goal: "Creez ~/rapports/2024/janvier en UNE commande (mkdir -p).".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Comment creer des dossiers imbriques?".into(),
            required_commands: vec!["mkdir".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "mkdir".into(), expected: None }, MissionCheck { check_type: "dir_exists".into(), target: "~/rapports/2024/janvier".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "mkdir".into(), name: "mkdir".into(), description: "mkdir".into() }),
        },
        Mission {
            id: "2.3".into(), chapter: 2, chapter_title: "Organisation de la cabine".into(), act: 1,
            title: "Sauvegarder dans les soutes".into(),
            description: "Copiez le rapport dans les soutes.".into(),
            story: "Jean revient, plus sérieux cette fois: « Capitaine, avant qu'on fasse quoi que ce soit
de plus, il faut sauvegarder ce rapport dans les soutes sécurisées.
Les pirates ont déjà tenté de modifier nos fichiers une fois. »

Il vous raconte l'incident du mois dernier: un fichier de logs a été altéré,
et personne n'a pu prouver qui était l'attaquant.

« Depuis, on copie chaque rapport important dans /soutes/ dès sa création.
Les soutes sont le compartiment le plus protégé du vaisseau,
avec un blindage triple et une surveillance constante. »

Il vous donne la procédure exacte: copier le fichier depuis votre cabine
vers le répertoire /soutes/ du vaisseau.

« Faites-le maintenant, Capitaine. On ne sait jamais quand les pirates frapperont. »".into(),
            goal: "Copiez ~/incident_15jan.txt vers /soutes/ (cp).".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Comment copier un fichier?".into(),
            required_commands: vec!["cp".into(), "ls".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "cp".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "/soutes/incident_15jan.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "cp".into(), name: "cp".into(), description: "cp".into() }),
        },
        Mission {
            id: "2.4".into(), chapter: 2, chapter_title: "Organisation de la cabine".into(), act: 1,
            title: "Classer definitivement".into(),
            description: "Deplacez le rapport dans son dossier final.".into(),
            story: "Marie secoue la tête en voyant votre bureau: « Capitaine, ce fichier incident_15jan.txt
traîne encore à la racine de votre cabine! Il doit être dans rapports/2024/janvier/! »

Elle explique l'importance d'un classement rigoureux:
« Quand on aura 50 rapports, si tout est en vrac, on passera des heures à chercher.
Et pendant ce temps, les pirates avancent. »

Elle vous montre la commande qui fait les deux opérations en une seule:
déplacer le fichier ET le remettre dans le bon dossier.

« mv fait les deux: déplacer et renommer. C'est comme prendre un classeur
et le ranger dans la bonne étagère avec la bonne étiquette, en un seul geste. »

« Déplacez ~/incident_15jan.txt vers ~/rapports/2024/janvier/, Capitaine. »".into(),
            goal: "Deplacez ~/incident_15jan.txt vers ~/rapports/2024/janvier/ (mv).".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Comment deplacer/renommer?".into(),
            required_commands: vec!["mv".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "mv".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "~/rapports/2024/janvier/incident_15jan.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "mv".into(), name: "mv".into(), description: "mv".into() }),
        },
        Mission {
            id: "2.5".into(), chapter: 2, chapter_title: "Organisation de la cabine".into(), act: 1,
            title: "Nettoyer".into(),
            description: "Virez les fichiers temporaires.".into(),
            story: "Pierre, le mécanicien, passe par votre cabine en courant:
« Capitaine! J'ai laissé traîner des fichiers temporaires partout!
temp1.txt, temp2.txt, temp3.txt... Il faut nettoyer avant l'inspection de l'Amiral! »

Il explique que les fichiers temporaires s'accumulent vite sur un vaisseau,
et qu'ils prennent de la place inutile dans les systèmes de stockage.

« En plus, dit-il, si un pirate tombe sur un fichier temporaire,
il peut y trouver des infos sensibles. Même les fichiers de test. »

Il vous demande de créer ces 3 fichiers pour simuler le nettoyage,
puis de les supprimer proprement.

« rm supprime définitivement. Attention, dans l'espace, pas de corbeille:
une fois supprimé, c'est parti pour de bon. »".into(),
            goal: "Creez temp1.txt temp2.txt temp3.txt puis supprimez-les (rm).".into(),
            bloom_level: "Apply".into(),
            bloom_question: "Comment supprimer des fichiers?".into(),
            required_commands: vec!["rm".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "rm".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "rm".into(), name: "rm".into(), description: "rm".into() }),
        },
    ]
}

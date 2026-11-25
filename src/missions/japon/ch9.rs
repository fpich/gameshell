//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 9 — Enquête: les espions (Acte 3) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "9.1".into(), chapter: 9, chapter_title: "Enquete: les espions".into(), act: 3,
            title: "Lire l'alerte".into(),
            description: "Une alerte ronin a été levée.".into(),
            story: "Le capitaine des gardes vous attend au seuil du donjon, la mâchoire serrée: « Seigneur, un courrier urgent est déjà rédigé aux scribes. Les hommes ont repéré une silhouette rôdant autour du rempart est, trois nuits de suite. Avant de donner le moindre ordre, je veux que vous lisiez l'alerte et que vous sachiez ce que nous avons rapporté: le fichier alerte_ronin.txt se trouve dans le dossier /scribes/alertes. »

Vous sentez le poids de la nuit sur vos épaules. Chaque nuage qui glisse devant la lune change les ombres entre les créneaux, et le capitaine attend votre parole.

Dame Kaguya vous rejoint, une bougie à la main: « Nous avons besoin de vous, Seigneur. Si cette alerte confirme ce que je crains, la garde de nuit doit être renforcée avant l'aube. »

Le moine Tetsuo passe sans un bruit, psalmodiant pour la vigilance du domaine.

Vous hochez la tête: il est temps de lire ce que les scribes ont consigné.".into(),
            goal: "Lisez le fichier alerte_ronin.txt.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier d'alerte?".into(),
            required_commands: vec!["cat /scribes/alertes/alerte_ronin.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "alerte_ronin.txt".into(), expected: None }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "9.2".into(), chapter: 9, chapter_title: "Enquete: les espions".into(), act: 3,
            title: "Tous les registres".into(),
            description: "Listez tous les fichiers 'registre*' du château.".into(),
            story: "Dame Kaguya déroule un plan du château sur la table, le doigt courant d'un bâtiment à l'autre: « Seigneur, les registres sont la mémoire du domaine: impôts, rizières, prison, forge... Je dois m'assurer que TOUS les registres sont bien rangés et qu'aucun ne manque à l'appel. »

Elle pousse le plan vers vous: « Le problème, c'est qu'ils sont éparpillés dans tout le château, chacun dans son bâtiment. Je ne connais que leur nom: ils commencent tous par 'registre'. »

Vous contemplez le plan: greniers, donjon, scribes, forge, portail... Des dizaines de pièces, et autant de recoins où un rouleau peut s'égarer.

Le moine Tetsuo, qui méditait près de la porte, ouvre un œil: « Un domaine qui connaît ses registres ne se laisse pas surprendre, Seigneur. »

La nuit est tombée. Si un registre manque, ce sera demain matin, au rapport, que le manque se verra.

Il faut parcourir le château en entier, bâtiment par bâtiment, et montrer à Kaguya chaque fichier dont le nom commence par 'registre'. Tous, sans exception.".into(),
            goal: "Listez tous les fichiers dont le nom commence par 'registre'.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment chercher avec un motif?".into(),
            required_commands: vec!["find / -name 'registre*'".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "find".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-name".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "find".into(), name: "find".into(), description: "find -name — motif".into() }),
        },
        Mission {
            id: "9.3".into(), chapter: 9, chapter_title: "Enquete: les espions".into(), act: 3,
            title: "Journal des gardes complet".into(),
            description: "Lisez tout le journal des gardes.".into(),
            story: "Au donjon, le capitaine des gardes vous tend son propre journal: « Seigneur, avant que vous ne jugiez, lisez ce que nous avons réellement consigné. Tout y est: chaque ronde, chaque bruit, chaque relève. Je n'ai rien fait effacer, rien fait réécrire. »

Il pose le rouleau sur la table: « Le moindre détail peut compter dans la nuit où quelqu'un a forcé le portail est. C'est le fichier /donjon/gardes/journal_gardes.txt, et je vous demande de le lire en entier, depuis la première ligne jusqu'à la dernière. »

Vous déroulez le rouleau. Les heures s'égrènent, la plume du capitaine trace la nuit en caractères serrés.

Dame Kaguya attend dans l'encadrement de la porte, les mains croisées. Elle ne dira rien tant que vous n'aurez pas lu.

Le moine Tetsuo vous observe du fond de la pièce, un sourire impénétrable sur les lèvres: « La vérité est dans le détail, Seigneur. Lis tout, ou ne lis rien. »

Vous vous penchez sur le journal: pas une ligne ne doit vous échapper.".into(),
            goal: "Lisez tout le journal des gardes.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier entier?".into(),
            required_commands: vec!["cat /donjon/gardes/journal_gardes.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "journal_gardes.txt".into(), expected: None }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "9.4".into(), chapter: 9, chapter_title: "Enquete: les espions".into(), act: 3,
            title: "Registre des visiteurs".into(),
            description: "Qui est entré au château aujourd'hui?".into(),
            story: "Un garde du portail accourt: « Seigneur! L'ordre de Dame Kaguya est formel: avant la tombée de la nuit, il faut savoir qui a franchi le portail aujourd'hui. »

Il désigne la guérite: « Tous les passages sont notés dans le registre, sans exception. Marchands, moines, visiteurs... chacun laisse une trace à l'entrée. Le fichier /portail/registre_visiteurs.txt vous dira tout. »

Vous suivez le garde. Le gravier crisse sous vos sandales, et la porte du château se découpe contre le ciel orange.

Dame Kaguya vous attend sous l'auvent, une lanterne à la main: « Quelqu'un est peut-être passé aujourd'hui qui n'aurait pas dû, Seigneur. Lisez le registre et dites-moi ce que valent ces inscriptions. »

Le vent du soir soulève la poussière du chemin. Derrière vous, les remparts est s'assombrissent lentement.

Le garde ouvre la guérite et vous tend le rouleau: la réponse est écrite dans le registre des visiteurs.".into(),
            goal: "Lisez le registre des visiteurs du portail.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire le registre du portail?".into(),
            required_commands: vec!["cat /portail/registre_visiteurs.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "registre_visiteurs.txt".into(), expected: None }],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "9.5".into(), chapter: 9, chapter_title: "Enquete: les espions".into(), act: 3,
            title: "Taille du journal".into(),
            description: "Combien de lignes dans auth.log au total?".into(),
            story: "Dame Kaguya vous attend dans le bureau des scribes, un épais journal de connexions sous le bras: « Seigneur, nos scribes notent chaque tentative d'entrée par la voie électronique, et ce journal, auth.log, est censé contenir tout l'historique: réussites comme échecs. »

Elle pose le journal sur le pupitre: « Or pendant la nuit, deux pages ont disparu. Si le journal est complet, le total des lignes doit être exact. Comptez toutes les lignes d'auth.log, sans en oublier une: le fichier est /scribes/logs/auth.log. »

Vous prenez une profonde inspiration. Deux pages manquantes, c'est peut-être un accident... ou un indice que quelqu'un veut faire disparaître.

Le moine Tetsuo, assis près de la fenêtre, ajoute sans se retourner: « Les mots comptés sont plus sûrs que les mots supposés, Seigneur. »

La mèche de la bougie vacille. Kaguya croise les bras et attend votre verdict: combien de lignes ce journal contient-il, vraiment, du début à la fin?".into(),
            goal: "Comptez toutes les lignes de auth.log.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment compter toutes les lignes?".into(),
            required_commands: vec!["wc -l /scribes/logs/auth.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "wc".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "wc -l".into(), name: "wc -l".into(), description: "wc -l — total des lignes".into() }),
        },
    ]
}

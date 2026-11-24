//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 6 — Surveiller le château (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "6.1".into(), chapter: 6, chapter_title: "Surveiller le chateau".into(), act: 2,
            title: "Tous les processus".into(),
            description: "Quels systèmes tournent dans le château?".into(),
            story: "Le forgeron arrive des salles des machines, la suie au visage. Il essuie ses
mains sur son tablier et s'incline devant vous.

« Seigneur, on me dit que des espions du clan rival rôdent près des remparts
depuis trois nuits. S'ils ont passé le portail, ils tournent quelque part dans
le château, en ce moment même. Je veux savoir quels systèmes vivent et
travaillent. »

Il sort un rouleau. « Donnez-moi la liste complète de tous les processus qui
tournent, avec leur propriétaire et leur consommation. Nous verrons bien ce
qui ne devrait pas être là. »

Vous vous penchez sur les registres du château: les gardes et les scribes
apparaissent, chacun avec ce qu'il dévore. Le forgeron parcourt vos notes et
hoche la tête. « Tout est là, Seigneur. Les processus connus, et les autres.
On saura quoi surveiller. »".into(),
            goal: "Dressez la liste complète de tous les processus du château, avec leur propriétaire et leur consommation.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment lister tous les processus?".into(),
            required_commands: vec!["ps aux".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "ps".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "aux".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "ps aux".into(), name: "ps aux".into(), description: "ps aux — tous les processus".into() }),
        },
        Mission {
            id: "6.2".into(), chapter: 6, chapter_title: "Surveiller le chateau".into(), act: 2,
            title: "Filtrer les gardes".into(),
            description: "Quels processus concernent les gardes?".into(),
            story: "Le capitaine des gardes pousse la porte, son armure cliquetant à chaque pas.
Il déroule votre précédent rapport et le tapote du doigt.

« La liste complète est utile, Seigneur, mais elle est trop longue à lire
d'un coup. Je veux seulement les processus qui concernent la garde: ceux dont
le nom parle des gardes, rien d'autre. »

Il marque un temps. « Filtrez le résultat pour ne garder que les lignes qui
contiennent ces mots. Je dois voir la surveillance du château d'un seul
regard, sans mêler les forges ni les greniers. »

Vous isolez les lignes de la garde du reste des registres. Le capitaine
examine le rouleau épuré et approuve d'un geste sec. « Voilà ce qu'il me
fallait, Seigneur. La garde est sous nos yeux, et le reste ne nous distrait
plus. »".into(),
            goal: "Affichez uniquement les processus contenant 'garde'.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment filtrer la sortie d'une commande?".into(),
            required_commands: vec!["ps aux | grep garde".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "|".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "ps".into(), name: "ps".into(), description: "ps — liste les processus en cours".into() }),
        },
        Mission {
            id: "6.3".into(), chapter: 6, chapter_title: "Surveiller le chateau".into(), act: 2,
            title: "Processus par nom".into(),
            description: "Y a-t-il des processus 'espion'?".into(),
            story: "Hanzō, le maître des ombres, surgit sans un bruit derrière vous. Sa voix ne
porte pas plus haut qu'un murmure, mais chacun de ses mots pèse.

« Seigneur, mes informateurs ont surpris un échange entre un ronin et un
messager du clan rival. Un nom revenait sans cesse: \"espion\". Si ce
processus dormait caché dans le château, il réveillerait ses semblables dès
ce soir. »

Il glisse un doigt sur la table où sont consignés les processus. « Cherchez
les processus dont le nom contient \"espion\", et montrez-les-moi en entier:
leur identifiant et leur commande. Je dois savoir où il se terre pour le
rabattre. »

Vous fouillez les registres et en tirez le fil de l'espion. Hanzō déchiffre
la ligne, puis son ombre s'étire en un sourire glacé. « Le voilà, Seigneur.
Il ne nous échappera plus. »".into(),
            goal: "Listez les processus dont le nom contient 'espion'.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment lister des processus par nom?".into(),
            required_commands: vec!["pgrep -a espion".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "pgrep".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-a".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "pgrep -a".into(), name: "pgrep -a".into(), description: "pgrep -a — processus par nom".into() }),
        },
        Mission {
            id: "6.4".into(), chapter: 6, chapter_title: "Surveiller le chateau".into(), act: 2,
            title: "Jauges du château".into(),
            description: "Aperçu instantané des ressources.".into(),
            story: "Le forgeron revient deux heures plus tard, les bras chargés de feuillets. Il
les dépose devant vous, visiblement épuisé.

« Seigneur, les salles des machines chauffent anormalement depuis ce matin.
La mémoire du château est sous tension, les fours tirent trop de souffle, et
quelque chose dévore nos forces. J'ai besoin d'un cliché des jauges: l'état
de la mémoire, du souffle, et les processus les plus gourmands. »

Il pose la main sur les feuillets. « Un seul aperçu suffit, pris d'un coup,
sans que je doive rester devant l'écran à l'interroger. En mode non
interactif, et une seule itération. »

Vous prenez le cliché; les jauges s'affichent d'un trait. Le forgeron le
compare à ses chiffres et souffle. « Voilà qui lève le doute, Seigneur. Ce
qui avale nos forces apparaît noir sur blanc. »".into(),
            goal: "Donnez au forgeron un cliché unique des jauges du château (mémoire, CPU, processus gourmands), en mode lot non interactif et en une seule itération.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment surveiller les ressources?".into(),
            required_commands: vec!["top -bn1".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "top".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-bn1".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "top -bn1".into(), name: "top -bn1".into(), description: "top -bn1 — aperçu CPU/mémoire".into() }),
        },
        Mission {
            id: "6.5".into(), chapter: 6, chapter_title: "Surveiller le chateau".into(), act: 2,
            title: "Éliminer l'intrus".into(),
            description: "Le processus espion PID 6666 tourne encore.".into(),
            story: "Hanzō surgit à nouveau, mais cette fois son calme a disparu. Son poignard est
déjà tiré, et sa voix porte.

« Seigneur! Le processus espion a été repéré: il porte le numéro 6666 et il
est en train de copier nos registres, un rouleau après l'autre, vers le clan
rival. Chaque instant qui passe, c'est un secret de Shiro qui s'en va. »

Il vous fixe. « Il faut couper son alimentation, de force. Une demande
polie, il l'ignorerait et continuerait à voler: il faut le signal qu'il ne
peut ni ignorer ni intercepter. Visez le numéro exact, 6666. Rien d'autre. »

Vous abattez le processus d'un geste net. Le cliquetis s'arrête. Hanzō
consulte les registres, puis range son poignard. « Le voleur est tombé,
Seigneur. Nos rouleaux restent à Shiro. »".into(),
            goal: "Abattez de force le processus espion 6666, avec le signal qu'il ne peut ni ignorer ni intercepter.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment tuer un processus par PID?".into(),
            required_commands: vec!["kill -9 6666".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "kill".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-9".into(), expected: None }, MissionCheck { check_type: "process_absent".into(), target: "6666".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "kill -9".into(), name: "kill -9".into(), description: "kill -9 — tue de force".into() }),
        },
    ]
}

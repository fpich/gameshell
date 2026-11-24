//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 1 — Arrivee au chateau (Acte 1) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "1.1".into(), chapter: 1, chapter_title: "Arrivee au chateau".into(), act: 1,
            title: "Premier contact avec Shiro".into(),
            description: "Vous arrivez au château Shiro. Dame Kaguya confirme votre position.".into(),
            story: "Le portail du château Shiro se referme derrière vous avec un
grincement. Vous êtes le nouveau seigneur du domaine, tout juste
installé dans vos quartiers, indiqués par le symbole ~ dans le
prompt.

Dame Kaguya, votre intendante, s'adresse à vous par le porte-voix:
« Seigneur? Vous m'entendez? Vos quartiers sont prêts, mais nos
registres doivent être exacts. Dites-moi où vous êtes précisément
dans le château. »

Un silence. Elle reprend, plus pressante:
« Et décrivez-moi ce qui vous entoure: quels documents, quels
objets se trouvent dans vos quartiers en ce moment même? Je note
tout dans le registre du domaine. »

Les registres servent à coordonner les gardes, les intendants et
les messagers. Faites-lui voir votre position et ce qui vous
entoure, Seigneur: elle attend votre réponse.

Dame Kaguya hoche la tête, soulagée:
« Parfait, Seigneur. Votre position et le contenu de vos quartiers
sont inscrits dans les registres. Tout est synchronisé. Bienvenue
au château Shiro. »".into(),
            goal: "Montrez à Dame Kaguya où vous êtes dans le château et ce qui vous entoure.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelle commande affiche le répertoire courant?".into(),
            required_commands: vec!["pwd".into(), "ls".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "pwd".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "ls".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            skill: Some(Skill { command: "pwd".into(), name: "pwd".into(), description: "pwd — affiche le répertoire courant".into() }),
        },
        Mission {
            id: "1.2".into(), chapter: 1, chapter_title: "Arrivee au chateau".into(), act: 1,
            title: "Le kaisho des quartiers".into(),
            description: "Lisez le kaisho (guide) posé dans vos quartiers.".into(),
            story: "Dame Kaguya poursuit, depuis le couloir:
« Seigneur, avant toute chose, prenez le temps de lire le kaisho
de vos quartiers. C'est le guide du domaine, rédigé jadis par les
intendants de vos prédécesseurs. »

Sur votre bureau repose un document: kaisho.txt. Il recense les
usages du château — comment vous déplacer entre les bâtiments,
quels officiers contacter, et les consignes de sécurité à
respecter.

« Ouvrez-le et lisez-le, Seigneur. Je dois être certaine que vous
connaissez les bases du domaine avant de confier les sceaux. »

Elle ajoute, plus bas:
« Vous y trouverez aussi le code d'accueil des quartiers. Il vous
faudra me le donner pour clôturer votre installation. »

La lecture complète du document vous préparera aux jours à venir:
un seigneur qui connaît son domaine ne se laisse pas surprendre.

Quand vous aurez lu le kaisho, donnez le code d'accueil à Dame
Kaguya.".into(),
            goal: "Lisez le guide kaisho.txt de vos quartiers.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier texte?".into(),
            required_commands: vec!["cat kaisho.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "kaisho.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: Some("SHIRO-ACCUEIL".into()),
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche le contenu d'un fichier".into() }),
        },
        Mission {
            id: "1.3".into(), chapter: 1, chapter_title: "Arrivee au chateau".into(), act: 1,
            title: "Lettre du clan Hattori".into(),
            description: "Le clan Hattori a déposé une lettre dans les archives.".into(),
            story: "Le marchand Genji se présente à la porte de vos quartiers, un peu
essoufflé:
« Seigneur! Un messager du clan Hattori vient de traverser les
remparts. Sa lettre est déposée aux archives, dans le dossier
des lettres: /archives/lettres/lettre_clan_hattori.txt. »

Il marque une pause:
« Le clan Hattori craint un siège imminent. Cette lettre contient
leur réponse à notre appel — il faut savoir quels renforts ils
nous accordent. Allez la lire, Seigneur. »

Rendez-vous aux archives, ouvrez le dossier des lettres et lisez
le message du clan Hattori. Genji attend votre rapport sur la
nature des renforts proposés.

Dans la lettre figure un code de confirmation, que le messager
devra présenter aux gardes de l'alliance. Notez-le: il vous
faudra le donner pour attester l'accord.

Genji, en vous voyant revenir, s'incline:
« Et alors, Seigneur? Le clan Hattori nous soutient-il? Leur
réponse scellera la défense du domaine. »".into(),
            goal: "Allez lire la lettre du clan Hattori dans /archives/lettres/.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment aller dans un autre bâtiment et lire un fichier?".into(),
            required_commands: vec!["cd /archives/lettres".into(), "cat lettre_clan_hattori.txt".into()],
            checks: vec![MissionCheck { check_type: "dir_reached".into(), target: "/archives/lettres".into(), expected: None }, MissionCheck { check_type: "file_read".into(), target: "lettre_clan_hattori.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: Some("HATTORI-20".into()),
            skill: Some(Skill { command: "cd".into(), name: "cd".into(), description: "cd — change de répertoire".into() }),
        },
        Mission {
            id: "1.4".into(), chapter: 1, chapter_title: "Arrivee au chateau".into(), act: 1,
            title: "Qui êtes-vous, Seigneur?".into(),
            description: "Le garde en chef vérifie votre identité.".into(),
            story: "Le garde en chef vous attend au portail, la main sur la hampe de
sa lance:
« Seigneur, je ne peux pas vous laisser entrer dans le donjon
sans une vérification. Le château tient un registre de chaque
membre du domaine: nom de la forteresse, nom du compte, badges
et groupes d'accès. »

Il sort son registre:
« Prouvez-moi qui vous êtes. D'abord, quel est le nom de ce
château? Ensuite, sous quel nom d'utilisateur êtes-vous enregistré
auprès du système? »

Il pointe une colonne du registre:
« Et vos identifiants complets, Seigneur: le numéro de votre
badge (uid), celui de votre maison (gid) et les groupes qui vous
ouvrent les portes. Le tout doit correspondre aux archives. »

Le système du château conserve une trace de tout: ne donnez que
l'identité que le domaine vous a attribuée, rien de plus, rien
d'inventé.

Le garde compare, puis s'incline:
« Identité confirmée, Seigneur. Le donjon vous est ouvert.
Bienvenue dans votre domaine. »".into(),
            goal: "Prouvez votre identité au garde en chef : nom du château, votre nom d'utilisateur et vos identifiants complets.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelles commandes donnent hostname/user/uid?".into(),
            required_commands: vec!["hostname".into(), "whoami".into(), "id".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "hostname".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "whoami".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "id".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "id".into(), name: "id".into(), description: "id — affiche uid, gid et groupes".into() }),
        },
        Mission {
            id: "1.5".into(), chapter: 1, chapter_title: "Arrivee au chateau".into(), act: 1,
            title: "L'heure du château".into(),
            description: "Synchronisez l'horloge du beffroi pour les registres.".into(),
            story: "Dame Kaguya vous rejoint dans la cour, une dernière vérification
en tête:
« Seigneur, tout est presque en ordre. Il reste l'horloge du
beffroi. Les rondes de garde sont consignées à l'heure exacte —
si nos horloges divergent, une patrouille en difficulté passera
inaperçue. »

Elle sort son cadran solaire:
« Voici l'heure que je note dans les registres. Dites-moi à
quelle heure est réglé le système du château, et je compare. »

Le beffroi donne le rythme du domaine: levée des gardes, relève
des sentinelles, ouverture des greniers. Une heure juste évite
les malentendus entre les tours, la pagode et les quartiers.

Interrogez le système: quelle date et quelle heure affiche le
château en ce moment?

Kaguya compare avec son cadran et hoche la tête:
« L'heure est exacte, Seigneur. Les registres de garde seront
désormais fiables. Le domaine est prêt. »".into(),
            goal: "Affichez la date et l'heure du système du château.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Quelle commande donne la date et l'heure?".into(),
            required_commands: vec!["date".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "date".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            skill: Some(Skill { command: "date".into(), name: "date".into(), description: "date — affiche la date et l'heure".into() }),
        },
    ]
}

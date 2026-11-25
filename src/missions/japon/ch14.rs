//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 14 — Comptes et accès (Acte 4) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "14.1".into(), chapter: 14, chapter_title: "Comptes et acces".into(), act: 4,
            title: "Mot de passe du samouraï".into(),
            description: "Changez le mot de passe de samourai2.".into(),
            story: "Le capitaine des gardes: « Seigneur, le compte de samourai2
n'a encore aucun mot de passe. N'importe qui peut se faire
passer pour lui devant mes portes. Donnez-lui un mot de
passe digne d'un samouraï avant mon prochain rapport. »

Le renfort du clan allié est arrivé et s'entraîne à la cour
des gardes. Mais un compte sans secret, c'est une porte
sans serrure: le premier venu peut entrer dans les quartiers
de samourai2, lire ses ordres et se présenter à la porte
en son nom.

Choisissez un mot de passe que vous saurez retrouver: il
vous sera encore utile dans les jours qui viennent. Le jeu
vous le demandera une première fois, puis une seconde pour
confirmer, avant de le graver dans les registres des ombres
du château.

Le capitaine des gardes fera reconnaître samourai2 à la
porte dès que son compte sera protégé. Un samouraï sans
secret est un espion qui s'ignore: tout le domaine veille
sur ce détail ce soir.


Le mot de passe ne s'écrit nulle part: il se grave dans
la mémoire du seigneur. Gardez la même valeur au moment
de la confirmation, sinon le château refusera de l'inscrire
et le compte restera ouvert à tous les vents.
".into(),
            goal: "Définissez un nouveau mot de passe pour le compte samourai2 afin de sécuriser son accès.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment changer un mot de passe?".into(),
            required_commands: vec!["sudo passwd samourai2".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "passwd".into(), expected: None }, MissionCheck { check_type: "user_has_password".into(), target: "samourai2".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "passwd".into(), name: "passwd".into(), description: "passwd — change un mot de passe".into() }),
        },
        Mission {
            id: "14.2".into(), chapter: 14, chapter_title: "Comptes et acces".into(), act: 4,
            title: "Session du samouraï".into(),
            description: "Testez le compte samourai2.".into(),
            story: "Le capitaine des gardes: « Seigneur, le compte est protégé,
dites-vous. Prouvez-le: entrez dans la session de samourai2
comme s'il allait s'y asseoir, puis montrez-moi le nom sous
lequel vous agissez à cet instant précis. Je veux voir de
mes yeux que sa porte s'ouvre avec son secret. »

Prendre la place de quelqu'un exige son secret: celui que
vous avez posé sur le compte tout à l'heure. Le garde veut
que vous entriez dans la peau du samouraï, le temps d'une
seule commande: celle qui révèle qui parle. Le secret qui
vous sera demandé est celui de samourai2, pas le vôtre.

Une session complète commence par son nom, puis la commande
unique qui dit son identité. Si la porte s'ouvre et que la
réponse porte le nom du samouraï, la preuve est faite: le
compte est bien fermé aux inconnus.

Le capitaine des gardes attend devant la tour, prêt à
consigner ce que vous lui montrerez, parchemin à la main.


Le secret demandé au passage de la porte est bien celui
fixé sur le compte tout à l'heure: c'est la clé qui ferme
les quartiers de samourai2. Une seule réponse suffit,
celle que le domaine connaît déjà.
".into(),
            goal: "Endossez l'identité de samourai2 dans une session de connexion complète, le temps d'exécuter une seule commande qui révèle sous quel nom vous agissez.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment exécuter une commande en tant qu'autre utilisateur?".into(),
            required_commands: vec!["su - samourai2 -c whoami".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "su".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-c".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "su -c".into(), name: "su -c".into(), description: "su - user -c cmd — session temporaire".into() }),
        },
        Mission {
            id: "14.3".into(), chapter: 14, chapter_title: "Comptes et acces".into(), act: 4,
            title: "Vos pouvoirs".into(),
            description: "Quelles commandes sudo pouvez-vous lancer?".into(),
            story: "Dame Kaguya: « Seigneur, l'audit du domaine commence par
vos propres pouvoirs. Montrez-moi la liste des commandes
que le château vous autorise à lancer en administrateur,
sans en exécuter aucune. Je la comparerai au registre des
privilèges de la cour. »

La cour impériale s'interroge sur la répartition des
pouvoirs au château Shiro. Dame Kaguya doit prouver que
vous détenez seuls les clés d'administration, et que la
liste affichée correspond exactement à celle qui est
consignée dans ses rouleaux.

Un administrateur peut consulter ses droits sans modifier
quoi que ce soit: il suffit de demander l'inventaire au
maître des clés du domaine. Rien ne sera lancé, seule la
liste s'affichera à l'écran, prête à être recopiée.

Dame Kaguya attend devant le trône, la plume en l'air,
pour cocher chaque privilège un à un sur son parchemin.


La liste se lit à l'écran sous la forme d'autorisations
précises: chaque privilège correspond à une entrée du
registre. Afficher cet inventaire ne coûte rien et ne
change rien: il se contente de répondre.
".into(),
            goal: "Listez les commandes que vous êtes autorisé à exécuter avec les pouvoirs du seigneur.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment lister ses droits sudo?".into(),
            required_commands: vec!["sudo -l".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "sudo".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "sudo -l".into(), name: "sudo -l".into(), description: "sudo -l — droits sudo".into() }),
        },
        Mission {
            id: "14.4".into(), chapter: 14, chapter_title: "Comptes et acces".into(), act: 4,
            title: "Clé maître".into(),
            description: "Prouvez que sudo vous reconnaît comme seigneur.".into(),
            story: "Le capitaine des gardes: « Seigneur, on murmure au château
que vos clés d'administration seraient brisées. Prouvez-moi
le contraire: prenez vos pleins pouvoirs et faites dire au
domaine qui vous êtes à cet instant précis. Je veux une
preuve que je puisse montrer à la section. »

Une rumeur court dans les couloirs: la clé maître du
seigneur ne répondrait plus. Les gardes hésitent à obéir,
les intendants refont leurs comptes. Il faut une preuve
éclatante, devant témoins, que le pouvoir obéit toujours
à votre voix.

La commande qui dit qui l'on est est celle que répond par
votre nom: tous les serviteurs la connaissent. Il reste à
l'appeler avec les pleins pouvoirs, pour que la réponse
porte la marque de l'administrateur du domaine.

Le capitaine a rassemblé la section devant le trône: ils
attendent que vous leur montriez cette preuve, sans plus
de délai.


Un serviteur du domaine a répandu la rumeur pour tester
la vigilance des gardes; il sera prudent d'y répondre par
une démonstration publique. La preuve sera d'autant plus
nette qu'elle portera la marque des pleins pouvoirs.
".into(),
            goal: "Prouvez que vous disposez des pouvoirs du seigneur en exécutant une commande d'identité.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment exécuter une commande en root?".into(),
            required_commands: vec!["sudo whoami".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "sudo".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "sudo".into(), name: "sudo".into(), description: "sudo — exécute en root".into() }),
        },
        Mission {
            id: "14.5".into(), chapter: 14, chapter_title: "Comptes et acces".into(), act: 4,
            title: "Transférer un registre".into(),
            description: "Le registre des impôts appartient au seigneur.".into(),
            story: "Dame Kaguya: « Seigneur, le registre des impôts a été tenu
par l'intendant pendant des années. Pour l'audit de la cour,
il doit porter votre nom: transférez-lui la propriété de
/salle-du-trone/registres/registre_impots.txt. »

Le registre des impôts recense chaque koku de riz et chaque
ryo qui entrent dans le domaine. La cour impériale veut
vérifier les comptes, et un document sans propriétaire ne
peut pas être présenté comme preuve: il lui faut le sceau
de son légitime maître, c'est-à-dire vous.

L'intendant a déjà posé le rouleau sur la table du trône;
il ne manque que le nom du nouveau propriétaire. Avec les
pleins pouvoirs, le seigneur transfère le bien d'un trait,
sans toucher à une seule ligne du registre.

Dame Kaguya attend au pied du trône, prête à sceller le
registre dès qu'il portera votre nom.


Gardez la main légère: la propriété change de nom, rien
d'autre. Aucune ligne du registre ne doit être lue ou
modifiée: seul le sceau du propriétaire se déplace, et
le rouleau reste intact pour les vérificateurs de la cour.
".into(),
            goal: "Avec les droits d'administrateur, transférez la propriété de /salle-du-trone/registres/registre_impots.txt à votre propre compte pour l'audit.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment changer le propriétaire d'un fichier?".into(),
            required_commands: vec!["sudo chown $USER /salle-du-trone/registres/registre_impots.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chown".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chown".into(), name: "chown".into(), description: "chown — change le propriétaire".into() }),
        },
    ]
}

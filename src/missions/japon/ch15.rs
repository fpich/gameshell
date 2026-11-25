//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 15 — Le coffre-fort du donjon (Acte 4) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "15.1".into(), chapter: 15, chapter_title: "Le coffre-fort du donjon".into(), act: 4,
            title: "Verrouiller le coffre".into(),
            description: "Le coffre-fort ne doit être lisible que par vous: 700.".into(),
            story: "Le capitaine des gardes: « Seigneur, les sceaux du domaine
dorment dans le coffre-fort du donjon, dans le dossier
/donjon/coffre-fort. Tant que ce dossier reste ouvert à
tous, n'importe qui peut lire ou emporter les sceaux.
Réglez l'accès pour que seul vous, le seigneur du
château, puissiez y entrer. »

La cérémonie d'alliance avec le clan voisin exige de
montrer les sceaux au matin. S'ils disparaissent cette
nuit, l'alliance se brise avant l'aube et les remparts
devront répondre seuls du domaine.

Le dossier du coffre est un lieu à part: il n'est pas
question d'y déposer un sceau de plus, mais de fermer la
porte derrière les sceaux déjà gardés. Seul le maître du
château doit conserver la clé de cette porte.

Le capitaine inspectera le dossier avant la relève de
garde. Quand l'accès ne laissera passer que vous, il
jugera le coffre en sûreté et les sceaux resteront à
leur place pour la cérémonie.".into(),
            goal: "Mettez les permissions 700 sur /donjon/coffre-fort.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment verrouiller un dossier entier?".into(),
            required_commands: vec!["chmod 700 /donjon/coffre-fort".into()],
            checks: vec![MissionCheck { check_type: "permission".into(), target: "/donjon/coffre-fort".into(), expected: Some("700".into()) }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "chmod 700".into(), name: "chmod 700".into(), description: "chmod 700 — verrouillage total".into() }),
        },
        Mission {
            id: "15.2".into(), chapter: 15, chapter_title: "Le coffre-fort du donjon".into(), act: 4,
            title: "Lire les sceaux".into(),
            description: "Lisez le fichier des sceaux du coffre-fort.".into(),
            story: "Seigneur Hattori: « Maintenant que le coffre est fermé,
ouvrez-le pour moi: je veux entendre la liste des sceaux
du domaine, celle qui dort dans
/donjon/coffre-fort/sceaux.txt. Lisez-moi le contenu du
parchemin, tel qu'il est écrit. »

Chaque sceau est la promesse d'une récolte, d'un serment
ou d'une dette. Avant de négocier avec le clan voisin,
le seigneur doit savoir exactement quels engagements le
domaine peut encore honorer.

Les sceaux ont été vérifiés une dernière fois avant
d'être enfermés. Le dossier est maintenant fermé à
double tour, mais vous êtes le seul à pouvoir y entrer
et en rapporter le contenu.

Hattori attend votre lecture pour rassurer les émissaires
du clan voisin. Récitez-lui les noms tels qu'ils figurent
sur le parchemin: rien de plus, rien de moins. Il saura
alors quels serments engager devant les envoyés.".into(),
            goal: "Lisez le fichier sceaux.txt du coffre-fort.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire les sceaux?".into(),
            required_commands: vec!["cat /donjon/coffre-fort/sceaux.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "sceaux.txt".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "15.3".into(), chapter: 15, chapter_title: "Le coffre-fort du donjon".into(), act: 4,
            title: "Code du coffre aux archives".into(),
            description: "Le code du coffre est aussi aux archives.".into(),
            story: "Dame Kaguya: « Seigneur, le code du coffre-fort n'est pas
gravé sur le bois: il est consigné dans les archives
secrètes, dans /archives/secret/code_coffre.txt. Je dois
vérifier qu'il correspond au coffre avant la cérémonie.
Lisez-le-moi tel qu'il est écrit. »

Les archives secrètes ne s'ouvrent qu'au seigneur et à
sa dame. Chaque parchemin y a sa place, chaque code y
attend son heure. Celui du coffre-fort est le plus
ancien de tous: il ouvre la chambre où dorment les sceaux.

La cérémonie commence au lever du jour. Si le code ne
correspond pas, la porte restera close devant les
émissaires et l'alliance sera perdue par la faute d'un
chiffre mal recopié.

Kaguya prépare la salle du trône. Lisez-lui le code qui
figure dans les archives secrètes: elle confirmera qu'il
ouvre bien la chambre du coffre avant de laisser entrer
les envoyés du clan voisin.".into(),
            goal: "Lisez code_coffre.txt dans les archives secrètes.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire le code du coffre?".into(),
            required_commands: vec!["cat /archives/secret/code_coffre.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "code_coffre.txt".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "15.4".into(), chapter: 15, chapter_title: "Le coffre-fort du donjon".into(), act: 4,
            title: "Note au registre des impôts".into(),
            description: "Ajoutez une ligne au registre des impôts.".into(),
            story: "Dame Kaguya: « Seigneur, le village du haut n'a pas payé
sa part de la récolte: dix koku de retard. Consignez le
retard à la FIN du registre des impôts, dans
/salle-du-trone/registres/registre_impots.txt, sans
effacer les lignes déjà écrites. Écrivez exactement:
'Retard: village du haut - 10 koku'. »

Le registre est la mémoire du domaine: chaque entrée y
date d'une saison, chaque chiffre y engage une parole.
Effacer l'ancien registre pour y mettre le retard serait
pire que le retard lui-même: personne ne croirait plus
aux comptes du château.

Les recettes et les dettes y sont consignées au fur et
à mesure, ligne après ligne. Il faut que le retard
apparaisse comme une entrée de plus, à sa place, avec
le nom du village et le montant dû.

Kaguya relira le registre ce soir. Quand la ligne du
retard du village du haut s'y trouvera, sans qu'aucune
des anciennes entrées manque, les comptes du domaine
seront à nouveau exacts.".into(),
            goal: "Ajoutez la ligne 'Retard: village du haut - 10 koku' à la fin de registre_impots.txt, sans rien effacer.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter une ligne à un registre?".into(),
            required_commands: vec!["echo 'Retard: village du haut - 10 koku' >> /salle-du-trone/registres/registre_impots.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }, MissionCheck { check_type: "config_contains".into(), target: "/salle-du-trone/registres/registre_impots.txt".into(), expected: Some("Retard: village du haut".into()) }],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: ">>".into(), name: ">>".into(), description: ">> — ajoute au registre".into() }),
        },
        Mission {
            id: "15.5".into(), chapter: 15, chapter_title: "Le coffre-fort du donjon".into(), act: 4,
            title: "Fiche du coffre".into(),
            description: "Métadonnées du dossier coffre-fort.".into(),
            story: "Le capitaine des gardes: « Seigneur, avant de fermer le
rapport de la nuit, il me faut la fiche complète du
dossier /donjon/coffre-fort: sa taille, ses permissions,
son propriétaire et les dates marquées dessus. Sans ces
détails, je ne peux pas certifier que le coffre est bien
celui que j'ai scellé. »

Chaque objet du château a sa fiche chez les gardes: qui
le possède, quand il a été créé, qui peut y toucher.
Quand un objet passe de main en main, la fiche suit et
porte la trace de chaque étape.

Le coffre a été verrouillé cette nuit, puis les sceaux y
ont été remis. Pour clore le rapport, le capitaine veut
la fiche du dossier elle-même, pas son contenu: la
mesure exacte de ce qui le protège.

Montrez-lui la fiche complète du dossier: il y lira la
taille, les permissions, le propriétaire et les dates.
Dès qu'il aura ces détails sous les yeux, il signera le
rapport et la garde de nuit pourra reprendre son tour.".into(),
            goal: "Remettez au capitaine la fiche technique complète du dossier /donjon/coffre-fort: taille, permissions, propriétaire et dates.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir les métadonnées d'un dossier?".into(),
            required_commands: vec!["stat /donjon/coffre-fort".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "stat".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "stat".into(), name: "stat".into(), description: "stat — métadonnées".into() }),
        },
    ]
}

//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 20 — Le siège final (Acte 5) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "20.1".into(), chapter: 20, chapter_title: "Le siege final".into(), act: 5,
            title: "Sceau du seigneur".into(),
            description: "Prouvez que vous connaissez le sceau secret (fichier caché .secret).".into(),
            story: "Seigneur Hattori paraît dans l'encadrement de la porte
de vos quartiers, une corde de messager autour du cou: «
Seigneur, le clan Hattori consent à envoyer vingt
samouraïs, mais je dois d'abord être certain que vous
êtes le seigneur de Shiro. »

Il baisse la voix: « Chaque seigneur du domaine possède
un sceau secret, gravé dans un petit parchemin caché au
milieu de ses quartiers, dans un fichier invisible dont
le nom commence par un point. Trouvez-le et lisez-le moi
à voix haute. »

Les espions du clan rival rôdent déjà sous les remparts
et la nuit avance. Le sceau secret est la preuve que
vous êtes le véritable maître du château; sans lui,
aucune troupe ne partira.

Il ajoute: « Quand vous m'aurez récité le sceau, je
l'attesterai auprès de mon clan. Tout le domaine attend
ce geste. »

Une fois le parchemin lu, Hattori compare le sceau à ses
registres et s'incline: « Le sceau est authentique,
Seigneur. Les vingt samouraïs Hattori marchent sur
Shiro. Le siège peut commencer. »".into(),
            goal: "Trouvez le sceau secret dans .secret et validez avec ce code.".into(),
            bloom_level: "Évaluer".into(),
            bloom_question: "Où se trouve le sceau secret?".into(),
            required_commands: vec!["cat .secret".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: ".secret".into(), expected: None }],
            xp_reward: 200,
            code_required: Some("LACUS-7-ONI".into()),
            skill: Some(Skill { command: "cat .secret".into(), name: "cat .secret".into(), description: "cat .secret — lit le sceau caché".into() }),
        },
        Mission {
            id: "20.2".into(), chapter: 20, chapter_title: "Le siege final".into(), act: 5,
            title: "Code du donjon".into(),
            description: "Validez avec le code du coffre-fort.".into(),
            story: "Le capitaine des gardes se tient devant la porte du
donjon, tandis que les sentinelles tendent l'oreille
vers les collines: « Seigneur, la salle de guerre est
verrouillée par le code du coffre-fort, et moi seul ne
peux pas l'ouvrir. Il me faut votre autorité. »

Il précise: « Le code est conservé en deux endroits:
dans les sceaux du coffre-fort, sous le donjon, dans le
fichier sceaux.txt, et dans un second document des
archives secrètes, code_coffre.txt. Consultez l'un,
vérifiez l'autre, puis donnez-moi le code. »

Deux écritures du même code pour éviter qu'un espion ne
le corrompe: c'est la règle des anciens intendants. La
salle de guerre abrite les cartes de la contre-attaque,
et le conseil refuse de siéger tant qu'elle est fermée.

Le capitaine écoute le code, le répète aux gardes, puis
la lourde porte s'ouvre dans un grincement: « Le code
est exact, Seigneur. La salle de guerre est ouverte, le
conseil peut siéger. »".into(),
            goal: "Trouvez le code du coffre-fort et validez avec ce code.".into(),
            bloom_level: "Évaluer".into(),
            bloom_question: "Où se trouve le code du coffre-fort?".into(),
            required_commands: vec!["cat /donjon/coffre-fort/sceaux.txt".into(), "cat /archives/secret/code_coffre.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "sceaux.txt".into(), expected: None }, MissionCheck { check_type: "file_read".into(), target: "code_coffre.txt".into(), expected: None }],
            xp_reward: 200,
            code_required: Some("KAMI-442-TORA".into()),
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — lit les codes".into() }),
        },
        Mission {
            id: "20.3".into(), chapter: 20, chapter_title: "Le siege final".into(), act: 5,
            title: "Rapport de siège".into(),
            description: "Rédigez le rapport de siège complet.".into(),
            story: "Dame Kaguya traverse la cour, un rouleau de consignes à
la main, les yeux fatigués par les alertes de la nuit: «
Seigneur, les scribes ont consigné chaque tentative
d'intrusion du jour dans auth.log, sous les remparts des
scribes. Les portes ont été assaillies plusieurs fois:
chaque assaut y est marqué du mot Failed. »

Elle vous tend un parchemin vierge: « Rédigez le rapport
de siège dans rapport_siege.txt, dans vos quartiers:
d'abord la liste des attaques Failed du jour, puis, à la
suite, l'état des remparts que me transmet le maître des
travaux, dans remparts/etat_remparts.txt. »

Le conseil lira ce rapport pour décider où renforcer les
défenses dès l'aube: les attaques d'abord, l'état des
fortifications ensuite, rien d'autre.

Elle relit le rapport que vous déposez et hoche la tête:
« Les attaques puis l'état des remparts, à la suite,
comme demandé. Le conseil a tout ce qu'il lui faut,
Seigneur. »".into(),
            goal: "Créez rapport_siege.txt avec les attaques puis l'état des remparts.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Comment composer un rapport multi-sources?".into(),
            required_commands: vec!["grep Failed /scribes/logs/auth.log > rapport_siege.txt".into(), "cat /remparts/etat_remparts.txt >> rapport_siege.txt".into()],
            checks: vec![MissionCheck { check_type: "file_exists".into(), target: "~/rapport_siege.txt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: ">".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: ">".into(), name: ">".into(), description: "> — compose le rapport".into() }),
        },
        Mission {
            id: "20.4".into(), chapter: 20, chapter_title: "Le siege final".into(), act: 5,
            title: "Archiver les preuves".into(),
            description: "Archivez tous les logs dans les archives.".into(),
            story: "Dame Kaguya vous attend sous la galerie des archives,
une lampe à la main, les registres ouverts autour
d'elle: « Le procès des espions s'ouvre à l'aube, et les
scribes ont accumulé des mois de logs dans
/scribes/logs/. Toutes les dépositions, tous les
passages suspects, tout s'y trouve. »

Elle palpe la poussière des étagères: « Il faut tout
emporter dans les archives, en un seul paquet, pour que
rien ne se perde en route. Déposez l'archive complète
des logs dans /archives/preuves_siege.tar.gz. »

Le nom du paquet comprend le mot preuves: il partira
scellé au tribunal avec les sceaux du domaine. Aucun log
ne doit être laissé derrière.

Elle vérifie le paquet posé sur l'établi, en défait les
liens, compte les journaux et approuve: « Tous les logs
y sont, Seigneur. Les preuves du siège sont en sûreté
pour le procès. »".into(),
            goal: "Créez /archives/preuves_siege.tar.gz contenant /scribes/logs/.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Comment archiver toutes les preuves?".into(),
            required_commands: vec!["tar czf /archives/preuves_siege.tar.gz /scribes/logs/".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tar".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "/archives/preuves_siege.tar.gz".into(), expected: None }],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill { command: "tar".into(), name: "tar".into(), description: "tar — crée et lit des archives".into() }),
        },
        Mission {
            id: "20.5".into(), chapter: 20, chapter_title: "Le siege final".into(), act: 5,
            title: "Protocole de siège final".into(),
            description: "Dernière épreuve: validez avec le protocole de siège du coffre-fort.".into(),
            story: "Seigneur Hattori monte les marches du donjon, entouré de
ses officiers, et s'arrête devant la porte de la salle
de guerre: « Seigneur, voici la dernière épreuve. Le
protocole de siège est gravé dans les sceaux du
coffre-fort, dans le fichier sceaux.txt sous le donjon.
Récitez-le-moi. »

La nuit du siège approche, et chaque seigneur de Shiro
doit prouver qu'il sait lire les sceaux et garder le
secret des archives. Hattori n'accorde sa confiance
qu'à ceux qui maîtrisent ces deux gestes.

Il désigne la galerie des archives secrètes: « Et placez
une caméra d'audit du domaine sur les archives secrètes,
/archives/secret, pour que chaque ouverture soit
consignée: écrite ou modifiée, avant la contre-attaque.
»

Lire les sceaux pour révéler le protocole, verrouiller
les archives secrètes sous surveillance: c'est le rituel
des anciens seigneurs avant chaque bataille décisive.

Quand les deux gestes sont accomplis, Hattori s'incline
et frappe le sol de son bâton: « Le protocole est
confirmé et les archives sont gardées, Seigneur. La
contre-attaque décisive peut commencer. Shiro ne tombera
pas! »".into(),
            goal: "Lisez les sceaux, installez une caméra d'audit sur /archives/secret, validez avec le protocole de siège.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Quel est le protocole de siège?".into(),
            required_commands: vec!["cat /donjon/coffre-fort/sceaux.txt".into(), "sudo auditctl -w /archives/secret -p wa -k sceau_final".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "sceaux.txt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "auditctl".into(), expected: None }, MissionCheck { check_type: "sim_contains".into(), target: "audit.rules".into(), expected: Some("-w /archives/secret".into()) }],
            xp_reward: 500,
            code_required: Some("TORA-442-KAMI".into()),
            skill: Some(Skill { command: "auditctl -w".into(), name: "auditctl -w".into(), description: "auditctl -w — caméra finale".into() }),
        },
    ]
}

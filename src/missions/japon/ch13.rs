//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 13 — Recrutement (Acte 4) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "13.1".into(), chapter: 13, chapter_title: "Recrutement".into(), act: 4,
            title: "Recruter un samouraï".into(),
            description: "Créez le compte du nouveau samouraï 'samourai2'.".into(),
            story: "Le seigneur Hattori: « Seigneur, un samouraï renfort arrive
du clan allié: samourai2. Donnez-lui un compte qui existe
vraiment, avec ses propres quartiers, pour qu'il ne dorme
pas dans le couloir. Puis montrez-moi sa fiche d'identité,
pour que je sache le numéro qui lui est attribué. »

Un compte sans quartier, c'est un soldat sans chambre:
il ne peut ni ranger ses affaires ni se reposer entre
deux gardes. Le renfort veut servir le domaine, mais il
lui faut un foyer à son nom, un dossier personnel ouvert
au château.

Le moine Tetsuo a préparé une paillasse dans les
quartiers des invités et attend le nouveau venu. Une
fois le compte créé avec son logement, vous appellerez
sa fiche: le numéro d'identité, le groupe principal,
tout ce qui permettra de reconnaître samourai2 dans
les registres du domaine.

Le seigneur Hattori consulte les registres avant de
partir accueillir le renfort à la porte.
".into(),
            goal: "Créez le compte samourai2 avec son dossier personnel pour le renfort du seigneur Hattori, puis affichez sa fiche d'identité.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer un utilisateur avec son home?".into(),
            required_commands: vec!["sudo useradd -m samourai2".into(), "id samourai2".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "useradd".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-m".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "id".into(), expected: None }, MissionCheck { check_type: "user_exists".into(), target: "samourai2".into(), expected: None }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "useradd -m".into(), name: "useradd -m".into(), description: "useradd -m — crée un utilisateur avec home".into() }),
        },
        Mission {
            id: "13.2".into(), chapter: 13, chapter_title: "Recrutement".into(), act: 4,
            title: "Créer une garnison".into(),
            description: "Créez le groupe 'garnison'.".into(),
            story: "Le capitaine des gardes: « Seigneur, les recrues du
clan allié arrivent par petits groupes. Rassemblez-les
sous un même drapeau: créez le groupe garnison, pour
qu'elles aient toutes un foyer commun dans les
registres du château. »

Une unité sans nom, c'est une troupe que personne ne
peut compter. Quand le groupe garnison existera, les
capitaines pourront y ranger chaque nouvelle recrue
et savoir, d'un regard, qui défend le domaine. C'est
le quartier-maître qui tient ce registre, et il attend
votre ordre.

Vous vous installez devant les registres des gardes.
Les noms des soldats s'y inscriront bientôt sous une
même bannière. Le capitaine hoche la tête: dès que le
groupe sera fondé, il fera défiler les recrues.

Chaque recrue aura ainsi une bannière commune, lisible
d'un seul regard dans les registres du château.
".into(),
            goal: "Fondez le groupe garnison qui accueillera les nouvelles recrues du château.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment créer un groupe?".into(),
            required_commands: vec!["sudo groupadd garnison".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "groupadd".into(), expected: None }, MissionCheck { check_type: "group_exists".into(), target: "garnison".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "groupadd".into(), name: "groupadd".into(), description: "groupadd — crée un groupe".into() }),
        },
        Mission {
            id: "13.3".into(), chapter: 13, chapter_title: "Recrutement".into(), act: 4,
            title: "Affecter le samouraï".into(),
            description: "Ajoutez samourai2 au groupe garnison.".into(),
            story: "Le capitaine des gardes: « Seigneur, samourai2 doit
rejoindre la garnison. Ajoutez-le au groupe garnison
sans lui retirer ses autres affectations: il reste du
clan allié, et il devient nôtre. »

Un soldat peut servir deux bannières quand les deux
domaines sont alliés. Le renfort garde ses liens
d'origine et gagne sa place dans la garnison: rien ne
doit être perdu en route. Il ne s'agit pas de remplacer
ses autres groupes, seulement d'en ajouter un.

Vous ouvrez le registre de samourai2. Sa ligne contient
déjà des affectations anciennes, qu'il faut respecter.
Vous ajoutez la garnison à la suite, sans rayer le
reste. Le capitaine vérifiera d'un coup d'œil que le
nouveau soldat est bien dans les deux colonnes.

Le groupe garnison sera ajouté à sa ligne, et les
anciennes affectations resteront en place: un soldat
ne s'efface pas quand il prête serment ailleurs.
".into(),
            goal: "Affectez samourai2 au groupe garnison en complément, sans lui retirer ses autres affectations.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter un utilisateur à un groupe?".into(),
            required_commands: vec!["sudo usermod -aG garnison samourai2".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "usermod".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-aG".into(), expected: None }, MissionCheck { check_type: "user_in_group".into(), target: "samourai2".into(), expected: Some("garnison".into()) }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "usermod -aG".into(), name: "usermod -aG".into(), description: "usermod -aG — ajoute à un groupe".into() }),
        },
        Mission {
            id: "13.4".into(), chapter: 13, chapter_title: "Recrutement".into(), act: 4,
            title: "Vos groupes".into(),
            description: "À quels groupes appartenez-vous?".into(),
            story: "Dame Kaguya: « Seigneur, le donjon est verrouillé selon
les groupes d'accès. Avant de vous en remettre les
clés, montrez-moi la liste des groupes auxquels votre
propre compte appartient. »

Les portes du donjon s'ouvrent selon les bannières que
vous portez. Si un groupe manque à votre liste, un
couloir vous restera fermé. La dame veut vérifier que
votre présence est bien enregistrée: c'est votre propre
fiche qu'elle demande.

Vous fouillez vos quartiers pour trouver les parchemins
de votre maisonnée. Inutile: la réponse est inscrite
dans le compte lui-même, là où le château garde trace
de vos affectations. Il suffit de l'appeler pour que
la liste s'affiche sous les yeux de Dame Kaguya.

Elle compare avec son rouleau et marque votre nom d'un
sceau: les clés du donjon suivront.
".into(),
            goal: "Montrez à Dame Kaguya la liste des groupes auxquels votre propre compte appartient.".into(),
            bloom_level: "Se souvenir".into(),
            bloom_question: "Comment voir ses groupes?".into(),
            required_commands: vec!["groups".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "groups".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "groups".into(), name: "groups".into(), description: "groups — groupes de l'utilisateur".into() }),
        },
        Mission {
            id: "13.5".into(), chapter: 13, chapter_title: "Recrutement".into(), act: 4,
            title: "Fiche du samouraï".into(),
            description: "Identité complète de samourai2.".into(),
            story: "Le capitaine des gardes: « Seigneur, je dois inscrire
samourai2 au registre des tours de garde, et il me
faut sa fiche complète: son numéro d'identité, son
groupe principal, et tous les groupes auxquels il
appartient. Montrez-les-moi. »

Le registre des gardes exige l'état exact du soldat:
numéro, famille, unités. Une seule ligne manquante, et
le capitaine ne saura pas où poster l'homme quand la
cloche sonnera la nuit.

Vous appelez la fiche complète de samourai2, celle que
le château tient à jour depuis sa création. Tout s'y
trouve en une seule ligne: son numéro, sa maison et
ses affectations. Le capitaine la recopie sur son
rouleau et range sa plume: le tour de garde peut être
écrit.

La fiche sortira dans son entier, de la première
colonne à la dernière, pour que rien ne soit oublié
lors de la recopie.
".into(),
            goal: "Affichez l'identité complète de samourai2: numéro, groupe principal et toutes ses garnisons.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'identité d'un autre utilisateur?".into(),
            required_commands: vec!["id samourai2".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "id".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "id".into(), name: "id".into(), description: "id — identité d'un utilisateur".into() }),
        },
    ]
}

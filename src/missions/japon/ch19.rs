//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 19 — Durcissement final (Acte 5) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "19.1".into(), chapter: 19, chapter_title: "Durcissement final".into(), act: 5,
            title: "Config SSH du château".into(),
            description: "Lisez la configuration SSH des scribes.".into(),
            story: "Le capitaine des gardes: « Seigneur, avant de durcir les
portes, il faut connaître les réglages qui les gardent
aujourd'hui. Les scribes tiennent un registre de la
passe SSH du château: lisez-le pour moi, il est dans
/scribes/registres/sshd_config. Je veux savoir ce que
la porte accepte et ce qu'elle refuse. »

Chaque réglage dit qui peut entrer, par quelle voie et
combien de fois. Tant qu'on ne l'a pas lu, on durcit à
l'aveugle: on pourrait fortifier une porte déjà ouverte
et laisser une autre béante derrière soi.

Le seigneur Hattori attend votre rapport avant de fixer
le plan de défense. Les espions du clan rival sondent
les remparts chaque nuit; le capitaine ne veut pas
décider sans les faits.

Lisez la passe du château telle que les scribes l'ont
écrite. Quand le capitaine l'aura sous les yeux, il
saura quels verrous renforcer en premier.".into(),
            goal: "Lisez le fichier sshd_config des scribes.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire la config SSH?".into(),
            required_commands: vec!["cat /scribes/registres/sshd_config".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "sshd_config".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche un fichier".into() }),
        },
        Mission {
            id: "19.2".into(), chapter: 19, chapter_title: "Durcissement final".into(), act: 5,
            title: "Interdire root en SSH".into(),
            description: "Ajoutez 'PermitRootLogin no' à sshd_config.".into(),
            story: "Le capitaine des gardes, la lecture finie: « Le registre
montre une faille grave, Seigneur: le compte du maître
peut encore se connecter directement depuis l'extérieur,
sans passer par la garde. C'est la porte que l'ennemi
utiliserait pour se faire passer pour vous. Condamnez-la:
ajoutez la ligne 'PermitRootLogin no' à la fin de la
passe, dans /scribes/registres/sshd_config. »

La passe est le document de référence des scribes: toute
règle s'y ajoute à la fin, pour que l'histoire des verrous
reste lisible. La ligne interdira le passage direct du
compte maître.

Le capitaine veut que la règle soit écrite noir sur
blanc dans le registre, ni ailleurs ni plus tôt dans le
fichier: le dernier mot doit être le plus sévère.

Ajoutez la condamnation à la fin de la passe des scribes.
Quand la ligne y sera inscrite, plus aucun imposteur ne
franchira la porte sous le nom du maître.".into(),
            goal: "Ajoutez 'PermitRootLogin no' à la fin de sshd_config.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter une ligne de config?".into(),
            required_commands: vec!["echo 'PermitRootLogin no' >> /scribes/registres/sshd_config".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }, MissionCheck { check_type: "config_contains".into(), target: "/scribes/registres/sshd_config".into(), expected: Some("PermitRootLogin no".into()) }],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: ">>".into(), name: ">>".into(), description: ">> — ajoute la config".into() }),
        },
        Mission {
            id: "19.3".into(), chapter: 19, chapter_title: "Durcissement final".into(), act: 5,
            title: "Caméra sur le coffre".into(),
            description: "Surveillez les accès au coffre-fort.".into(),
            story: "Le capitaine des gardes: « Seigneur, quelqu'un a touché
aux sceaux du donjon cette nuit. Je veux une caméra
d'audit sur les sceaux: qu'elle consigne chaque écriture
et chaque changement d'attributs sur le coffre-fort,
/donjon/coffre-fort, et qu'elle marque ses prises de
l'étiquette sceaux. »

Le coffre-fort garde les sceaux du domaine: qui les
signe engage le château entier. Une écriture sans trace
est un vol sans coupable; un attribut changé sans récit
est une porte ouverte sans témoin.

La caméra doit veiller sans relâche, de la pose de la
règle jusqu'à la fin du siège, et chaque événement doit
porter le nom du trésor surveillé pour qu'on le retrouve
dans les registres.

Posez la surveillance d'audit sur le coffre-fort du
donjon, comme le capitaine l'a demandée. Quand elle
sera en place, chaque geste sur les sceaux laissera une
trace écrite.".into(),
            goal: "Posez une surveillance d'audit sur /donjon/coffre-fort qui enregistre chaque écriture et chaque changement d'attributs, marquée de l'étiquette 'sceaux'.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Comment surveiller un dossier sensible?".into(),
            required_commands: vec!["sudo auditctl -w /donjon/coffre-fort -p wa -k sceaux".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "auditctl".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-w".into(), expected: None }, MissionCheck { check_type: "sim_contains".into(), target: "audit.rules".into(), expected: Some("-w /donjon/coffre-fort".into()) }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            skill: Some(Skill { command: "auditctl -w".into(), name: "auditctl -w".into(), description: "auditctl -w — caméra d'audit".into() }),
        },
        Mission {
            id: "19.4".into(), chapter: 19, chapter_title: "Durcissement final".into(), act: 5,
            title: "Isoler un prisonnier".into(),
            description: "Enfermez un processus dans une cellule (chroot).".into(),
            story: "Le capitaine des gardes: « Seigneur, le prisonnier du
donjon a encore tenté de gagner la tour par la nuit:
c'est un homme dangereux, mais surtout un homme qui
sait. Enfermez-le dans la cellule d'isolement du donjon,
/donjon/prison, et lancez-y l'outil de travail réduit
du château, pour qu'il n'atteigne plus rien au-delà des
murs de sa cellule. »

La cellule d'isolement ne montre au prisonnier que ce
que le château veut bien lui laisser voir: pas de
registres de garde, pas de carte des remparts, rien de
la vie du domaine. Il reste enfermé, mais utile.

Le capitaine veut qu'aucun ordre du prisonnier ni aucun
fichier extérieur ne puisse sortir de la cellule: seul
l'outil réduit doit y vivre avec lui.

Enfermez le prisonnier dans la cellule du donjon avec
seul l'environnement confiné, sous les yeux du
capitaine. Quand la porte de la cellule se refermera,
ses regards ne dépasseront plus les murs.".into(),
            goal: "Isolez un prisonnier dans la cellule du donjon en lançant un environnement confiné.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Comment isoler un processus dans un chroot?".into(),
            required_commands: vec!["sudo chroot /donjon/prison /bin/bash".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chroot".into(), expected: None }, MissionCheck { check_type: "sim_event".into(), target: "chroot /donjon/prison".into(), expected: None }],
            hints: vec![],
            xp_reward: 175,
            code_required: None,
            skill: Some(Skill { command: "chroot".into(), name: "chroot".into(), description: "chroot — isole dans une cellule".into() }),
        },
        Mission {
            id: "19.5".into(), chapter: 19, chapter_title: "Durcissement final".into(), act: 5,
            title: "Mise à jour du domaine".into(),
            description: "Mettez à jour les paquets du château (apt update).".into(),
            story: "Dame Kaguya: « Seigneur, le siège approche et le château
se bat avec des outils d'hier. Avant la bataille,
rafraîchissez avec les droits d'administrateur la liste
des réparations disponibles pour le domaine: les
artisans du royaume publient chaque jour de nouveaux
moyens, et nous devons les connaître pour défendre nos
murs. »

Sans la liste à jour, le château ne sait pas ce qui est
réparable: des verrous renforcés existent peut-être
depuis la pleine lune sans que personne le sache. Un
domaine qui ignore ses propres réserves part vaincu.

Dame Kaguya veut la liste fraîche, pas l'installation
d'aucun outil: c'est le recensement qui manque, pour que
le conseil choisisse en connaissance de cause.

Rafraîchissez la liste des réparations du domaine avec
les droits d'administrateur. Quand le recensement sera
à jour, le château saura ce qu'il peut préparer avant
l'arrivée de l'ennemi.".into(),
            goal: "Avant le siège, rafraîchissez avec les droits d'administrateur la liste des paquets disponibles pour le domaine.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment mettre à jour les paquets?".into(),
            required_commands: vec!["sudo apt update".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "apt".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "update".into(), expected: None }, MissionCheck { check_type: "sim_event".into(), target: "apt update".into(), expected: None }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "apt update".into(), name: "apt update".into(), description: "apt update — met à jour les paquets".into() }),
        },
    ]
}

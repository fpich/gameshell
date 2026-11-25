//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 18 — Services du château (Acte 5) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "18.1".into(), chapter: 18, chapter_title: "Services du chateau".into(), act: 5,
            title: "État du service SSH".into(),
            description: "Le service SSH tourne-t-il?".into(),
            story: "Le capitaine des gardes: « Seigneur, les messagers du
clan Hattori passeront par le canal chiffré, mais un
canal ne vaut que si le service qui le garde fonctionne.
Avant d'ouvrir les portes, montrez-moi l'état du service
qui reçoit leurs messages. »

Le passage chiffré est la seule voie sûre entre le
château et ses alliés: s'il est éteint, la parole des
messagers reste à la porte et l'ennemi gagne le temps de
la surprendre. Un service muet est une porte condamnée
sans que personne ne s'en aperçoive.

Le capitaine veut lire l'état exact du service: est-il
en marche? Depuis quand? Quelle est sa santé telle que
le système la consigne? C'est la première question d'un
siège bien préparé: savoir ce qui veille, avant de
savoir qui vient.

Montrez-lui l'état du service du canal chiffré, tel que
le système le rapporte. Quand il verra la ligne de vie
du service, il saura si les messagers peuvent entrer.".into(),
            goal: "Affichez l'état du service ssh.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'état d'un service?".into(),
            required_commands: vec!["systemctl status ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "systemctl".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "status".into(), expected: None }],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "systemctl status".into(), name: "systemctl status".into(), description: "systemctl status — état d'un service".into() }),
        },
        Mission {
            id: "18.2".into(), chapter: 18, chapter_title: "Services du chateau".into(), act: 5,
            title: "Redémarrer SSH".into(),
            description: "Redémarrez SSH après les réglages.".into(),
            story: "Le capitaine des gardes: « Seigneur, le service du canal
chiffré fonctionne, mais ses réglages viennent d'être
modifiés: ordres de veille, écoutes à rejeter. Tant que
le service ne repart pas, il continue avec les anciennes
consignes. Relancez-le avec les droits d'administrateur,
pour qu'il applique les nouveaux réglages. »

Un service réglé mais jamais relancé est une sentinelle
qui garde l'ancien mot de passe: tout le monde croit la
consigne changée, personne ne le sait. Il faut que le
service s'arrête et reparte pour charger sa nouvelle
mission.

La manœuvre est brève: le canal sera muet quelques
instants, puis reprendra selon les nouveaux réglages.

Relancez le service du canal chiffré avec les droits
d'administrateur. Quand il repartira sous sa nouvelle
consigne, les messagers pourront reprendre leurs allées
et venues sans crainte.".into(),
            goal: "Avec les droits d'administrateur, redémarrez le service d'accès distant sécurisé du château pour appliquer les nouveaux réglages.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment redémarrer un service?".into(),
            required_commands: vec!["sudo systemctl restart ssh".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "restart".into(), expected: None }, MissionCheck { check_type: "sim_event".into(), target: "systemctl restart ssh".into(), expected: None }],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill { command: "systemctl restart".into(), name: "systemctl restart".into(), description: "systemctl restart — redémarre un service".into() }),
        },
        Mission {
            id: "18.3".into(), chapter: 18, chapter_title: "Services du chateau".into(), act: 5,
            title: "Logs du service SSH".into(),
            description: "Les 10 dernières lignes du journal de ssh.".into(),
            story: "Le capitaine des gardes: « Seigneur, le service a repris,
mais je ne me fie pas à une simple lecture de veille: je
veux voir ce que le service a écrit lui-même. Consultez
son journal, et montrez-m'en les dix dernières lignes,
celles qui racontent la reprise. »

Chaque service note son histoire au fil des heures:
ouvertures, refus, erreurs. Le journal ne ment pas, lui
qui consigne les faits à mesure. Les dix dernières
lignes diront si la reprise s'est bien passée ou si le
service boite depuis son redémarrage.

Le capitaine veut le récit des derniers instants du
service: les dix dernières lignes de son journal, ni
plus ni moins. Trois lignes de plus ou de moins et le
récit serait tronqué ou bavard: il veut la fin exacte.

Montrez-lui ces dix dernières lignes telles que le
système les a écrites. Il y lira la santé du service et
décidera si le canal peut être rouvert.".into(),
            goal: "Affichez les 10 dernières lignes du journal de ssh.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les logs d'un service?".into(),
            required_commands: vec!["journalctl -u ssh -n 10".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "journalctl".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-u".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-n".into(), expected: None }],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "journalctl".into(), name: "journalctl".into(), description: "journalctl -u ssh -n 10 — logs d'un service".into() }),
        },
        Mission {
            id: "18.4".into(), chapter: 18, chapter_title: "Services du chateau".into(), act: 5,
            title: "Bannissements fail2ban".into(),
            description: "Quels espions sont bannis?".into(),
            story: "Le capitaine des gardes: « Seigneur, le videur
automatique veille jour et nuit derrière le passage
chiffré: il repère les assauts répétés et bannit les
adresses qui insistent. J'ai besoin de voir qui il tient
à l'écart. Montrez-moi l'état de cette protection, avec
les droits d'administrateur. »

Le videur est la sentinelle invisible du domaine: il ne
dort pas et garde la liste des adresses interdites. En
le consultant, le capitaine saura quels agresseurs ont
été écartés et si la liste correspond aux visages qu'il
redoutait.

Le château vient d'essuyer des vagues de tentatives: qui
le videur a-t-il attrapé? Voilà ce que la consultation
doit révéler.

Montrez-lui l'état de la protection anti-intrusion.
Quand la liste des bannis sera sous ses yeux, il saura
si le domaine a été bien défendu.".into(),
            goal: "Affichez l'état de la protection anti-intrusion du château.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment voir les bannissements?".into(),
            required_commands: vec!["sudo fail2ban-client status".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "fail2ban-client".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "status".into(), expected: None }],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "fail2ban-client status".into(), name: "fail2ban-client status".into(), description: "fail2ban-client status — bannissements".into() }),
        },
        Mission {
            id: "18.5".into(), chapter: 18, chapter_title: "Services du chateau".into(), act: 5,
            title: "Débannir l'allié".into(),
            description: "Le clan Hattori (10.0.0.99) a été banni par erreur.".into(),
            story: "Le capitaine des gardes, alarmé: « Seigneur, le videur
automatique a commis une méprise: il a banni le messager
du clan Hattori, 10.0.0.99, en pleine négociation! Ce
n'est pas un espion, c'est notre allié. Levez son
bannissement immédiatement, avec les droits
d'administrateur. »

Le videur juge sur les apparences: des tentatives
répétées venaient de ce quartier, et il a frappé sans
demander conseil. Mais le messager allié insistait à
juste titre: il cherchait à entrer, pas à forcer.

Chaque instant de bannissement est une nouvelle perdue
pour la négociation: l'allié attend à la porte, empêché
d'approcher par une erreur. Le capitaine ne veut pas
qu'une décision automatique fasse fuir un clan ami.

Levez le bannissement du messager 10.0.0.99, avec les
droits d'administrateur. Quand la porte lui sera rouverte,
le clan Hattori reprendra sa place au conseil.".into(),
            goal: "Levez immédiatement le bannissement infligé par erreur au clan Hattori, 10.0.0.99, par le videur anti-intrusion.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment révoquer un bannissement?".into(),
            required_commands: vec!["sudo fail2ban-client unban 10.0.0.99".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "unban".into(), expected: None }, MissionCheck { check_type: "sim_not_contains".into(), target: "fail2ban.bans".into(), expected: Some("10.0.0.99".into()) }],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill { command: "fail2ban-client unban".into(), name: "fail2ban-client unban".into(), description: "fail2ban-client unban IP — révoque un ban".into() }),
        },
    ]
}

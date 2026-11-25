//! CHAPITRE 10 - Durcissement
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "18.1".into(), chapter: 18, chapter_title: "Durcissement".into(), act: 5,
            title: "fail2ban".into(),
            description: "Bannissez automatiquement!".into(),
            story: "Draz M'kal vous fait signe d'approcher de la console de sécurité.

« Les pirates ont frappé toute la nuit, Capitaine : des dizaines de
tentatives d'accès, depuis les adresses 10.0.0.66 et 10.0.0.99. Heureusement,
le videur automatique est à bord : il surveille les connexions et bannit
tout seul les adresses qui se comportent mal. »

Il tape sur le clavier.

« Le videur tourne, mais je veux vérifier qu'il fait bien son travail : qu'il
est opérationnel et qu'il surveille les bons services. Les adresses
malveillantes doivent être prises en charge automatiquement, sans que
personne n'ait à intervenir. »

Il s'écarte de l'écran.

« Interrogez le videur : montrez-moi son état et les services qu'il surveille.
S'il est bien en place, nous sommes protégés. »

Vous interrogez le videur automatique : il est opérationnel et surveille les
services attendus.

« Parfait, Capitaine. Le videur est prêt à bannir tout intrus. »".into(),
            goal: "Les pirates reviennent sans cesse. Il faut un videur automatique.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Automatiser le bannissement?".into(),
            required_commands: vec!["sudo fail2ban-client status".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "fail2ban-client".into(), expected: None }],
            hints: vec![],
            xp_reward: 300,
            code_required: None,
            skill: Some(Skill { command: "fail2ban-client".into(), name: "fail2ban-client".into(), description: "fail2ban-client — bannit automatiquement les IPs pirates (jail, ban, unban)".into() }),
        },
        Mission {
            id: "18.2".into(), chapter: 18, chapter_title: "Durcissement".into(), act: 5,
            title: "Durcir SSH".into(),
            description: "Interdisez root et MDP!".into(),
            story: "Draz M'kal vous reçoit dans la salle de sécurité, un comprimé de config à la main.

« Nos pare-feu repoussent les attaques, Capitaine, mais le service d'accès à
distance est encore vulnérable. Regardez : il accepte les connexions du
compte root, et il accepte les mots de passe. C'est une porte ouverte pour
les pirates : deux lignes du fichier de configuration les laissent entrer. »

Il vous tend le comprimé.

« La configuration du service ssh est dans /securite/config/sshd_config.
Il faut fermer ces deux portes : interdire les connexions du compte root,
et exiger autre chose que les mots de passe. Si un pirate essayait
d'entrer en root avec un mot de passe deviné, il serait refusé. »

Il recule d'un pas.

« Modifiez la configuration pour bloquer root et bloquer les mots de passe.
C'est un durcissement simple mais vital. »

Vous modifiez la configuration : le compte root est refusé et les mots de
passe sont désactivés.

« Deux failles de moins, Capitaine. La porte est fermée. »".into(),
            goal: "Fermez la porte aux attaques: plus de root, plus de mots de passe en SSH.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Securiser SSH?".into(),
            required_commands: vec!["nano /securite/config/sshd_config".into(), "echo 'PermitRootLogin no' >> /securite/config/sshd_config".into(), "echo 'PasswordAuthentication no' >> /securite/config/sshd_config".into()],
            checks: vec![MissionCheck { check_type: "config_contains".into(), target: "/securite/config/sshd_config".into(), expected: Some("PermitRootLogin no".into()) }, MissionCheck { check_type: "config_contains".into(), target: "/securite/config/sshd_config".into(), expected: Some("PasswordAuthentication no".into()) }],
            hints: vec![],
            xp_reward: 350,
            code_required: None,
            skill: Some(Skill { command: "nano".into(), name: "nano".into(), description: "nano — éditeur de texte terminal pour modifier une config".into() }),
        },
        Mission {
            id: "18.3".into(), chapter: 18, chapter_title: "Durcissement".into(), act: 5,
            title: "iptables".into(),
            description: "Limitez SSH a 3 conn/min.".into(),
            story: "Draz M'kal vous attend devant la baie réseau, un câble à la main.

« Les pirates entrent par les connexions, Capitaine. UFW filtre déjà les
adresses classiques, mais le noyau a son propre pare-feu, plus profond : il
décide du sort de chaque paquet qui traverse la station. Avant d'y toucher,
il faut savoir exactement ce qui filtre déjà. »

Il pointe l'écran.

« Je veux voir toutes les règles actives du pare-feu du noyau : les chaînes,
les politiques et surtout les adresses en chiffres, sans les traduire en
noms. Ce n'est qu'une inspection : rien ne sera modifié, mais je dois
connaître l'état exact des lieux avant de renforcer quoi que ce soit. »

Il vous tend la console.

« Affichez les règles du pare-feu du noyau, avec les adresses en chiffres.
Je veux voir ce qui est filtré aujourd'hui. »

Les règles défilent : chaque chaîne, chaque politique, chaque adresse en
chiffres.

« Voilà la carte exacte de nos défenses, Capitaine. On pourra la renforcer. »".into(),
            goal: "Avant de toucher au pare-feu du noyau, inspectez toutes ses règles actives, adresses en chiffres.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Limiter les connexions?".into(),
            required_commands: vec!["sudo iptables -L -n".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "iptables".into(), expected: None }],
            hints: vec![],
            xp_reward: 350,
            code_required: None,
            skill: Some(Skill { command: "iptables".into(), name: "iptables".into(), description: "iptables — configure les règles de filtrage réseau du noyau".into() }),
        },
        Mission {
            id: "18.4".into(), chapter: 18, chapter_title: "Durcissement".into(), act: 5,
            title: "Audit".into(),
            description: "Surveillez passwd et shadow.".into(),
            story: "Draz M'kal ferme la porte de la sécurité avant de parler.

« Un pirate entré une fois essaiera de rester, Capitaine. S'il parvenait à
créer un compte sur la station, nous devrions le savoir immédiatement. Le
garde d'audit est à bord et peut tracer chaque modification sensible. »

Il ouvre une console.

« Le fichier des comptes enregistrés, /etc/passwd, est la cible : si un
intrus y ajoute une ligne, c'est qu'un compte vient d'être créé. Il faut
poser une surveillance sur ce fichier : enregistrer chaque écriture et
chaque changement d'attribut, et étiqueter ces événements « comptes » pour
les retrouver facilement dans le journal d'audit. »

Il vous montre la liste des règles actives.

« Posez cette surveillance, puis affichez les règles d'audit pour confirmer
qu'elle est en place. »

La surveillance est posée : chaque écriture ou changement d'attribut sur
/etc/passwd sera tracé sous l'étiquette « comptes ».

« Si un compte apparaît, nous le saurons, Capitaine. Merci. »".into(),
            goal: "Si un pirate crée un compte, il faut le savoir: surveillez les écritures et changements d'attributs de /etc/passwd, étiquette « comptes ».".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Tracer les modifications?".into(),
            required_commands: vec!["sudo auditctl -w /etc/passwd -p wa -k comptes".into(), "sudo auditctl -l".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "auditctl".into(), expected: None }, MissionCheck { check_type: "command_executed".into(), target: "-l".into(), expected: None }, MissionCheck { check_type: "sim_contains".into(), target: "audit.rules".into(), expected: Some("-w /etc/passwd".into()) }],
            hints: vec![],
            xp_reward: 350,
            code_required: None,
            skill: Some(Skill { command: "auditctl".into(), name: "auditctl".into(), description: "auditctl — configure l'audit système (journalisation des événements)".into() }),
        },
        Mission {
            id: "18.5".into(), chapter: 18, chapter_title: "Durcissement".into(), act: 5,
            title: "Chroot".into(),
            description: "Isolez un processus!".into(),
            story: "Draz M'kal vous conduit jusqu'à une soute condamnée, au bout du couloir.

« Nous avons capturé un processus pirate, Capitaine. Il rôdait dans la
station et tentait de lire et d'écrire où il ne fallait pas. On ne peut pas
le détruire, mais on peut l'enfermer : le mettre dans une cellule où il ne
verra plus que ce qu'on lui donne à voir. »

Il ouvre la trappe de la soute.

« La cellule d'isolement est prête dans /soutes/chroot. Lisez d'abord le
guide qui s'y trouve : il explique comment fonctionne la cellule et vous
donnera le code d'accès. Ensuite, lancez le processus à l'intérieur de la
cellule pour qu'il soit définitivement contenu. »

Il s'écarte.

« Quand ce sera fait, entrez le code d'accès de la cellule pour sceller
l'opération. »

Vous enfermez le processus dans la cellule d'isolement : il ne peut plus
voir ni toucher au reste de la station. Le code d'accès scelle la
procédure.

« Dernière barrière franchie, Capitaine. Le vaisseau est durci. »".into(),
            goal: "Dernière barrière: enfermez un processus dans une cellule d'isolement.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Isoler un processus?".into(),
            required_commands: vec!["sudo chroot /soutes/chroot /bin/bash".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chroot".into(), expected: None }, MissionCheck { check_type: "sim_event".into(), target: "chroot /soutes/chroot".into(), expected: None }],
            hints: vec![],
            xp_reward: 400,
            code_required: Some("FORTRESS".into()),
            skill: Some(Skill { command: "chroot".into(), name: "chroot".into(), description: "chroot — exécute une commande dans un environnement racine isolé".into() }),
        },
    ]
}

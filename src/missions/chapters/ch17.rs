//! CHAPITRE 9 - Maintenance
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "17.1".into(),
            chapter: 17,
            chapter_title: "Maintenance".into(),
            act: 5,
            title: "Mise a jour".into(),
            description: "Mettez a jour les paquets.".into(),
            story:
                "Tael Rhen vous attend à la console du dépôt, un comprimé d'inventaire à la main.

« Capitaine, notre réserve de pièces est particulière : les modules et les
correctifs de sécurité arrivent du dépôt central sous forme de fichiers, et le
système maintient une liste de tout ce qui est disponible à bord. Tout le
monde s'appuie sur cette liste pour savoir ce que l'on peut installer. »

Il fait défiler un écran de chiffres.

« Un lot de correctifs est arrivé tout à l'heure, mais la liste n'a pas été
rafraîchie : le système croit encore que les anciennes versions sont les
seules disponibles. Tant que l'on ne met pas cette liste à jour, les nouvelles
pièces restent invisibles et les failles connues ne sont pas traitées. »

Il vous tend le comprimé.

« Pouvez-vous mettre la liste des pièces disponibles à jour ? C'est une
opération de maintenance de base, mais elle doit être faite avant toute
autre chose. »

Vous actualisez l'inventaire. La liste affiche les versions livrées par le
dernier lot.

« Parfait, Capitaine. Le dépôt sait désormais ce qu'il peut proposer. »"
                    .into(),
            goal: "Mettez à jour la liste des pièces détachées disponibles.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Rafraichir les paquets?".into(),
            required_commands: vec!["sudo apt update".into()],
            checks: vec![
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "apt update".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "sim_event".into(),
                    target: "apt update".into(),
                    expected: None,
                },
            ],
            hints: vec![],
            xp_reward: 200,
            code_required: None,
            skill: Some(Skill {
                command: "apt".into(),
                name: "apt".into(),
                description: "apt — gestionnaire de paquets (install, remove, update)".into(),
            }),
        },
        Mission {
            id: "17.2".into(),
            chapter: 17,
            chapter_title: "Maintenance".into(),
            act: 5,
            title: "Installer".into(),
            description: "Installez cowsay.".into(),
            story: "Nira Okoro, aux communications, teste les haut-parleurs de la passerelle quand
vous entrez.

« Capitaine, je dois vérifier que le système d'annonces fonctionne sur tous
les écrans du vaisseau. J'ai trouvé un petit programme bien utile dans notre
dépôt : cowsay, une vache qui affiche le message que l'on lui donne dans une
bulle. Parfait pour diffuser une annonce de test bien visible. »

Elle consulte son comprimé.

« Le programme est dans le catalogue local, mais il n'est pas installé à
bord. Installez l'outil cowsay, puis vérifiez auprès du système d'installation
qu'il est bien enregistré. Avec ça, je pourrai lancer mon annonce d'essai. »

Vous installez cowsay et confirmez son installation.

« C'est noté : cowsay est à bord. Je prépare l'annonce de test. Merci,
Capitaine. »"
                .into(),
            goal: "Installez un outil pour tester les annonces de la station.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Installer un logiciel?".into(),
            required_commands: vec!["sudo apt install -y cowsay".into(), "dpkg -s cowsay".into()],
            checks: vec![
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "apt install".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "package_installed".into(),
                    target: "cowsay".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "dpkg".into(),
                    expected: None,
                },
            ],
            hints: vec![],
            xp_reward: 225,
            code_required: None,
            skill: Some(Skill {
                command: "dpkg".into(),
                name: "dpkg".into(),
                description: "dpkg — gestionnaire de paquets Debian de bas niveau".into(),
            }),
        },
        Mission {
            id: "17.3".into(),
            chapter: 17,
            chapter_title: "Maintenance".into(),
            act: 5,
            title: "Redemarrer SSH".into(),
            description: "Apres modif config.".into(),
            story: "Draz M'kal, chef de la sécurité, vous arrête devant la salle des machines.

« Les tentatives d'accès à distance de la nuit dernière ne me plaisent pas,
Capitaine. J'ai durci la configuration du service d'administration à
distance : nouvelle clé, restrictions d'accès, journalisation renforcée. »

Il montre une console.

« Les réglages sont écrits dans le fichier de configuration, mais le service
tourne encore avec l'ancienne version. Tant que l'on ne le relance pas, il
continue d'appliquer les paramètres obsolètes. »

Il recule d'un pas.

« Relancez le service ssh pour qu'il reparte avec la nouvelle configuration.
Après cela, les connexions devront passer par les règles renforcées. »

Vous redémarrez le service. Il repart proprement en chargeant les nouveaux
réglages.

« Bon travail, Capitaine. Le service tourne avec la configuration durcie. »"
                .into(),
            goal: "La config SSH a changé, il faut recharger le service.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Redemarrer un service?".into(),
            required_commands: vec!["sudo systemctl restart ssh".into()],
            checks: vec![
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "systemctl restart".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "sim_event".into(),
                    target: "systemctl restart ssh".into(),
                    expected: None,
                },
            ],
            hints: vec![],
            xp_reward: 225,
            code_required: None,
            skill: Some(Skill {
                command: "systemctl restart".into(),
                name: "systemctl restart".into(),
                description: "systemctl restart — redémarre un service système".into(),
            }),
        },
        Mission {
            id: "17.4".into(),
            chapter: 17,
            chapter_title: "Maintenance".into(),
            act: 5,
            title: "Journal".into(),
            description: "Historique recent.".into(),
            story: "Draz M'kal vous appelle depuis la sécurité, l'air préoccupé.

« Cette nuit, le vaisseau a repoussé des tentatives d'intrusion, Capitaine.
Tout est consigné dans le journal de bord, événement par événement. Je veux
comprendre l'ordre exact des choses : ce qui s'est passé en dernier, en
particulier. »

Il se penche vers l'écran.

« Le journal complet remonte à des heures et il est difficile à lire. Ce qui
m'intéresse, c'est la fin : les événements les plus récents, ceux de la
période où les défenses ont réagi. »

Il vous fait signe d'approcher.

« Consultez le journal du système et montrez-moi ce qui s'est passé
récemment à bord. Je veux voir les dernières entrées. »

Vous affichez les derniers événements : tentatives rejetées, blocages et
réactions des défenses s'enchaînent.

« Voilà la séquence exacte, Capitaine. Bien vu. »"
                .into(),
            goal: "Qu'est-ce qui s'est passé récemment sur la station? Consultez le journal."
                .into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Voir les logs systemd?".into(),
            required_commands: vec!["sudo journalctl -n 20".into()],
            checks: vec![MissionCheck {
                check_type: "command_executed".into(),
                target: "journalctl".into(),
                expected: None,
            }],
            hints: vec![],
            xp_reward: 250,
            code_required: None,
            skill: Some(Skill {
                command: "journalctl".into(),
                name: "journalctl".into(),
                description: "journalctl — consulte les journaux système (logs)".into(),
            }),
        },
        Mission {
            id: "17.5".into(),
            chapter: 17,
            chapter_title: "Maintenance".into(),
            act: 5,
            title: "Nettoyer les paquets".into(),
            description: "Liberez de l'espace.".into(),
            story: "Tael Rhen descend de l'échelle de la soute, un sac de maintenance à l'épaule.

« Capitaine, le stock de modules de la station est encombré. Il reste des
vieilles versions de composants qui ne servent plus, laissées là après les
dernières mises à jour. Chaque ancien fichier qui traîne prend de la place et
embrouille les diagnostics. »

Il jette un coup d'œil aux étagères.

« Il y a deux choses à faire : d'abord retirer les modules devenus inutiles,
ceux que plus aucun système ne réclame. Ensuite, vider les fichiers
d'installation qui ont été téléchargés et qui restent stockés à bord sans
service. »

Il vous tend une liste.

« Faites ce nettoyage pour libérer de l'espace et rendre le stock sain.
C'est de l'entretien courant, mais il faut le faire. »

Le stock est purgé : les vieilleries ont disparu et l'espace est libéré.

« Stock propre, esprit clair, Capitaine. Merci. »"
                .into(),
            goal: "Les soutes sont pleines de vieilleries. Nettoyez le système.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Nettoyer le systeme?".into(),
            required_commands: vec!["sudo apt autoremove -y".into(), "sudo apt clean".into()],
            checks: vec![
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "apt autoremove".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "apt clean".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "sim_event".into(),
                    target: "apt autoremove".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "sim_event".into(),
                    target: "apt clean".into(),
                    expected: None,
                },
            ],
            hints: vec![],
            xp_reward: 225,
            code_required: None,
            skill: Some(Skill {
                command: "apt update".into(),
                name: "apt update".into(),
                description: "apt update — met à jour la liste des paquets disponibles".into(),
            }),
        },
    ]
}

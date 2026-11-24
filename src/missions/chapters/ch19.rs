//! CHAPITRE 19 - Durcissement SSH (Acte 5) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "19.1".into(),
            chapter: 19,
            chapter_title: "Durcissement SSH".into(),
            act: 5,
            title: "Lire la config SSH".into(),
            description: "Vex Korshun audite la configuration SSH de la station.".into(),
            story: "Zyra-7, l'IA de bord, s'illumine dans la passerelle. « Capitaine, le
durcissement de la liaison sécurisée est en cours. Avant de modifier un seul
réglage, j'ai besoin que vous me montriez l'état actuel de la configuration :
elle est enregistrée dans /securite/config/sshd_config. »

Elle affiche une copie du plan de travail.

« Lisez le fichier de configuration en entier : je dois savoir exactement
quelles portes sont encore ouvertes avant de les fermer une à une. Les
réglages que nous allons changer sont dedans. »

Vous lisez la configuration actuelle de la liaison sécurisée. Les options
y sont listées les unes après les autres.

« Je vois que vous avez lu le fichier, Capitaine. Le rapport est
enregistré : nous savons maintenant ce qui doit être fermé. »"
                .into(),
            goal: "Lisez le contenu du fichier de configuration /securite/config/sshd_config."
                .into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire un fichier de configuration?".into(),
            required_commands: vec!["cat /securite/config/sshd_config".into()],
            checks: vec![MissionCheck {
                check_type: "file_read".into(),
                target: "sshd_config".into(),
                expected: None,
            }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "cat".into(),
                name: "cat".into(),
                description: "cat — affiche le contenu complet d'un fichier texte".into(),
            }),
        },
        Mission {
            id: "19.2".into(),
            chapter: 19,
            chapter_title: "Durcissement SSH".into(),
            act: 5,
            title: "Interdire le login root".into(),
            description: "Ajoutez 'PermitRootLogin no' à la config SSH.".into(),
            story: "Draz M'kal, de la sécurité, pointe l'écran de la console. « Capitaine, regardez
la configuration que vous venez de lire : le compte root a encore le droit de
se connecter. C'est la porte que les pirates essaient en premier, et elle est
grande ouverte. »

Il vérifie le plan de durcissement.

« Nous allons la fermer : ajoutez la ligne 'PermitRootLogin no' à la fin du
fichier /securite/config/sshd_config, sans effacer le reste. Une fois cette
ligne en place, root ne pourra plus ouvrir de session à distance. »

Vous ajoutez la ligne d'interdiction à la fin du fichier. La porte que les
pirates visaient en premier est désormais condamnée.

« Bien, Capitaine. Le compte root est hors de portée des intrus. »"
                .into(),
            goal: "Ajoutez la ligne 'PermitRootLogin no' à la fin du fichier sshd_config.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter une ligne de configuration à la fin d'un fichier?"
                .into(),
            required_commands: vec![
                "echo 'PermitRootLogin no' >> /securite/config/sshd_config".into()
            ],
            checks: vec![
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: ">>".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "config_contains".into(),
                    target: "/securite/config/sshd_config".into(),
                    expected: Some("PermitRootLogin no".into()),
                },
            ],
            hints: vec![],
            xp_reward: 150,
            code_required: None,
            skill: Some(Skill {
                command: "cmd >> fichier".into(),
                name: ">>".into(),
                description: "COMMANDE >> fichier — ajoute la sortie à la fin du fichier".into(),
            }),
        },
        Mission {
            id: "19.3".into(),
            chapter: 19,
            chapter_title: "Durcissement SSH".into(),
            act: 5,
            title: "Etat du service SSH".into(),
            description: "Le service SSH tourne-t-il encore sur la station?".into(),
            story: "Tael Rhen, depuis la salle des machines, vous appelle sur l'intercom. « Le
fichier est modifié, Capitaine. Avant de redémarrer quoi que ce soit, vérifions
le service de la liaison sécurisée tourne encore. »

Il vous laisse un instant pour réfléchir.

« Affichez l'état du service ssh : je veux savoir s'il est actif en ce
moment, avec les détails de son fonctionnement. S'il est tombé, autant le
savoir avant de lancer le redémarrage. »

Vous affichez l'état du service : la liaison sécurisée est bien active et
prête.

« Parfait, Capitaine. Le service est vivant — nous pouvons passer à la
suite en toute sécurité. »"
                .into(),
            goal: "Affichez l'état actuel du service ssh.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment voir l'état d'un service systemd?".into(),
            required_commands: vec!["systemctl status ssh".into()],
            checks: vec![MissionCheck {
                check_type: "command_executed".into(),
                target: "systemctl".into(),
                expected: None,
            }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "systemctl status".into(),
                name: "systemctl status".into(),
                description: "systemctl status — affiche l'état d'un service".into(),
            }),
        },
        Mission {
            id: "19.4".into(),
            chapter: 19,
            chapter_title: "Durcissement SSH".into(),
            act: 5,
            title: "Appliquer la config SSH".into(),
            description: "Appliquez la nouvelle configuration en redémarrant SSH.".into(),
            story: "Tael Rhen revient sur l'intercom, la voix pressée. « Le durcissement est en
place : la ligne d'interdiction est dans le fichier, et le service tourne
toujours. Il ne reste qu'une étape : redémarrer la liaison sécurisée pour
qu'elle reparte avec la nouvelle configuration. »

Le ronronnement des machines vous parvient en fond.

« Relancez le service ssh, Capitaine. C'est lui qui applique les réglages
au démarrage : tant qu'il n'a pas redémarré, la porte la plus dangereuse est
encore ouverte. »

Vous relancez la liaison sécurisée. Le service repart aussitôt avec les
réglages durcis.

« Le service est reparti avec la porte condamnée. Bien joué, Capitaine. »"
                .into(),
            goal: "Redémarrez le service ssh pour appliquer la nouvelle configuration.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment redémarrer un service après modification de sa config?".into(),
            required_commands: vec!["sudo systemctl restart ssh".into()],
            checks: vec![
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "restart".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "sim_event".into(),
                    target: "systemctl restart ssh".into(),
                    expected: None,
                },
            ],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill {
                command: "systemctl restart".into(),
                name: "systemctl restart".into(),
                description: "systemctl restart — redémarre un service pour appliquer sa config"
                    .into(),
            }),
        },
        Mission {
            id: "19.5".into(),
            chapter: 19,
            chapter_title: "Durcissement SSH".into(),
            act: 5,
            title: "Logs du service SSH".into(),
            description: "Consultez les 10 dernières lignes du journal du service SSH.".into(),
            story: "Draz M'kal vous fait signe près de la console de sécurité. « Le service a
redémarré, Capitaine. Maintenant, vérifions que tout s'est bien passé — le
journal du service raconte ce qui se passe à l'intérieur. »

Il se penche sur l'écran.

« Affichez les 10 dernières entrées du journal de la liaison sécurisée (ssh) :
je veux voir ce qui s'est passé récemment, pas toute l'histoire depuis le
début. C'est là qu'on repère une connexion suspecte dès son arrivée. »

Vous affichez les dernières entrées du journal. La relance du service y
figure, ainsi que les connexions récentes.

« Le journal montre une relance propre, Capitaine. La liaison sécurisée
est durcie et surveillée. »"
                .into(),
            goal: "Affichez les 10 dernières lignes du journal du service ssh.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment consulter les logs d'un service précis?".into(),
            required_commands: vec!["journalctl -u ssh -n 10".into()],
            checks: vec![MissionCheck {
                check_type: "command_executed".into(),
                target: "journalctl".into(),
                expected: None,
            }],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill {
                command: "journalctl".into(),
                name: "journalctl".into(),
                description: "journalctl -u service -n N — logs d'un service (N dernières lignes)"
                    .into(),
            }),
        },
    ]
}

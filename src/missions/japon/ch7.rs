//! Campagne 2 (Japon Féodal, niveau MOYEN) — CHAPITRE 7 — Lire les registres (Acte 2) — missions EN DUR
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "7.1".into(),
            chapter: 7,
            chapter_title: "Lire les registres".into(),
            act: 2,
            title: "Échecs de connexion".into(),
            description: "Quelles tentatives ont échoué dans auth.log?".into(),
            story: "Le capitaine des gardes pousse la porte des scribes, un rouleau
de registre à la main: « Seigneur, le scribe a consigné toutes
les tentatives de connexion de la nuit. Un inconnu a frappé au
portail à plusieurs reprises. Montrez-moi les tentatives
ÉCHOUÉES, celles marquées 'Failed'. »

Vous déroulez auth.log, le registre des connexions du château,
rangé dans la salle des scribes. Chaque ligne raconte une
arrivée: qui a frappé, d'où, et si la porte s'est ouverte.

Le capitaine insiste: « Je ne veux pas savoir qui est entré, je
veux les refusés. Cherchez 'Failed' dans le registre: les
refusés sont les seuls qui cherchent à forcer notre porte. »

Vous ouvrez /scribes/logs/auth.log et y cherchez les lignes
'Failed'.

Le capitaine lit les lignes une à une: « Quatre refusés. Un
ronin a frappé à l'aube, puis un espion, puis un voleur. Et
cette autre IP, 10.0.0.99. Nous avons de la lecture, Seigneur. »"
                .into(),
            goal: "Affichez les lignes 'Failed' de /scribes/logs/auth.log.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment chercher un mot dans un fichier?".into(),
            required_commands: vec!["grep Failed /scribes/logs/auth.log".into()],
            checks: vec![MissionCheck {
                check_type: "command_executed".into(),
                target: "grep".into(),
                expected: None,
            }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "grep".into(),
                name: "grep".into(),
                description: "grep — recherche un motif".into(),
            }),
        },
        Mission {
            id: "7.2".into(),
            chapter: 7,
            chapter_title: "Lire les registres".into(),
            act: 2,
            title: "Combien d'attaques?".into(),
            description: "Comptez les 'Failed' dans auth.log.".into(),
            story: "Le capitaine revient, une liasse de contrats sous le bras: «
Seigneur, votre liste des refusés m'aide, mais elle est longue
à compter. Je dois annoncer au seigneur Hattori le NOMBRE exact
de tentatives échouées. Juste le chiffre. »

Vous regardez la liste: compter les lignes une à une, c'est
lent, et l'on se trompe à mi-chemin. Le registre reste ouvert
sur la table des scribes, à la même page: le rouleau auth.log.

Le capitaine précise: « Ne me récitez pas les lignes, Seigneur.
Combien de refusés en tout? Que le registre réponde lui-même:
cherchez 'Failed' et demandez le compte. »

Vous comptez les occurrences 'Failed' dans /scribes/logs/auth.log.

Le capitaine note le chiffre: « Quatre refusés cette nuit.
Quatre, c'est déjà un siège en préparation. Le seigneur Hattori
doit le savoir avant le coucher du soleil. »"
                .into(),
            goal: "Comptez les lignes 'Failed' de auth.log.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment compter les résultats d'une recherche?".into(),
            required_commands: vec!["grep -c Failed /scribes/logs/auth.log".into()],
            checks: vec![MissionCheck {
                check_type: "command_executed".into(),
                target: "-c".into(),
                expected: None,
            }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "grep -c".into(),
                name: "grep -c".into(),
                description: "grep -c — compte les lignes trouvées".into(),
            }),
        },
        Mission {
            id: "7.3".into(),
            chapter: 7,
            chapter_title: "Lire les registres".into(),
            act: 2,
            title: "L'espion avec numéros de ligne".into(),
            description: "Lignes de 10.0.0.66 avec numéros.".into(),
            story: "Hanzō arrive sans bruit, un rouleau de preuves à la main: «
Seigneur, une IP revient sans cesse dans vos registres:
10.0.0.66. Son propriétaire frappe, se retire, revient. Je
veux chaque ligne qui parle de cette IP, avec son NUMÉRO dans
le registre. »

Vous comprenez la demande: pour le procès-verbal, chaque preuve
doit être localisable. Un numéro de ligne, c'est la référence
que les scribes et les juges du clan pourront retrouver.

Hanzō ajoute: « Toutes les lignes où 10.0.0.66 apparaît, pas
une de plus. Et n'oubliez pas le numéro: sans lui, la preuve
est introuvable. »

Vous ouvrez /scribes/logs/auth.log et cherchez 10.0.0.66 en
affichant le numéro de chaque ligne trouvée.

Hanzō relit les lignes numérotées: « Trois visites de cette IP.
Le procès-verbal sera solide: le château saura qui chercher. »"
                .into(),
            goal: "Affichez les lignes de 10.0.0.66 avec leur numéro.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment afficher les numéros de ligne?".into(),
            required_commands: vec!["grep -n 10.0.0.66 /scribes/logs/auth.log".into()],
            checks: vec![MissionCheck {
                check_type: "command_executed".into(),
                target: "-n".into(),
                expected: None,
            }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "grep -n".into(),
                name: "grep -n".into(),
                description: "grep -n — numéros de ligne".into(),
            }),
        },
        Mission {
            id: "7.4".into(),
            chapter: 7,
            chapter_title: "Lire les registres".into(),
            act: 2,
            title: "Chercher 'ronin' sans la casse".into(),
            description: "Trouvez 'ronin' quelle que soit la casse.".into(),
            story: "Le capitaine des gardes tend un rapport griffonné: « Le
scribe de quart écrit vite, et son écriture change selon
l'heure: on trouve 'ronin' en minuscules, mais parfois il lui
échappe une majuscule, 'Ronin', ou même 'RONIN' en capitales.
Je veux toutes les mentions du ronin dans auth.log, quelle que
soit l'écriture. »

Vous comprenez: le registre de la nuit mêle les mains des
différents scribes, et chacun écrit le nom à sa façon. Une
recherche stricte raterait les autres orthographes.

Le capitaine explique: « Si je cherche 'ronin' et que le
scribe a écrit 'RONIN', je ne trouve rien et je crois le
château tranquille. Cherchez le mot peu importe la casse:
majuscule ou minuscule, tout doit remonter. »

Vous cherchez 'ronin' dans /scribes/logs/auth.log sans tenir
compte des majuscules et des minuscules.

Le capitaine lit le résultat: « La mention est là. Une seule,
cette nuit, mais elle est trouvée quelle que soit l'écriture.
Rien ne nous échappe, Seigneur. »"
                .into(),
            goal: "Cherchez 'ronin' dans auth.log insensible à la casse.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment chercher sans tenir compte des majuscules?".into(),
            required_commands: vec!["grep -i ronin /scribes/logs/auth.log".into()],
            checks: vec![MissionCheck {
                check_type: "command_executed".into(),
                target: "-i".into(),
                expected: None,
            }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill {
                command: "grep -i".into(),
                name: "grep -i".into(),
                description: "grep -i — insensible à la casse".into(),
            }),
        },
        Mission {
            id: "7.5".into(),
            chapter: 7,
            chapter_title: "Lire les registres".into(),
            act: 2,
            title: "Deux registres en une commande".into(),
            description: "Comptez les lignes de auth.log ET syslog.".into(),
            story: "Dame Kaguya consulte les rouleaux de la journée: « Le scribe a
compté séparément les registres, et sûrement faux quelque part.
Combien de lignes font auth.log et syslog? Faites-moi le compte
des deux d'un coup, sans recommencer. »

Vous connaissez les deux registres: auth.log, les connexions,
et syslog, les événements du château. Tous deux dorment dans la
salle des scribes, sous /scribes/logs. Compter l'un puis
l'autre prendrait deux fois plus de temps.

Kaguya précise: « Je veux les deux totaux en une seule
opération: qu'on lise les deux registres d'un même geste, comme
on inspecte deux portes d'une même ronde. »

Vous comptez les lignes de /scribes/logs/auth.log et de
/scribes/logs/syslog en une seule commande.

Dame Kaguya note les totaux: « Cinq lignes de connexions,
quatre d'événements. Les comptes du scribe étaient faux. Merci,
Seigneur: vos registres sont dignes de confiance. »"
                .into(),
            goal: "Comptez les lignes des deux fichiers de logs.".into(),
            bloom_level: "Analyser".into(),
            bloom_question: "Comment donner deux fichiers à une commande?".into(),
            required_commands: vec!["wc -l /scribes/logs/auth.log /scribes/logs/syslog".into()],
            checks: vec![
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "wc".into(),
                    expected: None,
                },
                MissionCheck {
                    check_type: "command_executed".into(),
                    target: "-l".into(),
                    expected: None,
                },
            ],
            hints: vec![],
            xp_reward: 125,
            code_required: None,
            skill: Some(Skill {
                command: "wc".into(),
                name: "wc".into(),
                description: "wc — compte lignes, mots et caractères".into(),
            }),
        },
    ]
}

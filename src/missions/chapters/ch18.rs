//! CHAPITRE 10 - Durcissement
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "18.1".into(), chapter: 18, chapter_title: "Durcissement".into(), act: 5,
            title: "fail2ban".into(),
            description: "Bannissez automatiquement!".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Les pirates reviennent sans cesse. fail2ban les bannira automatiquement.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Installer fail2ban est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Les pirates reviennent sans cesse. fail2ban les bannira automatiquement.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Le videur automatique est en place. Les adresses malveillantes seront bannies.

« Le videur fait son travail », note Draz M'kal. « Mais il faut vérifier qu'il ne bannit pas des alliés par erreur. »".into(),
            goal: "Les pirates reviennent sans cesse. Il faut un videur automatique.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Automatiser le bannissement?".into(),
            required_commands: vec!["sudo fail2ban-client status".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "fail2ban".into(), expected: None }],
            hints: vec![],
            xp_reward: 300,
            code_required: None,
            skill: Some(Skill { command: "fail2ban-client".into(), name: "fail2ban-client".into(), description: "fail2ban-client — bannit automatiquement les IPs pirates (jail, ban, unban)".into() }),
        },
        Mission {
            id: "18.2".into(), chapter: 18, chapter_title: "Durcissement".into(), act: 5,
            title: "Durcir SSH".into(),
            description: "Interdisez root et MDP!".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. SSH est vulnérable: root autorisé + mots de passe autorisés = porte ouverte.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Durcir SSH est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: SSH est vulnérable: root autorisé + mots de passe autorisés = porte ouverte.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Le login root est interdit et les mots de passe sont désactivés en SSH.

« Bien », approuve Draz M'kal. « Des failles de moins. Chaque ligne durcie renforce nos défenses. »".into(),
            goal: "Fermez la porte aux attaques: plus de root, plus de mots de passe en SSH.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Securiser SSH?".into(),
            required_commands: vec!["nano /securite/config/sshd_config".into()],
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
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Au-delà d'UFW, iptables permet de limiter le débit de connexions SSH.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Règles iptables est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Au-delà d'UFW, iptables permet de limiter le débit de connexions SSH.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Les connexions SSH sont limitées au strict minimum par le pare-feu noyau.

« Le noyau filtre chaque paquet », explique Draz M'kal. « Ces règles sont la dernière barrière. »".into(),
            goal: "Limitez les connexions SSH au strict minimum avec le pare-feu noyau.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Limiter les connexions?".into(),
            required_commands: vec!["sudo iptables -L -n".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "iptables".into(), expected: None }],
            hints: vec![],
            xp_reward: 350,
            code_required: None,
            skill: Some(Skill { command: "iptables".into(), name: "iptables".into(), description: "iptables".into() }),
        },
        Mission {
            id: "18.4".into(), chapter: 18, chapter_title: "Durcissement".into(), act: 5,
            title: "Audit".into(),
            description: "Surveillez passwd et shadow.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Si un pirate crée un compte, il faut le détecter. Auditez passwd et shadow.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Audit système est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Si un pirate crée un compte, il faut le détecter. Auditez passwd et shadow.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

La surveillance des comptes est en place. Chaque création sera tracée.

« La surveillance est notre meilleure alliée », conclut Draz M'kal. « Si un pirate crée un compte, nous le saurons. »".into(),
            goal: "Si un pirate crée un compte, il faut le savoir. Mettez en place la surveillance.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Tracer les modifications?".into(),
            required_commands: vec!["sudo auditctl -l".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "auditctl".into(), expected: None }],
            hints: vec![],
            xp_reward: 350,
            code_required: None,
            skill: Some(Skill { command: "auditctl".into(), name: "auditctl".into(), description: "auditctl".into() }),
        },
        Mission {
            id: "18.5".into(), chapter: 18, chapter_title: "Durcissement".into(), act: 5,
            title: "Chroot".into(),
            description: "Isolez un processus!".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Dernière barrière: isolez un processus dans un chroot pour le contenir définitivement.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre rapport
depuis l'Andromède.

Chroot - la forteresse est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Dernière barrière: isolez un processus dans un chroot pour le contenir définitivement.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.

Le processus pirate est enfermé dans une cellule d'isolement. Il ne peut plus nuire.

« Dernière barrière franchie », conclut Draz M'kal. « Le vaisseau est désormais durci contre les intrusions. »".into(),
            goal: "Dernière barrière: enfermez un processus dans une cellule d'isolement.".into(),
            bloom_level: "Créer".into(),
            bloom_question: "Isoler un processus?".into(),
            required_commands: vec!["sudo chroot /soutes/chroot /bin/bash".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chroot".into(), expected: None }, MissionCheck { check_type: "dir_exists".into(), target: "/soutes/chroot".into(), expected: None }],
            hints: vec![],
            xp_reward: 400,
            code_required: Some("FORTRESS".into()),
            skill: Some(Skill { command: "chroot".into(), name: "chroot".into(), description: "chroot".into() }),
        },
    ]
}

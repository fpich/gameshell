//! CHAPITRE 10 - Durcissement
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "10.1".into(), chapter: 10, chapter_title: "Durcissement".into(), act: 5,
            title: "fail2ban".into(),
            description: "Bannissez automatiquement!".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Les pirates reviennent sans cesse. fail2ban les bannira automatiquement.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Installer fail2ban est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Les pirates reviennent sans cesse. fail2ban les bannira automatiquement.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "apt install fail2ban + enable + start + status.".into(),
            bloom_level: "Create".into(),
            bloom_question: "Automatiser le bannissement?".into(),
            required_commands: vec!["fail2ban".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "fail2ban".into(), expected: None }],
            hints: vec![],
            xp_reward: 300,
            code_required: None,
            skill: Some(Skill { command: "fail2ban".into(), name: "fail2ban".into(), description: "fail2ban".into() }),
        },
        Mission {
            id: "10.2".into(), chapter: 10, chapter_title: "Durcissement".into(), act: 5,
            title: "Durcir SSH".into(),
            description: "Interdisez root et MDP!".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. SSH est vulnérable: root autorisé + mots de passe autorisés = porte ouverte.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Durcir SSH est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: SSH est vulnérable: root autorisé + mots de passe autorisés = porte ouverte.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Modifiez /securite/config/sshd_config: PermitRootLogin no + PasswordAuthentication no.".into(),
            bloom_level: "Create".into(),
            bloom_question: "Securiser SSH?".into(),
            required_commands: vec!["sshd_config".into()],
            checks: vec![MissionCheck { check_type: "config_contains".into(), target: "/securite/config/sshd_config".into(), expected: Some("PermitRootLogin no".into()) }, MissionCheck { check_type: "config_contains".into(), target: "/securite/config/sshd_config".into(), expected: Some("PasswordAuthentication no".into()) }],
            hints: vec![],
            xp_reward: 350,
            code_required: None,
            skill: Some(Skill { command: "ssh_hardening".into(), name: "ssh_hardening".into(), description: "ssh_hardening".into() }),
        },
        Mission {
            id: "10.3".into(), chapter: 10, chapter_title: "Durcissement".into(), act: 5,
            title: "iptables".into(),
            description: "Limitez SSH a 3 conn/min.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Au-delà d'UFW, iptables permet de limiter le débit de connexions SSH.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Règles iptables est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Au-delà d'UFW, iptables permet de limiter le débit de connexions SSH.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "iptables -A INPUT ... + iptables -L.".into(),
            bloom_level: "Create".into(),
            bloom_question: "Limiter les connexions?".into(),
            required_commands: vec!["iptables".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "iptables".into(), expected: None }],
            hints: vec![],
            xp_reward: 350,
            code_required: None,
            skill: Some(Skill { command: "iptables".into(), name: "iptables".into(), description: "iptables".into() }),
        },
        Mission {
            id: "10.4".into(), chapter: 10, chapter_title: "Durcissement".into(), act: 5,
            title: "Audit".into(),
            description: "Surveillez passwd et shadow.".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Si un pirate crée un compte, il faut le détecter. Auditez passwd et shadow.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Audit système est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Si un pirate crée un compte, il faut le détecter. Auditez passwd et shadow.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "auditctl -w /etc/passwd -p wa -k modifs + auditctl -l.".into(),
            bloom_level: "Create".into(),
            bloom_question: "Tracer les modifications?".into(),
            required_commands: vec!["auditctl".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "auditctl".into(), expected: None }],
            hints: vec![],
            xp_reward: 350,
            code_required: None,
            skill: Some(Skill { command: "auditctl".into(), name: "auditctl".into(), description: "auditctl".into() }),
        },
        Mission {
            id: "10.5".into(), chapter: 10, chapter_title: "Durcissement".into(), act: 5,
            title: "Chroot".into(),
            description: "Isolez un processus!".into(),
            story: "La situation à bord d'Orion-7 se tend, Capitaine. Dernière barrière: isolez un processus dans un chroot pour le contenir définitivement.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent
nos défenses. Chaque seconde compte. L'Amiral Kowalski attend votre rapport
depuis le Galactica.

Chroot - la forteresse est une étape cruciale pour préparer la station à l'attaque imminente.

Sans cette étape, les systèmes de défense resteront vulnérables et les pirates
pourront prendre le contrôle du vaisseau.

Votre mission: Dernière barrière: isolez un processus dans un chroot pour le contenir définitivement.

N'oubliez pas: dans l'espace, la préparation fait la différence entre la victoire
et la défaite. Chaque commande que vous apprenez ici sera utile pour sauver
la station et son équipage de 5 personnes.".into(),
            goal: "Creez /soutes/chroot/ + sudo chroot.".into(),
            bloom_level: "Create".into(),
            bloom_question: "Isoler un processus?".into(),
            required_commands: vec!["chroot".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "chroot".into(), expected: None }, MissionCheck { check_type: "dir_exists".into(), target: "/soutes/chroot".into(), expected: None }],
            hints: vec![],
            xp_reward: 400,
            code_required: Some("FORTRESS".into()),
            skill: Some(Skill { command: "chroot".into(), name: "chroot".into(), description: "chroot".into() }),
        },
    ]
}

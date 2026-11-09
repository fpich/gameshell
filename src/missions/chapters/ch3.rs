//! CHAPITRE 3 - Explorer la station (Acte 1) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "3.1".into(), chapter: 3, chapter_title: "Explorer la station".into(), act: 1,
            title: "Fichiers caches de la cabine".into(),
            description: "Zyra-7: certains documents de la cabine sont invisibles au premier abord.".into(),
            story: "Zyra-7, l'IA de bord, vous parle par l'intercom depuis la salle des machines:
« Capitaine, avant de poursuivre, familiarisez-vous avec votre cabine.
Certains fichiers sont cachés: ils commencent par un point et n'apparaissent
pas avec un simple listing. »

Elle vous explique que sur Orion-7, les documents sensibles du capitaine
sont stockés dans des fichiers cachés, comme le fichier .secret posé sur
votre bureau.

« Affichez TOUT le contenu de votre cabine, y compris les fichiers cachés,
puis lisez ce fichier .secret. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

Zyra-7 approuve: « Bien vu, Capitaine. Les fichiers cachés sont la mémoire discrète du vaisseau. »

Le contenu du fichier .secret s'affiche à l'écran. Vous comprenez que rien, à bord, ne doit vous échapper: chaque document, même invisible, peut contenir une information capitale.".into(),
            goal: "Zyra-7 veut la liste complète de votre cabine, fichiers cachés compris, et le contenu du fichier caché.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment lister aussi les fichiers caches?".into(),
            required_commands: vec!["ls -la".into(), "cat .secret".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "-la".into(), expected: None }, MissionCheck { check_type: "file_read".into(), target: ".secret".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "ls -la".into(), name: "ls -la".into(), description: "ls -la — liste tout, y compris les fichiers cachés".into() }),
        },
        Mission {
            id: "3.2".into(), chapter: 3, chapter_title: "Explorer la station".into(), act: 1,
            title: "Trouver un document".into(),
            description: "Nira Okoro a besoin du dernier message de l'Andromède, perdu quelque part.".into(),
            story: "Nira Okoro, officier communications, coupe l'intercom:
« Capitaine, j'ai égaré le chemin exact du dernier message de l'Amiral Xylia Vorn.
Le fichier s'appelle dernier_message.txt mais je ne sais plus dans quel
compartiment il se trouve. »

Elle vous conseille d'utiliser l'outil de recherche du vaisseau qui fouille
TOUS les compartiments à la recherche d'un nom de fichier.

« Trouvez où se cache dernier_message.txt, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Nira Okoro pousse un soupir de soulagement: « Le voilà! Merci, Capitaine. Sans vous, ce message serait resté introuvable. »

Vous réalisez que le vaisseau est vaste et que chaque fichier a sa place. Savoir fouiller méthodiquement vous sera précieux pour la suite.".into(),
            goal: "Retrouvez dans quel compartiment du vaisseau se trouve le fichier dernier_message.txt.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment rechercher un fichier par son nom dans tout le vaisseau?".into(),
            required_commands: vec!["find / -name dernier_message.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "find".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "find".into(), name: "find".into(), description: "find — recherche des fichiers par nom dans tout le vaisseau".into() }),
        },
        Mission {
            id: "3.3".into(), chapter: 3, chapter_title: "Explorer la station".into(), act: 1,
            title: "Poids des soutes".into(),
            description: "Tael Rhen veut savoir combien occupent les soutes.".into(),
            story: "Tael Rhen, ingénieur-chef des machines, vous interpelle:
« Capitaine, avant qu'on remplisse les soutes de nouvelles caisses,
je voudrais savoir combien d'espace elles occupent déjà. »

Il explique que chaque compartiment du vaisseau a un poids numérique:
la somme de tous les fichiers qu'il contient.

« Donnez-moi la taille totale des soutes, Capitaine. Une seule commande. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Tael Rhen note la taille sur sa tablette: « Parfait. Je saurai maintenant combien de caisses je peux encore ranger. »

Les soutes sont le ventre du vaisseau: savoir ce qu'elles contiennent et combien elles pèsent évite bien des surprises lors des ravitaillements.".into(),
            goal: "Indiquez à Tael Rhen l'espace total occupé par le compartiment /soutes.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment connaître la taille totale d'un compartiment?".into(),
            required_commands: vec!["du -sh /soutes".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "du".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "du -sh".into(), name: "du -sh".into(), description: "du -sh — taille totale lisible d'un compartiment".into() }),
        },
        Mission {
            id: "3.4".into(), chapter: 3, chapter_title: "Explorer la station".into(), act: 1,
            title: "Monter au pont".into(),
            description: "L'Amiral Xylia Vorn vous attend au pont de commandement.".into(),
            story: "L'intercom grésille: « Capitaine! Amiral Xylia Vorn, vaisseau-mère Andromède.
Je vous attends au PONT de commandement pour le briefing de défense. »

Vous quittez votre cabine et empruntez les coursives jusqu'au pont.
Le pont est le compartiment /pont à la racine du vaisseau.

« Venez me voir au pont, Capitaine. Les Corbeaux Noirs approchent. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Vous poussez la porte du pont de commandement. L'Amiral Xylia Vorn vous accueille d'un signe de tête.

« Capitaine, vous voilà enfin. Les Corbeaux Noirs approchent, et c'est d'ici que nous allons les repousser. »".into(),
            goal: "Rejoignez le pont de commandement (/pont) où vous attend l'Amiral.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment se déplacer jusqu'au pont avec un chemin absolu?".into(),
            required_commands: vec!["cd /pont".into()],
            checks: vec![MissionCheck { check_type: "dir_reached".into(), target: "/pont".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "cd".into(), name: "cd".into(), description: "cd — change de répertoire (cd /pont, cd .., cd ~)".into() }),
        },
        Mission {
            id: "3.5".into(), chapter: 3, chapter_title: "Explorer la station".into(), act: 1,
            title: "Premier rappel sur le bureau".into(),
            description: "Zyra-7 vous demande de noter un rappel sur votre bureau.".into(),
            story: "Zyra-7 revient à l'intercom, plus calme:
« Capitaine, notez sur votre bureau un rappel pour ne rien oublier:
'Verifier les boucliers avant l'attaque'. »

Elle vous explique que pour écrire directement dans un fichier sans l'ouvrir,
on redirige un texte vers le fichier: le contenu est enregistré tel quel.

« Créez le fichier rappel.txt sur votre bureau avec ce message dedans. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Zyra-7 vérifie votre fichier: « C'est noté, Capitaine. Les boucliers seront vérifiés avant l'attaque. »

Vous refermez votre carnet. Un bon capitaine note tout: les rappels écrits sont la garantie que rien ne sera oublié dans le feu de l'action.".into(),
            goal: "Créez un fichier rappel.txt dans votre cabine contenant le texte demandé par Zyra-7.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment écrire un texte directement dans un fichier?".into(),
            required_commands: vec!["echo 'Verifier les boucliers' > rappel.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">".into(), expected: None }, MissionCheck { check_type: "file_exists".into(), target: "~/rappel.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 100,
            code_required: None,
            skill: Some(Skill { command: "echo ... > fichier".into(), name: ">".into(), description: "COMMANDE > fichier — redirige la sortie vers un fichier (écrase)".into() }),
        },
    ]
}

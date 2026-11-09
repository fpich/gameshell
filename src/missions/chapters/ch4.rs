//! CHAPITRE 4 - Journal de bord (Acte 1) — missions EN DUR (règle C1)
use crate::missions::types::*;

pub fn missions() -> Vec<Mission> {
    vec![
        Mission {
            id: "4.1".into(), chapter: 4, chapter_title: "Journal de bord".into(), act: 1,
            title: "Lire vos notes personnelles".into(),
            description: "Vos propres notes de cabine: notes.txt.".into(),
            story: "Sur votre bureau, à côté de bienvenue.txt, se trouve votre carnet personnel:
notes.txt.

Zyra-7 vous conseille de le relire: le précédent capitaine y a noté des
choses importantes sur la station.

« Lisez vos notes, Capitaine. Elles contiennent des indices sur les
habitudes de sécurité de l'équipage. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Vous relisez vos notes. Chaque ligne vous rappelle une consigne, une astuce, un avertissement.

« Gardez ce carnet précieusement, Capitaine », murmure Zyra-7. « C'est votre mémoire, et elle vous servira tout au long de votre mission. »".into(),
            goal: "Lisez le contenu de votre fichier notes.txt dans votre cabine.".into(),
            bloom_level: "Comprendre".into(),
            bloom_question: "Comment lire le contenu d'un fichier texte?".into(),
            required_commands: vec!["cat notes.txt".into()],
            checks: vec![MissionCheck { check_type: "file_read".into(), target: "notes.txt".into(), expected: None }],
            hints: vec![],
            xp_reward: 50,
            code_required: None,
            skill: Some(Skill { command: "cat".into(), name: "cat".into(), description: "cat — affiche le contenu complet d'un fichier texte".into() }),
        },
        Mission {
            id: "4.2".into(), chapter: 4, chapter_title: "Journal de bord".into(), act: 1,
            title: "Ajouter une ligne au journal".into(),
            description: "Ajoutez une entrée à la fin de notes.txt sans l'écraser.".into(),
            story: "Zyra-7: « Capitaine, ajoutez une nouvelle entrée à la FIN de votre carnet
sans effacer le reste: 'Verifier les boucliers avant attaque'. »

Elle vous prévient: avec une simple redirection, le fichier serait écrasé.
Il faut une redirection qui AJOUTE à la fin du fichier existant.

« Ne perdez pas vos notes précédentes, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Votre note s'ajoute à la fin du carnet sans rien effacer. Zyra-7 approuve: « Bien. Chaque ajout enrichit votre mémoire sans perdre l'existant. »

Vous comprenez l'importance de consigner sans détruire: l'historique complet est souvent plus précieux que la dernière ligne.".into(),
            goal: "Ajoutez la ligne demandée à la fin de notes.txt sans effacer le contenu existant.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment ajouter du texte à la fin d'un fichier sans l'écraser?".into(),
            required_commands: vec!["echo 'Verifier les boucliers avant attaque' >> notes.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: ">>".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "echo".into(), name: "echo".into(), description: "echo — affiche du texte à l'écran".into() }),
        },
        Mission {
            id: "4.3".into(), chapter: 4, chapter_title: "Journal de bord".into(), act: 1,
            title: "Compter les lignes du guide".into(),
            description: "Combien de lignes fait bienvenue.txt?".into(),
            story: "Nira Okoro passe par là: « Capitaine, pour calibrer l'affichage des écrans
du pont, j'ai besoin de savoir combien de lignes fait le guide de survie
bienvenue.txt. »

Elle vous demande de compter uniquement les lignes du fichier, sans afficher
le contenu.

« Juste le nombre, Capitaine. »

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Nira Okoro note le nombre: « Merci, Capitaine. Je saurai maintenant si le guide est complet ou s'il manque des sections. »

Compter les lignes d'un document, c'est en mesurer l'ampleur. Un outil simple, mais essentiel pour évaluer un fichier d'un coup d'œil.".into(),
            goal: "Donnez à Nira Okoro le nombre de lignes du fichier bienvenue.txt.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment compter les lignes d'un fichier?".into(),
            required_commands: vec!["wc -l bienvenue.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "wc".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "wc -l".into(), name: "wc".into(), description: "wc -l — compte les lignes d'un fichier".into() }),
        },
        Mission {
            id: "4.4".into(), chapter: 4, chapter_title: "Journal de bord".into(), act: 1,
            title: "Début de l'historique du pont".into(),
            description: "Les premières lignes du journal de bord du pont.".into(),
            story: "Au pont, l'Amiral Xylia Vorn consulte le grand journal de bord:
historique.txt dans /pont/journaux-de-bord/.

Â« Capitaine, montrez-moi seulement le DÃBUT de l'historique du pont.
Les 5 premiÃ¨res lignes. Pas besoin de tout dÃ©filer. Â»

Le fichier est long: il ne faut afficher que ses premiÃ¨res lignes.

Les Corbeaux Noirs ont Ã©tÃ© dÃ©tectÃ©s dans le secteur 7G et leurs sondes scannent nos dÃ©fenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mÃ¨re AndromÃ¨de.

La station Orion-7 est un vaisseau de classe Ã©claireur postÃ© en sentinelle. Cinq membres d'Ã©quipage comptent sur vous pour la dÃ©fendre.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

Le début du journal de bord s'affiche. Vous y lisez les premières entrées de la journée, écrites par l'équipage.

« Le début d'un journal raconte souvent l'essentiel », commente Zyra-7. « C'est là que se cachent les premières anomalies. »".into(),
            goal: "Affichez uniquement les premières lignes du journal de bord du pont.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment n'afficher que le début d'un fichier long?".into(),
            required_commands: vec!["head -5 /pont/journaux-de-bord/historique.txt".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "head".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "head".into(), name: "head".into(), description: "head -n N — affiche les N premières lignes d'un fichier".into() }),
        },
        Mission {
            id: "4.5".into(), chapter: 4, chapter_title: "Journal de bord".into(), act: 1,
            title: "Fin des logs d'authentification".into(),
            description: "Les dernières tentatives de connexion pirates dans auth.log.".into(),
            story: "Vex Korshun, officier sécurité, vous montre les logs d'authentification:
/securite/logs/auth.log.

« Capitaine, les dernières entrées de ce fichier sont les tentatives de
connexion des pirates cette nuit. Montrez-moi seulement la FIN du fichier. »

Le fichier est long: il ne faut afficher que ses dernières lignes.

Les Corbeaux Noirs ont été détectés dans le secteur 7G et leurs sondes scannent nos défenses. Chaque seconde compte. L'Amiral Xylia Vorn attend votre action depuis le vaisseau-mère Andromède.

La station Orion-7 est un vaisseau de classe éclaireur posté en sentinelle. Cinq membres d'équipage comptent sur vous pour la défendre.

Les dernières lignes du journal de sécurité défilent. Vous y repérez des tentatives de connexion suspectes.

Draz M'kal, chef de la sécurité, fronce les sourcils: « La fin du journal, c'est ce qui s'est passé en dernier. C'est là qu'on voit les intrusions en cours. »".into(),
            goal: "Affichez uniquement les dernières lignes du fichier auth.log de la sécurité.".into(),
            bloom_level: "Appliquer".into(),
            bloom_question: "Comment n'afficher que la fin d'un fichier long?".into(),
            required_commands: vec!["tail -3 /securite/logs/auth.log".into()],
            checks: vec![MissionCheck { check_type: "command_executed".into(), target: "tail".into(), expected: None }],
            hints: vec![],
            xp_reward: 75,
            code_required: None,
            skill: Some(Skill { command: "tail".into(), name: "tail".into(), description: "tail -n N — affiche les N dernières lignes d'un fichier".into() }),
        },
    ]
}

# 📋 Suivi de revue des 300 missions — GameShell

> Historique complet: le suivi initial (~210 Ko, tout en un seul fichier) a été
> découpé en fichiers par chapitre dans ce répertoire. Les bilans et l'état de clôture
> historique sont conservés ci-dessous. Pour l'état RÉEL actuel, voir `ETAT_ACTUEL.md`
> à la racine du dépôt.

## Header original

# 📋 SUIVI DE REVUE DES 300 MISSIONS — GameShell Orion-7

Suivi compact de la relecture/jouabilité des missions (3 campagnes × 100).
Format: `Campagne N° | compréhension initiale | défaut | modification | rejeu | point restant`.

**Version de travail**: HEAD 1f79a47 + corrections en cours (branche master).
**Règles préservées**: identifiants, numérotation, sauvegardes, difficulté par campagne, AGENTS.md.

---

## Méthode (cycle débutant)

Méthode : lire le cadre mission affiché (story + 🎯 + briefs 🆕) SANS consulter required_commands/checks ; déduire la commande ; jouer en pipe (`GAMESHELL_HOME=/tmp/gsh_debutant`, monde régénéré) ; `gsh check` ; consigner. Chaque mission = une exécution propre (rm -rf).

## Campagne 3 — Nexus Réseau (contexte initial)

### Chapitre 1 (1.1 → 1.5) — RÉÉCRIT + REJOUÉ ✅

**Défauts communs** (toutes les missions du chapitre) :
- 6 paragraphes « bouche-trou » répétés à l'identique dans TOUTES les missions reseau (scans SYN 10.0.0.66, "NOC Nexus-9 est le cœur...", "un réseau bien surveillé ne tombe jamais", "bruteforce SSH depuis 10.0.0.99", "le pare-feu bloque déjà", "Le NOC Neon-9 poursuit sa veille" + "Chaque commande que vous apprenez ici...") — hors-sujet, informatif redondant, nuit à la compréhension de la consigne.
- Confirmation du PNJ (remerciement/réussite) arrivait AVANT l'action du joueur.
- Incohérence: « NOC Nexus-9 » vs « NOC Neon-9 » (monde/hostname).

| Mission | Compréhension initiale | Défaut observé | Modification | Rejeu | Point restant |
|---|---|---|---|---|---|
| 3.1.1 Premier contact avec Nexus | Le joueur doit dire à Rosa où il est (chemin) et ce qui l'entoure (contenu) → `pwd` + `ls` | Story remplie de bouche-trous hors-sujet; Rosa confirme AVANT l'action | Story réécrite: Rosa demande le chemin exact puis le contenu du poste, avec indice « le système doit afficher le chemin complet »; confirmation APRÈS l'action; suppression des paragraphes parasites | ✅ `pwd`+`ls`+`gsh check` → « ✅ Mission réussie! » | AUCUN |
| 3.1.2 Le guide du NOC | Lire `bienvenue.txt` au poste → `cat`, puis donner le code | Bouche-trous; confirmation précoce | Story réécrite: Rosa explique où trouver le guide (« dans ton répertoire de travail »), annonce le code d'accueil; confirmation après lecture | ✅ `cat bienvenue.txt`+`check`+`gsh code NEXUS-ACCUEIL` → « ✅ Code correct! » | AUCUN |
| 3.1.3 Config SSH du cluster | Aller dans `/configs/sshd` et lire `sshd_config` → `cd` + `cat` | Bouche-trous; « L'IDS... bruteforce SSH » contredisait le besoin de durcir (l'option était déjà compromise?) | Story réécrite: Karl explique le but (durcir avant sync des badges), le chemin exact, le code de vérification | ✅ `cd /configs/sshd`+`cat sshd_config`+`check`+`code NEXUS-SSH-22` → « ✅ Code correct! » | AUCUN |
| 3.1.4 Qui êtes-vous? | Prouver l'identité (nom du NOC, user, uid/gid/groupes) → `hostname`+`whoami`+`id` | Bouche-trous; confirmation précoce | Story réécrite: l'agent demande explicitement nom de la machine, user, identifiants complets; confirmation après | ✅ `hostname`+`whoami`+`id`+`check` → « ✅ Mission réussie! » | AUCUN |
| 3.1.5 Horodatage des incidents | Afficher l'heure système → `date` | Bouche-trous; confirmation précoce | Story réécrite: Rosa explique pourquoi UTC est nécessaire pour corréler les logs; confirmation après | ✅ `date`+`check` → « ✅ Mission réussie! » | AUCUN |

**Vérifications réalisées**:
- `cargo build --release` : 0 erreur.
- `cargo test --release missions_conformes` : PASS (C3/C4/C5/C13/A15/P1.1).
- Rejeu joueur 1.1-1.5 (essais pipes) : missions validées dans l'ordre, codes phase 2 OK.

### Chapitre 2 (2.1 → 2.5) — RÉÉCRIT + REJOUÉ ✅

**Défauts corrigés** : mêmes bouche-trous que ch1 (retirés), confirmation PNJ après l'action, « NOC Nexus-9 » → « NOC Neon-9 ». Story 2.5 complétée (C3 : 19 → 20+ lignes) avec l'indice « Les deux captures sont dans votre répertoire de travail ».

| Mission | Compréhension initiale | Défaut observé | Modification | Rejeu | Point restant |
|---|---|---|---|---|---|
| 3.2.1 Nouveau ticket | Créer un ticket d'incident au poste → `touch ticket_incident.txt` + `ls -la` | Bouche-trous; « le pare-feu bloque déjà » contredisait la mission | Story réécrite: Rosa demande un ticket nommé `ticket_incident.txt` puis la liste détaillée — « avec la taille, les droits… j'ai besoin de tout voir » (`ls -la`) | ✅ `touch ticket_incident.txt`+`ls -la`+`check` → réussi | AUCUN |
| 3.2.2 Classer les rapports | Créer la hiérarchie `archives/2024/incidents` → `mkdir -p` | Bouche-trous | Story réécrite: Karl explique l'arborescence exacte, « le dossier doit exister même si vide » | ✅ `mkdir -p archives/2024/incidents`+`check` → réussi | AUCUN |
| 3.2.3 Backup de la config | Copier `/configs/sshd/sshd_config` dans `/archives/2024/incidents/` → `mkdir -p` + `cp` | Bouche-trous; ambiguïté des deux `archives/` (poste vs racine) | Story réécrite: Rosa précise « dans le dossier d'archives de la racine, pas dans votre poste » | ✅ `mkdir -p /archives/2024/incidents`+`cp`+`check` → réussi | AUCUN (distinction ~/archives vs /archives assumée, cohérente) |
| 3.2.4 Renommer un ticket | Renommer `ticket_incident.txt` en `ticket_urgent.txt` → `mv` | Bouche-trous | Story réécrite: Rosa voit que le ticket doit changer de nom (priorité urgente), confirmation après | ✅ `mv ticket_incident.txt ticket_urgent.txt`+`check` → réussi | AUCUN |
| 3.2.5 Ménage | Supprimer les captures de test `test1.pcap` `test2.pcap` → `rm` | Bouche-trous; C3 19 lignes | Story réécrite (20+ lignes): Rosa indique les fichiers exacts et leur emplacement (« votre répertoire de travail ») | ✅ `rm test1.pcap test2.pcap`+`check` → réussi | AUCUN |

**Vérifications**: build 0 erreur; `missions_conformes` PASS; rejeu pipe 1.1→2.5: 10/10 « ✅ Mission réussie! ».
- **Correction annexe**: `monde-reseau-gen.sh:56` — « Code du coffre-fort réseau: voir /firewall/codes.txt » → `/firewall/coffre/codes.txt` (cohérent avec ch20 et le vrai fichier généré ligne 188).

### Chapitre 3 (3.1 → 3.5) — RÉÉCRIT + REJOUÉ ✅

**Défauts corrigés** : mêmes 6 bouche-trous retirés (dont « le pare-feu bloque déjà les ports sensibles » qui contredisait le rôle du netrunner et « Chaque commande que vous apprenez ici... ») ; confirmation PNJ APRÈS l'action ; « NOC Nexus-9 » → « NOC Neon-9 » ; indices exacts ajoutés (chemin absolu, nom du fichier, phrase mot pour mot) ; stories allongées pour respecter C3 (wrap 60) : 3.1 17→20+, 3.2 18→20+, 3.3 17→20+, 3.4 15→20+, 3.5 14→20+.

| Mission | Compréhension initiale | Défaut observé | Modification | Rejeu | Point restant |
|---|---|---|---|---|---|
| 3.3.1 Fichiers cachés du poste | Lister tout (cachés compris) + lire `.secret` → `ls -la` + `cat .secret` | Bouche-trous hors-sujet; C3 17 lignes | Story réécrite: Karl explique pourquoi les fichiers cachés (tokens d'accès) et demande la liste complète puis la lecture | ✅ `ls -la`+`cat .secret`+`check` → réussi | AUCUN |
| 3.3.2 Trouver un log | Chercher `firewall.log` par nom partout → `find / -name` | Bouche-trous; C3 18 lignes | Story réécrite: Rosa a perdu le chemin, scripts d'archivage en échec, demande la recherche par nom partout | ✅ `find / -name firewall.log`+`check` → réussi | AUCUN |
| 3.3.3 Taille des logs | Mesurer la taille totale de `/logs` → `du -sh /logs` | Bouche-trous; C3 17 lignes | Story réécrite: Karl doit prévoir l'espace d'archivage, demande une mesure unique avec unité | ✅ `du -sh /logs`+`check` → réussi | AUCUN |
| 3.3.4 Salle des baies | Aller à `/baies` → `cd /baies` | Bouche-trous; C3 15 lignes | Story réécrite: convocation réunion salle BAIES, indice « à la racine, chemin absolu depuis n'importe où » | ✅ `cd /baies`+`check` → réussi | AUCUN |
| 3.3.5 Note d'alerte | Créer `alerte.txt` avec la phrase exacte → `echo '...' > alerte.txt` | Bouche-trous (dont « scans SYN » répétitif); C3 14 lignes | Story réécrite: Rosa dicte la phrase mot pour mot, fichier au poste | ✅ `echo 'Scan SYN depuis 10.0.0.66' > alerte.txt`+`check` → « ✅ Mission réussie! » | AUCUN |

**Vérifications**: build 0 erreur; `missions_conformes` PASS; rejeu pipe 1.1→3.5 (15/15 validées, 3.5 reprise séparément après timeout du pipe — pas un échec de mission); `monde-reseau-gen.sh` régénéré (monde de test frais).

### Chapitre 4 (4.1 → 4.5) — RÉÉCRIT + REJOUÉ ✅

**Défauts corrigés** : mêmes 6 bouche-trous retirés; confirmation PNJ après l'action; « NOC Nexus-9 » → « NOC Neon-9 »; indices ajoutés (dossier /logs pour auth.log/syslog, « sans afficher le contenu » pour wc -l, distinction `>>` vs `>` expliquée); stories allongées pour C3 (4.1 16→20+, 4.2 19→20+, 4.3 15→20+, 4.4 18→20+, 4.5 16→20+).

| Mission | Compréhension initiale | Défaut observé | Modification | Rejeu | Point restant |
|---|---|---|---|---|---|
| 3.4.1 Lire vos notes | Lire `notes.txt` au poste → `cat` | Bouche-trous; C3 16 | Story réécrite: Karl explique que le carnet est la mémoire commune des procédures | ✅ `cat notes.txt`+`check` → réussi | AUCUN |
| 3.4.2 Ajouter au registre | Ajouter une ligne à la fin de `notes.txt` sans écraser → `>>` | Bouche-trous; C3 19 | Story réécrite: Rosa insiste « ajoute, ne remplace pas », indice sur les chevrons | ✅ `echo 'Verifier ids.log a 14h' >> notes.txt`+`check` → réussi | AUCUN |
| 3.4.3 Compter les lignes | Compter les lignes de `bienvenue.txt` → `wc -l` | Bouche-trous; C3 15 | Story réécrite: « uniquement le nombre, sans afficher le contenu »; le guide est au poste | ✅ `wc -l bienvenue.txt`+`check` → réussi | AUCUN |
| 3.4.4 Début d'un log | 5 premières lignes d'`auth.log` → `head -5 /logs/auth.log` | Bouche-trous; C3 18 | Story réécrite: Karl précise /logs et « uniquement l'amorce » | ✅ `head -5 /logs/auth.log`+`check` → réussi | AUCUN |
| 3.4.5 Fin du syslog | 3 dernières lignes du syslog → `tail -3 /logs/syslog` | Bouche-trous; C3 16 | Story réécrite: « les plus récentes sont en bas », « seulement la fin » | ✅ `tail -3 /logs/syslog`+`check` → réussi | AUCUN |

### Chapitre 5 (5.1 → 5.5) — RÉÉCRIT + REJOUÉ ✅

**Défauts corrigés** : mêmes 6 bouche-trous retirés; confirmation PNJ avant action → Rosa/Karl expliquent le besoin et confirment après; « NOC Nexus-9 » → « NOC Neon-9 »; stories réécrites autour du diagnostic réseau (interfaces eth0/lo, passerelle 10.0.0.1, table ARP avec 10.0.0.66/10.0.0.254, FAI 10.0.0.254) cohérentes avec `sim/net.rs`; goal 5.5 reformulé (l'objectif nommait quasi « ping » → interdit par C4); paragraphes additionnels pour 5.2/5.3/5.4/5.5 (C3: 19/17/17/18 → 20+).

| Mission | Compréhension initiale | Défaut observé | Modification | Rejeu | Point restant |
|---|---|---|---|---|---|
| 3.5.1 Carte réseau du NOC | Voir interfaces+IP → `ip addr` | Bouche-trous; récit confus | Story réécrite: contexte intrusion SSH, nommer eth0/lo, besoin explicite | ✅ `ip addr`+`check` → réussi | AUCUN |
| 3.5.2 Table de routage | Voir passerelle par défaut → `ip route` | Bouche-trous; C3 19 | Story réécrite: passerelle unique, 2 lignes attendues; paragraphe additionnel | ✅ `ip route`+`check` → réussi | AUCUN |
| 3.5.3 Liens réseau | État interfaces → `ip link` | Bouche-trous; C3 17 | Story réécrite: lien instable, UP/DOWN+adresse matérielle; paragraphe additionnel | ✅ `ip link`+`check` → réussi | AUCUN |
| 3.5.4 Voisins ARP | Table voisins → `ip neigh` | Bouche-trous; C3 17 | Story réécrite: qui était sur le LAN pendant l'attaque; 10.0.0.66/.254 nommées; paragraphe additionnel | ✅ `ip neigh`+`check` → réussi | AUCUN |
| 3.5.5 Lien vers le FAI | 4 pings → 10.0.0.254 | Bouche-trous; goal quasi « ping » (C4); C3 18 | Story réécrite: paquets perdus, « sans limite la cmd continue » → justifie `-c`; goal: « vérifiez… 4 paquets » | ✅ `ping -c 4 10.0.0.254`+`check` → réussi | AUCUN |

**Vérifications ch5** : build 0 erreur; `missions_conformes` PASS; rejeu parcours **1.1→5.5 complet** (25/25 validées, phases code incluses; sauvegarde réinitialisée puis reconstruite par blocs; position finale 6.1).

### Chapitre 6 (6.1 → 6.5) — RÉÉCRIT + REJOUÉ ✅

**Défauts corrigés** : mêmes 6 bouche-trous retirés; confirmation PNJ avant action → Karl/Rosa justifient le besoin (surface d'attaque, sessions actives, procédures netstat des serveurs de secours, port 3306 ciblé, chemins de routeurs); « NOC Nexus-9 » → « NOC Neon-9 »; paragraphes additionnels pour C3 (6.1 18→20+, 6.2 18→20+, 6.4 19→20+, 6.5 18→20+): affichage numérique comparé aux logs, états des connexions, scan limité à un port sans alerter, relais muets de traceroute.

| Mission | Compréhension initiale | Défaut observé | Modification | Rejeu | Point restant |
|---|---|---|---|---|---|
| 3.6.1 Ports en écoute | Sockets en écoute+proc → `ss -tlnp` | Bouche-trous; C3 18 | Story réécrite: surface d'attaque après tentative SSH; « affichage numérique » justifié | ✅ `ss -tlnp`+`check` → réussi | AUCUN |
| 3.6.2 Connexions établies | Sessions TCP actives → `ss -tn state established` | Bouche-trous; C3 18 | Story réécrite: qui tient une session maintenant; états expliqués | ✅ `ss -tn state established`+`check` → réussi | AUCUN |
| 3.6.3 Ancien outil netstat | Ports via netstat → `netstat -tlnp` | Bouche-trous | Story réécrite: serveurs de secours non migrés, comparaison des deux outils | ✅ `netstat -tlnp`+`check` → réussi | AUCUN |
| 3.6.4 Scan d'un port précis | Scan 3306 de 10.0.0.66 → `nmap -p 3306 10.0.0.66` | Bouche-trous; C3 19 | Story réécrite: pourparlers sur services obsolètes, scan limité sans alerter | ✅ `nmap -p 3306 10.0.0.66`+`check` → réussi | AUCUN |
| 3.6.5 Chemin vers la cible | Routeurs vers 10.0.0.66 → `traceroute 10.0.0.66` | Bouche-trous; C3 18 | Story réécrite: routeurs = points de coupe possibles; relais muets expliqués | ✅ `traceroute 10.0.0.66`+`check` → réussi | AUCUN |

**Vérifications ch6** : build 0 erreur; `missions_conformes` PASS; rejeu 6.1→6.5 en pipe (5/5 validées; position finale 7.1).

## Bilans historiques

- **Campagne 1 (espace) TERMINÉE ✅** — ch1→ch20 : stories réécrites + rejouées (5/5 par chapitre) + victoire A19 confirmée + tests limites A17 ; ch1/ch2 vérifiés conformes dès HEAD (seul le rôle de Vex Korshun corrigé : officier sécurité → commandant de la station).
- **Campagne 2 (japon) TERMINÉE ✅** — ch1 → ch20 : stories réécrites + rejouées 100/100 + écran de victoire A19 confirmé (XP 11300, niv 114, skills 76, badges 6). Sauvegarde test finale : `current_mission=100` (`/tmp/gsh_japon/state/save_campagne2_testeur.json`).
- **Campagne 3 (nexus) TERMINÉE ✅** — ch1 → ch20 : stories réécrites + rejouées 100/100 (positions 5, 10, 15 … 100) + écran de victoire A19 confirmé (XP 12700, 80 compétences, 6 badges). Corrections monde : `monde-reseau-gen.sh` (chemin coffre `/firewall/coffre/codes.txt`, `dns_nexus.txt` → 10.0.0.9 cohérent avec `sim/net.rs`) ; `monde-japon-gen.sh` (RONIN majuscule) ; `vaisseau-gen.sh` (tailles soutes calibrées pour 13.4, retrait echo parasite).
- **Clôture du cycle FAITE** : `cargo build --release` 0 erreur/warning + `cargo test --release` 11/11 + `bash make-archive.sh` (gameshell.sh 1,5M) + `./tests/run_tests.sh` **54/54 PASS** (AUTO/EDGE/TORTURE 300/300, A13/A14/A16/A18-A25, victoires 3 campagnes).
- **Précision d'exhaustivité (vérification post-clôture)** : le commit `24377ec` modifie **292 stories** et **9 retouches hors story**, toutes des améliorations assumées et vérifiées cohérentes avec leur mission — (a) camp.1 ch5.1 description : corrigeait un vrai défaut (l'ancienne visait `.secret`/FALCON-9, la cible du check est `notes.txt`) ; (b) camp.1 ch14 : 5 descriptions tronquées réécrites en phrases complètes alignées sur story/goal ; (c) reseau ch8 description : « Tous les démons actifs » → « Tous les processus du cluster » (`ps aux`) ; (d) reseau ch5.5 title+goal : « Ping vers le FAI » → « Test de liaison vers le FAI » + objectif sans nommer `ping` (C4) ; (e) reseau ch10 : accents rétablis dans le seul `chapter_title` « Analyse avancée des logs » — **les identifiants 10.1–10.5 n'ont pas changé**. Les `id`, `required_commands`, `checks`, `skill`, `xp_reward`, `code_required`, `bloom_level`, `goal` (sauf ch5.5) sont restés identiques pour les 300 missions.
- **Commit + push FAITS** : `24377ec` « ✨ Revue complète des 300 missions: réécriture des stories (3 campagnes) » poussé sur `origin master` (`1f79a47..24377ec`), dépôt propre, footer co-auteur Mistral Vibe conforme. Cycle suivant : reprendre uniquement les points ouverts ou les missions affectées par des changements récents ; sinon cycle terminé.
- **Chasse aux erreurs/cas limites (cycle courant)** : correction d'un **mojibake (double encodage UTF‑8)** dans la story de la mission **1.5** (campagne espace, `src/missions/chapters/ch1.rs`) — 24 artefacts `Ã`/`Â` (ex. « Ã l'intercom », « Â« », « derniÃ¨re vÃ©rification », « systÃ¨me ») décodés via latin‑1 → utf‑8 en texte français correct. Vérifié : **zéro artefact `Ã`/`Â` restant sur les 62 fichiers des 300 missions** (les `É`/`È` initialement signalés étaient du français correct, faux positifs). `cargo build --release` 0 erreur/warning + `cargo test --release` 11/11 + `bash make-archive.sh` (gameshell.sh 1,5M) + `./tests/run_tests.sh` **54/54 PASS**. Contenu/narration inchangés (seul l'encodage rétabli).

---

- Missions vérifiées/jouées : **300/300** (3 campagnes × 100, ch1→ch20 chacune).
- Missions corrigées (stories réécrites) : **290+** (toutes sauf camp.1 ch1/ch2, déjà conformes).
- Missions encore à rejouer : **0**.

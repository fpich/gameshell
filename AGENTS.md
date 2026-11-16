# 🚨 RÈGLES STRICTES DE CONFORMITÉ — GameShell Orion-7

Ce fichier est la **bible du projet**. Toute modification doit respecter ces règles.
Toute régression = bug critique. Numérotation fixe, ne jamais renuméroter.

---

# SECTION A — MOTEUR (fonctionnement figé, testé par tests/run_tests.sh)

## RÈGLE A1 — Isolation du monde de jeu (bac à sable bubblewrap)
- Le joueur ne voit JAMAIS le vrai système de fichiers hôte.
- Chaque ligne SHELL s'exécute dans un bac à sable `bwrap` (`shell::run_line`): le monde de la
  campagne est monté en `/`, `/usr` de l'hôte en LECTURE SEULE, `/etc` = celui du JEU
  (`~/.gameshell/systeme/<campagne>/etc`: passwd, group, shadow, état des simulateurs),
  aucun réseau (`--unshare-all`), uid 1000 = capitaine/seigneur/netrunner, `USER`/`HOME`/`HOSTNAME` du jeu.
- La ligne est passée INTACTE à bash (guillemets, redirections collées, `cd /pont && ls`):
  le dossier final est relu sur le descripteur 3.
- `cd /..`, `cat /../../etc/passwd`, `echo $USER`, `ls -l` ne révèlent jamais l'hôte.
- `ls /` et `find /` masquent les points de montage techniques (bin, usr, etc, proc, dev, lib*).
- Sans bwrap: mode DÉGRADÉ (réécriture des chemins + garde-fou cd) signalé au joueur; en root sans bwrap le lanceur refuse de démarrer.
- Prompt: `pseudo@orion7:chemin$` avec chemins propres (`~`, `/pont/...`).

## RÈGLE A2 — Historique des commandes par mission
- Tracké en mémoire (`mission_commands: Vec<String>`), reset à chaque mission ET au lancement.
- `gsh check` ne valide JAMAIS sur l'historique d'une session précédente.

## RÈGLE A3 — Pas de validation automatique
- Une mission ne se termine JAMAIS automatiquement. Seul `gsh check` (manuel) valide.
- Si `code_required` est défini, phase 2: après `gsh check`, le joueur tape `gsh code <CODE>`
  (ou le code seul s'il est en MAJUSCULES/chiffres/tirets). Le shell reste utilisable pendant
  la phase 2 (cat, gsh help, gsh goal...), `gsh cancel` annule la saisie.

## RÈGLE A4 — Check matche sur la commande EFFECTIVE (analyse de la ligne, `game/parse.rs`)
- La ligne est découpée en commandes (`|`, `&&`, `||`, `;`); on compare la commande effective
  (derrière `sudo`, `env`, `nice`, `nohup`, `su -c`, `bash -c`...) — jamais une sous-chaîne.
- Mission requiert `ls -la` (cible multi-mots), joueur tape `ls` → passe (options ignorées).
  `sudo chown x` matche `chown`. `cmd | grep` matche `|` (opérateur hors guillemets).
- Cible multi-mots « base sous-commande » (`apt update`, `systemctl restart`): la sous-commande est exigée.
- Cible option seule (`-la`, `-c`, `-n`, `-aG`, `+x`, `czf`): l'option doit être portée par LA
  commande qui la porte dans la solution (`ls -al` ≡ `ls -la`, `head -n 5` ≡ `head -5`, `kill -KILL` ≡ `kill -9`).
- `file_read`: une commande de LECTURE (cat, less, head, tail, grep...) a reçu le fichier.
- Seules les lignes RÉUSSIES (code de retour 0) comptent: `echo pwd ls`, `ls /armes` ou une commande en erreur ne valident rien.
- Jamais bloquer sur les autres arguments ni sur les préfixes (sudo...).

## RÈGLE A12 — Toute cible de fichier/commande d'une mission doit exister/être valide
- Un fichier référencé dans une mission (ex: plan_defense.sh) DOIT exister dans le vaisseau (généré par vaisseau-gen.sh).
- Un nom d'user dans un exemple (ex: chown) doit être valide → utiliser $USER, jamais un nom fictif.
- Test: jouer chaque mission manuellement au moins une fois (voir TESTS_PLAYTHROUGH.md).

## RÈGLE A5 — Parsing robuste
- Toujours `split_whitespace()`, jamais `split(' ')`.

## RÈGLE A6 — Commandes interceptées
- `pwd` (commande simple) → chemin propre (display_path); `man X` → double définition (C7).
- Dans une ligne composée (`pwd > f`, `whoami | tr`), la vraie commande s'exécute dans le bac à sable.
- `hostname` → orion7, `whoami` → capitaine, `id`, `uname`... : simulateurs (règle A16).
- `cd` (simple ou composé, `cd -` compris) → `current_dir` = dossier final réel (`pwd -P`) du bac à sable.

## RÈGLE A7 — Reset position à chaque nouvelle mission
- Quand une mission est validée, le joueur retourne à sa cabine (`HOME` → prompt `~`).
- Sinon un fichier créé ailleurs fait échouer la mission suivante sans explication.

## RÈGLE A8 — gsh check en échec = JAMAIS révéler quoi que ce soit
- Affiche UNIQUEMENT: `❌ Mission non validée. Relisez le contexte et réessayez.`
- INTERDIT d'indiquer quelle commande/fichier manque (ex: jamais "Non validé: pwd")
- Le joueur doit relire la mission et déduire lui-même

## RÈGLE A11 — Édition de ligne façon terminal réel (OBLIGATOIRE)
- En mode tty (raw mode via stty), la saisie est un vrai éditeur de ligne:
  - ← → déplacent le curseur dans la ligne, insertion au curseur, backspace/suppr effacent au curseur, Home/End (^[[H/^[[F, Ctrl+A/E), Ctrl+U efface la ligne, Ctrl+W efface le mot.
  - ↑ ↓ naviguent dans l'historique de session.
  - Ctrl+C annule la ligne, Ctrl+D sur ligne vide = EOF (sortie propre).
  - Toute AUTRE séquence ANSI est ignorée, jamais traitée comme commande.
- En mode non-tty (pipe, tests): TOUTES les séquences d'échappement ANSI sont nettoyées (strip), input tronqué à 4096 (A13).
- Le terminal est restauré (stty) après chaque lecture, même en cas de sortie anticipée.

## RÈGLE A13 — Robustesse input (OBLIGATOIRE)
- EOF (Ctrl+D, read_line retourne 0) = déconnexion propre avec message.
- Input > 4096 caractères = tronqué (anti-freeze), jamais de crash/panic.
- Lignes vides, tabs, espaces multiples = ignorées proprement (split_whitespace).
- Jamais de panic sur input utilisateur, même binaire/invalide.

## RÈGLE A14 — UX anti-blocage (OBLIGATOIRE)
- Après 2 `gsh check` échoués consécutifs, suggérer 'gsh goal'.
- Le prompt affiche la progression: `pseudo@orion7:~$ [N/100]`.
- Compteur: message `gsh goal` à 2, 5, 8... échecs consécutifs; rappel « relisez le PNJ » à 3, 6, 9...

## RÈGLE A15 — Compétence débloquée = commande de LA mission (OBLIGATOIRE)
- La compétence débloquée (`skill`) doit être une des `required_commands` de la mission (mot de base, ou symbole | > >> présent dans la ligne).
- Jamais une commande d'une autre mission (ex: mission "lire bienvenue.txt" ne débloque PAS ls -la).
- Test: `cargo test` (conformity_tests) vérifie les 300 missions (3 campagnes × 100).

## RÈGLE A18 — Sauvegarde/reprise par campagne (OBLIGATOIRE)
- Progression sauvegardée automatiquement à chaque mission validée, PAR JOUEUR:
  `~/.gameshell/state/save_campagne<N>_<joueur>.json` (nom du joueur, mission courante, XP, niveau, compétences, faites).
- Au lancement, la campagne du joueur reprend là où elle en était; un autre élève sur le même compte repart de zéro.
- Modes test (AUTO/EDGE/TORTURE) = ni sauvegarde ni historique (`Game.test_mode`).
- Test: `run_tests.sh` tests 43 (reprise) et 44 (sauvegarde par joueur).

## RÈGLE A19 — Écran de victoire par campagne (OBLIGATOIRE)
- Quand les 100 missions d'une campagne sont faites: panneau récapitulatif (campagne, missions,
  XP, niveau, liste des compétences), pas une simple ligne.
- Test: test 46 (3 écrans de victoire en auto).

## RÈGLE A21 — gsh restart: recommencer une campagne (OBLIGATOIRE)
- `gsh restart` supprime la sauvegarde de la campagne, remet mission 1.1 / XP 0 / compétences reset,
  et régénère le monde ET l'état système fictif (comptes, pare-feu, processus...).
  Alternative manuelle: `rm ~/.gameshell/state/save_campagne<N>_<joueur>.json`.
- Test: test 47.

## RÈGLE A22 — Historique horodaté de chaque partie (OBLIGATOIRE, pour le RETEX)
- Toute action est loguée avec timestamp dans `~/.gameshell/state/history_campagne<N>_<joueur>.log`:
  début de session, chaque commande SHELL (`CMD ...`), chaque `gsh check` (succès/échec),
  chaque mission réussie (`MISSION x.y réussie (+XP)`), codes phase 2, restart, export.
- `gsh export` embarque cet historique dans le JSON exporté.
- Test: test 48.

## RÈGLE A20 — gsh export enseignant (OBLIGATOIRE)
- `gsh export` exporte la progression en JSON + CSV dans `~/.gameshell/state/export_campagne<N>_<joueur>_<date>.{json,csv}`
  (par mission: id, titre, statut, XP, tentatives; + totaux + compétences). JSON produit par serde (toujours valide),
  CSV avec champs entre guillemets. Le test 45 PARSE le JSON.
- Test: test 45.

## RÈGLE A17 — Checks jamais vacuels (OBLIGATOIRE, testé par GAMESHELL_EDGE=1)
- `gsh check` à vide (aucune commande tapée) doit TOUJOURS échouer.
- `dir_reached` = ÉGALITÉ stricte (jamais starts_with, sinon "/" est toujours atteint).
- Un check `permission` ne doit pas être déjà vrai à l'état initial du monde
  (ex: plan_defense.sh créé 644, coffre-fort 755 — c'est la mission qui les verrouille).
- `file_not_exists` / `permission` sans expected ne valident jamais seuls; un type de check inconnu ne valide jamais.
- Checks d'état système (`user_exists`, `group_exists`, `user_in_group`, `user_has_password`, `package_installed`,
  `process_absent`, `sim_contains`, `sim_not_contains`) jamais vrais dans l'état initial (`cargo test`).
- `sim_event` ne compte que les actions faites PENDANT la mission en cours.
- Test: `GAMESHELL_EDGE=1 ./gameshell.sh` joue les cas limites sur les 300 missions
  (check vide, commande garbage, mauvais code phase 2, bon code, reset position/historique).
- `GAMESHELL_TORTURE=1 ./gameshell.sh` tord chaque mission dans tous les sens (commande partielle,
  typo, 2 mauvais codes) et teste l'isolation sur l'EXÉCUTION réelle (cat /../../etc/passwd, $USER, ls -l,
  cd /.., fichier témoin dans le vrai /tmp, sudo useradd sans effet sur l'hôte, pas de réseau).
- Les bancs n'exécutent QUE les `required_commands` (`solve_mission`): ils ne préparent jamais eux-mêmes
  l'état attendu. Une mission qui échoue au banc = bug de contenu à corriger dans la mission.

## RÈGLE A16 — Identité vaisseau simulée, jamais l'hôte (OBLIGATOIRE)
- `id`, `uname`, `logname`, `whoami`, `hostname`, `groups` sont des simulateurs qui répondent avec l'identité du vaisseau
  (capitaine / orion7 / Linux orion7 6.6.7-orion7...), y compris dans un pipe ou une redirection.
- Aucune commande du jeu ne doit révéler le vrai user, hostname, dossier ou groupes de la machine hôte.

## RÈGLE A23 — Commandes système SIMULÉES, jamais exécutées sur l'hôte (OBLIGATOIRE)
- Le binaire est « multi-appel » (`src/sim/`): installé dans `~/.gameshell/simbin` (monté en `/usr/local/bin`
  du bac à sable, en tête du PATH) sous les noms sudo, su, useradd, groupadd, usermod, passwd, chown,
  ps, kill, top, pgrep, systemctl, journalctl, ufw, iptables, fail2ban-client, auditctl, apt, dpkg,
  chroot, ping, ip, ss, netstat, traceroute, nmap, dig, curl, ssh, mount, df...
- Ils lisent/écrivent l'état fictif de la campagne (`systeme/<campagne>/etc`). `sudo X` = X avec les droits
  « root » du JEU; aucun compte, pare-feu, service, paquet ou processus RÉEL n'est jamais modifié.
- Le « système pirate » (PID 9999, 4242, 6666...) existe vraiment dans la table de processus du jeu.
- Réseau scénarisé (10.0.0.x), catalogue de paquets local: 100% offline (E2).

## RÈGLE A24 — Ctrl+C n'interrompt que la commande en cours (OBLIGATOIRE)
- Le jeu a un gestionnaire SIGINT/SIGQUIT (jamais tué par Ctrl+C), ignore Ctrl+Z.
- bwrap ignore SIGINT; le relais `gsh-sim __exec` rétablit le comportement par défaut pour la commande du joueur.

## RÈGLE A25 — Les tests ne touchent jamais les données réelles (OBLIGATOIRE)
- `run_tests.sh` travaille dans un `GAMESHELL_HOME=$(mktemp -d)` supprimé à la fin; jamais `rm -rf ~/.gameshell`.
- `vaisseau-gen.sh CAMPAGNE DOSSIER` exige 2 arguments, un dossier nommé vaisseau/monde_japon/monde_reseau,
  situé sous `$GAMESHELL_HOME` ou un dossier temporaire, et efface avec `rm -rf --one-file-system`.

## RÈGLE A9 — Pas d'indices
- `gsh hint` est SUPPRIMÉ. Ne jamais le réintroduire.

## RÈGLE A10 — Suite de tests obligatoire
- Avant chaque commit: `cargo test --release` (conformité) + `./tests/run_tests.sh` → **54/54 PASS** obligatoire.

---

# SECTION B — AFFICHAGE & VISUEL (terminal xfce 80x24 fond clair)

## RÈGLE B1 — Bannière de lancement
- Écran d'accueil: saisie du nom puis choix de la campagne; ENSUITE clear écran + bannière de la campagne.
- Campagne 1: espace étoilé compact (~8 lignes) + titre entouré d'étoiles (une seule fois):
  `* 🏴‍☠️ LES PIRATES DE L'ESPACE 🏴‍☠️ *` / `* Mission SISR — Station Orion-7 *`
- **PAS de vaisseau au lancement** (le vaisseau est dans les cartes de mission).

## RÈGLE B2 — gsh help affiché automatiquement au lancement
- Section "Pour revoir l'aide: gsh help" + Bloom expliqué + compétence man + syntaxe shell générique.

## RÈGLE B3 — Syntaxe shell dans gsh help = générique uniquement
```
COMMANDE                  → seule (ex: pwd)
COMMANDE argument         → avec argument (ex: ls /pont)
COMMANDE -option argument → avec option (ex: ls -la /pont)
man COMMANDE              → manuel
COMMANDE | COMMANDE       → chaîne
COMMANDE > fichier        → redirection
```
Jamais de liste de 20 commandes avec exemples ici.

## RÈGLE B4 — Dans la mission: commande affichée telle quelle
- `▸ pwd`, `▸ ls` — sauf si un check exige une option exacte (`-la`, `-n`...): dans ce cas
  on affiche la commande complète (`▸ ls -la`) pour que le check soit atteignable (équité).

## RÈGLE B5 — Cadre mission style "fenêtre terminal"
- `┏━ MISSION X.X — Titre ━┓` ... `┗━━┛`, largeur intérieure 64, bordure droite `┃` alignée
  (largeur d'affichage calculée, emoji = 2 colonnes — `game/ui.rs`), story COMPLÈTE affichée (pas de troncature).

## RÈGLE B6 — Palette fond clair
- Texte courant = `.black()`/`.normal()`. Éléments importants = couleurs vives (vert/jaune).
- Jamais de blanc sur fond clair, jamais de noir invisible.

## RÈGLE B7 — Footer mission = `gsh help` seul
- `┃  🏆 50 XP  │  🧠 Bloom [Se souvenir]  │  gsh help`
- Le joueur trouve `gsh check` dans `gsh help`.

## RÈGLE B8 — Bloom en français
- Se souvenir, Comprendre, Appliquer, Analyser, Évaluer, Créer.

## RÈGLE B9 — ASCII art bien formé
- Testé en 80x24, lignes alignées, reconnaissable visuellement.

---

# SECTION C — CONTENU DES MISSIONS (100 missions par campagne, 20 chapitres, 5 actes)

## RÈGLE C1 — Structure
- **100 missions EN DUR par campagne**, 20 chapitres × 5 missions, 5 actes narratifs.
- **20 missions par acte** (4 chapitres × 5 missions par acte). PAS de génération procédurale:
  tout le contenu est écrit à la main (`chapters/ch1..20.rs` et `japon/ch1..20.rs`).
- Chaque mission: 1-3 nouvelles commandes shell à apprendre.

## RÈGLE C2 — Compétences persistantes
- Chaque mission débloque UNE compétence (commande shell) qui reste acquise.
- Compétence de départ: **man** (déjà acquise, affichée dans gsh skills).

## RÈGLE C3 — Minimum 20 lignes de contexte par mission
- `story` + `description` = 20+ lignes de narration (qui parle, pourquoi, urgence, enjeux).
- Toute la story est affichée au joueur.

## RÈGLE C4 — Objectif narratif, jamais la commande nommée
- Interdit: "Affichez votre position (pwd)".
- Autorisé: "Montrez-lui où vous êtes dans le vaisseau" → l'étudiant déduit `pwd`.
- Une option exigée est décrite par le BESOIN ("avec le numéro de chaque ligne", "sans l'extraire").
- Test: `cargo test` (aucune commande de la solution n'apparaît comme mot de l'objectif).

## RÈGLE C5 — Noms exacts (fichiers, chemins) dans le dialogue PNJ
- Si une mission exige `incident_15jan.txt`, le PNJ doit le dire: « crée incident_15jan.txt ».
- Jamais faire deviner un nom de fichier exact.
- Test: `cargo test` (tout fichier cible de `file_exists`/`file_read` apparaît dans l'histoire ou l'objectif).

## RÈGLE C6 — Carte vue de dessus SUPPRIMÉE (décision utilisateur)
- Plus de champ `map`, plus d'affichage "VUE DE DESSUS" dans les missions: jugé sans intérêt dans le jeu.
- La position du joueur se lit dans le prompt (`pseudo@orion7:/pont$`) et via `pwd`.

## RÈGLE C7 — Double définition commande: roleplay PUIS technique
```
▸ pwd
  🚀 comme demander à l'ordinateur de bord dans quel compartiment vous êtes
  💻 affiche le chemin du répertoire courant
```
Ordre fixe: 🚀 roleplay d'abord, 💻 technique ensuite. Fonction `cmd_brief()`.

## RÈGLE C8 — Analogies roleplay cohérentes
- Le hublot montre l'ESPACE, pas la pièce. Toute analogie doit être visualisable dans le vaisseau.

## RÈGLE C9 — Taxonomie de Bloom par mission
- Chaque mission a un `bloom_level`: Se souvenir → Comprendre → Appliquer → Analyser → Évaluer → Créer.

## RÈGLE C10 — Vocabulaire strict
- "commandes SHELL" (pwd, ls...) vs "commandes gsh" (gsh check...). Jamais "commandes" seul.

## RÈGLE C11 — gsh goal = show_mission
- `gsh goal` réaffiche EXACTEMENT le même cadre `┏━ MISSION ━┓` que le lancement.

## RÈGLE C12 — Textes gsh cohérents
- `gsh reset` = "réinitialiser la mission en cours" (pas "le vaisseau").

## RÈGLE C13 — PNJ et vaisseaux nommés science-fiction (OBLIGATOIRE)
- Liste figée: Zyra-7 (IA de bord), Vex Korshun (commandant), Nira Okoro (communications),
  Tael Rhen (ingénieur-chef machines), Draz M'kal (sécurité), Boz Tark (mécanicien),
  Amiral Xylia Vorn, Medic-IA Andromède. Vaisseau-mère: **l'Andromède**.
- INTERDIT partout (chapitres des 3 campagnes, vaisseau-gen.sh, monde-*-gen.sh): Marie, Sophie, Jean,
  Pierre, Kowalski, Galactica, et tout nom contemporain banal.
- Jamais de nom de joueur en dur dans vaisseau-gen.sh (utiliser "le Capitaine").
- Test: `cargo test` (vaisseau_gen_sans_noms_interdits + blacklist dans toutes les missions).

## RÈGLE C14 — Campagnes (OBLIGATOIRE)
- **3 campagnes** sélectionnées au lancement (après le nom du joueur):
  - **1 = Les Pirates de l'Espace** (Station Orion-7, niveau FACILE) — monde `vaisseau/`, hôte `orion7`, user `capitaine`.
  - **2 = Le Japon Féodal** (Château Shiro, niveau MOYEN) — monde `monde_japon/`, hôte `shiro`, user `seigneur`.
  - **3 = Nexus Réseau** (NOC Neon-9, niveau DIFFICILE, orienté réseau) — monde `monde_reseau/`, hôte `nexus`, user `netrunner`.
- Chaque campagne = 100 missions en dur, 5 actes × 20, difficulté propre (Japon = checks plus exigeants:
  options exactes -la/-n/-c/-i, multi-checks, codes phase 2, permissions, config_contains).
- Bannière, prompt, hostname/whoami/id/groups/uname, analogies 🚀 roleplay : tout est thématisé par campagne.
- Test: `cargo test` vérifie les 3 campagnes (100 missions chacune, 20/acte, conformité); `run_tests.sh` tests 39/41 = campagnes 2 et 3.

---

# SECTION D — MONDE DU JEU

## RÈGLE D1 — Arborescence vaisseau (pas serveur Linux)
`/pont/ /cabines/ /soutes/ /reacteur/ /hangar/ /securite/ /communications/ /navigation/ /armes/ /salle-des-machines/ /archives/`
Générée par `vaisseau-gen.sh`. Jamais `/srv/`, `/home/etudiant/`, ni pièces incohérentes avec un vaisseau
(parking, garage, zone-industrielle...). Le `/etc` visible dans le bac à sable est celui du JEU (règle A1).

## RÈGLE D2 — Monde dédié par campagne (OBLIGATOIRE)
- Campagne 1 (espace): `~/.gameshell/vaisseau/` généré par `vaisseau-gen.sh espace`.
- Campagne 2 (Japon Féodal): `~/.gameshell/monde_japon/` généré par `vaisseau-gen.sh japon` → `monde-japon-gen.sh`
- Campagne 3 (Nexus Réseau): `~/.gameshell/monde_reseau/` généré par `vaisseau-gen.sh reseau` → `monde-reseau-gen.sh`
  (datacenter: baies, serveurs, logs, configs, captures, firewall, noc/netrunner... home = `noc/netrunner`).
  (château Shiro: donjon, pagode, jardins, remparts, greniers, scribes, quartiers/seigneur... home = `quartiers/seigneur`).
- `shell::world_dir()` = `$GAMESHELL_WORLD` (sélectionné par campagne). `gsh reset` régénère le monde
  de la campagne courante via le `vaisseau-gen.sh` embarqué.
- Les deux mondes sont embarqués dans `gameshell.sh` (générés par `make-archive.sh`).

---

# SECTION E — BUILD, TEST, DÉPÔT

## RÈGLE E1 — Build avant commit
```bash
cargo build --release 2>&1 | grep -E "^error"   # vide
bash make-archive.sh                               # régénère gameshell.sh
./tests/run_tests.sh                               # 54/54 PASS + cargo test (conformité)
```

## RÈGLE E2 — Distribution 100% offline
- Un seul fichier `gameshell.sh` auto-extractible. `./gameshell.sh` installe dans `~/.gameshell/` et lance.
- `gameshell.sh` embarque un fichier VERSION: une nouvelle version réinstalle binaire, lanceur et générateurs
  sans toucher à `state/` ni aux mondes en cours; `--install` force la réinstallation (jamais transmis au jeu).
- Les options `--campaign N`, `--locked`, `--name NOM` sont transmises au jeu (mode enseignant).
- Aucune connexion internet requise. Prérequis machine: `bubblewrap` (voir `install-debian13.sh`).

## RÈGLE E3 — Dépôt git
- Branche: **master**. Remote: `https://github.com/fpich/gameshell.git`.
- Commit avec le heredoc Mistral Vibe standard. Push sur `origin master`.

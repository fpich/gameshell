# 🚨 RÈGLES STRICTES DE CONFORMITÉ — GameShell Orion-7

Ce fichier est la **bible du projet**. Toute modification doit respecter ces règles.
Toute régression = bug critique. Numérotation fixe, ne jamais renuméroter.

---

# SECTION A — MOTEUR (fonctionnement figé, testé par tests/run_tests.sh)

## RÈGLE A1 — Isolation du monde de jeu
- Le joueur ne voit JAMAIS le vrai système de fichiers hôte.
- Tous les chemins absolus `/...` sont mappés vers `~/.gameshell/vaisseau/...`.
- `ls /` affiche les compartiments du vaisseau (pont, soutes...), jamais `/bin`, `/usr`.
- Prompt: `pseudo@orion7:chemin$` avec chemins propres (`~`, `/pont/...`).

## RÈGLE A2 — Historique des commandes par mission
- Tracké en mémoire (`mission_commands: Vec<String>`), reset à chaque mission ET au lancement.
- `gsh check` ne valide JAMAIS sur l'historique d'une session précédente.

## RÈGLE A3 — Pas de validation automatique
- Une mission ne se termine JAMAIS automatiquement. Seul `gsh check` (manuel) valide.
- Si `code_required` est défini, phase 2: le jeu demande le code après les commandes.

## RÈGLE A4 — Check matche sur la commande de BASE, n'importe où dans la ligne
- Mission requiert `ls -la`, joueur tape `ls` → passe. `sudo chown x` matche check `chown`. `cmd | grep` matche check `|`.
- Le check matche si: la cible est un mot de la ligne OU la cible est contenue dans la ligne (pour `|`).
- Jamais bloquer sur les arguments ou les préfixes (sudo...).

## RÈGLE A12 — Toute cible de fichier/commande d'une mission doit exister/être valide
- Un fichier référencé dans une mission (ex: plan_defense.sh) DOIT exister dans le vaisseau (généré par vaisseau-gen.sh).
- Un nom d'user dans un exemple (ex: chown) doit être valide → utiliser $USER, jamais un nom fictif.
- Test: jouer chaque mission manuellement au moins une fois (voir TESTS_PLAYTHROUGH.md).

## RÈGLE A5 — Parsing robuste
- Toujours `split_whitespace()`, jamais `split(' ')`.

## RÈGLE A6 — Commandes interceptées
- `pwd` → chemin propre (display_path), `hostname` → orion7, `whoami` → capitaine.
- `cd` → met à jour `current_dir` avec résolution + canonicalisation (realpath).

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
- Le prompt affiche la progression: `pseudo@orion7:~$ [N/50]`.

## RÈGLE A15 — Compétence débloquée = commande de LA mission (OBLIGATOIRE)
- La compétence débloquée (`skill`) doit être une des `required_commands` de la mission (mot de base, ou symbole | > >> présent dans la ligne).
- Jamais une commande d'une autre mission (ex: mission "lire bienvenue.txt" ne débloque PAS ls -la).
- Test: `cargo test` (conformity_tests) vérifie les 50 missions manuelles + les 500 embarquées.

## RÈGLE A16 — Identité vaisseau simulée, jamais l'hôte (OBLIGATOIRE)
- `id`, `uname`, `logname`, `whoami`, `hostname` sont interceptés et répondent avec l'identité du vaisseau
  (capitaine / orion7 / Linux orion7 6.6.7-orion...).
- Aucune commande du jeu ne doit révéler le vrai user, hostname ou groupes de la machine hôte.

## RÈGLE A9 — Pas d'indices
- `gsh hint` est SUPPRIMÉ. Ne jamais le réintroduire.

## RÈGLE A10 — Suite de tests obligatoire
- Avant chaque commit: `cargo test --release` (conformité) + `./tests/run_tests.sh` → **39/39 PASS** obligatoire.

---

# SECTION B — AFFICHAGE & VISUEL (terminal xfce 80x24 fond clair)

## RÈGLE B1 — Bannière de lancement
- Clear écran + espace étoilé compact (~8 lignes) + titre entouré d'étoiles:
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

## RÈGLE B4 — Dans la mission: nom de base de la commande seul
- `▸ pwd`, `▸ ls` — pas `ls -la`. Les exemples d'arguments sont dans gsh help.

## RÈGLE B5 — Cadre mission style "fenêtre terminal"
- `┏━ MISSION X.X — Titre ━┓` ... `┗━━┛`, largeur 64, story COMPLÈTE affichée (pas de troncature).

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

# SECTION C — CONTENU DES MISSIONS (50 missions, 10 chapitres, 5 actes)

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

## RÈGLE C5 — Noms exacts (fichiers, chemins) dans le dialogue PNJ
- Si une mission exige `incident_15jan.txt`, le PNJ doit le dire: « crée incident_15jan.txt ».
- Jamais faire deviner un nom de fichier exact.

## RÈGLE C6 — Carte ASCII vue de dessus obligatoire
- Gabarit FIXE aligné (boîtes de 14 car., 3 salles, ▶ = position, └────► = cible), lignes ≤ 60 car.:
  testé par `cargo test` (règle B9) sur les 200 missions des 2 campagnes.
- Champ `map` dans chaque mission: carte du vaisseau avec ▶ position, flèche vers la cible.
- 100% role play: indique OÙ on est, jamais QUELLE commande taper.

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
- INTERDIT partout (chapitres, generator, generated.json, vaisseau-gen.sh): Marie, Sophie, Jean,
  Pierre, Kowalski, Galactica, et tout nom contemporain banal.
- Jamais de nom de joueur en dur dans vaisseau-gen.sh (utiliser "le Capitaine").
- Test: `cargo test` (vaisseau_gen_sans_noms_interdits + blacklist dans toutes les missions).

## RÈGLE C14 — Campagnes (OBLIGATOIRE)
- **2 campagnes** sélectionnées au lancement (après le nom du joueur):
  - **1 = Les Pirates de l'Espace** (Station Orion-7, niveau FACILE) — monde `vaisseau/`, hôte `orion7`, user `capitaine`.
  - **2 = Le Japon Féodal** (Château Shiro, niveau MOYEN) — monde `monde_japon/`, hôte `shiro`, user `seigneur`.
- Chaque campagne = 100 missions en dur, 5 actes × 20, difficulté propre (Japon = checks plus exigeants:
  options exactes -la/-n/-c/-i, multi-checks, codes phase 2, permissions, config_contains).
- Bannière, prompt, hostname/whoami/id/groups/uname, analogies 🚀 roleplay : tout est thématisé par campagne.
- Test: `cargo test` vérifie les 2 campagnes (100 missions, 20/acte, conformité); `run_tests.sh` test 39 = campagne 2.

---

# SECTION D — MONDE DU JEU

## RÈGLE D1 — Arborescence vaisseau (pas serveur Linux)
`/pont/ /cabines/ /soutes/ /reacteur/ /hangar/ /securite/ /communications/ /navigation/ /armes/ /salle-des-machines/ /archives/`
Générée par `vaisseau-gen.sh`. Jamais `/srv/`, `/etc/`, `/home/etudiant/`.

## RÈGLE D2 — Monde dédié par campagne (OBLIGATOIRE)
- Campagne 1 (espace): `~/.gameshell/vaisseau/` généré par `vaisseau-gen.sh espace`.
- Campagne 2 (Japon Féodal): `~/.gameshell/monde_japon/` généré par `vaisseau-gen.sh japon` → `monde-japon-gen.sh`
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
./tests/run_tests.sh                               # 37/37 PASS + cargo test (conformité)
```

## RÈGLE E2 — Distribution 100% offline
- Un seul fichier `gameshell.sh` auto-extractible. `./gameshell.sh` installe dans `~/.gameshell/` et lance.
- Aucune connexion internet requise.

## RÈGLE E3 — Dépôt git
- Branche: **master**. Remote: `https://github.com/fpich/gameshell.git`.
- Commit avec le heredoc Mistral Vibe standard. Push sur `origin master`.

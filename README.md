# 🏴‍☠️ GameShell — Les Pirates de l'Espace

[![CI](https://github.com/fpich/gameshell/actions/workflows/ci.yml/badge.svg)](https://github.com/fpich/gameshell/actions/workflows/ci.yml)

Jeu de rôle éducatif pour apprendre les **commandes shell Linux** et le **durcissement d'un serveur Debian 13**, conçu pour les étudiants de **BTS SIO SISR**.

**100% local et offline** — aucune connexion internet requise après installation.

---

## 🚀 Installation et lancement

```bash
# Depuis le dépôt source: compiler et générer l'archive auto-extractible
cargo build --release && ./make-archive.sh
# → génère gameshell.sh (un seul fichier ~1,5 Mo)

# Sur la machine de l'étudiant:
./gameshell.sh          # installe dans ~/.gameshell/ et lance le jeu
```

### 🖥️ Installation sur Debian 13 (Trixie) — sans cloner le dépôt

Le jeu se distribue en **un seul fichier** `gameshell.sh` (auto-extractible, 100% offline).
Aucun clone git ni connexion internet requis après installation.

```bash
# 1. Copier gameshell.sh sur la machine (clé USB, scp, ENT...)
# 2. Installer les prérequis UNE FOIS (administrateur):
sudo bash install-debian13.sh
# 3. Lancer avec un compte élève normal (pas root):
chmod +x gameshell.sh
./gameshell.sh                       # ou: ./gameshell.sh --campaign 2 --locked --name Alice
```

**Prérequis**: `bubblewrap` (bac à sable), `bash`, `tar`, `coreutils`, `findutils`, `grep`,
`sed`, `gawk`, `gzip`, `zip`/`unzip`, `nano`, `less`, `man-db` — tous installés par `install-debian13.sh`.

### 🔒 Isolation de la machine (sécurité)

- Chaque commande tapée s'exécute dans un **bac à sable bubblewrap**: le monde de la campagne
  devient `/`, le reste de la machine est invisible, `/usr` est en lecture seule, **aucun réseau**.
  `cd /..`, `cat /../../etc/passwd`, `echo $USER` ou `ls -l` ne révèlent jamais la machine hôte.
- Les commandes d'administration (`sudo`, `useradd`, `passwd`, `ufw`, `iptables`, `systemctl`,
  `apt`, `kill`, `ps`, `ping`, `ssh`, `nmap`, `curl`...) sont **simulées** par le jeu sur un état
  fictif propre à chaque campagne: aucun compte, pare-feu, service, paquet ou processus réel
  n'est jamais modifié. Inutile d'installer ou de démarrer ssh, ufw ou fail2ban.
- Sans bubblewrap, le jeu démarre en **mode dégradé** (isolation partielle, signalée à l'écran)
  et refuse de démarrer en root.
- Mise à jour: un nouveau `gameshell.sh` réinstalle automatiquement le binaire sans toucher
  aux sauvegardes (`./gameshell.sh --install` pour forcer).

---

## 🎮 Comment jouer

Le joueur est le **capitaine de la station spatiale Orion-7**, attaquée par les pirates de l'espace « Les Corbeaux Noirs ».

### Commandes du jeu (préfixe `gsh`)

| Commande | Action |
|----------|--------|
| `gsh help` | Aide complète (commandes gsh + syntaxe shell + Bloom) |
| `gsh export` | Exporte progression + historique horodaté en JSON + CSV (`~/.gameshell/state/export_campagne<N>_<joueur>_*.json/csv`) pour l'enseignant |
| `gsh restart` | Recommence la campagne en cours depuis la mission 1.1 (supprime la sauvegarde) |
| `gsh goal` | Réaffiche la mission en cours (même cadre que le lancement) |
| `gsh check` | Valide la mission en cours |
| `gsh code CODE` | Donne le code demandé en phase 2 (`gsh cancel` pour annuler) |
| `gsh reset` | Réinitialise la mission en cours |
| `gsh skills` | Voir les compétences shell acquises |
| `exit` | Quitter |

### Commandes shell

Syntaxe générale: `COMMANDE`, `COMMANDE argument`, `COMMANDE -option argument`, `man COMMANDE`, `COMMANDE | COMMANDE`, `COMMANDE > fichier`.

**Compétence de départ acquise: `man`** — `man COMMANDE` affiche le manuel.

---

## 🗺️ Le vaisseau Orion-7 (monde isolé)

Tout le jeu se déroule dans `~/.gameshell/vaisseau/`. Les chemins absolus `/...` sont automatiquement mappés vers l'intérieur du vaisseau.

```
/pont/              pont de commandement
/cabines/           quartiers équipage (capitaine, zyra-7, vex...)
/securite/          alertes, logs, coffre-fort, config SSH, pare-feu
/soutes/            cargo, secrets
/reacteur/          cœur énergétique
/hangar/            corvettes (dont une pirate!)
/communications/    messages de la station Orion-7
/navigation/        coordonnées, cartes
/armes/             phasers, missiles, boucliers
/salle-des-machines/  processus, services, monitoring
/archives/
```

---

## 🕹️ Campagnes — 3 campagnes de 100 missions EN DUR chacune (5 actes × 20 missions)

Choix au lancement (après le nom du joueur):

| # | Campagne | Monde | Niveau | Hôte / rôle |
|---|----------|-------|--------|-------------|
| 1 | 🏴‍☠️ Les Pirates de l'Espace | `vaisseau/` (station Orion-7) | FACILE | orion7 / capitaine |
| 2 | 🏯 Le Japon Féodal | `monde_japon/` (château Shiro) | MOYEN | shiro / seigneur |
| 3 | 🌐 Nexus — NOC Neon-9 (réseau) | `monde_reseau/` (datacenter) | DIFFICILE | nexus / netrunner |

La campagne 2 est plus difficile: options exactes requises (`-la`, `-n`, `-c`, `-i`...), multi-checks,
codes en phase 2, permissions, config_contains. Bannière, prompt, identité et analogies roleplay thématisés.

## 📚 Missions — 100 missions EN DUR par campagne (20 chapitres × 5, soit 20 missions par acte)

**Aucune génération procédurale**: tout est écrit à la main dans `src/missions/chapters/ch1..20.rs` (100% offline).

| Acte | Chapitres | Thème | Exemples de compétences |
|------|-----------|-------|------------------------|
| I | 1–4 | Arrivée à bord, exploration, organisation, journal de bord | man (départ), pwd, ls, cd, cat, ls -la, find, du, echo >, wc, head, tail |
| II | 5–8 | Sécuriser les accès, surveiller les systèmes | chmod, chown, stat, ps, top, pgrep, kill, nice, \| |
| III | 9–12 | Enquête sur les pirates, diagnostic réseau | grep (-c/-n/-i), find, >, >>, ip, ping, ss |
| IV | 13–16 | Sauvegarde des preuves, gestion de l'équipage | tar, zip/unzip, df, du, mount, useradd, groupadd, usermod, su, sudo, groups |
| V | 17–20 | Maintenance, durcissement final, boucliers | apt, dpkg, systemctl, journalctl, fail2ban-client, iptables, ufw, auditctl, chroot, nano |

Voir `PLAN.md` pour le plan d'action complet.

Chaque mission affiche:
- **Story complète** (20+ lignes de contexte narratif obligatoire)
- **Objectif narratif** (sans nommer la commande — l'étudiant déduit)
- **Commandes shell** avec double définition: 🚀 roleplay + 💻 technique
- **Niveau Bloom en français** (Se souvenir → Créer)
- **XP**

### Validation

- Jamais automatique — le joueur tape `gsh check`
- Certaines missions demandent un **code en phase 2** (trouvé dans le vaisseau)
- Historique des commandes tracké **par mission** (reset à chaque changement)

---

## 📋 Conformité — AGENTS.md

Le fichier `AGENTS.md` contient les règles strictes de conformité, en sections:

| Section | Contenu |
|---------|---------|
| A — Moteur (A1–A25) | Isolation bubblewrap (A1), historique par mission (A2), validation manuelle (A3), matching sur commande effective (A4), parsing (A5), interceptions (A6), reset de position (A7), `gsh check` jamais révélateur (A8), pas de hint (A9), suite de tests obligatoire (A10), édition de ligne tty (A11), cibles de missions valides (A12), robustesse input (A13), UX anti-blocage (A14), compétence débloquée = commande de la mission (A15), identité vaisseau simulée (A16), checks jamais vacuels (A17), sauvegarde/reprise par campagne (A18), écran de victoire (A19), export enseignant JSON/CSV (A20), `gsh restart` (A21), historique horodaté (A22), commandes système simulées (A23), Ctrl+C (A24), tests jamais sur de vraies données (A25) |
| B — Affichage & visuel (B1–B9) | Bannière de lancement sans vaisseau (B1), `gsh help` automatique au lancement (B2), syntaxe générique dans l'aide (B3), commande affichée telle quelle (B4), cadre mission « fenêtre terminal » (B5), palette fond clair (B6), footer = `gsh help` seul (B7), Bloom en français (B8), ASCII art bien formé (B9) |
| C — Contenu (C1–C14) | 100 missions EN DUR par campagne (C1), compétences persistantes (C2), 20+ lignes de contexte (C3), objectif narratif sans nommer la commande (C4), noms exacts déduits dans le dialogue PNJ (C5), carte vue de dessus SUPPRIMÉE (C6), double définition roleplay puis technique (C7), analogies roleplay cohérentes (C8), taxonomie de Bloom (C9), vocabulaire strict (C10), `gsh goal` = `show_mission` (C11), textes gsh cohérents (C12), PNJ nommés science-fiction (C13), 3 campagnes fixes (C14) |
| D — Monde (D1–D2) | Arborescence vaisseau (D1), monde dédié par campagne (D2) |
| E — Build (E1–E3) | Build avant commit (E1), distribution 100 % offline (E2), dépôt git (E3) |

---

## 🏗️ Structure du dépôt

```
gameshell/
├── AGENTS.md              # règles de conformité (A/B/C/D/E) — bible du projet
├── README.md              # cette documentation
├── ETAT_ACTUEL.md         # état source-de-vérité (tests, versions, structure)
├── CHANGELOG.md           # historique des changements (Keep a Changelog)
├── CONTRIBUTING.md        # conventions de contribution
├── COPYRIGHT.md           # notice de propriété (pas de LICENSE)
├── TESTS_PLAYTHROUGH.md   # rapport de tests
├── PLAN.md                # plan des 3 campagnes
├── Cargo.toml / Cargo.lock # dépendances (lock commité)
├── rust-toolchain.toml    # toolchain épinglée (1.100.0)
├── rustfmt.toml / .editorconfig
├── .github/workflows/ci.yml # CI : clippy, fmt, build, tests + fresh-install + audit deps
├── .github/workflows/release.yml # release : tag v* -> build + GitHub release
├── install-debian13.sh     # installation des prérequis (apt + smoke test bwrap)
├── make-archive.sh        # génère gameshell.sh auto-extractible
├── vaisseau-gen.sh        # génère les 3 mondes (espace|japon|reseau)
├── monde-japon-gen.sh     # monde campagne 2 (château Shiro)
├── monde-reseau-gen.sh   # monde campagne 3 (NOC Nexus)
├── launcher.sh            # lance le jeu installé (transmet --campaign/--locked/--name)
├── tests/
│   ├── run_tests.sh       # suite automatisée (57 tests, dossier temporaire)
│   └── playthrough_all.sh # playthrough complet en AUTO (bac à sable, non destructif)
├── docs/
│   ├── suivi/             # relecture des 300 missions (README + campagne1/2/3)
│   ├── historique/        # RETEX.md + CORRECTIONS.md (records figés)
│   └── release.md         # procédure de release (tag -> build + GitHub release)
└── src/
    ├── main.rs            # point d'entrée + menu campagnes + bannières + aiguillage multi-appel
    ├── sys.rs             # FFI libc (termios, signaux, largeur d'affichage)
    ├── game/mod.rs        # moteur de jeu (boucle, checks, phase 2, export)
    ├── game/parse.rs      # analyse des lignes shell pour la validation (A4)
    ├── game/bench.rs      # bancs AUTO / EDGE / TORTURE (commandes requises seules)
    ├── game/lineedit.rs   # éditeur de ligne (termios, Tab, historique)
    ├── game/ui.rs         # cadres alignés (largeur d'affichage)
    ├── game/brief.rs      # définitions roleplay + technique des commandes
    ├── shell/mod.rs       # bac à sable bubblewrap + sélection monde + sauvegardes
    ├── sim/               # commandes système simulées (comptes, processus, services, pare-feu, réseau...)
    └── missions/          # 300 missions en dur (3 campagnes x 100)
        ├── types.rs       # Mission, Skill, MissionCheck
        ├── chapters/ch1..20.rs   # campagne 1 (espace)
        ├── japon/ch1..20.rs      # campagne 2 (Japon féodal)
        └── reseau/ch1..20.rs     # campagne 3 (Nexus réseau)
```

## 💾 Progression & historique (RETEX par partie)
- Sauvegarde auto par campagne ET par joueur (`state/save_campagne<N>_<joueur>.json`) + reprise au lancement.
- **Historique horodaté de chaque action** dans `state/history_campagne<N>_<joueur>.log` (session, commandes, checks, missions, codes) — RETEX complet de chaque partie.
- `gsh restart` = recommencer la campagne à zéro (ou `rm ~/.gameshell/state/save_campagne<N>_<joueur>.json`).
- Tutoriel affiché au premier lancement.

## 👨‍🏫 Pour les enseignants

- Distribution: un seul fichier `gameshell.sh` (clé USB, ENT...)
- Suivi de classe: `gsh export` génère JSON + CSV par mission (id, titre, statut, XP, tentatives) + historique horodaté (`~/.gameshell/state/`)
- Mode enseignant: lancer avec `--campaign <1|2|3>` pour imposer une campagne (et `--locked` pour verrouiller l'ordre des missions)
- Missions modifiables dans `src/missions/{chapters,japon,reseau}/ch*.rs`
- Rebuild: `cargo build --release && ./make-archive.sh`
- Tests: `cargo test --release` + `./tests/run_tests.sh` (57 tests, dans un dossier temporaire: vos données ne sont jamais touchées)

## 🔒 Licence

**Propriétaire — tous droits réservés** (`COPYRIGHT.md`). Pas de fichier `LICENSE` :
aucune réutilisation publique n'est autorisée en l'état. Distribué en `gameshell.sh`
(fichier auto-extractible, 100 % offline) pour l'enseignement.

Pour l'enseignant : `ETAT_ACTUEL.md` (état du dépôt), `docs/suivi/` (relecture mission par mission),
`docs/historique/RETEX.md` (méthodologie et bug history).

## 📦 Dépôt git

```bash
git remote add origin https://github.com/fpich/gameshell.git
git push -u origin master
```

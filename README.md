# 🏴‍☠️ GameShell — Les Pirates de l'Espace

Jeu de rôle éducatif pour apprendre les **commandes shell Linux** et le **durcissement d'un serveur Debian 13**, conçu pour les étudiants de **BTS SIO SISR**.

**100% local et offline** — aucune connexion internet requise après installation.

---

## 🚀 Installation et lancement

```bash
# Depuis le dépôt source: compiler et générer l'archive auto-extractible
cargo build --release && ./make-archive.sh
# → génère gameshell.sh (un seul fichier ~540K)

# Sur la machine de l'étudiant:
./gameshell.sh          # installe dans ~/.gameshell/ et lance le jeu
```

Alternative: `./install.sh` installe dans `~/.gameshell/` ou `/opt/gameshell/` (sudo).

---

## 🎮 Comment jouer

Le joueur est le **capitaine de la station spatiale Orion-7**, attaquée par les pirates de l'espace « Les Corbeaux Noirs ».

### Commandes du jeu (préfixe `gsh`)

| Commande | Action |
|----------|--------|
| `gsh help` | Aide complète (commandes gsh + syntaxe shell + Bloom) |
| `gsh goal` | Réaffiche la mission en cours (même cadre que le lancement) |
| `gsh check` | Valide la mission en cours (phase 2: code si requis) |
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
/cabines/           quartiers équipage (capitaine, marie, jean...)
/securite/          alertes, logs, coffre-fort, config SSH, pare-feu
/soutes/            cargo, secrets
/reacteur/          cœur énergétique
/hangar/            corvettes (dont une pirate!)
/communications/    messages du Galactica
/navigation/        coordonnées, cartes
/armes/             phasers, missiles, boucliers
/salle-des-machines/  processus, services, monitoring
/archives/
```

---

## 🕹️ Campagnes — 2 campagnes de 100 missions EN DUR chacune (5 actes × 20 missions)

Choix au lancement (après le nom du joueur):

| # | Campagne | Monde | Niveau | Hôte / rôle |
|---|----------|-------|--------|-------------|
| 1 | 🏴‍☠️ Les Pirates de l'Espace | `vaisseau/` (station Orion-7) | FACILE | orion7 / capitaine |
| 2 | 🏯 Le Japon Féodal | `monde_japon/` (château Shiro) | MOYEN | shiro / seigneur |

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

Le fichier `AGENTS.md` contient **14 règles strictes** anti-régression:

| Règle | Contenu |
|-------|---------|
| 1 | Isolation du monde (jamais le vrai système) |
| 2 | Historique par mission, reset à chaque changement |
| 3 | Pas de validation automatique |
| 4 | Affichage obligatoire (gsh help auto, mission complète) |
| 4b | Double définition: roleplay PUIS technique |
| 4c | Minimum 20 lignes de contexte par mission |
| 4d | Cohérence des analogies roleplay |
| 4e | Textes gsh cohérents |
| 4f | Vocabulaire: "commandes SHELL" vs "commandes gsh" |
| 4g | ASCII art bien formé |
| 4h | Carte ASCII obligatoire par mission |
| 4i | Palette fond clair (texte noir, couleurs vives) |
| 4j | Ne pas donner la réponse (objectif narratif) |
| 4k | gsh goal = show_mission, man = compétence départ |
| 4l | Bloom en français, footer 'gsh help' seul |
| 4m | Check matche sur commande de base |

---

## 🏗️ Structure du dépôt

```
gameshell/
├── AGENTS.md              # 14 règles de conformité
├── README.md              # cette documentation
├── Cargo.toml
├── make-archive.sh        # génère gameshell.sh auto-extractible
├── vaisseau-gen.sh        # génère le monde du vaisseau
├── launcher.sh            # lance le jeu dans le vaisseau isolé
├── install.sh             # installation système
└── src/
    ├── main.rs            # point d'entrée + banner espace étoilé
    ├── game/mod.rs        # moteur de jeu (boucle, affichage, checks)
    ├── shell/mod.rs       # exécution commandes + isolation chemins
    └── missions/          # 100 missions en dur, 20 chapitres
        ├── types.rs       # Mission, Skill, MissionCheck
        └── chapters/ch1..20.rs
```

## 👨‍🏫 Pour les enseignants

- Distribution: un seul fichier `gameshell.sh` (clé USB, ENT...)
- Historique des commandes dans `~/.gameshell/state/commands.log` pour évaluation
- Missions modifiables dans `src/missions/chapters/ch*.rs`
- Rebuild: `cargo build --release && ./make-archive.sh`

## 📦 Dépôt git

```bash
git remote add origin https://github.com/fpich/gameshell.git
git push -u origin master
```

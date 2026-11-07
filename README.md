# 🚀 GameShell - Les Pirates de l'Espace

Jeu de rôle éducatif pour apprendre les commandes shell et le durcissement d'un serveur **Debian 13** — conçu pour les étudiants de **BTS SIO SISR**.

**100% local et offline** — aucune connexion internet requise après installation.

## 📦 Distribution (1 seul fichier)

```bash
# Compiler et créer l'archive auto-extractible
./make-archive.sh
# → génère gameshell.sh (un seul fichier ~500Ko)

# Distribuer gameshell.sh aux étudiants (USB, clé, ENT...)
# Sur la machine de l'étudiant:
./gameshell.sh          # installe dans ~/.gameshell/ et lance le jeu
```

## 🚀 Installation alternative

```bash
# Depuis le dépôt source
./install.sh            # installe dans ~/.gameshell/
sudo ./install.sh       # installe dans /opt/gameshell/ + lien dans /usr/local/bin
```

## 🎮 Lancer le jeu

```bash
~/.gameshell/gameshell          # après install.sh
~/.local/bin/gameshell          # si lien PATH créé
gameshell                       # si installé en root
./gameshell.sh                  # directement depuis l'archive
```

## 🎯 Commandes du jeu (style gameshell.sh)

| Commande | Description |
|----------|-------------|
| `gsh goal` | Voir l'objectif narratif de la mission en cours + indices |
| `gsh hint` | Indice progressif (en цепляет plusieurs fois) |
| `gsh check` | Vérifier si la mission est complétée |
| `gsh reset` | Réinitialiser l'environnement |
| `gsh help` | Aide complète |
| `gsh skills` | Voir les compétences acquises |
| `exit` | Quitter le jeu |

**Le prompt est propre** (pas de vrais chemins système) :
```
Fabien@orion7:~$ pwd
~
Fabien@orion7:~$ cd /srv/station-spatiale/logs
Fabien@orion7:/srv/station-spatiale/logs$ ls
auth.log  syslog  fail2ban.log
```

**Les missions n'affichent PAS les commandes à taper** — seulement l'objectif narratif. Le joueur doit réfléchir (taxonomie de Bloom).

## 🌍 Dossier isolé

Tout le jeu est contenu dans `~/.gameshell/` :

```
~/.gameshell/
├── bin/gameshell              # binaire du jeu
├── world/                     # 🌍 MONDE ISOLÉ (station Orion-7)
│   ├── home/etudiant/         # home du joueur
│   │   ├── station -> ../../srv/station-spatiale  (lien symbolique)
│   │   ├── documents/         # fichiers de mission
│   │   └── projets/
│   ├── srv/station-spatiale/  # la station spatiale
│   │   ├── logs/              # syslog, auth.log, fail2ban.log
│   │   ├── config/            # sshd_config, ufw_rules.txt
│   │   └── scripts/           # backup.sh, monitor.sh
│   ├── var/log/
│   └── etc/                   # hostname, motd, ssh/sshd_config
├── state/                     # sauvegarde + historique des commandes
│   ├── player.json
│   └── commands.log           # pour évaluation par l'enseignant
└── gameshell                  # launcher
```

## 📚 Les 10 chapitres (50 missions)

| Chapitre | Thème | Compétences débloquées |
|----------|-------|----------------------|
| 1 | Premiers Pas dans l'Espace | `pwd`, `ls`, `cat`, `cd`, `hostname` |
| 2 | Organisation des Données | `touch`, `mkdir`, `cp`, `mv`, `rm` |
| 3 | Permissions et Sécurité | `chmod`, `chown`, `ls -l`, `stat`, `getfacl` |
| 4 | Surveillance des Systèmes | `ps`, `top`, `systemctl`, `kill`, `nice` |
| 5 | Recherche et Filtrage | `grep`, `find`, `\|`, `>`, `>>` |
| 6 | Réseau et Communications | `ip`, `ping`, `ufw`, `ssh`, `ss/netstat` |
| 7 | Archivage et Stockage | `tar`, `zip`, `df`, `du`, `mount` |
| 8 | Gestion des Utilisateurs | `useradd`, `groupadd`, `passwd`, `su`, `sudo` |
| 9 | Maintenance du Système | `apt`, `dpkg`, `systemctl restart`, `journalctl`, `apt update` |
| 10 | Durcissement du Serveur | `fail2ban`, `ssh_hardening`, `iptables`, `auditctl`, `chroot` |

## 🧠 Taxonomie de Bloom

Chaque mission a un niveau Bloom : **Remember → Understand → Apply → Analyze → Evaluate/Create**

## 🎖️ Système de compétences

Chaque mission débloque une commande qui **reste acquise** pour toutes les missions suivantes.

## 👨‍🏫 Pour les enseignants

- Distribution offline via `gameshell.sh` (1 fichier)
- Historique des commandes dans `~/.gameshell/state/commands.log` pour évaluation
- Missions modifiables dans `src/missions/chapters/ch*.rs`
- Recompilation : `cargo build --release && ./make-archive.sh`

## 📁 Structure du projet source

```
gameshell/
├── gameshell.sh          # archive auto-extractible (générée)
├── make-archive.sh       # génère gameshell.sh
├── install.sh            # installation dans dossier isolé
├── world/                # monde du jeu (arborescence station Orion-7)
├── src/
│   ├── main.rs           # point d'entrée
│   ├── game/mod.rs       # boucle de jeu + prompt Linux
│   ├── shell/mod.rs      # exécution des commandes + état
│   └── missions/         # 50 missions en 10 chapitres
└── Cargo.toml
```

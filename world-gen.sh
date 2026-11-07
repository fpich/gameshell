#!/bin/bash
# Génère le monde complet du jeu - arborescence profonde et réaliste
# Style: serveur Debian 13 d'une station spatiale avec données de mission

set -e
WORLD="${1:-world}"
rm -rf "$WORLD"

# === RACINE TYPE LINUX ===
mkdir -p "$WORLD"/{bin,boot,dev,etc,home,lib,media,mnt,opt,root,run,sbin,srv,tmp,usr,var,proc,sys}
mkdir -p "$WORLD"/usr/{bin,lib,local,share,src}
mkdir -p "$WORLD"/var/{log,backups,cache,lib,spool,www,mail}
mkdir -p "$WORLD"/etc/{ssh,systemd,apt,cron.d,network,default}
mkdir -p "$WORLD"/opt/station

# === HOME DU JOUEUR ===
H="$WORLD/home/etudiant"
mkdir -p "$H"/{documents,projets,telechargements,archives,.config,.local,musique,images}

# === LA STATION SPATIALE (données de mission) ===
S="$WORLD/srv/station-spatiale"
mkdir -p "$S"/{logs,config,backups,scripts,secrets,rapports,missions,personnel,cargo,communications}

# --- LOGS riches ---
cat > "$S/logs/syslog" << 'EOF'
Jan 15 06:00:01 orion7 systemd[1]: Started Session 41 of user etudiant.
Jan 15 06:15:22 orion7 systemd[1]: Started Daily apt upgrade.
Jan 15 07:30:00 orion7 CRON[892]: (root) CMD (/opt/station/scripts/backup_auto.sh)
Jan 15 08:00:00 orion7 systemd[1]: Started OpenBSD Secure Shell server.
Jan 15 08:23:11 orion7 systemd[1]: Started Session 42 of user etudiant.
Jan 15 08:31:15 orion7 sshd[1589]: Failed password for invalid user admin from 10.0.0.66 port 44123 ssh2
Jan 15 08:31:22 orion7 sshd[1592]: Failed password for invalid user root from 10.0.0.66 port 44124 ssh2
Jan 15 08:31:45 orion7 sshd[1601]: Failed password for invalid user test from 10.0.0.66 port 44125 ssh2
Jan 15 08:32:01 orion7 sshd[1610]: Invalid user oracle from 10.0.0.66 port 44130
Jan 15 08:32:15 orion7 sshd[1612]: Invalid user postgres from 10.0.0.66 port 44131
Jan 15 08:40:55 orion7 kernel: [UFW BLOCK] IN=eth0 SRC=10.0.0.66 DST=192.168.1.10 PROTO=TCP SPT=44123 DPT=22 SYN
Jan 15 09:15:23 orion7 sshd[1892]: Accepted password for etudiant from 192.168.1.100 port 55432 ssh2
Jan 15 09:30:11 orion7 sudo: etudiant : TTY=pts/0 ; PWD=/home/etudiant ; USER=root ; COMMAND=/usr/bin/apt update
Jan 15 10:15:45 orion7 sshd[2103]: Failed password for invalid user postgres from 10.0.0.99 port 33221 ssh2
Jan 15 10:16:01 orion7 sshd[2110]: Invalid user admin from 10.0.0.99 port 33222
Jan 15 10:16:15 orion7 sshd[2115]: Failed password for root from 10.0.0.99 port 33223 ssh2
Jan 15 10:20:12 orion7 kernel: [UFW BLOCK] IN=eth0 SRC=10.0.0.99 DST=192.168.1.10 PROTO=TCP SPT=33221 DPT=5432 SYN
Jan 15 11:00:00 orion7 systemd[1]: Started Session 43 of user etudiant.
Jan 15 11:45:33 orion7 sshd[2450]: Failed password for invalid user git from 10.0.0.66 port 55001 ssh2
Jan 15 12:00:00 orion7 CRON[2500]: (root) CMD (/opt/station/scripts/scan_securite.sh)
EOF

cat > "$S/logs/auth.log" << 'EOF'
Jan 15 08:31:15 orion7 sshd[1589]: Failed password for invalid user admin from 10.0.0.66 port 44123 ssh2
Jan 15 08:31:22 orion7 sshd[1592]: Failed password for invalid user root from 10.0.0.66 port 44124 ssh2
Jan 15 08:31:45 orion7 sshd[1601]: Failed password for invalid user test from 10.0.0.66 port 44125 ssh2
Jan 15 08:32:01 orion7 sshd[1610]: Invalid user oracle from 10.0.0.66 port 44130
Jan 15 08:32:15 orion7 sshd[1612]: Invalid user postgres from 10.0.0.66 port 44131
Jan 15 09:15:23 orion7 sshd[1892]: Accepted password for etudiant from 192.168.1.100 port 55432 ssh2
Jan 15 10:15:45 orion7 sshd[2103]: Failed password for invalid user postgres from 10.0.0.99 port 33221 ssh2
Jan 15 10:16:01 orion7 sshd[2110]: Invalid user admin from 10.0.0.99 port 33222
Jan 15 10:16:15 orion7 sshd[2115]: Failed password for root from 10.0.0.99 port 33223 ssh2
Jan 15 11:45:33 orion7 sshd[2450]: Failed password for invalid user git from 10.0.0.66 port 55001 ssh2
EOF

cat > "$S/logs/fail2ban.log" << 'EOF'
2024-01-15 08:35:00,123 fail2ban.actions [812]: NOTICE [sshd] Ban 10.0.0.66
2024-01-15 10:20:00,456 fail2ban.actions [812]: NOTICE [sshd] Ban 10.0.0.99
2024-01-15 11:50:00,789 fail2ban.actions [812]: NOTICE [sshd] Unban 10.0.0.66
EOF

cat > "$S/logs/access.log" << 'EOF'
192.168.1.100 - - [15/Jan/2024:09:15:23] "GET /admin HTTP/1.1" 200
10.0.0.66 - - [15/Jan/2024:08:31:15] "POST /login HTTP/1.1" 401
10.0.0.66 - - [15/Jan/2024:08:32:01] "GET /.env HTTP/1.1" 404
10.0.0.99 - - [15/Jan/2024:10:15:45] "GET /wp-admin HTTP/1.1" 404
EOF

# --- CONFIGS ---
cat > "$S/config/sshd_config" << 'EOF'
# /etc/ssh/sshd_config - Station Orion-7
Port 22
PermitRootLogin yes
PasswordAuthentication yes
PermitEmptyPasswords no
MaxAuthTries 6
LoginGraceTime 120
ClientAliveInterval 300
ClientAliveCountMax 2
UsePAM yes
X11Forwarding no
PrintMotd no
AcceptEnv LANG LC_*
Subsystem sftp /usr/lib/openssh/sftp-server
EOF

cat > "$S/config/ufw_status.txt" << 'EOF'
Status: inactive
EOF

cat > "$S/config/reseau.txt" << 'EOF'
Interface eth0:
  IP: 192.168.1.10/24
  Gateway: 192.168.1.1
  DNS: 8.8.8.8, 1.1.1.1
  MAC: 00:11:22:33:44:55

Connexions ouvertes:
  22/tcp (SSH) - 0.0.0.0
  80/tcp (HTTP) - 0.0.0.0
  443/tcp (HTTPS) - 0.0.0.0
EOF

# --- SCRIPTS ---
cat > "$S/scripts/backup_auto.sh" << 'EOF'
#!/bin/bash
# Backup automatique toutes les heures
tar czf /srv/station-spatiale/backups/auto_$(date +%H%M).tar.gz /srv/station-spatiale/logs /home/etudiant/documents
EOF

cat > "$S/scripts/scan_securite.sh" << 'EOF'
#!/bin/bash
# Scan de sécurité quotidien
echo "=== Scan sécurité $(date) ==="
grep -c "Failed password" /srv/station-spatiale/logs/auth.log
echo "IPs suspectes:"
grep "Failed password" /srv/station-spatiale/logs/auth.log | awk '{print $11}' | sort -u
EOF

cat > "$S/scripts/monitor.sh" << 'EOF'
#!/bin/bash
echo "=== Orion-7 System Monitor ==="
echo "--- CPU/Mem ---"
top -bn1 | head -8
echo "--- Disques ---"
df -h / /home
echo "--- Réseau ---"
ss -tlnp 2>/dev/null | head -10
EOF

# --- DONNÉES PERSONNEL ---
cat > "$S/personnel/equipage.txt" << 'EOF'
ÉQUIPAGE ORION-7:
================
Capitaine: Fabien (vous) - admin système
Ingénieur: Marie Dupont - marie@orion7.local
Sécurité: Jean Martin - jean@orion7.local
Communications: Sophie Leroy - sophie@orion7.local
Mécanicien: Pierre Durand - pierre@orion7.local

Code d'accès niveaux:
  Niveau 1 (Equipage): ALPHA-7742
  Niveau 2 (Officiers): OMEGA-3319
  Niveau 3 (Capitaine): SIGMA-9981
  Niveau 4 (Commandement): DELTA-5567
EOF

cat > "$S/secrets/coffre_fort.txt" << 'EOF'
⚠️  ACCÈS RESTREINT - NIVEAU 4 REQUIS ⚠️

Coordonnées de la flotte Galactica: Sector 7G, Quadrant X-42
Code de lancement missiles: 8842-OMEGA
Position du trou de ver: [CLASSIFIÉ]
EOF

cat > "$S/alertes.txt" << 'EOF'
🚨 ALERTES ACTIVES - 15 JANVIER 2024 🚨

1. ATTAQUE EN COURS: 10.0.0.66 tente de forcer SSH (5+ tentatives)
2. ATTAQUE SECONDAIRE: 10.0.0.99 scanne les ports
3. UFW est INACTIF - aucune protection!
4. fail2ban installé mais PAS configuré
5. PermitRootLogin=yes - DANGER!
6. PasswordAuthentication=yes - les pirates peuvent essayer des MDP
EOF

cat > "$S/missions/ordre_du_capitaine.txt" << 'EOF'
ORDRE DE MISSION - CAPITAINE FABIEN

Les pirates de l'espace « Les Corbeaux Noirs » attaquent Orion-7.

PHASE 1: Reconnaissance
  → Explorez le système, lisez les logs, identifiez les menaces

PHASE 2: Organisation
  → Créez une structure de fichiers pour vos rapports

PHASE 3: Permissions
  → Protégez les fichiers sensibles

PHASE 4: Surveillance
  → Surveillez les processus et services

PHASE 5: Analyse
  → Recherchez les traces des pirates dans les logs

PHASE 6: Réseau
  → Identifiez les vecteurs d'attaque réseau

PHASE 7: Archivage
  → Sauvegardez les preuves avant contre-attaque

PHASE 8: Comptes
  → Gérez les utilisateurs et groupes

PHASE 9: Maintenance
  → Mettez à jour le système

PHASE 10: DURCISSEMENT
  → Sécurisez définitivement la station!
EOF

cat > "$S/cargo/manifest.txt" << 'EOF'
MANIFESTE CARGO ORION-7:
- 50 tonnes de carburant
- 200 unités de nourriture
- 10 caisses de pièces détachées (CAISSE-A à CAISSE-J)
- 3 containers de données classifiées
- 1 artefact alien (ne pas toucher!)
EOF

cat > "$S/communications/dernier_message.txt" << 'EOF'
De: Commandement Galactica
À: Capitaine Orion-7

« Capitaine, les capteurs détectent une flotte pirate
approchant du secteur 7G. Préparez la station à la défense.
Vos logs montrent déjà des tentatives d'intrusion.
Agissez vite. »

— Amiral Kowalski
EOF

# --- HOME DU JOUEUR ---
cat > "$H/documents/bienvenue.txt" << 'EOF'
Bienvenue à bord de la station Orion-7, Capitaine!

Votre bureau contient:
- documents/ : vos rapports et notes
- projets/ : vos projets en cours
- telechargements/ : fichiers téléchargés

La station est accessible via le lien: ~/station
ou directement: /srv/station-spatiale/

Tapez 'gsh help' pour voir les commandes du jeu.
EOF

cat > "$H/documents/notes_personnelles.txt" << 'EOF'
Notes du Capitaine - 15/01/2024

- Reçu ordre de défense de la station
- Des tentatives SSH bizarres dans les logs
- UFW inactif, c'est inquiétant
- Le coffre-fort (secrets/) contient des codes
- Ne pas oublier de changer les permissions!
EOF

cat > "$H/documents/rapport_mission.txt" << 'EOF'
RAPPORT INITIAL - ORION-7
=========================
Statut: ALERTE
Menace: Pirates de l'espace « Les Corbeaux Noirs »
Vecteur: SSH (port 22) - attaques par force brute
IPs hostile: 10.0.0.66, 10.0.0.99

Actions requises: voir /srv/station-spatiale/missions/ordre_du_capitaine.txt
EOF

cat > "$H/documents/secret.txt" << 'EOF'
CODE PERSONNEL CAPITAINE: FALCON-9
Ne pas divulguer!
EOF

cat > "$H/documents/public.txt" << 'EOF'
Informations publiques de la station Orion-7.
Tout le monde peut lire ce fichier.
EOF

cat > "$H/projets/README.md" << 'EOF'
# Projets en cours - Capitaine

## 🔴 URGENT:
- [ ] Analyser les logs de sécurité
- [ ] Sécuriser SSH
- [ ] Activer et configurer UFW

## 🟡 À faire:
- [ ] Configurer fail2ban
- [ ] Créer des comptes utilisateurs pour l'équipage
- [ ] Archiver les preuves

## 🟢 Planifié:
- [ ] Audit complet du système
- [ ] Chroot pour les services exposés
EOF

cat > "$H/projets/plan_defense.sh" << 'EOF'
#!/bin/bash
# Plan de défense de la station Orion-7
echo "Activation du plan de défense..."
echo "1. Vérification des logs"
echo "2. Activation UFW"
echo "3. Configuration fail2ban"
echo "4. Durcissement SSH"
echo "Plan prêt!"
EOF

# Fichiers cachés
cat > "$H/.secret_bash_history" << 'EOF'
sudo apt update
cat /srv/station-spatiale/secrets/coffre_fort.txt
ssh root@10.0.0.66
rm -rf /tmp/temp_*
EOF

cat > "$H/.notes_cachees.txt" << 'EOF'
Ne pas oublier: changer le mot de passe de 'matelot'
Le code du wifi est: Orion7-Secure-2024
EOF

# === VAR/LOG ===
cat > "$WORLD/var/log/messages" << 'EOF'
Jan 15 06:00:01 orion7 kernel: Linux version 6.1.0-18-amd64
Jan 15 06:00:02 orion7 systemd[1]: Mounted /boot
Jan 15 06:00:05 orion7 sshd[812]: Server listening on 0.0.0.0 port 22
EOF

# === ETC ===
cat > "$WORLD/etc/hostname" << 'EOF'
orion7
EOF

cat > "$WORLD/etc/motd" << 'EOF'
╔════════════════════════════════════╗
║   🚀 STATION SPATIALE ORION-7      ║
║   Debian 13 - Accès restreint      ║
║   Toute activité est surveillée.   ║
╚════════════════════════════════════╝
EOF

cp "$S/config/sshd_config" "$WORLD/etc/ssh/sshd_config"

cat > "$WORLD/etc/hosts" << 'EOF'
127.0.0.1   localhost orion7
192.168.1.10 orion7.station.local orion7
192.168.1.1  gateway.local
10.0.0.66    ?.pirate.local
10.0.0.99    ?.pirate.local
EOF

# === PERMISSIONS ===
chmod 600 "$H/documents/secret.txt" "$S/secrets/coffre_fort.txt" "$H/.secret_bash_history"
chmod 644 "$H/documents/"*.txt "$S/"*.txt "$S/logs/"* "$S/config/"* "$S/personnel/"* "$S/missions/"* "$S/cargo/"* "$S/communications/"*
chmod 700 "$H/projets/plan_defense.sh" "$S/scripts/"*.sh
chmod 755 "$WORLD" "$S" "$S/logs" "$S/config" "$S/scripts" "$H"
find "$WORLD" -type d -exec chmod 755 {} \; 2>/dev/null
chmod 600 "$H/documents/secret.txt" "$S/secrets/coffre_fort.txt" "$H/.secret_bash_history"
chmod 700 "$H/projets/plan_defense.sh" "$S/scripts/"*.sh

echo "✅ Monde généré: $(find "$WORLD" -type f | wc -l) fichiers, $(find "$WORLD" -type d | wc -l) répertoires"

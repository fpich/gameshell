#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  GÉNÉRATEUR DU MONDE "NEXUS" — NOC Neon-9 (campagne 3)   ║
# ║  Niveau DIFFICILE, orienté RÉSEAU — environnement dédié  ║
# ╚══════════════════════════════════════════════════════════╝
# Usage: ./monde-reseau-gen.sh [dossier_destination]

set -e
V="${1:-$HOME/.gameshell/monde_reseau}"
rm -rf "$V"

# ══════════ NIVEAU 0 — LE DATACENTER NEXUS ══════════
mkdir -p "$V"/{baies,serveurs,logs,configs,captures,noc,firewall,utilisateurs,archives,monitoring,tmp,var/tmp}

# ══════════ NOC (home ~ = /noc/netrunner) ══════════
N="$V/noc"
mkdir -p "$N"/{netrunner,netops,securite,invite}
cat > "$N/netrunner/bienvenue.txt" << 'EOF'
BIENVENUE AU NOC NEXUS-9, NETRUNNER!

Vous êtes l'administrateur réseau du datacenter Neon-9.
Depuis votre poste, vous surveillez le trafic:
- Consulter les configs et les logs
- Scanner les ports, bloquer les intrusions
- Durcir les serveurs du cluster

Le NOC vous écoute. Tapez 'gsh help' pour l'aide.

Code d'accueil du NOC: NEXUS-ACCUEIL
EOF
cat > "$N/netrunner/notes.txt" << 'EOF'
NOTES DU NETRUNNER:
- Surveiller les scans sur le port 22 (auth.log)
- Le pare-feu doit bloquer 10.0.0.66
- Code du coffre-fort réseau: voir /firewall/codes.txt
EOF
cat > "$N/netrunner/plan_incident.sh" << 'SHEOF'
#!/bin/bash
# Plan de réponse aux incidents réseau
echo "Isolez la cible! Coupez le trafic! Alertez la sécurité!"
SHEOF
chmod 644 "$N/netrunner/plan_incident.sh"
cat > "$N/netrunner/.secret" << 'EOF'
Token secret du netrunner: NEON-77-KAMI
EOF
cat > "$N/netops/rosa.txt" << 'EOF'
Rosa (netops): le lien vers le FAI est instable depuis ce matin.
EOF
cat > "$N/securite/karl.txt" << 'EOF'
Karl (sécurité): 3 IPs suspectes à bannir aujourd'hui.
EOF
cat > "$N/invite/stagiaire.txt" << 'EOF'
Stagiaire: où sont les logs de pare-feu? On m'a dit /logs/firewall.log.
EOF

# ══════════ CONFIGS ══════════
C="$V/configs"
mkdir -p "$C"/{sshd,ufw,iptables,services}
cat > "$C/sshd/sshd_config" << 'EOF'
# Configuration SSH - Nexus-9
Port 22
PermitRootLogin yes
PasswordAuthentication yes
MaxAuthTries 6
UsePAM yes
# Code de vérification: NEXUS-SSH-22
EOF
cat > "$C/ufw/regles.txt" << 'EOF'
RÈGLES PARE-FEU NEXUS:
ufw default deny incoming
ufw allow 22/tcp
ufw allow 443/tcp
ufw deny from 10.0.0.66
EOF
cat > "$C/iptables/rules.v4" << 'EOF'
*filter
:INPUT DROP [0:0]
:FORWARD DROP [0:0]
:OUTPUT ACCEPT [0:0]
-A INPUT -i lo -j ACCEPT
-A INPUT -p tcp --dport 22 -j ACCEPT
-A INPUT -p tcp --dport 443 -j ACCEPT
COMMIT
EOF
cat > "$C/services/services_critiques.txt" << 'EOF'
SERVICES CRITIQUES DU CLUSTER:
ssh, ufw, fail2ban, systemd-journald, nginx
EOF

# ══════════ LOGS ══════════
L="$V/logs"
cat > "$L/auth.log" << 'EOF'
Jan 15 08:31:15 nexus sshd[1589]: Failed password for invalid user admin from 10.0.0.66 port 44123 ssh2
Jan 15 08:31:22 nexus sshd[1592]: Failed password for invalid user root from 10.0.0.66 port 44124 ssh2
Jan 15 08:32:01 nexus sshd[1610]: Invalid user oracle from 10.0.0.66 port 44130
Jan 15 09:15:23 nexus sshd[1892]: Accepted password for netrunner from 192.168.1.100 port 55432 ssh2
Jan 15 10:15:45 nexus sshd[2103]: Failed password for invalid user postgres from 10.0.0.99 port 33221 ssh2
Jan 15 10:16:15 nexus sshd[2115]: Failed password for root from 10.0.0.99 port 33223 ssh2
Jan 15 11:45:33 nexus sshd[2450]: Failed password for invalid user git from 10.0.0.66 port 55001 ssh2
EOF
cat > "$L/firewall.log" << 'EOF'
Jan 15 08:31:15 nexus kernel: [UFW BLOCK] IN=eth0 SRC=10.0.0.66 DST=10.0.0.1 PROTO=TCP DPT=22
Jan 15 08:35:02 nexus kernel: [UFW BLOCK] IN=eth0 SRC=10.0.0.99 DST=10.0.0.1 PROTO=TCP DPT=22
Jan 15 09:00:00 nexus kernel: [UFW ALLOW] IN=eth0 SRC=192.168.1.100 DST=10.0.0.1 PROTO=TCP DPT=443
Jan 15 10:16:15 nexus kernel: [UFW BLOCK] IN=eth0 SRC=10.0.0.66 DST=10.0.0.1 PROTO=TCP DPT=22
Jan 15 11:45:33 nexus kernel: [UFW BLOCK] IN=eth0 SRC=10.0.0.66 DST=10.0.0.1 PROTO=TCP DPT=80
EOF
cat > "$L/ids.log" << 'EOF'
Jan 15 08:31:15 nexus snort[900]: ALERT scan SYN 10.0.0.66 -> 10.0.0.1 ports 1-1024
Jan 15 09:40:00 nexus snort[900]: ALERT bruteforce SSH 10.0.0.99 -> 10.0.0.1:22
Jan 15 10:00:00 nexus snort[900]: INFO heartbeat OK
Jan 15 10:16:15 nexus snort[900]: ALERT scan UDP 10.0.0.66 -> 10.0.0.1 ports 53,161
Jan 15 12:00:00 nexus snort[900]: INFO heartbeat OK
EOF
cat > "$L/fail2ban.log" << 'EOF'
Jan 15 08:35:00 nexus fail2ban[800]: BAN 10.0.0.66 (sshd)
Jan 15 09:40:30 nexus fail2ban[800]: BAN 10.0.0.99 (sshd)
Jan 15 10:20:00 nexus fail2ban[800]: UNBAN 10.0.0.66 (timeout)
EOF
cat > "$L/syslog" << 'EOF'
Jan 15 06:00:01 nexus CRON[900]: backup des configs
Jan 15 08:31:15 nexus sshd[1589]: tentative échouée (admin)
Jan 15 10:00:00 nexus monitoring: heartbeat cluster OK
Jan 15 12:30:00 nexus backup: 2.1 Go archivés
EOF

# ══════════ CAPTURES RÉSEAU ══════════
P="$V/captures"
cat > "$P/scan_10.0.0.66.txt" << 'EOF'
SCAN NMAP - 10.0.0.66 (hostile):
22/tcp   open  ssh
80/tcp   open  http
443/tcp  open  https
3306/tcp open  mysql  <- DANGER
EOF
cat > "$P/ports_nexus.txt" << 'EOF'
PORTS OUVERTS SUR NEXUS (10.0.0.1):
22/tcp ssh, 443/tcp https, 80/tcp http, 53/udp dns
EOF
cat > "$P/dns_nexus.txt" << 'EOF'
DNS DU NOC:
nexus.neon9.local -> 10.0.0.1
fai.neon9.local   -> 10.0.0.254
EOF

# ══════════ BAIES & SERVEURS ══════════
B="$V/baies"
mkdir -p "$B"/{A1,A2,A3,B1}
cat > "$B/etat_baies.txt" << 'EOF'
BAIES DU DATACENTER:
A1: serveurs web (OK), A2: BDD (OK), A3: monitoring (1 disque à changer), B1: backup (OK)
EOF
S="$V/serveurs"
cat > "$S/cluster.txt" << 'EOF'
CLUSTER NEXUS: nexus-01 (10.0.0.1), nexus-02 (10.0.0.2), nexus-03 (10.0.0.3)
EOF
cat > "$S/processus_suspects.txt" << 'EOF'
PROCESSUS SUSPECTS: PID 6666 (miner clandestin) détecté sur nexus-02.
EOF

# ══════════ FIREWALL (coffre) ══════════
F="$V/firewall"
mkdir -p "$F"/{coffre,regles}
cat > "$F/coffre/codes.txt" << 'EOF'
🔒 COFFRE-FORT RÉSEAU NEXUS 🔒
Token netrunner: NEON-77-KAMI
Code pare-feu: BLACKOUT-42
Protocole isolement total: SHADOW-9-REI
EOF
cat > "$F/regles/etat_ufw.txt" << 'EOF'
ÉTAT PARE-FEU: actif. 2 règles entrantes bloquées.
EOF

# ══════════ MONITORING & ARCHIVES ══════════
M="$V/monitoring"
cat > "$M/rapport_jour.txt" << 'EOF'
RAPPORT QUOTIDIEN NEXUS: 12 attaques bloquées, 1 scan détecté, 0 incident critique.
EOF
A="$V/archives"
mkdir -p "$A"/{logs,configs,2023,2024}
cat > "$A/configs/backup_sshd.txt" << 'EOF'
# backup sshd_config (avant durcissement)
Port 22
PermitRootLogin yes
EOF

# ══════════ PERMISSIONS ══════════
find "$V" -type d -exec chmod 755 {} \; 2>/dev/null
chmod 600 "$N/netrunner/.secret" "$F/coffre/codes.txt"
chmod 755 "$F"

echo "✅ NOC Nexus (campagne réseau) généré: $(find "$V" -type f | wc -l) fichiers, $(find "$V" -type d | wc -l) salles"

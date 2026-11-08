#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  GÉNÉRATEUR DU VAISSEAU SPATIAL ORION-7                   ║
# ║  Arborescence 100% immersive - pas un serveur Linux!     ║
# ╚══════════════════════════════════════════════════════════╝
# Usage: ./vaisseau-gen.sh [campagne] [dossier_destination]
#   campagne = espace (défaut) | japon
set -e
CAMP="${1:-espace}"
if [ "$CAMP" = "japon" ]; then V="${2:-$HOME/.gameshell/monde_japon}"; else V="${2:-${1:-vaisseau}}"; fi
rm -rf "$V"

if [ "$CAMP" = "japon" ]; then
  bash "$(dirname "$0")/monde-japon-gen.sh" "$V"
  exit 0
fi

# ═══════════════════════════════════════════════
#  NIVEAU 0 — LA RACINE DU VAISSEAU
# ═══════════════════════════════════════════════
mkdir -p "$V"/{pont,cabines,soutes,reacteur,hangar,salle-des-machines,infirmerie,cantine,navigation,communications,securite,armes,cale-a-carburant,cale-a-oxygene,soute-a-munitions,archives,corvettes,sas-entree,soute-dechets,atelier,bibliotheque,salle-de-reunion,cuisine,douches,toilettes,soute-refroidissement,soute-chauffage,vide-ordures,local-technique,antenne,soute-reserve,parking,garage,zone-industrielle,tmp,var/tmp}

# ═══════════════════════════════════════════════
#  PONT DE COMMANDEMENT (le QG)
# ═══════════════════════════════════════════════
P="$V/pont"
mkdir -p "$P/fauteuil-capitaine" "$P/consoles" "$P/ecrans" "$P/table-de-carte" "$P/communications" "$P/journaux-de-bord" "$P/plan-de-la-station" "$P/urgence"

cat > "$P/fauteuil-capitaine/notice.txt" << 'EOF'
FAUTEUIL DU CAPITAINE - ORION-7
================================
Bienvenue,Capitaine.

Depuis ce fauteuil,vous commandez toute la station.
Les consoles autour de vous donnent accès à tous les systèmes.

En cas d'alerte pirate,consultez:
- /pont/journaux-de-bord/dernier_rapport.txt
- /securite/alertes/
- /communications/messages/
EOF

cat > "$P/consoles/console_principale.txt" << 'EOF'
CONSOLE PRINCIPALE - ORION-7
État: OPÉRATIONNELLE

Systèmes connectés:
  - Navigation: OK
  - Communications: OK
  - Boucliers: 67% (dégradés)
  - Réacteur: 89%
  - Armes: PRÊTES
  - Détection pirates: 2 SIGNATURES HOSTILES
EOF

cat > "$P/ecrans/ecran_detection.txt" << 'EOF'
ÉCRAN DE DÉTECTION - SECTEUR 7G

🚨 2 signatures hostiles détectées:
  - 10.0.0.66 (Corbeaux Noirs) - approche rapide
  - 10.0.0.99 (Corbeaux Noirs) - scan de nos ports

Vaisseaux alliés: Andromède (secteur voisin)
EOF

cat > "$P/journaux-de-bord/dernier_rapport.txt" << 'EOF'
JOURNAL DE BORD - 15 JANVIER 2024 - 08:30

08:00 - Départ de l'Andromède — transfert du nouveau Capitaine
08:15 - Arrivée à bord d'Orion-7
08:23 - Connexion admin établie
08:31 - 🚨 ALERTE: Tentatives SSH échouées depuis 10.0.0.66
08:32 - 🚨 Plusieurs utilisateurs invalides testés (admin,root,test,oracle,postgres)
08:40 - Boucliers UFW ont bloqué les paquets
09:15 - Connexion légitime depuis 192.168.1.100
10:15 - 🚨 Nouvelle attaque depuis 10.0.0.99
10:20 - Paquet bloqué sur port 5432 (PostgreSQL)
11:45 - 🚨 Tentative avec utilisateur 'git' depuis 10.0.0.66

CONCLUSION: Attaque coordonnée des Corbeaux Noirs en cours!
ACTIONS: Durcir la station immédiatement!
EOF

cat > "$P/journaux-de-bord/historique.txt" << 'EOF'
HISTORIQUE DES MISSIONS D'ORION-7:

2023-12-01: Mise en service de la station
2023-12-15: Premier scan du secteur 7G - RAS
2024-01-10: Détection d'une flotte pirate lointaine
2024-01-14: L'Andromède nous transfère un nouveau capitaine
2024-01-15: 🚨 JOUR D'ATTAQUE - Corbeaux Noirs détectés
EOF

cat > "$P/plan-de-la-station/plan.txt" << 'EOF'
PLAN DE LA STATION ORION-7 (niveau 1 - pont)

        ┌─────────┐
        │  PONT   │ ← vous êtes ici
        │(vous)   │
   ┌────┴────┬────┘
   │         │
┌──┴──┐   ┌──┴──┐
│NAV  │   │COMMS│
└──┬──┘   └──┬──┘
   └────┬────┘
        │
   ┌────┴────┐
   │CABINES  │ ← quartiers équipage
   │SOUTES   │ ← cargo
   │RÉACTEUR │ ← cœur de la station
   └─────────┘
EOF

cat > "$P/urgence/code_urgence.txt" << 'EOF'
🚨 CODES D'URGENCE ORION-7 🚨

Code auto-destruction: OMEGA-9981-DELTA
Code abandon du vaisseau: ALPHA-7742-SIGMA
Code confinement: 5567-XRAY

⚠️ Ne pas utiliser sans autorisation du Capitaine!
EOF

# ═══════════════════════════════════════════════
#  CABINES (quartiers de l'équipage)
# ═══════════════════════════════════════════════
C="$V/cabines"
mkdir -p "$C"/{capitaine,zyra,vex,nira,tael,matelot,invite}

cat > "$C/capitaine/bienvenue.txt" << 'EOF'
BIENVENUE DANS VOTRE CABINE,CAPITAINE!

Vous êtes dans vos quartiers privés.

Depuis ici,vous pouvez:
- Consulter vos documents personnels
- Gérer vos projets
- Vous reposer (théoriquement)

La station vous écoute. Tapez 'gsh help' pour l'aide.
EOF

cat > "$C/capitaine/notes.txt" << 'EOF'
NOTES PERSONNELLES DU CAPITAINE

- Reçu ordre de l'Amiral Xylia Vorn: défendre Orion-7
- Attaques SSH détectées depuis 10.0.0.66 et 10.0.0.99
- UFW inactif → activer!
- fail2ban pas configuré → installer!
- PermitRootLogin=yes → DANGER,changer!
- Le coffre-fort est dans /securite/coffre-fort/
- Mot de passe perso: ne pas oublier!
EOF

cat > "$C/capitaine/plan_defense.sh" << 'SHEOF'
#!/bin/bash
# Plan de défense Orion-7 — Capitaine
echo "Activation du plan de défense..."
echo "1. Vérification des logs"
echo "2. Activation UFW"
echo "3. Configuration fail2ban"
echo "4. Durcissement SSH"
echo "Plan prêt!"
SHEOF
chmod 644 "$C/capitaine/plan_defense.sh"

cat > "$C/capitaine/.secret" << 'EOF'
CODE PERSONNEL: FALCON-9
Mot de passe wifi station: Orion7-Secure-2024
Ne pas divulguer!
EOF

cat > "$C/zyra/rapport_technique.txt" << 'EOF'
RAPPORT TECHNIQUE - MARIE (Ingénieur)

Le réacteur tourne à 89%. Tout va bien de ce côté.
Par contre,nos systèmes de sécurité sont faibles:
- Pare-feu UFW: INACTIF
- fail2ban: non installé
- SSH: configuration faible (root autorisé,MDP autorisés)

Il faut agir vite,Capitaine!
EOF

cat > "$C/vex/dossier_securite.txt" << 'EOF'
DOSSIER SÉCURITÉ - JEAN (Officier sécurité)

Menace: Les Corbeaux Noirs
- Groupe de pirates connus pour leurs attaques SSH
- Ils scannent les ports ouverts
- Ils tentent des utilisateurs communs (root,admin,test...)

Recommandations:
1. Activer UFW immédiatement
2. Installer fail2ban
3. Changer la config SSH
4. Surveiller les processus suspects
EOF

cat > "$C/nira/message_andromede.txt" << 'EOF'
📡 MESSAGE DU VAISSEAU-MÈRE GALACTICA

De: Amiral Xylia Vorn
À: le nouveau Capitaine,Orion-7

« Capitaine,nos capteurs confirment l'approche
des Corbeaux Noirs. Préparez la station.

Vos logs montrent déjà des tentatives.
Défendez Orion-7 à tout prix.

Nous restons en soutien à portée de phaser. »

— Amiral Xylia Vorn,15/01/2024 10:00
EOF

cat > "$C/tael/etat_machines.txt" << 'EOF'
ÉTAT DES MACHINES - PIERRE (Mécanicien)

Réacteur: 89% - stable
Boucliers: 67% - endommagés par les scans pirates
Carburant: 73% - suffisant
Oxygène: 91% - OK
Nourriture: 88% - OK

Problème: des processus inconnus détectés!
Vérifier avec ps aux,Capitaine.
EOF

# ═══════════════════════════════════════════════
#  SÉCURITÉ (le cœur de la défense)
# ═══════════════════════════════════════════════
S="$V/securite"
mkdir -p "$S"/{alertes,logs,coffre-fort,config,rapports,firewall,antivirus,cles}

cat > "$S/alertes/alerte_rouge.txt" << 'EOF'
🚨🚨🚨 ALERTE ROUGE - 15/01/2024 11:50 🚨🚨🚨

ATTAQUE EN COURS DES CORBEAUX NOIRS!

Vecteurs détectés:
1. SSH force brute depuis 10.0.0.66 (6 tentatives)
2. Scan de ports depuis 10.0.0.99
3. Tentative d'accès au coffre-fort

ACTIONS IMMÉDIATES:
□ Activer le pare-feu (UFW)
□ Installer fail2ban
□ Durcir SSH (plus de root,plus de MDP)
□ Changer les permissions du coffre-fort
□ Surveiller les processus
□ Archiver les preuves
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

cat > "$S/logs/syslog" << 'EOF'
Jan 15 08:00:00 orion7 systemd[1]: Started OpenBSD Secure Shell server.
Jan 15 08:31:15 orion7 sshd[1589]: Failed password for invalid user admin from 10.0.0.66 port 44123 ssh2
Jan 15 08:32:15 orion7 sshd[1612]: Invalid user postgres from 10.0.0.66 port 44131
Jan 15 08:40:55 orion7 kernel: [UFW BLOCK] IN=eth0 SRC=10.0.0.66 DST=192.168.1.10 PROTO=TCP SPT=44123 DPT=22 SYN
Jan 15 09:15:23 orion7 sshd[1892]: Accepted password for etudiant from 192.168.1.100 port 55432 ssh2
Jan 15 10:15:45 orion7 sshd[2103]: Failed password for invalid user postgres from 10.0.0.99 port 33221 ssh2
Jan 15 10:20:12 orion7 kernel: [UFW BLOCK] IN=eth0 SRC=10.0.0.99 DST=192.168.1.10 PROTO=TCP SPT=33221 DPT=5432 SYN
Jan 15 11:45:33 orion7 sshd[2450]: Failed password for invalid user git from 10.0.0.66 port 55001 ssh2
EOF

cat > "$S/logs/fail2ban.log" << 'EOF'
2024-01-15 08:35:00,123 fail2ban.actions [812]: NOTICE [sshd] Ban 10.0.0.66
2024-01-15 10:20:00,456 fail2ban.actions [812]: NOTICE [sshd] Ban 10.0.0.99
EOF

cat > "$S/coffre-fort/codes_acces.txt" << 'EOF'
🔒 COFFRE-FORT ORION-7 - NIVEAU 4 - ACCÈS RESTREINT 🔒

Coordonnées flotte Andromède: Secteur 7G,Quadrant X-42
Code lancement missiles: 8842-OMEGA
Position trou de ver: [CLASSIFIÉ]
Protocole auto-destruction: OMEGA-9981-DELTA
EOF

cat > "$S/config/sshd_config" << 'EOF'
# Configuration SSH - Orion-7
Port 22
PermitRootLogin yes
PasswordAuthentication yes
PermitEmptyPasswords no
MaxAuthTries 6
UsePAM yes
X11Forwarding no
EOF

cat > "$S/config/firewall_regles.txt" << 'EOF'
RÈGLES PARE-FEU RECOMMANDÉES:
ufw default deny incoming
ufw default allow outgoing
ufw allow 2222/tcp  # SSH durci
ufw allow from 192.168.1.0/24 to any port 22
ufw enable
EOF

cat > "$S/rapports/.placeholder" << 'EOF'
Dossier pour vos rapports d'incident.
EOF

cat > "$S/cles/cles_publiques.txt" << 'EOF'
Clés SSH publiques autorisées:
etudiant: ssh-ed25519 AAAAC3...commentaire
marie:   ssh-ed25519 AAAAC3...marie@orion7
EOF

# ═══════════════════════════════════════════════
#  RÉACTEUR (cœur énergétique)
# ═══════════════════════════════════════════════
R="$V/reacteur"
mkdir -p "$R"/{noyau,refroidissement,controle,securite}

cat > "$R/noyau/etat.txt" << 'EOF'
RÉACTEUR ORION-7 - ÉTAT: 89% - STABLE

Température cœur: 4.2M°C
Pression: nominale
Aucune fuite détectée
EOF

cat > "$R/controle/commandes.txt" << 'EOF'
COMMANDES DU RÉACTEUR (réservé ingénieur):
- status: état actuel
- boost: augmenter puissance (DANGER)
- eco: mode économique
- urgence: arrêt d'urgence
EOF

# ═══════════════════════════════════════════════
#  SOUTES (cargo)
# ═══════════════════════════════════════════════
SO="$V/soutes"
mkdir -p "$SO"/{cargo,armes,municions,carburant,nourriture,pieces,dechets,secrets}

cat > "$SO/manifest.txt" << 'EOF'
MANIFESTE DES SOUTES - ORION-7

Cargo principal:
- 50 tonnes carburant (soute-carburant)
- 200 unités nourriture (cantine)
- 10 caisses pièces détachées (CAISSE-01 à CAISSE-10)
- 3 containers données classifiées (soutes/secrets)
- 1 artefact alien (NE PAS TOUCHER)

Munitions: soute-a-munitions (restreint)
EOF

cat > "$SO/secrets/donnees_classifiees.txt" << 'EOF'
🔒 DONNÉES CLASSIFIÉES - NIVEAU 4 🔒

Projet Stargate: coordonnées trou de ver X-42
Opération Phoenix: plan d'évacuation
Flotte pirate Corbeaux Noirs: composition connue
EOF

# ═══════════════════════════════════════════════
#  COMMUNICATIONS
# ═══════════════════════════════════════════════
CO="$V/communications"
mkdir -p "$CO"/{messages,frequences,antenne,journal}

cat > "$CO/messages/dernier_message.txt" << 'EOF'
📡 DERNIER MESSAGE REÇU - 15/01 10:00

De: Andromède (Amiral Xylia Vorn)
À: Orion-7 (le nouveau Capitaine)

« Capitaine,les Corbeaux Noirs approchent.
Préparez la défense. Nous sommes en soutien.

Ne laissez pas la station tomber! »
EOF

cat > "$CO/frequences/frequences_ennemies.txt" << 'EOF'
FRÉQUENCES DÉTECTÉES (pirates):
- 10.0.0.66: fréquence 42.7 MHz (Corbeaux Noirs)
- 10.0.0.99: fréquence 88.1 MHz (Corbeaux Noirs - scan)

Fréquences alliées:
- Andromède: 121.5 MHz (détresse)
- Terre: 8.8.8.8 (DNS)
EOF

# ═══════════════════════════════════════════════
#  HANGAR & CORVETTES
# ═══════════════════════════════════════════════
H="$V/hangar"
mkdir -p "$H"/{corvettes,navettes,atelier-reparation}

cat > "$H/corvettes/liste.txt" << 'EOF'
CORVETTES AMARRÉES:
- FV-01 "Faucon" - opérationnelle
- FV-02 "Épervier" - en réparation
- FV-03 "Milan" - opérationnelle
- FV-04 "Pirate" 🚨 - CORVETTE PIRATE AMARRÉE! (ne pas approcher)
EOF

cat > "$H/corvettes/FV-04_pirate.txt" << 'EOF'
🚨 CORVETTE PIRATE DÉTECTÉE DANS LE HANGAR! 🚨

Nom: "Le Corbeau"
Équipage: 4 pirates armés
Mission: voler les codes du coffre-fort

ACTION: Ne pas laisser cette corvette décoller!
Tuer les processus pirates (PID dans /securite/logs/)
EOF

# ═══════════════════════════════════════════════
#  NAVIGATION
# ═══════════════════════════════════════════════
N="$V/navigation"
mkdir -p "$N"/{cartes,coordonnees,trajectoire,secteurs}

cat > "$N/coordonnees/orion7.txt" << 'EOF'
echo "orion7" > "$N/coordonnees/hostname"
COORDONNÉES ORION-7:
  Secteur: 7G
  Quadrant: X-42
  Position: 192.168.1.10
  Gateway: 192.168.1.1
EOF

cat > "$N/secteurs/secteur_7G.txt" << 'EOF'
CARTE DU SECTEUR 7G:

    [7G - Orion-7 ★]
         ↑
    [7F]--+--[7H]
         ↓
    [Andromède] ← vaisseau-mère allié

⚠️ Zone pirate: quadrant Y (Corbeaux Noirs)
EOF

# ═══════════════════════════════════════════════
#  SALLE DES MACHINES
# ═══════════════════════════════════════════════
M="$V/salle-des-machines"
mkdir -p "$M"/{processus,services,logs,monitoring}

cat > "$M/processus/processus_suspects.txt" << 'EOF'
PROCESSUS SUSPECTS DÉTECTÉS:

PID 9999 - "backdoor" - user: pirate - 🚨 À ÉLIMINER
PID 8888 - "keylogger" - user: ? - 🚨 À ÉLIMINER
PID 7777 - "cryptominer" - user: ? - 🚨 À ÉLIMINER

Commande pour tuer: kill PID
EOF

cat > "$M/services/services_critiques.txt" << 'EOF'
SERVICES CRITIQUES ORION-7:
- sshd (SSH) - port 22 - ⚠️ vulnérable
- ufw (pare-feu) - INACTIF ⚠️
- fail2ban - NON INSTALLÉ ⚠️
- reactor (réacteur) - actif
- navigation - actif
- communications - actif
EOF

cat > "$M/monitoring/rapport_systeme.txt" << 'EOF'
RAPPORT MONITORING - 15/01 11:50

CPU: 23% - normal
RAM: 4.2/16 Go - normal
Disque: 67% utilisé
Connexions: 3 actives (1 suspecte)
Processus: 47 actifs (3 suspects)
EOF

# ═══════════════════════════════════════════════
#  ARMES
# ═══════════════════════════════════════════════
A="$V/armes"
mkdir -p "$A"/{phasers,missiles,torpilles,boucliers,systeme}

cat > "$A/systeme/etat_armes.txt" << 'EOF'
SYSTÈME D'ARMES ORION-7 - PRÊT AU COMBAT

Phasers: 100% - 12 canons
Missiles: 8/8 - prêts
Torpilles: 4/4 - prêtes
Boucliers: 67% - avant 45%,arrière 89%

Cible prioritaire: Corvette "Le Corbeau" (hangar)
EOF

# ═══════════════════════════════════════════════
#  ARCHIVES (données historiques)
# ═══════════════════════════════════════════════
AR="$V/archives"
mkdir -p "$AR"/{2023,2024,missions-passees,personnel}

cat > "$AR/missions-passees/mission_2023-12.txt" << 'EOF'
MISSION DÉCEMBRE 2023:
Exploration du secteur 7G. RAS.
Station mise en service.
Équipage: 5 personnes.
EOF

# ═══════════════════════════════════════════════
#  PERMISSIONS
# ═══════════════════════════════════════════════
find "$V" -type d -exec chmod 755 {} \; 2>/dev/null
chmod 600 "$C/capitaine/.secret" "$S/coffre-fort/codes_acces.txt" "$SO/secrets/donnees_classifiees.txt"
chmod 700 "$S/coffre-fort" "$SO/secrets"

echo "✅ Vaisseau Orion-7 généré: $(find "$V" -type f | wc -l) fichiers,$(find "$V" -type d | wc -l) compartiments"

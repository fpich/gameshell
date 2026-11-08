#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  GÉNÉRATEUR DU MONDE "LE JAPON FÉODAL" — Château Shiro   ║
# ║  Campagne 2 (niveau MOYEN) — environnement dédié (D2)   ║
# ╚══════════════════════════════════════════════════════════╝
# Usage: ./monde-japon-gen.sh [dossier_destination]

set -e
V="${1:-$HOME/.gameshell/monde_japon}"
rm -rf "$V"

# ══════════ NIVEAU 0 — LE CHÂTEAU SHIRO ══════════
mkdir -p "$V"/{donjon,pagode,jardins,remparts,portail,armurerie,forge,cuisine,greniers,ecuries,quartiers,scribes,archives,bibliotheque,salle-de-the,salle-du-trone,entrepot,cellier,toilettes,bains,tmp,var/tmp}

# ══════════ QUARTIERS DU SEIGNEUR (home ~) ══════════
Q="$V/quartiers"
mkdir -p "$Q"/{seigneur,samourai,ronin,moine,marchand,garde}
cat > "$Q/seigneur/kaisho.txt" << 'EOF'
BIENVENUE DANS VOS QUARTIERS, SEIGNEUR!

Vous êtes le daimyo du château Shiro, perché sur sa colline.
Depuis vos quartiers, vous administrez le domaine:
- Consulter vos documents et sceaux
- Gérer les récoltes et les greniers
- Préparer la défense contre les clans rivaux

Le château vous écoute. Tapez 'gsh help' pour l'aide.
EOF
cat > "$Q/seigneur/notes.txt" << 'EOF'
NOTES DU SEIGNEUR:
- Vérifier les stocks de riz dans les greniers
- Signer les lettres du clan Hattori
- Ne pas oublier le code du coffre (voir archives/secret)
EOF
cat > "$Q/seigneur/plan_defense.sh" << 'SHEOF'
#!/bin/bash
# Plan de défense du château Shiro
echo "Fermez le portail! Sonnez le gong! Tout le monde aux remparts!"
SHEOF
chmod 755 "$Q/seigneur/plan_defense.sh"
cat > "$Q/seigneur/.secret" << 'EOF'
Le sceau secret du seigneur: LACUS-7-ONI
EOF
cat > "$Q/samourai/ordre.txt" << 'EOF'
Ordre du jour: patrouille sur les remparts à l'aube.
Le ronin Musashi a été aperçu près du portail est.
EOF
cat > "$Q/ronin/lettre.txt" << 'EOF'
Seigneur, le clan rival convoite nos greniers de riz.
— Ronin Musashi
EOF
cat > "$Q/moine/priere.txt" << 'EOF'
Que les kami protègent le château Shiro et son seigneur.
EOF
cat > "$Q/marchand/inventaire.txt" << 'EOF'
INVENTAIRE DU MARCHAND GENJI:
- riz: 40 koku   - thé: 12 caisses   - soie: 7 ballots
EOF
cat > "$Q/garde/rapport.txt" << 'EOF'
Rapport de garde: RAS cette nuit. Le portail est verrouillé.
EOF

# ══════════ SALLE DU TRÔNE ══════════
T="$V/salle-du-trone"
mkdir -p "$T"/{fauteuil,registres,decrets}
cat > "$T/fauteuil/notice.txt" << 'EOF'
Le fauteuil du daimyo. Seul le seigneur s'y assoit.
EOF
cat > "$T/registres/registre_impots.txt" << 'EOF'
REGISTRE DES IMPOTS — AN 12 DE L'ÈRE SHIRO
- Village du bas: 120 koku de riz
- Village du haut: 80 koku de riz
- Marché du port: 15 ryo
EOF
cat > "$T/decrets/decret_ronin.txt" << 'EOF'
DÉCRET: tout ronin pénétrant dans le château sera arrêté.
Le clan Hattori reste notre allié malgré les rumeurs.
EOF

# ══════════ DONJON (défense) ══════════
D="$V/donjon"
mkdir -p "$D"/{coffre-fort,armes,gardes,prison}
cat > "$D/coffre-fort/sceaux.txt" << 'EOF'
🔒 COFFRE-FORT DU CHÂTEAU SHIRO 🔒
Sceau du seigneur: LACUS-7-ONI
Code du donjon: KAMI-442-TORA
Protocole de siège: TORA-442-KAMI
EOF
cat > "$D/armes/etat_armes.txt" << 'EOF'
ÉTAT DE L'ARMURERIE:
- katanas: 30   - yari: 50   - arcs: 25   - tambours de guerre: 4
EOF
cat > "$D/gardes/journal_gardes.txt" << 'EOF'
JOURNAL DES GARDES:
02:10 - ronde du rempart nord: RAS
03:45 - bruit près du portail est: ronin repéré, parti en courant
05:00 - relève effectuée
EOF
cat > "$D/prison/registre_prisonniers.txt" << 'EOF'
PRISONNIERS: 2 espions du clan rival, 1 voleur de riz.
EOF

# ══════════ PAGODE & ARCHIVES ══════════
P="$V/pagode"
mkdir -p "$P"/{autel,bibliotheque-sacree}
cat > "$P/autel/offrandes.txt" << 'EOF'
Offrandes déposées ce matin: 3 bols de saké, des fruits, de l'encens.
EOF
A="$V/archives"
mkdir -p "$A"/{lettres,traites,secret,2023,2024}
cat > "$A/lettres/lettre_clan_hattori.txt" << 'EOF'
Seigneur,
Le clan Hattori vous renouvelle son allégeance.
Nous enverrons 20 samouraïs en renfort si le siège commence.
— Seigneur Hattori
EOF
cat > "$A/traites/traite_paix.txt" << 'EOF'
TRAITÉ DE PAIX entre Shiro et le clan voisin:
- échange de riz contre de la soie
- frontière fixée à la rivière
EOF
cat > "$A/secret/code_coffre.txt" << 'EOF'
Code du coffre-fort du donjon: KAMI-442-TORA
EOF

# ══════════ GRENIERS & CUISINE (logistique) ══════════
G="$V/greniers"
mkdir -p "$G"/{riz,ble,sake,miso}
cat > "$G/riz/registre_riz.txt" << 'EOF'
STOCK DE RIZ:
- grenier nord: 200 koku
- grenier sud: 150 koku (à surveiller, le toit fuit)
EOF
cat > "$G/sake/stock_sake.txt" << 'EOF'
Saké: 40 barils. Réservé aux cérémonies et aux samouraïs de garde.
EOF
C="$V/cuisine"
mkdir -p "$C"/{preparations,recettes}
cat > "$C/recettes/recette_miso.txt" << 'EOF'
RECETTE DE LA SOUPE MISO DU CHÂTEAU:
miso, dashi, tofu, wakamé. Servir bien chaud.
EOF

# ══════════ SCRIBES (logs) ══════════
S="$V/scribes"
mkdir -p "$S"/{logs,registres,alertes}
cat > "$S/logs/auth.log" << 'EOF'
Jan 15 08:31:15 shiro sshd[1589]: Failed password for invalid user ronin from 10.0.0.66 port 44123 ssh2
Jan 15 08:31:45 orion7 sshd[1601]: Failed password for invalid user espion from 10.0.0.66 port 44125 ssh2
Jan 15 09:15:23 shiro sshd[1892]: Accepted password for seigneur from 192.168.1.100 port 55432 ssh2
Jan 15 10:15:45 shiro sshd[2103]: Failed password for invalid user ninja from 10.0.0.99 port 33221 ssh2
Jan 15 11:45:33 shiro sshd[2450]: Failed password for invalid user voleur from 10.0.0.66 port 55001 ssh2
EOF
cat > "$S/logs/syslog" << 'EOF'
Jan 15 06:00:01 shiro CRON[900]: sauvegarde des registres
Jan 15 08:31:15 shiro sshd[1589]: tentative de connexion échouée (ronin)
Jan 15 10:00:00 shiro GARDE: ronde du rempart est
Jan 15 12:30:00 shiro CUISINE: préparation du dîner du seigneur
EOF
cat > "$S/registres/sshd_config" << 'EOF'
# Configuration SSH - Château Shiro
Port 22
PermitRootLogin yes
PasswordAuthentication yes
MaxAuthTries 6
UsePAM yes
EOF
cat > "$S/alertes/alerte_ronin.txt" << 'EOF'
🚨 ALERTE: ronin Musashi vu près du portail est à 3 reprises cette semaine.
EOF

# ══════════ ARMurERIE & FORGE ══════════
AR="$V/armurerie"
mkdir -p "$AR"/{katana,armures,forge}
cat > "$AR/armures/etat_armures.txt" << 'EOF'
ARMURES: 25 en réparation, 40 prêtes au combat.
EOF
F="$V/forge"
cat > "$F/registre_forge.txt" << 'EOF'
FORGE DU CHÂTEAU: 3 forgerons. Commandes: 12 lances, 5 katanas.
EOF

# ══════════ JARDINS & SALLE DE THÉ ══════════
J="$V/jardins"
mkdir -p "$J"/{zen,cerisiers,bassin}
cat > "$J/zen/note_moine.txt" << 'EOF'
« Un esprit calme tient une armée. » — Moine Tetsuo
EOF
ST="$V/salle-de-the"
cat > "$ST/ceremonie.txt" << 'EOF'
Cérémonie du thé de demain: 10 invités, service à 14h précises.
EOF

# ══════════ REMPARTS & PORTAIL ══════════
R="$V/remparts"
mkdir -p "$R"/{nord,sud,est,ouest}
cat > "$R/etat_rembarts.txt" << 'EOF'
REMPARTS: nord OK, sud OK, est fissuré (réparer), ouest OK.
EOF
PO="$V/portail"
cat > "$PO/registre_visiteurs.txt" << 'EOF'
VISITEURS: marchand Genji (matin), moine Tetsuo (midi), inconnu parti en courant (soir).
EOF

# ══════════ PERMISSIONS ══════════
find "$V" -type d -exec chmod 755 {} \; 2>/dev/null
chmod 600 "$Q/seigneur/.secret" "$D/coffre-fort/sceaux.txt" "$A/secret/code_coffre.txt"
chmod 700 "$D/coffre-fort"

echo "✅ Château Shiro (Japon Féodal) généré: $(find "$V" -type f | wc -l) fichiers, $(find "$V" -type d | wc -l) pièces"

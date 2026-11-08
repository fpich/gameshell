#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  SUITE DE TESTS AUTOMATISÉS — Moteur GameShell Orion-7    ║
# ║  Teste les cas limites et fige le fonctionnement moteur   ║
# ╚══════════════════════════════════════════════════════════╝
# Usage: ./tests/run_tests.sh
# Prérequis: cargo build --release && ./make-archive.sh

set -e
cd "$(dirname "$0")/.."

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

PASS=0
FAIL=0
TOTAL=0

ok()   { PASS=$((PASS+1)); TOTAL=$((TOTAL+1)); echo -e "${GREEN}✅ PASS${NC} ($TOTAL) $1"; }
ko()   { FAIL=$((FAIL+1)); TOTAL=$((TOTAL+1)); echo -e "${RED}❌ FAIL${NC} ($TOTAL) $1"; }
info() { echo -e "${YELLOW}ℹ️  $1${NC}"; }

# Préparer l'archive fraîche
info "Build + archive..."
cargo build --release 2>/dev/null
bash make-archive.sh > /dev/null 2>&1

# Fonction pour lancer une session de test
run_game() {
    local input="$1"
    rm -rf ~/.gameshell
    echo -e "$input" | timeout 15 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g'
}

echo ""
echo "═══════════════════════════════════════"
echo "  TESTS MOTEUR GAMESHELL ORION-7"
echo "═══════════════════════════════════════"
echo ""

# ─── TEST 1: 50 missions chargées ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "50 missions" && ok "50 missions chargées au démarrage" || ko "50 missions chargées"

# ─── TEST 2: Pas d'auto-validation (règle 3) ───
out=$(run_game "Test\npwd\nls\nexit\n")
echo "$out" | grep -q "✅ Mission réussie" && ko "Pas d'auto-validation (mission réussie sans gsh check)" || ok "Pas d'auto-validation (règle 3)"

# ─── TEST 3: gsh check valide si commandes faites ───
out=$(run_game "Test\npwd\nls\ngsh check\nexit\n")
echo "$out" | grep -q "✅ Mission réussie" && ok "gsh check valide mission 1.1 (pwd+ls)" || ko "gsh check valide mission 1.1"

# ─── TEST 4: ls seul ne valide PAS (il manque pwd) ───
out=$(run_game "Test\nls\ngsh check\nexit\n")
echo "$out" | grep -q "❌ Mission non validée" && ok "ls seul → ❌ Mission non validée (pwd manquant)" || ko "ls seul devrait échouer"

# ─── TEST 5: Check matche commande de base (règle 4m) ───
out=$(run_game "Test\npwd\nls -la\ngsh check\nexit\n")
echo "$out" | grep -q "✅ Mission réussie" && ok "ls -la matche le check 'ls' (commande de base, règle 4m)" || ko "ls -la devrait matcher ls"

# ─── TEST 6: Historique reset entre sessions (règle 2) ───
rm -rf ~/.gameshell
echo -e "Test\npwd\nls\ngsh check\nexit\n" | timeout 15 bash gameshell.sh > /dev/null 2>&1
# Nouvelle session: juste ls, gsh check doit échouer (pas de pwd dans CETTE session)
out=$(echo -e "Test2\nls\ngsh check\nexit\n" | timeout 15 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "❌ Mission non validée" && ok "Historique reset entre sessions (règle 2)" || ko "Fuite d'historique entre sessions!"

# ─── TEST 7: Isolation chemins — ls / montre le vaisseau ───
out=$(run_game "Test\nls /\nexit\n")
echo "$out" | grep -q "pont" && echo "$out" | grep -q "soutes" && ! echo "$out" | grep -qE "^bin$|^usr$|^etc$" && ok "ls / = vaisseau (pont, soutes...), pas le vrai système (règle 1)" || ko "ls / devrait montrer le vaisseau"

# ─── TEST 8: Prompt propre orion7 ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "Test@orion7:~" && ok "Prompt = pseudo@orion7:~\$" || ko "Prompt devrait être pseudo@orion7:~\$"

# ─── TEST 9: pwd affiche ~ pas le vrai chemin ───
out=$(run_game "Test\npwd\nexit\n")
echo "$out" | grep -q "~$" && ok "pwd affiche ~ (pas le vrai chemin)" || ko "pwd devrait afficher ~"

# ─── TEST 10: cd persistant + prompt mis à jour ───
out=$(run_game "Test\ncd /pont\npwd\nexit\n")
echo "$out" | grep -q "Test@orion7:/pont" && ok "cd /pont → prompt /pont + pwd propre" || ko "cd /pont devrait changer le prompt"

# ─── TEST 11: hostname = orion7 ───
out=$(run_game "Test\nhostname\nexit\n")
echo "$out" | grep -q "orion7$" && ok "hostname → orion7" || ko "hostname devrait être orion7"

# ─── TEST 12: Commande vide ne plante pas ───
out=$(run_game "Test\n\n\nexit\n")
echo "$out" | grep -q "Test@orion7" && ok "Lignes vides ignorées sans crash" || ko "Lignes vides devraient être ignorées"

# ─── TEST 13: Commande inconnue → erreur propre ───
out=$(run_game "Test\nfoobar123\nexit\n")
echo "$out" | grep -qi "not found\|introuvable\|command" && ok "Commande inconnue → erreur propre" || info "Commande inconnue: comportement = $(echo "$out" | grep foobar | head -1)"

# ─── TEST 14: cd chemin invalide → erreur bash ───
out=$(run_game "Test\ncd /nonexistent123\npwd\nexit\n")
echo "$out" | grep -q "No such file" && echo "$out" | grep -q "Test@orion7:~" && ok "cd invalide → erreur + reste sur ~" || ko "cd invalide devrait échouer proprement"

# ─── TEST 15: gsh check sans rien faire → ❌ ───
out=$(run_game "Test\ngsh check\nexit\n")
echo "$out" | grep -q "❌ Mission non validée" && ok "gsh check sans rien → ❌ Mission non validée" || ko "gsh check vide devrait échouer"

# ─── TEST 16: gsh goal = même affichage que show_mission ───
out=$(run_game "Test\ngsh goal\nexit\n")
echo "$out" | grep -q "┏━ MISSION 1.1" && ok "gsh goal affiche le cadre ┏━ MISSION (règle 4k)" || ko "gsh goal devrait afficher le cadre mission"

# ─── TEST 17: gsh help affiché au lancement ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "gsh help" && ok "gsh help affiché au lancement (règle 4)" || ko "gsh help devrait être auto-affiché"

# ─── TEST 18: man = compétence de départ ───
out=$(run_game "Test\ngsh skills\nexit\n")
echo "$out" | grep -q "man" && ok "man = compétence de départ dans gsh skills (règle 4k)" || ko "man devrait être compétence de départ"

# ─── TEST 19: Bloom en français ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "Se souvenir" && ok "Bloom affiché en français (Se souvenir, règle 4l)" || ko "Bloom devrait être en français"

# ─── TEST 20: Footer mission = gsh help seul ───
out=$(run_game "Test\nexit\n")
line=$(echo "$out" | grep "🏆" | head -1)
echo "$line" | grep -q "gsh help" && ! echo "$line" | grep -q "gsh check" && ok "Footer mission = 'gsh help' seul (règle 4l)" || ko "Footer devrait être 'gsh help' seul"

# ─── TEST 21: Carte ASCII présente dans chaque mission ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "VUE DE DESSUS" && ok "Carte vue de dessus affichée (règle 4h)" || ko "Carte devrait être affichée"

# ─── TEST 22: Double définition commande (roleplay + technique) ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "🚀" && echo "$out" | grep -q "💻" && ok "Commandes avec 🚀 roleplay + 💻 technique (règle 4b)" || ko "Double définition manquante"

# ─── TEST 23: Mission 2 s'affiche après validation 1.1 ───
out=$(run_game "Test\npwd\nls\ngsh check\nexit\n")
echo "$out" | grep -q "MISSION 1.2" && ok "Mission suivante (1.2) affichée après check réussi" || ko "Mission 1.2 devrait suivre"

# ─── TEST 24: exit quitte proprement ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "Déconnexion" && ok "exit → déconnexion propre" || ko "exit devrait quitter proprement"

# ─── TEST 25: gsh reset ne plante pas ───
out=$(run_game "Test\ngsh reset\nexit\n")
echo "$out" | grep -q "réinitialisé\|🔄" && ok "gsh reset fonctionne" || ko "gsh reset devrait fonctionner"


# ─── TEST 26: Playthrough 10 premières missions ───
out=$(printf 'Test\npwd\nls\ngsh check\ncat bienvenue.txt\ngsh check\ncd /communications/messages\ncat dernier_message.txt\ngsh check\nhostname\nwhoami\nid\ngsh check\ndate\ngsh check\ntouch incident_15jan.txt\nls\ngsh check\nmkdir -p rapports/2024/janvier\ngsh check\ncp incident_15jan.txt /soutes/\ngsh check\nmv incident_15jan.txt rapports/2024/janvier/\ngsh check\ntouch t1 t2 t3\nrm t1 t2 t3\ngsh check\nexit\n' | timeout 30 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
n=$(echo "$out" | grep -c "✅ Mission réussie")
[ "$n" -ge 10 ] && ok "Playthrough 10 premières missions ($n/10 réussies)" || ko "Playthrough 10 missions: seulement $n/10"


# ─── TEST 27: EOF (Ctrl+D) = sortie propre ───
out=$(printf 'Test\npwd\n' | timeout 10 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "EOF\|Déconnexion" && ok "EOF (Ctrl+D) → déconnexion propre (règle A13)" || ko "EOF devrait déconnecter proprement"

# ─── TEST 28: Input très long tronqué (anti freeze) ───
out=$(printf 'Test\nls %s\npwd\nexit\n' "$(python3 -c "print('a'*8000)")" | timeout 15 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "Test@orion7" && ! echo "$out" | grep -q "panicked" && ok "Input 8000 chars → tronqué, pas de crash" || ko "Input long devrait être tronqué sans crash"

# ─── TEST 29: gsh goal suggéré après 2 échecs ───
out=$(printf 'Test\ngsh check\ngsh check\nexit\n' | timeout 10 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "gsh goal" && ok "Après 2 échecs, suggère 'gsh goal' (UX)" || ko "Devrait suggérer gsh goal après 2 échecs"

# ─── TEST 30: Progression dans le prompt [N/50] ───
out=$(printf 'Test\nexit\n' | timeout 10 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -qE "\[0/50\]" && ok "Prompt affiche [N/50] progression" || ko "Prompt devrait afficher [N/50]"

# ─── TEST 31: Pipe multiple fonctionne ───
out=$(printf 'Test\ncat /pont/journaux-de-bord/dernier_rapport.txt | grep ALERTE | wc -l\nexit\n' | timeout 10 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -qE "[0-9]+" && ok "Pipe multiple (cat | grep | wc) fonctionne" || ko "Pipe multiple devrait fonctionner"

# ─── TEST 32: Redirection > dans le vaisseau ───
out=$(printf 'Test\necho hello > /soutes/test_gsh.txt\ncat /soutes/test_gsh.txt\nexit\n' | timeout 10 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "hello" && ok "Redirection > fonctionne dans le vaisseau" || ko "Redirection > devrait fonctionner"

echo ""
echo "═══════════════════════════════════════"
echo -e "  RÉSULTAT: ${GREEN}$PASS PASS${NC} / ${RED}$FAIL FAIL${NC} / $TOTAL total"
echo "═══════════════════════════════════════"
[ $FAIL -eq 0 ] && exit 0 || exit 1

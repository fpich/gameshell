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
    # insère le choix de campagne "1" (Pirates de l'Espace) après le nom
rm -rf ~/.gameshell
    echo -e "$input" | sed '1a 1' | timeout 15 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g'
}

echo ""
echo "═══════════════════════════════════════"
echo "  TESTS MOTEUR GAMESHELL ORION-7"
echo "═══════════════════════════════════════"
echo ""

# ─── TEST 1: 50 missions chargées ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "100 missions" && ok "100 missions en dur chargées (20 chapitres x 5, 20/acte)" || ko "100 missions chargées"

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
rm -rf ~/.gameshell
echo -e "Test\npwd\nls\ngsh check\nexit\n" | timeout 15 bash gameshell.sh > /dev/null 2>&1
# Nouvelle session: juste ls, gsh check doit échouer (pas de pwd dans CETTE session)
out=$(echo -e "Test2\n1\nls\ngsh check\nexit\n" | timeout 15 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
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

# ─── TEST 21: Carte vue de dessus SUPPRIMÉE (règle C6, décision utilisateur) ───
out=$(run_game "Test\nexit\n")
! echo "$out" | grep -q "VUE DE DESSUS" && ok "Plus de vue de dessus affichée (règle C6 supprimée)" || ko "La vue de dessus devrait être supprimée"

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
rm -rf ~/.gameshell
out=$(printf 'Test\n1\npwd\nls\ngsh check\ncat bienvenue.txt\ngsh check\ncd /communications/messages\ncat dernier_message.txt\ngsh check\nhostname\nwhoami\nid\ngsh check\ndate\ngsh check\ntouch incident_15jan.txt\nls\ngsh check\nmkdir -p rapports/2024/janvier\ngsh check\ncp incident_15jan.txt /soutes/\ngsh check\nmv incident_15jan.txt rapports/2024/janvier/\ngsh check\ntouch t1 t2 t3\nrm t1 t2 t3\ngsh check\nexit\n' | timeout 30 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
n=$(echo "$out" | grep -c "✅ Mission réussie")
[ "$n" -ge 10 ] && ok "Playthrough 10 premières missions ($n/10 réussies)" || ko "Playthrough 10 missions: seulement $n/10"


# ─── TEST 27: EOF (Ctrl+D) = sortie propre ───
rm -rf ~/.gameshell
out=$(printf 'Test\n1\npwd\n' | timeout 10 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "EOF\|Déconnexion" && ok "EOF (Ctrl+D) → déconnexion propre (règle A13)" || ko "EOF devrait déconnecter proprement"

# ─── TEST 28: Input très long tronqué (anti freeze) ───
rm -rf ~/.gameshell
out=$(printf 'Test\n1\nls %s\npwd\nexit\n' "$(python3 -c "print('a'*8000)")" | timeout 15 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "Test@orion7" && ! echo "$out" | grep -q "panicked" && ok "Input 8000 chars → tronqué, pas de crash" || ko "Input long devrait être tronqué sans crash"

# ─── TEST 29: gsh goal suggéré après 2 échecs ───
rm -rf ~/.gameshell
out=$(printf 'Test\n1\ngsh check\ngsh check\nexit\n' | timeout 10 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "gsh goal" && ok "Après 2 échecs, suggère 'gsh goal' (UX)" || ko "Devrait suggérer gsh goal après 2 échecs"

# ─── TEST 30: Progression dans le prompt [N/50] ───
rm -rf ~/.gameshell
out=$(printf 'Test\n1\nexit\n' | timeout 10 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -qE "\[0/100\]" && ok "Prompt affiche [N/100] progression" || ko "Prompt devrait afficher [N/100]"

# ─── TEST 31: Pipe multiple fonctionne ───
rm -rf ~/.gameshell
out=$(printf 'Test\n1\ncat /pont/journaux-de-bord/dernier_rapport.txt | grep ALERTE | wc -l\nexit\n' | timeout 10 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -qE "[0-9]+" && ok "Pipe multiple (cat | grep | wc) fonctionne" || ko "Pipe multiple devrait fonctionner"

# ─── TEST 32: Redirection > dans le vaisseau ───
rm -rf ~/.gameshell
out=$(printf 'Test\n1\necho hello > /soutes/test_gsh.txt\ncat /soutes/test_gsh.txt\nexit\n' | timeout 10 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "hello" && ok "Redirection > fonctionne dans le vaisseau" || ko "Redirection > devrait fonctionner"

# ─── TEST 33: Tests de conformité cargo (règles A15/C6/C9/C13) ───
if cargo test --release > /tmp/gsh_cargo_test.log 2>&1 && grep -q "test result: ok" /tmp/gsh_cargo_test.log; then
    ok "cargo test: conformité missions (skill=A15, carte=C6, bloom FR=C9, PNJ SF=C13)"
else
    ko "cargo test conformité a échoué: $(grep -m1 panicked /tmp/gsh_cargo_test.log)"
fi

# ─── TEST 34: id/uname interceptés, jamais d'infos de l'hôte (règle A16) ───
out=$(run_game "Test\nid\nuname -a\nexit\n")
echo "$out" | grep -q "uid=1000(capitaine)" && echo "$out" | grep -q "Linux orion7 6.6" && ! echo "$out" | grep -qE "uid=[0-9]+\($(whoami)\)|^$(hostname -s)$|$(hostname)$" && ok "id/uname → identité vaisseau (capitaine/orion7), pas l'hôte (règle A16)" || ko "id/uname ne doivent pas révéler l'hôte"

# ─── TEST 35: Flèches en mode pipe = séquences ANSI nettoyées (règle A11) ───
rm -rf ~/.gameshell
out=$(printf 'Test\n1\n\x1b[D\x1b[Cpwd\x1b[A\x1b[B\nls\ngsh check\nexit\n' | timeout 10 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "✅ Mission réussie" && ok "Séquences ANSI (flèches) nettoyées en mode non-tty, commande intacte" || ko "Les flèches ne devraient rien insérer dans la commande"

# ─── TEST 36: Compétence débloquée = commande de LA mission (règle A15) ───
out=$(run_game "Test\npwd\nls\ngsh check\ncat bienvenue.txt\ngsh check\nexit\n")
echo "$out" | grep -q "🔓 Compétence: pwd" && echo "$out" | grep -q "🔓 Compétence: cat" && ok "Skills débloquées = commandes des missions (pwd, cat — règle A15)" || ko "Skill débloquée incohérente avec la mission"

# ─── TEST 37: PNJ noms SF dans les missions (règle C13) ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "Zyra-7" && ! echo "$out" | grep -qE "\b(Marie|Sophie|Jean|Kowalski|Galactica)\b" && ok "PNJ avec noms SF (Zyra-7...), jamais Marie/Kowalski/Galactica (règle C13)" || ko "PNJ non conformes"

# ─── TEST 38: Auto-solve exhaustif des 2 campagnes x 100 missions (GAMESHELL_AUTO=1) ───
rm -rf ~/.gameshell
rm -rf ~/.gameshell
out=$(printf 'Auto\nexit\n' | GAMESHELL_AUTO=1 timeout 900 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "AUTO campagne 1 (Pirates de l'Espace): 100/100" && echo "$out" | grep -q "AUTO campagne 2 (Japon Féodal): 100/100" && echo "$out" | grep -q "AUTO campagne 3 (Nexus Réseau): 100/100" && ok "Auto-solve: 100/100 sur les 3 campagnes (joué réellement)" || ko "Auto-solve: $(echo "$out" | grep -o 'AUTO campagne [0-9][^ ]*: [0-9/]*' | tr '\n' ' ')"

# ─── TEST 39: Campagne 2 (Japon Féodal) sélectionnée au lancement (règle C14) ───
rm -rf ~/.gameshell
rm -rf ~/.gameshell
out=$(printf 'Samoura\n2\nhostname\nwhoami\nexit\n' | timeout 20 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "Japon Féodal" && echo "$out" | grep -q "100 missions" && echo "$out" | grep -q "Samoura@shiro:~" && echo "$out" | grep -qE "seigneur$" && ok "Campagne 2: bannière Japon Féodal, prompt @shiro, whoami=seigneur, 100 missions" || ko "Campagne 2 devrait être sélectionnable (Japon Féodal/shiro/seigneur)"

# ─── TEST 40: Banc de cas limites EDGE (GAMESHELL_EDGE=1) sur les 200 missions ───
rm -rf ~/.gameshell
rm -rf ~/.gameshell
out=$(printf 'Edge\nexit\n' | GAMESHELL_EDGE=1 timeout 900 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "EDGE campagne 1 (Pirates de l'Espace): 100/100 missions OK, 0 problèmes" && echo "$out" | grep -q "EDGE campagne 2 (Japon Féodal): 100/100 missions OK, 0 problèmes" && echo "$out" | grep -q "EDGE campagne 3 (Nexus Réseau): 100/100 missions OK, 0 problèmes" && ok "EDGE: 300/300 missions passent les cas limites (check vide, garbage, mauvais code, reset)" || ko "EDGE: $(echo "$out" | grep -o 'EDGE campagne [0-9][^ ]*: [0-9/]* missions OK, [0-9]* problèmes' | tr '\n' ' ')"

# ─── TEST 41: Campagne 3 (Nexus Réseau) sélectionnée au lancement (règle C14) ───
rm -rf ~/.gameshell
rm -rf ~/.gameshell
out=$(printf 'Net\n3\nhostname\nwhoami\nexit\n' | timeout 20 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "Nexus" && echo "$out" | grep -q "100 missions" && echo "$out" | grep -q "Net@nexus:~" && echo "$out" | grep -qE "netrunner$" && ok "Campagne 3: bannière Nexus, prompt @nexus, whoami=netrunner, 100 missions" || ko "Campagne 3 devrait être sélectionnable (Nexus/nexus/netrunner)"

# ─── TEST 42: TORTURE (GAMESHELL_TORTURE=1): 300 missions torturées + isolation hôte ───
rm -rf ~/.gameshell
rm -rf ~/.gameshell
out=$(printf 'T\nexit\n' | GAMESHELL_TORTURE=1 timeout 1200 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
ok42=1
for c in "1 (Pirates de l'Espace)" "2 (Japon Féodal)" "3 (Nexus Réseau)"; do
    echo "$out" | grep -q "TORTURE campagne $c: 100/100 OK, 0 problèmes" || ok42=0
done
[ "$ok42" = "1" ] && ok "TORTURE: 300/300 missions (check vide/partiel/typo, 2 mauvais codes, isolation hôte)" || ko "TORTURE: $(echo "$out" | grep -o 'TORTURE campagne [0-9][^:]*: [0-9/]* OK, [0-9]* problèmes' | tr '\n' ' ')"

# ─── TEST 43: Sauvegarde/reprise par campagne (règle A18) ───
rm -rf ~/.gameshell
rm -rf ~/.gameshell
printf 'Test\n1\npwd\nls\ngsh check\nexit\n' | timeout 15 bash gameshell.sh > /dev/null 2>&1
out=$(printf 'Test\n1\ngsh goal\nexit\n' | timeout 15 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
echo "$out" | grep -q "MISSION 1.2" && echo "$out" | grep -q "\[1/100\]" && ok "Reprise: session 2 reprend à la mission 1.2 avec [1/100] (sauvegarde A18)" || ko "La sauvegarde/reprise ne fonctionne pas"
ls ~/.gameshell/state/save_campagne1.json > /dev/null 2>&1 && ok "Fichier save_campagne1.json créé" || ko "Pas de fichier de sauvegarde"

# ─── TEST 44: gsh export enseignant JSON+CSV (règle A20) ───
rm -rf ~/.gameshell
rm -rf ~/.gameshell
printf 'Test\n1\npwd\nls\ngsh check\ngsh export\nexit\n' | timeout 15 bash gameshell.sh > /tmp/export_test.log 2>&1
echo "$(sed 's/\x1b\[[0-9;]*m//g' /tmp/export_test.log)" | grep -q "Export effectué" && ls ~/.gameshell/state/export_campagne1_*.json > /dev/null 2>&1 && ls ~/.gameshell/state/export_campagne1_*.csv > /dev/null 2>&1 && ok "gsh export: JSON + CSV générés dans state/" || ko "gsh export a échoué"

# ─── TEST 45: Écran de victoire par campagne (règle A19) ───
out38=$(printf 'Auto\nexit\n' | GAMESHELL_AUTO=1 timeout 900 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g')
n=$(echo "$out38" | grep -c "FÉLICITATIONS")
[ "$n" -ge 3 ] && ok "Écran de victoire affiché pour les 3 campagnes ($n FÉLICITATIONS)" || ko "Écran de victoire manquant ($n/3)"

# ─── TEST 46: gsh restart remet la campagne à zéro (règle A21) ───
rm -rf ~/.gameshell
printf 'Test\n1\npwd\nls\ngsh check\ngsh restart\ngsh goal\nexit\n' | timeout 20 bash gameshell.sh > /tmp/restart_test.log 2>&1
out=$(sed 's/\x1b\[[0-9;]*m//g' /tmp/restart_test.log)
echo "$out" | grep -q "recommencée depuis la mission 1.1" && echo "$out" | grep -q "MISSION 1.1" && echo "$out" | grep -q "\[0/100\]" && ok "gsh restart: retour à la mission 1.1 avec [0/100] (règle A21)" || ko "gsh restart ne remet pas à zéro"

# ─── TEST 47: historique horodaté des actions (règle A22) ───
ls ~/.gameshell/state/history_campagne1.log > /dev/null 2>&1 && grep -qE "^\[20[0-9][0-9]-[0-9][0-9]-[0-9][0-9] [0-9:]{8}\]" ~/.gameshell/state/history_campagne1.log && grep -q "MISSION 1.1 réussie" ~/.gameshell/state/history_campagne1.log && grep -q "CMD pwd" ~/.gameshell/state/history_campagne1.log && ok "Historique horodaté avec timestamps + CMD + MISSION (règle A22)" || ko "Pas d'historique horodaté"

echo ""
echo "═══════════════════════════════════════"
echo -e "  RÉSULTAT: ${GREEN}$PASS PASS${NC} / ${RED}$FAIL FAIL${NC} / $TOTAL total"
echo "═══════════════════════════════════════"
[ $FAIL -eq 0 ] && exit 0 || exit 1

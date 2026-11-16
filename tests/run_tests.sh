#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  SUITE DE TESTS AUTOMATISÉS — Moteur GameShell            ║
# ║  Teste les cas limites et fige le fonctionnement moteur   ║
# ╚══════════════════════════════════════════════════════════╝
# Usage: ./tests/run_tests.sh
# Les tests tournent dans un GAMESHELL_HOME TEMPORAIRE (audit P0.5): la vraie installation
# ~/.gameshell (sauvegardes, historiques, exports) n'est jamais touchée.

set -e
cd "$(dirname "$0")/.."

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

PASS=0
FAIL=0
TOTAL=0
LAST_OUT=""

ok()   { PASS=$((PASS+1)); TOTAL=$((TOTAL+1)); echo -e "${GREEN}✅ PASS${NC} ($TOTAL) $1"; }
ko()   {
    FAIL=$((FAIL+1)); TOTAL=$((TOTAL+1)); echo -e "${RED}❌ FAIL${NC} ($TOTAL) $1"
    # P2.2: en cas d'échec, montrer la fin de la sortie (timeout, panic...)
    if [ -s "$LOGDIR/last.out" ]; then tail -8 "$LOGDIR/last.out" | sed 's/^/      │ /'; fi
}
info() { echo -e "${YELLOW}ℹ️  $1${NC}"; }

export GAMESHELL_HOME="$(mktemp -d "${TMPDIR:-/tmp}/gsh_tests.XXXXXX")"
trap 'rm -rf "$GAMESHELL_HOME"' EXIT
LOGDIR="$GAMESHELL_HOME.logs"
mkdir -p "$LOGDIR"
trap 'rm -rf "$GAMESHELL_HOME" "$LOGDIR"' EXIT
STATE="$GAMESHELL_HOME/state"

# Remet l'installation de test à zéro (uniquement le dossier temporaire)
fresh() {
    case "$GAMESHELL_HOME" in
        */gsh_tests.*) rm -rf "${GAMESHELL_HOME:?}"/* "${GAMESHELL_HOME:?}"/.[!.]* 2>/dev/null || true ;;
        *) echo "GAMESHELL_HOME inattendu: $GAMESHELL_HOME" >&2; exit 1 ;;
    esac
}

strip() { sed 's/\x1b\[[0-9;?]*[a-zA-Z]//g'; }

info "Build + archive..."
cargo build --release 2>/dev/null
bash make-archive.sh > /dev/null 2>&1

# Lance une session de test (choix de campagne "1" inséré après le nom)
run_game() {
    fresh
    LAST_OUT=$(echo -e "$1" | sed '1a 1' | timeout 30 bash gameshell.sh 2>&1 | strip)
    echo "$LAST_OUT" > "$LOGDIR/last.out"
    echo "$LAST_OUT"
}
# Session sans remise à zéro (reprise de sauvegarde)
run_keep() {
    LAST_OUT=$(printf "$1" | timeout 30 bash gameshell.sh 2>&1 | strip)
    echo "$LAST_OUT" > "$LOGDIR/last.out"
    echo "$LAST_OUT"
}

HOST_USER="$(id -un)"

echo ""
echo "═══════════════════════════════════════"
echo "  TESTS MOTEUR GAMESHELL"
echo "═══════════════════════════════════════"
echo ""

# ─── TEST 1: 100 missions chargées ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "100 missions" && ok "100 missions en dur chargées (20 chapitres x 5, 20/acte)" || ko "100 missions chargées"

# ─── TEST 2: Pas d'auto-validation (A3) ───
out=$(run_game "Test\npwd\nls\nexit\n")
echo "$out" | grep -q "✅ Mission réussie" && ko "Pas d'auto-validation (mission réussie sans gsh check)" || ok "Pas d'auto-validation (règle A3)"

# ─── TEST 3: gsh check valide si commandes faites ───
out=$(run_game "Test\npwd\nls\ngsh check\nexit\n")
echo "$out" | grep -q "✅ Mission réussie" && ok "gsh check valide mission 1.1 (pwd+ls)" || ko "gsh check valide mission 1.1"

# ─── TEST 4: ls seul ne valide PAS (il manque pwd) ───
out=$(run_game "Test\nls\ngsh check\nexit\n")
echo "$out" | grep -q "❌ Mission non validée" && ok "ls seul → ❌ Mission non validée (pwd manquant)" || ko "ls seul devrait échouer"

# ─── TEST 5: Check matche commande de base (A4) ───
out=$(run_game "Test\npwd\nls -la\ngsh check\nexit\n")
echo "$out" | grep -q "✅ Mission réussie" && ok "ls -la matche le check 'ls' (commande de base, A4)" || ko "ls -la devrait matcher ls"

# ─── TEST 6: Historique reset entre sessions (A2) ───
fresh
printf 'Test\n1\npwd\nls\ngsh check\nexit\n' | timeout 30 bash gameshell.sh > /dev/null 2>&1
out=$(run_keep 'Test2\n1\nls\ngsh check\nexit\n')
echo "$out" | grep -q "❌ Mission non validée" && ok "Historique reset entre sessions (A2)" || ko "Fuite d'historique entre sessions!"

# ─── TEST 7: Isolation chemins — ls / montre le vaisseau ───
out=$(run_game "Test\nls /\nexit\n")
echo "$out" | grep -q "pont" && echo "$out" | grep -q "soutes" && ! echo "$out" | grep -qE "^(bin|usr|etc|proc|lib|dev)$" && ok "ls / = vaisseau (pont, soutes...), pas le vrai système (A1)" || ko "ls / devrait montrer le vaisseau"

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
echo "$out" | grep -qi "not found\|introuvable" && ok "Commande inconnue → erreur propre" || ko "Commande inconnue: $(echo "$out" | grep foobar | head -1)"

# ─── TEST 14: cd chemin invalide → erreur bash ───
out=$(run_game "Test\ncd /nonexistent123\npwd\nexit\n")
echo "$out" | grep -qi "No such file\|Aucun fichier" && echo "$out" | grep -q "Test@orion7:~" && ok "cd invalide → erreur + reste sur ~" || ko "cd invalide devrait échouer proprement"

# ─── TEST 15: gsh check sans rien faire → ❌ ───
out=$(run_game "Test\ngsh check\nexit\n")
echo "$out" | grep -q "❌ Mission non validée" && ok "gsh check sans rien → ❌ Mission non validée" || ko "gsh check vide devrait échouer"

# ─── TEST 16: gsh goal = même affichage que show_mission ───
out=$(run_game "Test\ngsh goal\nexit\n")
echo "$out" | grep -q "┏━ MISSION 1.1" && ok "gsh goal affiche le cadre ┏━ MISSION (C11)" || ko "gsh goal devrait afficher le cadre mission"

# ─── TEST 17: gsh help affiché au lancement ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "gsh help" && ok "gsh help affiché au lancement (B2)" || ko "gsh help devrait être auto-affiché"

# ─── TEST 18: man = compétence de départ ───
out=$(run_game "Test\ngsh skills\nexit\n")
echo "$out" | grep -q "man" && ok "man = compétence de départ dans gsh skills (C2)" || ko "man devrait être compétence de départ"

# ─── TEST 19: Bloom en français ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "Se souvenir" && ok "Bloom affiché en français (Se souvenir, B8)" || ko "Bloom devrait être en français"

# ─── TEST 20: Footer mission = gsh help seul ───
out=$(run_game "Test\nexit\n")
line=$(echo "$out" | grep "🏆" | head -1)
echo "$line" | grep -q "gsh help" && ! echo "$line" | grep -q "gsh check" && ok "Footer mission = 'gsh help' seul (B7)" || ko "Footer devrait être 'gsh help' seul"

# ─── TEST 21: Carte vue de dessus SUPPRIMÉE (C6) ───
out=$(run_game "Test\nexit\n")
! echo "$out" | grep -q "VUE DE DESSUS" && ok "Plus de vue de dessus affichée (C6)" || ko "La vue de dessus devrait être supprimée"

# ─── TEST 22: Double définition commande (roleplay + technique) ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "🚀" && echo "$out" | grep -q "💻" && ok "Commandes avec 🚀 roleplay + 💻 technique (C7)" || ko "Double définition manquante"

# ─── TEST 23: Mission 2 s'affiche après validation 1.1 ───
out=$(run_game "Test\npwd\nls\ngsh check\nexit\n")
echo "$out" | grep -q "MISSION 1.2" && ok "Mission suivante (1.2) affichée après check réussi" || ko "Mission 1.2 devrait suivre"

# ─── TEST 24: exit quitte proprement ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "Déconnexion" && ok "exit → déconnexion propre" || ko "exit devrait quitter proprement"

# ─── TEST 25: gsh reset (monde régénéré, retour cabine) ───
out=$(run_game "Test\ncd /pont\ngsh reset\npwd\nexit\n")
echo "$out" | grep -q "réinitialisée" && echo "$out" | grep -q "^Test@orion7:~\\$ \[0/100\] ~$" && ok "gsh reset: mission réinitialisée + retour à la cabine (C12, P1.6)" || ko "gsh reset devrait réinitialiser et ramener en cabine"

# ─── TEST 26: Playthrough 10 premières missions ───
fresh
LAST_OUT=$(printf 'Test\n1\npwd\nls\ngsh check\ncat bienvenue.txt\ngsh check\nORION-7-ACCUEIL\ncd /communications/messages\ncat dernier_message.txt\ngsh check\ngsh code ANDROMEDE-7G\nhostname\nwhoami\nid\ngsh check\ndate\ngsh check\ntouch incident_15jan.txt\nls\ngsh check\nmkdir -p rapports/2024/janvier\ngsh check\ncp incident_15jan.txt /soutes/\ngsh check\nmv incident_15jan.txt rapports/2024/janvier/\ngsh check\nrm temp1.txt temp2.txt temp3.txt\ngsh check\nexit\n' | timeout 60 bash gameshell.sh 2>&1 | strip)
echo "$LAST_OUT" > "$LOGDIR/last.out"
n=$(echo "$LAST_OUT" | grep -cE "✅ (Code correct! )?Mission réussie")
[ "$n" -ge 10 ] && ok "Playthrough 10 premières missions ($n/10 réussies)" || ko "Playthrough 10 missions: seulement $n/10"

# ─── TEST 27: EOF (Ctrl+D) = sortie propre ───
fresh
out=$(run_keep 'Test\n1\npwd\n')
echo "$out" | grep -q "EOF\|Déconnexion" && ok "EOF (Ctrl+D) → déconnexion propre (A13)" || ko "EOF devrait déconnecter proprement"

# ─── TEST 28: Input très long tronqué (anti freeze) ───
fresh
LAST_OUT=$(printf 'Test\n1\nls %s\npwd\nexit\n' "$(python3 -c "print('a'*8000)")" | timeout 30 bash gameshell.sh 2>&1 | strip)
echo "$LAST_OUT" | grep -q "Test@orion7" && ! echo "$LAST_OUT" | grep -q "panicked" && echo "$LAST_OUT" | grep -q "tronqué" && ok "Input 8000 chars → tronqué, pas de crash (A13)" || ko "Input long devrait être tronqué sans crash"

# ─── TEST 29: gsh goal suggéré après 2 échecs ───
fresh
out=$(run_keep 'Test\n1\ngsh check\ngsh check\nexit\n')
echo "$out" | grep -q "gsh goal" && ok "Après 2 échecs, suggère 'gsh goal' (A14)" || ko "Devrait suggérer gsh goal après 2 échecs"

# ─── TEST 30: Progression dans le prompt [N/100] ───
fresh
out=$(run_keep 'Test\n1\nexit\n')
echo "$out" | grep -qE "\[0/100\]" && ok "Prompt affiche [N/100] progression (A14)" || ko "Prompt devrait afficher [N/100]"

# ─── TEST 31: Pipe multiple fonctionne ───
out=$(run_game 'Test\ncat /pont/journaux-de-bord/dernier_rapport.txt | grep ALERTE | wc -l\nexit\n')
echo "$out" | grep -qE "^Test@orion7:~\\$ \[0/100\] [0-9]+$" && ok "Pipe multiple (cat | grep | wc) fonctionne" || ko "Pipe multiple devrait fonctionner"

# ─── TEST 32: Redirection > dans le vaisseau ───
out=$(run_game 'Test\necho hello > /soutes/test_gsh.txt\ncat /soutes/test_gsh.txt\nexit\n')
echo "$out" | grep -q "hello" && [ -f "$GAMESHELL_HOME/vaisseau/soutes/test_gsh.txt" ] && ok "Redirection > fonctionne dans le vaisseau" || ko "Redirection > devrait fonctionner"

# ─── TEST 33: Tests de conformité cargo ───
if cargo test --release > "$LOGDIR/cargo_test.log" 2>&1 && grep -q "test result: ok" "$LOGDIR/cargo_test.log"; then
    ok "cargo test: conformité (A4/A12/A15/A17, C3/C4/C5/C9/C13, analyseur, cadres)"
else
    cp "$LOGDIR/cargo_test.log" "$LOGDIR/last.out"; ko "cargo test conformité a échoué"
fi

# ─── TEST 34: id/uname interceptés, jamais d'infos de l'hôte (A16) ───
out=$(run_game "Test\nid\nuname -a\nexit\n")
echo "$out" | grep -q "uid=1000(capitaine)" && echo "$out" | grep -q "Linux orion7 6.6" && ! echo "$out" | grep -qE "\($HOST_USER\)|$(hostname)" && ok "id/uname → identité vaisseau (capitaine/orion7), pas l'hôte (A16)" || ko "id/uname ne doivent pas révéler l'hôte"

# ─── TEST 35: Flèches en mode pipe = séquences ANSI nettoyées (A11) ───
fresh
out=$(run_keep 'Test\n1\n\x1b[D\x1b[Cpwd\x1b[A\x1b[B\nls\ngsh check\nexit\n')
echo "$out" | grep -q "✅ Mission réussie" && ok "Séquences ANSI (flèches) nettoyées en mode non-tty, commande intacte" || ko "Les flèches ne devraient rien insérer dans la commande"

# ─── TEST 36: Compétence débloquée = commande de LA mission (A15) ───
out=$(run_game "Test\npwd\nls\ngsh check\ncat bienvenue.txt\ngsh check\nORION-7-ACCUEIL\nexit\n")
echo "$out" | grep -q "🔓 Compétence: pwd" && echo "$out" | grep -q "🔓 Compétence: cat" && ok "Skills débloquées = commandes des missions (pwd, cat — A15)" || ko "Skill débloquée incohérente avec la mission"

# ─── TEST 37: PNJ noms SF dans les missions (C13) ───
out=$(run_game "Test\nexit\n")
echo "$out" | grep -q "Zyra-7" && ! echo "$out" | grep -qE "\b(Marie|Sophie|Jean|Kowalski|Galactica)\b" && ok "PNJ avec noms SF (Zyra-7...), jamais Marie/Kowalski/Galactica (C13)" || ko "PNJ non conformes"

# ─── TEST 38: AUTO — les 300 missions résolues par leurs SEULES commandes requises ───
fresh
LAST_OUT=$(printf 'Auto\nexit\n' | GAMESHELL_AUTO=1 timeout 900 bash gameshell.sh 2>&1 | strip)
out38="$LAST_OUT"
echo "$LAST_OUT" > "$LOGDIR/last.out"
echo "$LAST_OUT" | grep -q "AUTO campagne 1 (Pirates de l'Espace): 100/100" && echo "$LAST_OUT" | grep -q "AUTO campagne 2 (Japon Féodal): 100/100" && echo "$LAST_OUT" | grep -q "AUTO campagne 3 (Nexus Réseau): 100/100" && ok "AUTO: 300/300 missions résolues par leurs seules commandes requises (P2.1)" || ko "AUTO: $(echo "$LAST_OUT" | grep -o 'AUTO campagne [0-9][^:]*: [0-9/]*' | tr '\n' ' ')"

# ─── TEST 39: Campagne 2 (Japon Féodal) (C14) ───
fresh
out=$(run_keep 'Samoura\n2\nhostname\nwhoami\nexit\n')
echo "$out" | grep -q "Japon Féodal" && echo "$out" | grep -q "100 missions" && echo "$out" | grep -q "Samoura@shiro:~" && echo "$out" | grep -qE "seigneur$" && ok "Campagne 2: bannière Japon Féodal, prompt @shiro, whoami=seigneur, 100 missions" || ko "Campagne 2 devrait être sélectionnable (Japon Féodal/shiro/seigneur)"

# ─── TEST 40: EDGE — cas limites sur les 300 missions ───
fresh
LAST_OUT=$(printf 'Edge\nexit\n' | GAMESHELL_EDGE=1 timeout 900 bash gameshell.sh 2>&1 | strip)
echo "$LAST_OUT" > "$LOGDIR/last.out"
echo "$LAST_OUT" | grep -q "EDGE campagne 1 (Pirates de l'Espace): 100/100 missions OK, 0 problèmes" && echo "$LAST_OUT" | grep -q "EDGE campagne 2 (Japon Féodal): 100/100 missions OK, 0 problèmes" && echo "$LAST_OUT" | grep -q "EDGE campagne 3 (Nexus Réseau): 100/100 missions OK, 0 problèmes" && ok "EDGE: 300/300 (check vide, garbage, echo des cibles, ls /armes, mauvais code, reset)" || ko "EDGE: $(echo "$LAST_OUT" | grep -o 'EDGE campagne [0-9][^:]*: [0-9/]* missions OK, [0-9]* problèmes' | tr '\n' ' ')"

# ─── TEST 41: Campagne 3 (Nexus Réseau) (C14) ───
fresh
out=$(run_keep 'Net\n3\nhostname\nwhoami\nexit\n')
echo "$out" | grep -q "Nexus" && echo "$out" | grep -q "100 missions" && echo "$out" | grep -q "Net@nexus:~" && echo "$out" | grep -qE "netrunner$" && ok "Campagne 3: bannière Nexus, prompt @nexus, whoami=netrunner, 100 missions" || ko "Campagne 3 devrait être sélectionnable (Nexus/nexus/netrunner)"

# ─── TEST 42: TORTURE — 300 missions torturées + isolation hôte sur l'exécution réelle ───
fresh
LAST_OUT=$(printf 'T\nexit\n' | GAMESHELL_TORTURE=1 timeout 1200 bash gameshell.sh 2>&1 | strip)
echo "$LAST_OUT" > "$LOGDIR/last.out"
ok42=1
for c in "1 (Pirates de l'Espace)" "2 (Japon Féodal)" "3 (Nexus Réseau)"; do
    echo "$LAST_OUT" | grep -q "TORTURE campagne $c: 100/100 OK, 0 problèmes" || ok42=0
done
[ "$ok42" = "1" ] && ok "TORTURE: 300/300 (check vide/partiel/typo, 2 mauvais codes, isolation hôte réelle)" || ko "TORTURE: $(echo "$LAST_OUT" | grep -o 'TORTURE campagne [0-9][^:]*: [0-9/]* OK, [0-9]* problèmes' | tr '\n' ' ')"

# ─── TEST 43: Sauvegarde/reprise par campagne et par joueur (A18, P1.10) ───
fresh
printf 'Test\n1\npwd\nls\ngsh check\nexit\n' | timeout 30 bash gameshell.sh > /dev/null 2>&1
out=$(run_keep 'Test\n1\ngsh goal\nexit\n')
echo "$out" | grep -q "MISSION 1.2" && echo "$out" | grep -q "\[1/100\]" && ok "Reprise: session 2 reprend à la mission 1.2 avec [1/100] (A18)" || ko "La sauvegarde/reprise ne fonctionne pas"
out=$(run_keep 'Autre\n1\nexit\n')
[ -f "$STATE/save_campagne1_test.json" ] && echo "$out" | grep -q "\[0/100\]" && ok "Sauvegarde par joueur: save_campagne1_test.json, un autre élève repart de zéro (P1.10)" || ko "Pas de sauvegarde par joueur"

# ─── TEST 44: gsh export enseignant JSON+CSV valides (A20, P1.8) ───
fresh
LAST_OUT=$(printf 'Eve "le pirate"\n1\npwd\nls\ngsh check\ngsh export\nexit\n' | timeout 30 bash gameshell.sh 2>&1 | strip)
J=$(ls "$STATE"/export_campagne1_*.json 2>/dev/null | head -1)
C=$(ls "$STATE"/export_campagne1_*.csv 2>/dev/null | head -1)
echo "$LAST_OUT" | grep -q "Export effectué" && [ -n "$J" ] && [ -n "$C" ] && python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); assert d["joueur"]=="Eve \"le pirate\""; assert len(d["missions"])==100; assert d["missions_reussies"]==1' "$J" && ok "gsh export: JSON valide (nom avec guillemets) + CSV dans state/" || ko "gsh export a échoué ou JSON invalide"

# ─── TEST 45: Écran de victoire par campagne (A19) ───
n=$(echo "$out38" | grep -c "FÉLICITATIONS")
[ "$n" -ge 3 ] && ok "Écran de victoire affiché pour les 3 campagnes ($n FÉLICITATIONS)" || ko "Écran de victoire manquant ($n/3)"

# ─── TEST 46: gsh restart remet la campagne à zéro (A21) ───
fresh
out=$(run_keep 'Test\n1\npwd\nls\ngsh check\ngsh restart\ngsh goal\nexit\n')
echo "$out" | grep -q "recommencée depuis la mission 1.1" && echo "$out" | grep -q "MISSION 1.1" && echo "$out" | grep -q "\[0/100\]" && ok "gsh restart: retour à la mission 1.1 avec [0/100] (A21)" || ko "gsh restart ne remet pas à zéro"

# ─── TEST 47: historique horodaté des actions (A22) ───
H="$STATE/history_campagne1_test.log"
[ -f "$H" ] && grep -qE "^\[20[0-9][0-9]-[0-9][0-9]-[0-9][0-9] [0-9:]{8}\]" "$H" && grep -q "MISSION 1.1 réussie" "$H" && grep -q "CMD pwd" "$H" && ok "Historique horodaté avec timestamps + CMD + MISSION (A22)" || ko "Pas d'historique horodaté"

# ─── TEST 48: évasion impossible — cd /.., cat /../../etc/passwd, $USER, ls -l (A1/A16, P0.1) ───
out=$(run_game 'Test\ncd /..\ncd ../../../..\npwd\ncat /../../../../etc/passwd\necho USER=$USER HOME=$HOME\nls -l ~\nexit\n')
echo "$out" | grep -q "\] /$" && echo "$out" | grep -q "USER=capitaine HOME=/cabines/capitaine" && echo "$out" | grep -q "^capitaine:x:1000" && ! echo "$out" | grep -qw "$HOST_USER" && ok "Aucune évasion: cd /.. reste à /, /etc/passwd du jeu, USER=capitaine (P0.1)" || ko "Évasion du monde ou fuite de l'utilisateur hôte"

# ─── TEST 49: commandes système simulées, jamais exécutées sur l'hôte (P0.2) ───
out=$(run_game 'Test\nsudo useradd -m gsh_matelot_test\nid gsh_matelot_test\nsudo ufw enable\nsudo ufw status\nkill 9999\nps aux | grep backdoor\nexit\n')
! grep -q gsh_matelot_test /etc/passwd && echo "$out" | grep -q "uid=.*(gsh_matelot_test)" && echo "$out" | grep -q "Status: active" && ! echo "$out" | grep -q "backdoor --c2" && ok "sudo useradd/ufw/kill simulés dans le monde, hôte intact (P0.2)" || ko "Commandes système non simulées"

# ─── TEST 50: commandes interceptées compatibles pipes/redirections (P1.4, P1.5) ───
out=$(run_game 'Test\npwd > position.txt\ncat position.txt\nwhoami | tr a-z A-Z\necho "a  b" > espaces.txt\ncat espaces.txt\ncd /pont && ls\nexit\n')
echo "$out" | grep -q "\] /cabines/capitaine$" && echo "$out" | grep -q "CAPITAINE" && echo "$out" | grep -q "a  b" && echo "$out" | grep -q "Test@orion7:/pont" && ok "pwd > fichier, whoami | tr, guillemets et cd composé fonctionnent (P1.4/P1.5)" || ko "Pipes/redirections des commandes interceptées"

# ─── TEST 51: validation stricte — echo des commandes et commandes en échec ne valident pas (P1.1) ───
out=$(run_game 'Test\necho slsx pwdx\necho pwd ls\ngsh check\nls /fichier_inexistant\npwd\ngsh check\nexit\n')
[ "$(echo "$out" | grep -c "❌ Mission non validée")" = "2" ] && ! echo "$out" | grep -q "✅ Mission réussie" && ok "echo pwd ls / ls en échec ne valident pas la mission 1.1 (P1.1)" || ko "Validation trop permissive"

# ─── TEST 52: phase 2 non bloquante — le shell reste utilisable, gsh code (P1.2) ───
out=$(run_game 'Test\npwd\nls\ngsh check\ncat bienvenue.txt\ngsh check\ngsh help\ncat bienvenue.txt\ngsh code MAUVAIS\ngsh code ORION-7-ACCUEIL\nexit\n')
echo "$out" | grep -q "Code incorrect" && echo "$out" | grep -q "Code correct" && echo "$out" | grep -q "MISSION 1.3" && [ "$(echo "$out" | grep -c "COMMANDES SHELL")" -ge 2 ] && ok "Phase 2: gsh help/cat restent utilisables, gsh code valide (P1.2)" || ko "Phase 2 bloquante"

# ─── TEST 53: gsh avec espaces multiples (A5, P1.14) ───
out=$(run_game 'Test\npwd\nls\ngsh   check\nexit\n')
echo "$out" | grep -q "✅ Mission réussie" && ok "gsh   check (espaces multiples) reconnu (A5)" || ko "gsh avec espaces multiples non reconnu"

echo ""
echo "═══════════════════════════════════════"
echo -e "  RÉSULTAT: ${GREEN}$PASS PASS${NC} / ${RED}$FAIL FAIL${NC} / $TOTAL total"
echo "═══════════════════════════════════════"
[ $FAIL -eq 0 ] && exit 0 || exit 1

#!/bin/bash
# Playthrough exhaustif des 100 missions — les joue RÉELLEMENT via le mode auto (GAMESHELL_AUTO=1)
# et rapporte le résultat. Identifie les missions qui échouent.
#
# Sûreté: tout s'exécute dans un dossier GAMESHELL_HOME temporaire supprimé à la fin —
# la vraie installation ~/.gameshell (sauvegardes, historiques, exports) n'est JAMAIS touchée.
# Règle A25: les tests ne touchent jamais les données réelles.
set -e
cd "$(dirname "$0")/.."
cargo build --release 2>/dev/null
bash make-archive.sh > /dev/null 2>&1

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/gameshell_playthrough.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

export GAMESHELL_HOME="$TMP_DIR/home"
mkdir -p "$GAMESHELL_HOME"
cp gameshell.sh "$TMP_DIR/gameshell.sh"

LOG="$TMP_DIR/playthrough_100.log"
printf 'Auto\nexit\n' | GAMESHELL_AUTO=1 timeout 180 bash "$TMP_DIR/gameshell.sh" > "$LOG" 2>&1 || true
sed 's/\x1b\[[0-9;]*m//g' "$LOG" | grep -E "AUTO:|❌|FÉLICITATIONS" | tail -20
echo ""
if grep -q "AUTO: 100/100 missions résolues" "$LOG"; then
    echo "✅ PLAYTHROUGH 100/100 missions réussies"
    exit 0
else
    echo "❌ Playthrough incomplet: $(grep -o 'AUTO: [0-9/]*' "$LOG" | tail -1)"
    exit 1
fi

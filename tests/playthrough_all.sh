#!/bin/bash
# Playthrough exhaustif des 100 missions — les joue RÉELLEMENT via le mode auto (GAMESHELL_AUTO=1)
# et rapporte le résultat. Identifie les missions qui échouent.
set -e
cd "$(dirname "$0")/.."
cargo build --release 2>/dev/null
bash make-archive.sh > /dev/null 2>&1

rm -rf ~/.gameshell
cp gameshell.sh /tmp/
cd /tmp

LOG=/tmp/playthrough_100.log
printf 'Auto\nexit\n' | GAMESHELL_AUTO=1 timeout 180 bash gameshell.sh > "$LOG" 2>&1 || true
sed 's/\x1b\[[0-9;]*m//g' "$LOG" | grep -E "AUTO:|❌|FÉLICITATIONS" | tail -20
echo ""
if grep -q "AUTO: 100/100 missions résolues" "$LOG"; then
    echo "✅ PLAYTHROUGH 100/100 missions réussies"
    exit 0
else
    echo "❌ Playthrough incomplet: $(grep -o 'AUTO: [0-9/]*' "$LOG" | tail -1)"
    exit 1
fi

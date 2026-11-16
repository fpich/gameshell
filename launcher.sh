#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  LAUNCHER GAMESHELL — lance le jeu installé              ║
# ║  L'isolation est assurée par le jeu lui-même: chaque     ║
# ║  commande s'exécute dans un bac à sable bubblewrap où le ║
# ║  monde de la campagne est la racine « / ».               ║
# ╚══════════════════════════════════════════════════════════╝

GAMESHELL_HOME="${GAMESHELL_HOME:-$HOME/.gameshell}"
BIN="$GAMESHELL_HOME/bin/gameshell"
export GAMESHELL_HOME
mkdir -p "$GAMESHELL_HOME/state"

# P0.4: plus de mode chroot. En root, le jeu refuse de démarrer sans bac à sable:
# une commande ratée pourrait sinon toucher la vraie machine.
if [ "$(id -u)" = "0" ] && ! command -v bwrap >/dev/null 2>&1; then
    echo "❌ GameShell ne se lance pas en root sans bubblewrap." >&2
    echo "   Lancez-le avec un compte élève normal, ou installez bubblewrap (apt install bubblewrap)." >&2
    exit 1
fi
if [ "$(id -u)" = "0" ]; then
    echo "⚠️  Vous lancez GameShell en root: préférez un compte élève normal." >&2
fi

if [ ! -x "$BIN" ]; then
    echo "❌ Jeu non installé dans $GAMESHELL_HOME (relancez ./gameshell.sh --install)" >&2
    exit 1
fi

# P1.13: les options (--campaign, --locked, --name) sont transmises au jeu
exec "$BIN" "$@"

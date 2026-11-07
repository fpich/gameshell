#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  LAUNCHER GAMESHELL - Lance le jeu dans le vaisseau       ║
# ║  Isolation: chroot dans ~/.gameshell/vaisseau si sudo,   ║
# ║  sinon HOME isolé avec mapping des chemins               ║
# ╚══════════════════════════════════════════════════════════╝

GAMESHELL_HOME="${GAMESHELL_HOME:-$HOME/.gameshell}"
VAISSEAU="$GAMESHELL_HOME/vaisseau"
BIN="$GAMESHELL_HOME/bin/gameshell"

# Le home du joueur dans le vaisseau
export GAMESHELL_HOME
export GAMESHELL_HISTORY="$GAMESHELL_HOME/state/commands.log"
mkdir -p "$GAMESHELL_HOME/state"

# Mode 1: chroot si on a les droits root (vraie isolation)
if [ "$(id -u)" = "0" ] && [ -d "$VAISSEAU" ]; then
    # Copier le binaire dans le chroot
    mkdir -p "$VAISSEAU/bin" "$VAISSEAU/dev" "$VAISSEAU/proc" "$VAISSEAU/tmp"
    cp "$BIN" "$VAISSEAU/bin/gameshell" 2>/dev/null || true
    # Monter /dev et /proc si possible
    mount -t proc proc "$VAISSEAU/proc" 2>/dev/null || true
    mount --bind /dev "$VAISSEAU/dev" 2>/dev/null || true
    
    echo "🔒 Mode chroot activé - isolation complète du vaisseau!"
    exec chroot "$VAISSEAU" /bin/gameshell --chroot
fi

# Mode 2: isolation par HOME + mapping (sans root)
# Le home du joueur = cabine du capitaine dans le vaisseau
export HOME="$VAISSEAU/cabines/capitaine"
cd "$HOME" || exit 1

# Créer les liens symboliques pour navigation immersive
ln -sf ../../pont pont 2>/dev/null
ln -sf ../../securite securite 2>/dev/null
ln -sf ../../reacteur reacteur 2>/dev/null
ln -sf ../../soutes soutes 2>/dev/null
ln -sf ../../hangar hangar 2>/dev/null
ln -sf ../../navigation navigation 2>/dev/null
ln -sf ../../communications communications 2>/dev/null
ln -sf ../../armes armes 2>/dev/null
ln -sf ../../salle-des-machines machines 2>/dev/null
ln -sf ../../archives archives 2>/dev/null

exec "$BIN"

#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  🚀 GameShell - Les Pirates de l'Espace - Installation   ║
# ║     Installation 100% locale, fonctionne sans internet   ║
# ╚══════════════════════════════════════════════════════════╝
#
# Usage:
#   ./install.sh              # Installe dans ~/.gameshell/ (utilisateur)
#   sudo ./install.sh         # Installe dans /opt/gameshell/ (système)
#   ./install.sh /chemin/     # Installe dans un dossier personnalisé

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Déterminer le dossier d'installation
if [ -n "$1" ]; then
    INSTALL_DIR="$1"
elif [ "$(id -u)" = "0" ]; then
    INSTALL_DIR="/opt/gameshell"
else
    INSTALL_DIR="$HOME/.gameshell"
fi

echo "╔══════════════════════════════════════════════════════════╗"
echo "║     🚀 GameShell - Installation locale                   ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""
echo "📁 Dossier d'installation: $INSTALL_DIR"
echo ""

# 1. Compiler le binaire si nécessaire
if [ ! -f "$SCRIPT_DIR/target/release/gameshell" ]; then
    echo "🔨 Compilation du binaire Rust..."
    cd "$SCRIPT_DIR"
    cargo build --release
fi

# 2. Créer la structure
mkdir -p "$INSTALL_DIR"/{bin,world,state}

# 3. Copier le binaire
echo "📦 Copie du binaire..."
cp "$SCRIPT_DIR/target/release/gameshell" "$INSTALL_DIR/bin/gameshell"
chmod +x "$INSTALL_DIR/bin/gameshell"

# 4. Copier le monde du jeu (en préservant les permissions)
echo "🌍 Copie du monde Orion-7..."
cp -r "$SCRIPT_DIR/world/." "$INSTALL_DIR/world/"

# 5. Corriger les permissions dans le monde
chmod 600 "$INSTALL_DIR/world/home/etudiant/documents/secret.txt" 2>/dev/null || true
chmod 700 "$INSTALL_DIR/world/home/etudiant/projets/script.sh" 2>/dev/null || true
chmod 755 "$INSTALL_DIR/world/srv/station-spatiale/scripts/"*.sh 2>/dev/null || true

# 6. Créer le launcher
echo "🚀 Création du launcher..."
cat > "$INSTALL_DIR/gameshell" << LAUNCHER
#!/bin/bash
# GameShell - Launcher - Lance le jeu dans le monde isolé
export GAMESHELL_HOME="$INSTALL_DIR"
export GAMESHELL_HISTORY="$INSTALL_DIR/state/commands.log"
export HOME="$INSTALL_DIR/world/home/etudiant"
mkdir -p "$INSTALL_DIR/state"

# Créer les liens symboliques dans le home du joueur pour un accès facile
cd "\$HOME" || exit 1
ln -sf ../../srv/station-spatiale station 2>/dev/null
ln -sf ../../var var 2>/dev/null
ln -sf ../../etc etc 2>/dev/null

# Lancer le binaire
exec "$INSTALL_DIR/bin/gameshell" "\$@"
LAUNCHER
chmod +x "$INSTALL_DIR/gameshell"

# 7. Créer un lien dans le PATH (si possible)
if [ "$(id -u)" = "0" ] && [ -d /usr/local/bin ]; then
    ln -sf "$INSTALL_DIR/gameshell" /usr/local/bin/gameshell
    echo "🔗 Lien créé: /usr/local/bin/gameshell"
    LAUNCH_CMD="gameshell"
elif [ -d "$HOME/.local/bin" ]; then
    mkdir -p "$HOME/.local/bin"
    ln -sf "$INSTALL_DIR/gameshell" "$HOME/.local/bin/gameshell"
    echo "🔗 Lien créé: ~/.local/bin/gameshell (ajoutez ~/.local/bin à votre PATH)"
    LAUNCH_CMD="~/.local/bin/gameshell"
else
    LAUNCH_CMD="$INSTALL_DIR/gameshell"
fi

echo ""
echo "╔══════════════════════════════════════════════════════════╗"
echo "║  ✅ Installation terminée!                               ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""
echo "🚀 Pour jouer:"
echo "   $LAUNCH_CMD"
echo ""
echo "📁 Structure installée:"
echo "   $INSTALL_DIR/"
echo "   ├── bin/gameshell      # binaire du jeu"
echo "   ├── world/             # 🌍 monde isolé Orion-7"
echo "   │   ├── home/etudiant/ # votre home dans le jeu"
echo "   │   ├── srv/station-spatiale/"
echo "   │   └── ..."
echo "   ├── state/             # sauvegarde + historique"
echo "   └── gameshell          # launcher"
echo ""
echo "💡 Le jeu fonctionne 100% hors ligne. Aucune connexion internet requise."
echo "💡 Tapez 'gsh help' dans le jeu pour voir les commandes."

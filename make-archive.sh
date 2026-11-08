#!/bin/bash
# Crée gameshell.sh - archive auto-extractible 100% offline
# Le jeu s'installe dans ~/.gameshell/ avec le vaisseau Orion-7

set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo "🔨 Compilation..."
cargo build --release

echo "🚀 Génération du vaisseau Orion-7..."
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

mkdir -p "$TMPDIR/payload/bin"
cp target/release/gameshell "$TMPDIR/payload/bin/"
bash vaisseau-gen.sh espace "$TMPDIR/payload/vaisseau"
bash vaisseau-gen.sh japon "$TMPDIR/payload/monde_japon"

# Copier le lanceur comme "gameshell" à la racine du payload
cp launcher.sh "$TMPDIR/payload/gameshell"
chmod +x "$TMPDIR/payload/gameshell"

# embarquer les générateurs de monde (pour gsh reset par campagne)
cp vaisseau-gen.sh monde-japon-gen.sh "$TMPDIR/payload/"
chmod +x "$TMPDIR/payload/vaisseau-gen.sh" "$TMPDIR/payload/monde-japon-gen.sh"

tar czf "$TMPDIR/payload.tar.gz" -C "$TMPDIR/payload" .

cat > gameshell.sh << 'HEADER'
#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  🚀 GAMESHELL - Les Pirates de l'Espace                  ║
# ║  Jeu RPG éducatif shell - BTS SIO SISR                   ║
# ║  100% offline, monde isolé dans ~/.gameshell/            ║
# ╚══════════════════════════════════════════════════════════╝
set -e

GAMESHELL_HOME="${GAMESHELL_HOME:-$HOME/.gameshell}"
ARCHIVE_LINE=$(awk '/^__ARCHIVE_BELOW__$/ {print NR+1; exit}' "$0")

if [ "$1" = "--install" ] || [ ! -f "$GAMESHELL_HOME/bin/gameshell" ]; then
    echo "📦 Installation d'Orion-7 dans $GAMESHELL_HOME..."
    mkdir -p "$GAMESHELL_HOME"
    tail -n +$ARCHIVE_LINE "$0" | tar xz -C "$GAMESHELL_HOME"
    echo "✅ Installation terminée!"
    echo "🚀 Lancez: $GAMESHELL_HOME/gameshell"
    echo ""
fi

exec "$GAMESHELL_HOME/gameshell" "${@:1}"

__ARCHIVE_BELOW__
HEADER

cat "$TMPDIR/payload.tar.gz" >> gameshell.sh
chmod +x gameshell.sh

echo "✅ gameshell.sh créé ($(du -h gameshell.sh | cut -f1))"
echo "🚀 Usage: ./gameshell.sh"

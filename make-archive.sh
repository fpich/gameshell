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
trap 'rm -rf "$TMPDIR"' EXIT

mkdir -p "$TMPDIR/payload/bin"
cp target/release/gameshell "$TMPDIR/payload/bin/"
bash vaisseau-gen.sh espace "$TMPDIR/payload/vaisseau"
bash vaisseau-gen.sh japon "$TMPDIR/payload/monde_japon"
bash vaisseau-gen.sh reseau "$TMPDIR/payload/monde_reseau"

# Copier le lanceur comme "gameshell" à la racine du payload
cp launcher.sh "$TMPDIR/payload/gameshell"
chmod +x "$TMPDIR/payload/gameshell"

# embarquer les générateurs de monde (pour gsh reset par campagne)
cp vaisseau-gen.sh monde-japon-gen.sh monde-reseau-gen.sh "$TMPDIR/payload/"
chmod +x "$TMPDIR/payload/vaisseau-gen.sh" "$TMPDIR/payload/monde-japon-gen.sh" "$TMPDIR/payload/monde-reseau-gen.sh"

VERSION="$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)-$(sha256sum target/release/gameshell vaisseau-gen.sh monde-japon-gen.sh monde-reseau-gen.sh launcher.sh | sha256sum | cut -c1-12)"
echo "$VERSION" > "$TMPDIR/payload/VERSION"
tar czf "$TMPDIR/payload.tar.gz" -C "$TMPDIR/payload" .

cat > gameshell.sh << 'HEADER'
#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  🚀 GAMESHELL — RPG éducatif shell — BTS SIO SISR        ║
# ║  100% offline, monde isolé dans ~/.gameshell/            ║
# ║  Usage: ./gameshell.sh [--install] [--campaign N]        ║
# ║                        [--locked] [--name NOM]           ║
# ╚══════════════════════════════════════════════════════════╝
set -e

GAMESHELL_HOME="${GAMESHELL_HOME:-$HOME/.gameshell}"
export GAMESHELL_HOME
ARCHIVE_LINE=$(awk '/^__ARCHIVE_BELOW__$/ {print NR+1; exit}' "$0")
EMBEDDED_VERSION="__GSH_VERSION__"

# --install force la réinstallation; il n'est PAS transmis au jeu (P1.13)
FORCE=0
ARGS=()
for a in "$@"; do
    if [ "$a" = "--install" ]; then FORCE=1; else ARGS+=("$a"); fi
done

INSTALLED_VERSION="$(cat "$GAMESHELL_HOME/VERSION" 2>/dev/null || true)"
if [ "$FORCE" = 1 ] || [ ! -x "$GAMESHELL_HOME/bin/gameshell" ] || [ "$INSTALLED_VERSION" != "$EMBEDDED_VERSION" ]; then
    echo "📦 Installation de GameShell $EMBEDDED_VERSION dans $GAMESHELL_HOME..."
    mkdir -p "$GAMESHELL_HOME"
    TMPX="$(mktemp -d)"
    tail -n +"$ARCHIVE_LINE" "$0" | tar xz -C "$TMPX"
    # binaire, lanceur et générateurs: toujours mis à jour; la progression (state/) n'est jamais touchée
    mkdir -p "$GAMESHELL_HOME/bin"
    cp -f "$TMPX/bin/gameshell" "$GAMESHELL_HOME/bin/gameshell"
    cp -f "$TMPX/gameshell" "$TMPX/vaisseau-gen.sh" "$TMPX/monde-japon-gen.sh" "$TMPX/monde-reseau-gen.sh" "$TMPX/VERSION" "$GAMESHELL_HOME/"
    chmod +x "$GAMESHELL_HOME/bin/gameshell" "$GAMESHELL_HOME/gameshell" "$GAMESHELL_HOME"/*.sh
    # mondes: installés seulement s'ils manquent (une mise à jour garde la partie en cours)
    for w in vaisseau monde_japon monde_reseau; do
        [ -d "$GAMESHELL_HOME/$w" ] || cp -a "$TMPX/$w" "$GAMESHELL_HOME/$w"
    done
    rm -rf "$TMPX"
    echo "✅ Installation terminée!"
    echo ""
fi

exec "$GAMESHELL_HOME/gameshell" "${ARGS[@]}"

__ARCHIVE_BELOW__
HEADER

sed -i "s/__GSH_VERSION__/$VERSION/" gameshell.sh
cat "$TMPDIR/payload.tar.gz" >> gameshell.sh
chmod +x gameshell.sh

echo "✅ gameshell.sh créé ($(du -h gameshell.sh | cut -f1))"
echo "🚀 Usage: ./gameshell.sh"

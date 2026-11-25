#!/bin/bash
# ╔══════════════════════════════════════════════════════════╗
# ║  GARDE-FOU PARTAGÉ DES GÉNÉRATEURS DE MONDE              ║
# ║  Un seul gsh_check_dest sourcé par tous les générateurs   ║
# ╚══════════════════════════════════════════════════════════╝
# Usage: source gens-guard.sh (depuis vaisseau-gen.sh, monde-japon-gen.sh,
# monde-reseau-gen.sh). Destinataire: vérifier la destination avant effacement.

# ── Garde-fou (audit P0.6): les générateurs EFFACENT puis recréent la destination.
#    On refuse toute destination qui ne serait pas un monde du jeu sous
#    $GAMESHELL_HOME ou un dossier temporaire.
gsh_check_dest() {
    local dest="$1" want="$2"
    if [ -z "$dest" ] || [ "$(basename "$dest")" != "$want" ]; then
        echo "❌ Destination refusée: « $dest » (le dossier doit s'appeler $want)" >&2; exit 1
    fi
    local parent; parent="$(dirname "$dest")"
    mkdir -p "$parent" 2>/dev/null || true
    parent="$(cd "$parent" 2>/dev/null && pwd -P)" || { echo "❌ Dossier parent introuvable: $(dirname "$dest")" >&2; exit 1; }
    local ok=0 allowed
    for allowed in "${GAMESHELL_HOME:-$HOME/.gameshell}" "${TMPDIR:-/tmp}" /tmp /var/tmp; do
        [ -d "$allowed" ] || continue
        allowed="$(cd "$allowed" && pwd -P)"
        case "$parent/" in "$allowed"/*) ok=1 ;; esac
    done
    [ "$ok" = 1 ] || { echo "❌ Destination refusée hors de \$GAMESHELL_HOME ou d'un dossier temporaire: $dest" >&2; exit 1; }
}

#!/bin/bash
# ═══════════════════════════════════════════════════════════════
#  🚀 GAMESHELL — Installation des prérequis sur Debian 13 (Trixie)
#  Jeu RPG éducatif shell — BTS SIO SISR
#  À exécuter UNE FOIS par l'administrateur (root ou sudo), AVANT de lancer gameshell.sh.
#  Le jeu, lui, se lance ensuite avec un compte élève NORMAL (jamais en root).
# ═══════════════════════════════════════════════════════════════
set -e

if [ "$(id -u)" != "0" ]; then
    echo "❌ Ce script installe des paquets: lancez-le avec sudo (sudo bash install-debian13.sh)." >&2
    exit 1
fi

echo "📦 Mise à jour des dépôts..."
apt update

# bubblewrap = bac à sable du jeu (isolation complète de la machine hôte).
# Les autres paquets sont les outils SHELL réellement exécutés dans le bac à sable.
# Les commandes d'administration (useradd, ufw, systemctl, apt, ping, ssh...) sont
# SIMULÉES par le jeu: inutile d'installer ou de démarrer ssh/ufw/fail2ban pour jouer.
echo "📦 Installation des paquets nécessaires..."
apt install -y \
    bubblewrap \
    bash \
    coreutils \
    findutils \
    grep \
    sed \
    gawk \
    tar \
    gzip \
    zip \
    unzip \
    nano \
    less \
    man-db \
    util-linux

# Debian 13 autorise par défaut les espaces de noms utilisateur non privilégiés,
# nécessaires à bubblewrap. On le vérifie pour prévenir l'enseignant.
if [ -r /proc/sys/kernel/unprivileged_userns_clone ] && [ "$(cat /proc/sys/kernel/unprivileged_userns_clone)" = "0" ]; then
    echo "⚠️  kernel.unprivileged_userns_clone=0: le bac à sable ne pourra pas démarrer."
    echo "    Activez-le: echo 'kernel.unprivileged_userns_clone=1' > /etc/sysctl.d/99-gameshell.conf && sysctl --system"
fi

# Smoke test: bubblewrap doit démarrer un vrai bac à sable.
# Le jeu ne fonctionne qu'avec l'isolation des espaces de noms — un échec ici
# signifie que l'installation est incomplète et que le jeu échouera au lancement.
echo ""
echo "🧪 Test bubblewrap (bac à sable)..."
if ! command -v bwrap >/dev/null 2>&1; then
    echo "❌ bubblewrap introuvable après installation (paquet bubblewrap manquant?)" >&2
    exit 1
fi
if bwrap --unshare-all --ro-bind / / --proc /proc /bin/true 2>/dev/null; then
    echo "✅ bubblewrap fonctionne: bac à sable opérationnel."
else
    echo "❌ bubblewrap n'a pas pu démarrer le bac à sable."
    echo "   Vérifiez kernel.unprivileged_userns_clone=1 (voir fix ci-dessus) et faites:"
    echo "     bwrap --unshare-all --ro-bind / / /bin/true   (diagnostic manuel)"
    exit 1
fi

echo ""
echo "✅ Prérequis installés."
echo "🚀 Les élèves lancent maintenant, avec LEUR compte (pas root): ./gameshell.sh"
echo ""
echo "💡 Notes:"
echo "   - Le jeu s'exécute dans un bac à sable (bubblewrap): l'élève ne voit que le monde"
echo "     du jeu, sans réseau ni accès aux fichiers de la machine."
echo "   - useradd, ufw, systemctl, apt, kill, ping, ssh... sont simulés: aucun compte,"
echo "     pare-feu, service ou paquet RÉEL n'est jamais modifié par les missions."

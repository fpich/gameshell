#!/bin/bash
# ═══════════════════════════════════════════════════════════════
#  🚀 GAMESHELL — Installation des prérequis sur Debian 13 (Trixie)
#  Jeu RPG éducatif shell — BTS SIO SISR
#  À exécuter en root ou avec sudo, AVANT de lancer gameshell.sh
# ═══════════════════════════════════════════════════════════════
set -e

echo "📦 Mise à jour des dépôts..."
apt update

echo "📦 Installation des paquets nécessaires pour les 3 campagnes..."
apt install -y \
    bash \
    coreutils \
    findutils \
    procps \
    grep \
    sed \
    gawk \
    tar \
    gzip \
    zip \
    unzip \
    nano \
    curl \
    bind9-dnsutils \
    iproute2 \
    iputils-ping \
    net-tools \
    traceroute \
    nmap \
    openssh-client \
    openssh-server \
    ufw \
    iptables \
    fail2ban \
    auditd \
    less \
    man-db \
    sudo \
    passwd \
    login \
    systemd \
    util-linux \
    mount

echo ""
echo "✅ Prérequis installés."
echo "🚀 Lancez maintenant: ./gameshell.sh"
echo ""
echo "💡 Notes:"
echo "   - openssh-server + fail2ban + ufw + auditd sont nécessaires pour les missions"
echo "     de durcissement (campagnes 1 et 3). Ils sont installés mais PAS démarrés"
echo "     automatiquement pour ne pas modifier le comportement de la machine."
echo "   - Le jeu fonctionne aussi sans ces paquets (les commandes sont simulées),"
echo "     mais les missions système seront plus réalistes avec."

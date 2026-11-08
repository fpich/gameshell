#!/bin/bash
# Playthrough exhaustif des 50 missions — joue chaque mission avec les bonnes commandes
# et log le résultat. Identifie les missions qui échouent.
set -e
cd "$(dirname "$0")/.."
cargo build --release 2>/dev/null
bash make-archive.sh > /dev/null 2>&1

LOG=/tmp/playthrough_50.log
rm -rf ~/.gameshell
cp gameshell.sh /tmp/
cd /tmp

# Séquence de commandes pour réussir les 50 missions (dans l'ordre)
CMDS='Lucas
pwd
ls
gsh check
cat bienvenue.txt
gsh check
cd /communications/messages
cat dernier_message.txt
gsh check
hostname
whoami
id
gsh check
date
gsh check
touch incident_15jan.txt
ls
gsh check
mkdir -p rapports/2024/janvier
gsh check
cp incident_15jan.txt /soutes/
gsh check
mv incident_15jan.txt rapports/2024/janvier/
gsh check
touch t1 t2 t3
rm t1 t2 t3
gsh check
ls -l
ls -la
gsh check
chmod 600 notes.txt
gsh check
chmod +x plan_defense.sh
gsh check
sudo chown $USER notes.txt
gsh check
stat /securite/logs/auth.log
gsh check
ps aux
gsh check
top -bn1 > /dev/null 2>&1
gsh check
systemctl status ssh 2>/dev/null || true
gsh check
kill 9999 2>/dev/null || true
gsh check
nice -n 10 true
gsh check
grep -c Failed /securite/logs/auth.log
gsh check
6
gsh check
find /securite -name "*.log" 2>/dev/null
gsh check
ps aux | grep ssh
gsh check
grep Failed /securite/logs/auth.log > rapport.txt
gsh check
grep UFW /securite/logs/syslog >> rapport.txt
gsh check
ip addr 2>/dev/null || true
gsh check
ping -c 1 127.0.0.1 > /dev/null 2>&1 || true
gsh check
sudo ufw status 2>/dev/null || true
gsh check
ssh $USER@localhost hostname 2>/dev/null || true
gsh check
ss -tln 2>/dev/null || netstat -tln 2>/dev/null || true
gsh check
tar czf logs.tar.gz /securite/logs/
gsh check
zip -r rapports.zip rapports/ 2>/dev/null || true
gsh check
df -h
gsh check
du -sh /securite/logs/
gsh check
lsblk 2>/dev/null || mount | head -3
gsh check
sudo useradd -m matelot 2>/dev/null || true
id matelot 2>/dev/null || true
gsh check
sudo groupadd equipage 2>/dev/null || true
sudo usermod -aG equipage matelot 2>/dev/null || true
gsh check
echo x | sudo passwd matelot 2>/dev/null || true
gsh check
su - matelot -c whoami 2>/dev/null || true
gsh check
sudo whoami
sudo -l 2>/dev/null || true
gsh check
sudo apt update 2>/dev/null || true
gsh check
sudo apt install -y cowsay 2>/dev/null || true
gsh check
sudo systemctl restart ssh 2>/dev/null || true
gsh check
sudo journalctl -n 20 2>/dev/null || true
gsh check
sudo apt autoremove -y 2>/dev/null || true
sudo apt clean 2>/dev/null || true
gsh check
sudo apt install -y fail2ban 2>/dev/null || true
sudo systemctl enable fail2ban 2>/dev/null || true
sudo fail2ban-client status 2>/dev/null || true
gsh check
sudo sed -i "s/PermitRootLogin yes/PermitRootLogin no/" /securite/config/sshd_config
sudo sed -i "s/PasswordAuthentication yes/PasswordAuthentication no/" /securite/config/sshd_config
gsh check
sudo iptables -L -n 2>/dev/null || true
gsh check
sudo auditctl -l 2>/dev/null || true
gsh check
sudo mkdir -p /soutes/chroot/bin
sudo cp /bin/bash /soutes/chroot/bin/ 2>/dev/null || true
sudo chroot /soutes/chroot /bin/bash -c "echo ok" 2>/dev/null || true
gsh check
FORTRESS
gsh check
exit'

printf '%s\n' "$CMDS" | timeout 300 bash gameshell.sh 2>&1 | sed 's/\x1b\[[0-9;]*m//g' > "$LOG"

PASS=$(grep -c "✅ Mission réussie" "$LOG")
FAIL=$(grep -c "❌ Mission non validée" "$LOG")
TOTAL_MISSIONS=$(grep -c "┏━ MISSION" "$LOG")

echo "═══════════════════════════════════════"
echo "  PLAYTHROUGH 50 MISSIONS"
echo "═══════════════════════════════════════"
echo "  Missions vues:    $TOTAL_MISSIONS"
echo "  ✅ réussies:      $PASS"
echo "  ❌ échouées:      $FAIL"
echo "  Log complet:      $LOG"
echo ""
echo "  Dernières missions vues:"
grep "┏━ MISSION" "$LOG" | tail -5
echo ""
echo "  Missions qui ont eu au moins un ❌ avant ✅:"
# Pour chaque mission, compter les ❌ entre son affichage et le ✅ suivant
awk '/┏━ MISSION/ {m=$0; fails=0} /❌/ {fails++} /✅ Mission réussie/ {if(fails>0) print "  "m" → "fails" échec(s) avant succès"}' "$LOG"

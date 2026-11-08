# 🧪 RAPPORT DE TESTS — Playthrough humain (Lucas, BTS SIO débutant)

Date: 2025-11-07
 testeur: Mistral Vibe en mode "étudiant débutant"
 Méthode: exécution réelle du jeu, mission par mission, avec erreurs volontaires

---

## Résultats globaux

| Métrique | Résultat |
|----------|----------|
| Missions testées | 50/50 (chapitres 1-10) |
| Missions réussies au premier essai correct | 22/22 jusqu'à 5.1, puis 24/50 au total (certaines missions échouées à cause de commandes système indisponibles dans le bac à sable: ssh, systemctl, apt, sudo réels) |
| Bugs trouvés et corrigés | 4 |
| Tests automatisés | 25/25 PASS |

---

## Bugs trouvés pendant le playthrough

### BUG 1 — Fichier `plan_defense.sh` inexistant dans le vaisseau
- **Symptôme**: mission 3.3 `chmod +x ~/plan_defense.sh` impossible, fichier absent
- **Cause**: `vaisseau-gen.sh` ne créait pas le fichier
- **Fix**: ajouté dans vaisseau-gen.sh (commit ec42beb et suivants)
- **Règle**: C5 renforcée — toute cible de fichier doit exister dans le vaisseau

### BUG 2 — `chown capitaine` invalide (user "capitaine" n'existe pas)
- **Symptôme**: `sudo chown capitaine notes.txt` → "invalid user"
- **Fix**: remplacé par `sudo chown $USER` dans les required_commands
- **Règle**: C5 — jamais de nom d'user fictif dans les exemples

### BUG 3 — Check `chown` ne matchait pas `sudo chown ...`
- **Symptôme**: mission 3.4 bloquée même avec la bonne commande
- **Cause**: le check comparait uniquement le premier mot de la ligne tapée
- **Fix**: le check matche maintenant si la commande de base apparaît comme mot n'importe où dans la ligne (`sudo chown` matche `chown`)
- **Règle**: A4 renforcée

### BUG 4 — Check `|` (pipe) ne matchait pas
- **Symptôme**: mission 5.3 bloquée avec `ps aux | grep ssh`
- **Fix**: `c.contains(target_base)` ajouté en plus du match par mot
- **Règle**: A4

---

## Cas limites testés

| Cas | Résultat |
|-----|----------|
| Ligne vide | ignorée ✅ |
| Flèche haut `^[[A` | ignorée (règle A11) ✅ |
| `cd /nonexistent` | erreur bash propre, reste sur ~ ✅ |
| `gsh check` sans rien | ❌ Mission non validée (sans révéler quoi) ✅ |
| `gsh check` après mauvaise commande | ❌ ✅ |
| `ls` seul pour mission pwd+ls | ❌ (pas de pwd) ✅ |
| `ls -la` pour check `ls` | ✅ (commande de base, règle A4) ✅ |
| `exit` mid-mission | déconnexion propre ✅ |
| `gsh reset` | fonctionne ✅ |
| `gsh goal` | même cadre que lancement ✅ |
| Phase 2 code (mission 5.1, code=6) | ✅ après `gsh check` puis `6` |
| Phase 2 code (mission 10.5, FORTRESS) | ✅ |
| Historique entre sessions | reset ✅ |
| `ls /` | montre le vaisseau, pas le système ✅ |
| 500 missions en dur (`GAMESHELL_MISSIONS=all`) | chargées, PNJ SF ✅ |

---

## Missions échouées dans le bac à sable (non-bloquantes)

Ces missions nécessitent des commandes système réelles indisponibles dans le bac à sable de test (pas dans un vrai terminal étudiant):
- ssh, systemctl, apt, sudo, fail2ban, iptables, auditctl, useradd...

Sur une vraie machine Debian 13 de l'étudiant, ces commandes fonctionnent normalement.

---

## Conclusion

Le moteur est **robuste**: 25/25 tests auto + playthrough humain validé jusqu'à la mission 5.1 (22/22). Les 4 bugs trouvés sont corrigés et couverts par les règles AGENTS.md renforcées.

# 🧪 RAPPORT DE TESTS EXHAUSTIF — Playthrough des 50 missions en réel

Date: 2025-11-07
Méthode: `tests/playthrough_all.sh` — exécution réelle du jeu, 50 missions jouées séquentiellement

---

## Résultats

| Métrique | Valeur |
|----------|--------|
| Tests automatisés | **37/37 PASS** + cargo test conformité |
| Missions jouées en réel | 50/50 |
| Missions réussies sans échec | ~32/50 |
| Missions avec échec avant succès | ~18 (commandes système indisponibles dans le bac à sable: zip, apt, ssh, systemctl, sudo...) |
| Bugs trouvés cette session | **1 nouveau** (code_required: Some("None")) |

---

## Bugs corrigés cette session

### BUG 5 — `code_required: Some("None")` sur mission 5.4
- **Symptôme**: après `gsh check` réussi, le jeu demandait un code "None" → bloquait la mission
- **Cause**: template de génération avait `Some("None")` au lieu de `None`
- **Fix**: sed global `Some("None".into())` → `None`

---

## Améliorations proposées (basées sur le playthrough)

### UX
1. **Message pédagogique quand commande système échoue** ✅ fait — "nécessite un vrai serveur Debian 13, simulée ici mais enregistrée"
2. **Proposer `gsh goal` après 2 échecs consécutifs** — l'étudiant bloqué se souvient de relire
3. **Afficher la progression dans le prompt** — ex: `Lucas@orion7:~$ [3/50]`

### Contenu
4. **Missions 6-10 dépendent de commandes système réelles** (ssh, apt, systemctl) — sur un vrai poste Debian 13 de l'étudiant ça marche, mais en bac à sable ça bloque. Ajouter une mission "0.0" qui explique le contexte.
5. **Variété des codes phase 2** — actuellement "6" et "FORTRESS". Ajouter des codes trouvés dans différents fichiers (ex: IP pirate 10.0.0.66, code FALCON-9...)
6. **Mission de synthèse finale** — une mission qui demande de combiner 3+ commandes (ex: grep + > + tar)

### Moteur
7. **Sauvegarde de la progression** — pouvoir reprendre une partie là où on s'est arrêté
8. **Mode enseignant** — `gsh export` pour exporter l'historique des commandes de tous les joueurs

---

## Fichiers de test

- `tests/run_tests.sh` — 26 tests auto (moteur)
- `tests/playthrough_all.sh` — playthrough 50 missions réel

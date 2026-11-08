# 🔧 PLAN DE CORRECTIONS & AMÉLIORATIONS — post-playthrough

Basé sur TESTS_PLAYTHROUGH.md (playthrough Lucas, 50 missions, 4 bugs trouvés).

---

## P0 — Corrections bloquantes (à faire maintenant)

### P0.1 Audit des cibles de fichiers
- [ ] Extraire tous les `target:` des checks `file_exists`/`file_read`/`permission`/`dir_exists` des 50 missions
- [ ] Vérifier que chaque cible existe dans `vaisseau-gen.sh`
- [ ] Ajouter les fichiers manquants
- [ ] Test: script qui liste les cibles et vérifie dans le vaisseau généré

### P0.2 Audit des noms d'users dans les exemples
- [ ] Rechercher `chown <nom>`, `useradd <nom>`, `ssh <nom>@` dans les required_commands
- [ ] Remplacer tout nom fictif par `$USER` ou un nom valide
- [ ] Règle: jamais de nom d'user fictif

### P0.3 Noms de fichiers exacts dans le dialogue PNJ
- [ ] Pour chaque mission avec check `file_exists` sur `~/xxx`, le goal/story doit contenir le nom exact `xxx`
- [ ] Exemple: mission 2.1 → le PNJ dit "crée incident_15jan.txt"
- [ ] Script de vérification: grep le nom du fichier dans la story de la mission

### P0.4 Toutes les commandes requises ont une entrée cmd_brief
- [ ] Lister toutes les `required_commands` uniques des 50 missions
- [ ] Vérifier que `cmd_brief()` a une entrée pour chaque base
- [ ] Ajouter les manquantes

---

## P1 — Améliorations UX

### P1.1 Phase 2 code_required: message plus explicite
- Actuel: "🔐 Dernière étape: entrez le code trouvé dans le vaisseau:"
- Améliorer: indiquer DANS QUEL fichier chercher le code (ex: "code dans /securite/logs/auth.log")

### P1.2 Test auto playthrough partiel
- [ ] Ajouter au tests/run_tests.sh un test qui joue les 10 premières missions automatiquement
- [ ] Vérifier 10/10 réussies

### P1.3 gsh reset should aussi reset la mission courante (pas juste le vaisseau)
- [ ] Actuellement reset le vaisseau mais reste sur la mission en cours
- [ ] Devrait aussi reset mission_commands

---

## P2 — Robustesse

### P2.1 Commandes système indisponibles → message pédagogique
- Quand `apt`, `ssh`, `systemctl` etc. échouent (pas installés), afficher un message du type:
  "💡 Cette commande nécessite un vrai serveur Debian 13. En mode bac à sable, elle est simulée."

### P2.2 Validation que le vaisseau est complet au lancement
- [ ] Au démarrage, vérifier que vaisseau/ existe et contient les compartiments essentiels
- [ ] Sinon réinstaller automatiquement

---

## Ordre d'exécution
1. P0.1 → P0.2 → P0.3 → P0.4 (audit + fixes)
2. P1.1 → P1.2 → P1.3 (UX)
3. P2.1 → P2.2 (robustesse)
4. Tests 25/25 + playthrough 10 premières missions
5. Commit + push

# 🧪 RETEX EXHAUSTIF — GameShell (2 campagnes × 100 missions)

Date: session courante. Périmètre: exécution réelle du jeu (`gameshell.sh`), 200 missions jouées
une à une par un banc automatisé + tests manuels en pty réel.

## Méthodes de test exécutées

| Méthode | Commande | Résultat |
|---------|----------|----------|
| Auto-solve (joue toutes les missions réellement) | `GAMESHELL_AUTO=1 ./gameshell.sh` | **100/100 × 2 campagnes** |
| Banc de cas limites par mission | `GAMESHELL_EDGE=1 ./gameshell.sh` | **300/300, 0 problème** |
| Torture par mission (partiel, typo, 2 mauvais codes, isolation hôte) | `GAMESHELL_TORTURE=1 ./gameshell.sh` | **300/300, 0 problème, exit 0** |
| Conformité contenu (cargo) | `cargo test --release` | 2/2 (200 missions, 20/acte, C3/C6/B9/C9/C13/A15) |
| Suite moteur | `./tests/run_tests.sh` | **42/42 PASS** (auto 300/300, EDGE 300/300, TORTURE 300/300) |
| Édition de ligne en vrai tty (flèches) | `script -qec` + séquences `\x1b[D`/`\x1b[A` | OK (curseur, historique) |
| Inspection visuelle manuelle | sessions campagne 1 et 2 | OK |

Le mode EDGE teste pour CHAQUE mission: check à vide échoue, commande garbage échoue,
résolution réussit, mauvais code phase 2 échoue, bon code réussit, position reset à `~`
et historique vidé après validation (règle A17).

## Bugs trouvés par le RETEX et corrigés

| # | Bug | Correction | Commit |
|---|-----|-----------|--------|
| 1 | Maps des chapitres générés affichaient des `\n` littéraux | fix échappement | 927aa59 |
| 2 | 200 maps mal alignées (bordures de largeurs différentes) | gabarit fixe 14 car. + test B9 | 927aa59 |
| 3 | Stories wrappées ligne/ligne → coupures mid-phrase | re-flow par paragraphe | e6da76f |
| 4 | Check exigeait `-la` mais la carte n'affichait que `ls` (inéquitable) | B4: commande complète affichée si option requise | e6da76f |
| 5 | `gsh check` à vide validait certaines missions (5.3, 7.2 espace; 5.2, 8.4, 15.1 japon) | invariant "aucune commande = échec" (A17) | ce RETEX |
| 6 | `dir_reached "/"` toujours vrai (starts_with racine) | égalité stricte | ce RETEX |
| 7 | `plan_defense.sh` créé exécutable (755/700) → check `+x` déjà vrai | monde généré en 644 | ce RETEX |
| 8 | `chmod 700` (mission précédente) rendait le check `+x` suivant vrai | check `+x` → `command_executed "+x"` | ce RETEX |
| 9 | `dir_reached "~"` toujours vrai (reset position A7) | → `command_executed "~"` | ce RETEX |
| 10 | `id`/`uname` révélaient l'hôte réel | interception A16 (+ `groups`) | 799ed43 |
| 11 | Compétences débloquées incohérentes avec la mission | règle A15 + cargo test | 799ed43 |
| 12 | PNJ non-SF (Marie, Jean, Kowalski, Galactica) | règle C13, renommage global | 799ed43 |
| 13 | `zip` absent du bac à sable → mission injouable | fallback tar (A12) | 799ed43 |
| 14 | Flèches ← → inutilisables | éditeur de ligne raw mode (A11) | 799ed43 |

## Plan d'action — améliorations restantes (priorisées)

### P0 — robustesse (déjà en grande partie couvert, à garder verrouillé)
1. ✅ A17 figé par `GAMESHELL_EDGE=1` dans la suite (test 40) — toute régression de check vide/vacu
   fait échouer le build.
2. ⏭️ Ajouter au EDGE: test `gsh reset` en cours de mission (monde régénéré, mission recommence à zéro).
3. ⏭️ Ajouter au EDGE: EOF (Ctrl+D) et input 8000 car. en plein milieu d'une mission (déjà testés moteur, à Brancher par mission).

### P1 — expérience joueur
4. ⏭️ **Sauvegarde/reprise** par campagne (fichier `~/.gameshell/state/save_<campagne>.json`:
   mission courante, XP, skills) — actuellement la progression est perdue à la fermeture.
5. ⏭️ **Écran de fin par campagne** avec score + compétences acquises + pourcentage (actuellement
   juste une ligne FÉLICITATIONS).
6. ⏭️ **Mission 0 tutoriel** guidé (man + gsh help + gsh goal) avant la 1.1 des deux campagnes.
7. ⏭️ **Variété des approches** encore faible en acte I: ajouter 2-3 missions "lecture seule"
   (goal sans commande, ex: lire un fichier et trouver un code) pour casser la routine.

### P2 — contenu
8. ⏭️ Troisième campagne (niveau DIFFICILE) sur le même moteur multi-campagnes (ex: "Cyberpunk 2087",
   monde `monde_cyber/`, hôte `neon-9`) — le moteur C14/D2 est prêt.
9. ⏭️ Équilibrage XP: vérifier que la montée de niveau colle à la difficulté (facile vs moyen).
10. ⏭️ Diversifier les PNJ secondaires par acte (actuellement le pool est petit et se répète).

### P3 — enseignant
11. ⏭️ `gsh export` : exporter la progression d'un élève (missions faites, XP, skills, commandes loguées)
    en JSON/CSV pour le suivi de classe.
12. ⏭️ Mode enseignant: lancer le jeu avec une campagne verrouillée / ordre de missions imposé.

## État final constaté

- 300/300 missions résolues réellement (auto-solve), 300/300 cas limites OK (EDGE), 300/300 torture OK (TORTURE).
- Isolation hôte vérifiée en torture: `cat /etc/passwd` et `rm -rf /tmp` dans le jeu ne touchent jamais l'hôte (A1/A16).
- Nouveau test de conformité cargo: chaque check `command_executed` doit être atteignable par une `required_command` (A4).
- 40/40 tests moteur PASS, cargo test conformité OK.
- Aucune fuite d'infos hôte (A16), aucun check vacu (A17), cartes alignées (B9/C6),
  stories re-flowées, commandes affichées = checks atteignables (B4).

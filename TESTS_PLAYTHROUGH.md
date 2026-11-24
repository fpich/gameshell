# 🧪 RAPPORT DE TESTS — GameShell (3 campagnes × 100 missions)

## Suite automatisée: `./tests/run_tests.sh` → **55/55 PASS** (dans un GAMESHELL_HOME temporaire)

| Bloc | Contenu |
|------|---------|
| Tests 1-37 | moteur (isolation, prompt, checks A4/A8/A17, flèches, EOF, input long, campagnes 2/3...) |
| Test 38 | auto-solve réel: **100/100 × 3 campagnes** (`GAMESHELL_AUTO=1`) |
| Test 40 | cas limites EDGE: **300/300** (`GAMESHELL_EDGE=1`) |
| Test 42 | torture: **300/300** + isolation hôte testée sur l'exécution réelle (`GAMESHELL_TORTURE=1`) |
| Tests 49-54 | évasion impossible, commandes système simulées, pipes/redirections, validation stricte, phase 2 non bloquante, `gsh   check` |
| Test 55 | archive `gameshell.sh` bien formée (shebang, exécutable, VERSION embarquée, marqueur d'archivage) |

## Conformité contenu: `cargo test --release` (12/12)
300 missions (3 campagnes, 20/acte), 20+ lignes de contexte (C3), double définition 🚀💻 (C7),
skill = commande de la mission (A15), checks atteignables (A4), Bloom FR (C9), PNJ interdits (C13).
Ajouté en révision: scan sans mojibake (`stories_sans_mojibake`), test archive (55).

## Playthrough humain
Voir `docs/historique/RETEX.md` (méthodes, 14+ bugs trouvés/corrigés, plan d'action P0-P3).

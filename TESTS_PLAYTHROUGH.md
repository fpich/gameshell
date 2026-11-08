# 🧪 RAPPORT DE TESTS — GameShell (3 campagnes × 100 missions)

## Suite automatisée: `./tests/run_tests.sh` → **48/48 PASS**

| Bloc | Contenu |
|------|---------|
| Tests 1-37 | moteur (isolation, prompt, checks A4/A8/A17, flèches, EOF, input long, campagnes 2/3...) |
| Test 38 | auto-solve réel: **100/100 × 3 campagnes** (`GAMESHELL_AUTO=1`) |
| Test 40 | cas limites EDGE: **300/300** (`GAMESHELL_EDGE=1`) |
| Test 42 | torture: **300/300** + isolation hôte (`GAMESHELL_TORTURE=1`) |

## Conformité contenu: `cargo test --release`
300 missions (3 campagnes, 20/acte), 20+ lignes de contexte (C3), double définition 🚀💻 (C7),
skill = commande de la mission (A15), checks atteignables (A4), Bloom FR (C9), PNJ interdits (C13).

## Playthrough humain
Voir `RETEX.md` (méthodes, 14+ bugs trouvés/corrigés, plan d'action P0-P3).

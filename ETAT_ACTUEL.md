# 📋 ÉTAT ACTUEL — GameShell v2.0.0

> Page de synthèse — source de vérité de l'état du dépôt.
> Mis à jour à chaque changement notable. Historique : `docs/historique/`,
> suivi des missions : `docs/suivi/`.

## En bref

| Indicateur | Valeur |
|------------|--------|
| Version | **2.0.0** |
| Campagnes | 3 × 100 missions EN DUR (20 chapitres × 5, 5 actes × 20) |
| Conformité cargo | **18/18** (`cargo test --release`) |
| Suite moteur | **57/57** (`./tests/run_tests.sh`) |
| Clippy | propre (`cargo clippy --release --all-targets -- -D warnings`) |
| Formatage | `cargo fmt --check` (CI en échec sinon) |
| Chaîne Rust | épinglée `rust-toolchain.toml` (1.100.0, clippy + rustfmt) |
| Process de release | tag `v*` → CI : archive + GitHub release (voir `docs/release.md`) |
| CI | GitHub Actions (`.github/workflows/ci.yml`) |
| Licence | propriétaire — tous droits réservés (voir `COPYRIGHT.md`) |

## Tests

### `cargo test --release` — 18/18

| Test | Rôle |
|------|------|
| missions_conformes | 300 missions : 20/acte, A4/A12/A15/A17, C3/C4/C5/C9/C13, analyseur, cadres |
| checks_systeme_jamais_vrais_au_depart | A17 : checks état-système jamais vrais à l'état initial |
| cibles_existent_dans_le_monde | A12 : fichiers ciblés bien présents dans le monde |
| vaisseau_gen_sans_noms_interdits | C13 : aucun nom banal dans les mondes générés |
| stories_sans_mojibake | scan story/description/goal/title sur marqueurs UTF-8 corrompus |
| substring_ne_valide_plus | A4 : sous-chaînes ne valident jamais |
| commande_de_base_a4 | A4 : `ls -la` matche `ls` (cible de base) |
| options_exactes | A4 : option portée par la bonne commande |
| lecture_et_redirections | A4 : file_read / redirections |
| bordures_alignees | B5 : cadre mission aligné (largeur d'affichage) |
| reecriture_mode_degrade | mode dégradé : réécriture de chemins sans bwrap |
| slug | slugify (export JSON/CSV stable) |
| safe_join | unzip embarqué : chemin normal accepté, traversal refusé (anti-évasion) |
| unzip_embedded | extraction embarquée : sortie dans la destination, entrées traversal ignorées |
| safe_state_name | noms d'état fictifs acceptés, traversal refusé |

### `./tests/run_tests.sh` — 57/57

| Bloc | Contenu |
|------|---------|
| 1–37 | moteur (isolation A1, prompt, checks A4/A8/A17, flèches A11, EOF A13, input long, campagnes 2/3) |
| 38 | AUTO: 300/300 missions résolues par leurs seules commandes requises |
| 40 | EDGE: 300/300 (check vide, garbage, mauvais code, reset...) |
| 42 | TORTURE: 300/300 + isolation hôte sur l'exécution réelle |
| 43–48 | reprise par campagne, sauvegarde par joueur, export JSON/CSV, écrans victoire ×3, `gsh restart`, historique horodaté |
| 49–54 | évasion impossible, commandes simulées, pipes/redirections, validation stricte, phase 2 non bloquante, `gsh   check` |
| 55 | archive `gameshell.sh` bien formée (shebang, exécutable, VERSION de Cargo.toml embarquée, marqueur `__ARCHIVE_BELOW__`) |
| 56 | bout en bout : l'archive s'installe dans un HOME vierge (binaire, VERSION, 3 mondes, lancement campagne 1) |
| 57 | mode dégradé sans bwrap : démarrage signalé, garde-fou cd, prompt affiché |

## Structure du dépôt

```
.
├── AGENTS.md                # bible du projet (règles A/B/C/D/E)
├── README.md                # documentation principale
├── PLAN.md                  # plan des 3 campagnes
├── ETAT_ACTUEL.md           # ← cette page (source de vérité)
├── CHANGELOG.md             # historique des changements
├── CONTRIBUTING.md          # conventions de contribution
├── COPYRIGHT.md             # notice de propriété (pas de LICENSE)
├── TESTS_PLAYTHROUGH.md     # rapport de tests
├── Cargo.toml / Cargo.lock  # dépendances (lock COMMITTÉ)
├── rust-toolchain.toml      # toolchain épinglée
├── rustfmt.toml             # formatage standardisé
├── .editorconfig            # règles de base d'édition
├── .github/workflows/ci.yml               # CI : clippy, fmt, build, tests + fresh-install + audit deps
├── .github/workflows/release.yml          # release : tag v* -> archive + GitHub release
├── install-debian13.sh                    # prérequis Debian 13 (apt + smoke test bwrap)
├── make-archive.sh          # génère gameshell.sh auto-extractible
├── vaisseau-gen.sh          # générateur des 3 mondes
├── monde-*.sh               # mondes campagne 2 et 3
├── launcher.sh              # lanceur (transmet --campaign/--locked/--name)
├── tests/run_tests.sh       # suite (57 tests, dossier temporaire, jamais les données réelles)
├── tests/playthrough_all.sh # playthrough complet en AUTO (dossier sandbox, ne détruit jamais ~/.gameshell)
├── docs/
│   ├── suivi/               # relecture des 300 missions (campagne1/2/3, README d'index)
│   ├── historique/          # RETEX.md + CORRECTIONS.md (records historiques, figés)
│   └── release.md           # procédure de release (tag -> archive + GitHub release)
└── src/
    ├── main.rs              # entrée, menu campagnes, bannières
    ├── sys.rs               # FFI libc (termios, signaux)
    ├── game/                # moteur : boucle, checks, phase 2, export, bancs AUTO/EDGE/TORTURE
    ├── shell/               # bac à sable bubblewrap + sauvegardes
    ├── sim/                 # commandes système simulées (comptes, processus, services, réseau)
    └── missions/            # 300 missions en dur (chapters/, japon/, reseau/)
```

## Règles de non-régression (rappel)

- Toute modification respecte `AGENTS.md` (numérotation fixe, jamais renumérotée).
- Avant chaque commit : `cargo clippy --release --all-targets`, `cargo fmt --check`,
  `cargo test --release` (18/18) et `./tests/run_tests.sh` (57/57).
- Les tests tournent dans un `GAMESHELL_HOME` temporaire ; jamais sur les données réelles.
- `git push` et publication de release : uniquement sur demande explicite du propriétaire.
- Historique documentaire : `docs/historique/RETEX.md` et `CORRECTIONS.md` sont des records
  figés (48/48, "2 campagnes") — ne pas les reprendre comme état actuel.

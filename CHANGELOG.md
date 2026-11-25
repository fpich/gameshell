# Changelog — GameShell

Tous les changements notables du projet sont documentés ici.
Format : [Keep a Changelog](https://keepachangelog.com/fr/) — exemption : versions « unifiées »
(2.0.0) et pré-releases non publiées sans accord explicite.

## [2.0.0] — 2026-11-24 (révision intégrale)

### Ajouté
- CI GitHub Actions (`.github/workflows/ci.yml`) : clippy (`-D warnings`), `cargo fmt --check`,
  build, `cargo test --release`, `tests/run_tests.sh` sur `ubuntu-latest` avec `bubblewrap`.
- `.github/workflows/` + badge de CI dans `README.md`.
- `rust-toolchain.toml` (1.100.0, components clippy + rustfmt), `rustfmt.toml`, `.editorconfig`.
- `Cargo.lock` commité (reproductibilité maximale : 3 dépendances + transitives).
- Métadonnées `Cargo.toml` : `license = "Proprietary"`, `repository`, `keywords`, `categories`, `publish = false`.
- Tests : `stories_sans_mojibake` (scan story/description/goal/title sur marqueurs UTF-8 corrompus)
  et test 55 de la suite (archive `gameshell.sh` bien formée : shebang, exécutable, VERSION
  embarquée, marqueur d'archivage).
- `docs/suivi/` (56 fichiers chapitre par chapitre + index) remplace le monolithe
  `SUIVI_MISSIONS.md` (~210 Ko). `docs/historique/` (RETEX.md + CORRECTIONS.md) pour les records figés.
- `ETAT_ACTUEL.md` : page source-de-vérité de l'état du dépôt.
- `CHANGELOG.md`, `CONTRIBUTING.md`, `COPYRIGHT.md` (notice de propriété, pas de LICENSE).
- Tests 56 et 57 (`run_tests.sh`) : bout en bout — l'archive s'installe dans un HOME vierge
  (bin, VERSION, 3 mondes, lancement campagne 1) ; mode dégradé sans bwrap (signalé + garde-fou cd).
- `install-debian13.sh` : smoke test bubblewrap post-installation (`bwrap --unshare-all`) ;
  script refusé si le test échoue.
- CI : job `fresh-install` (l'archive s'installe dans un HOME vierge et lance la
  campagne 1) + job `audit-deps` (`cargo audit` non bloquant). Suite 57/57 (55→57).
- Workflow de release (`.github/workflows/release.yml`) : tag `v*` → build + checksum SHA-256
  + GitHub Release avec `gameshell.sh`. Voir `docs/release.md`.
- `docs/release.md` : procédure de versionnement et de publication.

### Corrigé
- `tests/playthrough_all.sh` : ne touche plus `~/.gameshell` — bac à sable temporaire
  (`mktemp -d`), aucun `rm -rf` sur les données réelles du joueur.
- Warnings `clippy` dans 8 fichiers (is_multiple_of, print_literal, redundant closure,
  sliced_string_as_bytes, Default).

### Modifié
- `cargo fmt` global (26 fichiers) — formatage aligné sur la CI.
- `SUIVI_MISSIONS.md` découpé en `docs/suivi/campagne{1,2,3}/ch<NN>[-reecrit].md`
  (campagne 1 : débutant + réécriture séparés).
- Docs : figures de test mises à jour (57/57 suite, 18/18 cargo), structure documentaire réécrite.
- `README.md` : section Structure du dépôt mise à jour + badge CI + note de licence.

### Supprimé
- `SUIVI_MISSIONS.md` (remplacé par `docs/suivi/`).

## [1.x] — pré-révision (avant 2.0.0)

Historique : voir `docs/historique/RETEX.md` et `docs/historique/CORRECTIONS.md`
(records figés, 48/48 à l'époque).

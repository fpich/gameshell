# 🚀 Release — GameShell

> Processus de publication des versions stables. **Ne jamais publier sans
> autorisation explicite du propriétaire du dépôt.**

## Versionnement

- Format `vMAJOR.MINOR.PATCH` (SemVer).
- La version est déclarée dans `Cargo.toml` (`version = "2.0.0"`) puis embarquée
  dans `gameshell.sh` par `make-archive.sh` (`VERSION` = `<version>-<hash12>`).

## Procédure

1. Vérifier l'état du dépôt :
   - `cargo clippy --release --all-targets -- -D warnings` — propre
   - `cargo fmt --check` — propre
   - `cargo test --release` — 103/103
   - `./tests/run_tests.sh` — 57/57 (inclut tests 55/56/57 : archive, fresh install, mode dégradé)
2. Mettre à jour `CHANGELOG.md` (section `[X.Y.Z]` avec la date du jour) et
   `ETAT_ACTUEL.md` (version).
3. Commit + push `master` (push = action externe, autorisation requise).
4. Tag : `git tag -a v2.0.0 -m "v2.0.0 — ..."` puis `git push origin v2.0.0`.
5. CI (workflow `release.yml`) déclenchée sur le tag :
   - build Release, `bash make-archive.sh`
   - calcul du checksum SHA-256 de `gameshell.sh`
   - publication du GitHub Release avec `gameshell.sh` + `gameshell.sh.sha256`
6. Vérifier la sortie de la release sur le dépôt GitHub.

## Règles de publication

- Le tag ne doit jamais être déplacé après publication (tag immuable).
- `gameshell.sh` est un artefact : il est reconstruit à chaque publication,
  il n'est pas édité à la main.
- `gameshell.sh` (fichier généré) n'est jamais modifié hors `make-archive.sh`.
- `docs/historique/` = records figés : ne jamais les modifier pour ajouter une release.

## Fichiers touchés

- `Cargo.toml` — `version`
- `CHANGELOG.md` — section de la release
- `ETAT_ACTUEL.md` — indicateur version
- `.github/workflows/release.yml` — workflow de publication

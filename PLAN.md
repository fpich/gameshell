# 📋 PLAN — GameShell Orion-7 : 100 missions en dur (20 par acte)

## Structure retenue (100% offline, AUCUNE génération procédurale)

- **100 missions écrites à la main** dans `src/missions/chapters/ch1..20.rs`
- **20 chapitres × 5 missions**, **5 actes × 20 missions** (4 chapitres par acte)
- Boucle narrative "Les Pirates de l'Espace": arrivée → enquête → défense → durcissement

| Acte | Chapitres | Thème |
|------|-----------|-------|
| I | 1–4 | Arrivée à bord, exploration, organisation, journal de bord |
| II | 5–8 | Sécuriser les accès, surveiller les processus |
| III | 9–12 | Enquête (logs), diagnostic réseau |
| IV | 13–16 | Sauvegarde/archives, gestion de l'équipage |
| V | 17–20 | Maintenance, durcissement SSH, boucliers, mission finale avec code |

## Conformité (figée par `cargo test` + `tests/run_tests.sh` 38/38)

Chaque mission respecte AGENTS.md: 20+ lignes de contexte affichées (C3), carte ASCII vue de dessus (C6),
double définition 🚀 roleplay + 💻 technique (C7), objectif narratif sans nommer la commande (C4),
compétence débloquée = commande de LA mission (A15), PNJ noms SF (C13), Bloom en français (C9).

## Extension future

Pour ajouter des missions: créer `ch21.rs`, ... et les enregistrer dans `src/missions/mod.rs`.
Toujours en dur, jamais de génération runtime.

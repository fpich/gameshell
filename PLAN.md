# 📋 PLAN — GameShell : 2 campagnes de 100 missions en dur (20 par acte)

## Campagne 1 — Les Pirates de l'Espace (FACILE) — `src/missions/chapters/ch1..20.rs`
Station Orion-7, hôte orion7, capitaine. 5 actes × 20 missions.

## Campagne 2 — Le Japon Féodal (MOYEN) — `src/missions/japon/ch1..20.rs`
Château Shiro, hôte shiro, seigneur. Monde dédié `monde_japon/` (monde-japon-gen.sh).
5 actes × 20 missions, checks plus exigeants (options exactes, multi-checks, codes phase 2).

| Acte | Chapitres (les 2 campagnes) | Thème |
|------|------------------------------|-------|
| I | 1–4 | Arrivée (vaisseau / château), exploration, organisation |
| II | 5–8 | Permissions, surveillance des processus |
| III | 9–12 | Enquête (logs/espions), réseau, combos de commandes, archives |
| IV | 13–16 | Recrutement/équipage, coffre-fort, inventaire |
| V | 17–20 | Pare-feu, services, durcissement, siège final avec code |

## Moteur multi-campagnes (règles C14/D2)
- Sélection au lancement; `GAMESHELL_CAMPAIGN` + `GAMESHELL_WORLD` par campagne.
- Bannière, prompt, hostname/whoami/id/groups/uname et analogies 🚀 thématisés par campagne.
- `gsh reset` régénère le monde de la campagne courante (vaisseau-gen.sh embarqué).

## Tests (figés)
- `cargo test`: conformité des 2 campagnes (100 missions chacune, 20/acte, C3/C6/C9/C13/A15).
- `tests/run_tests.sh`: **48/48 PASS** dont auto-solve 100/100 × 3 campagnes (GAMESHELL_AUTO=1), EDGE 300/300, TORTURE 300/300, sélections campagnes 2 et 3.

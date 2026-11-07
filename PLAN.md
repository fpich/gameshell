# 📋 PLAN D'ACTION — Génération de 5000 missions cohérentes

## Objectif
Passer de 50 missions manuelles à un **système de génération procédurale** produisant jusqu'à 5000 missions pédagogiquement cohérentes, organisées par actes, avec montée en compétences et variation Bloom.

---

## 1. Architecture du générateur

### 1.1 Templates de missions (MissionTemplate)
Chaque commande shell a un **template** avec:
- `command`: la commande de base (ls, cd, grep...)
- `bloom_levels`: niveaux Bloom possibles (ex: ls = [Se souvenir, Comprendre, Analyser])
- `contexts`: liste de contextes RP possibles (qui parle, urgence, enjeu)
- `targets`: compartiments/fichiers cibles possibles dans le vaisseau
- `variations`: formulations d'objectif possibles
- `checks_template`: type de check (command_executed, file_exists, dir_reached...)

### 1.2 Génération procédurale
Pour chaque template × contexte × cible × variation × niveau Bloom:
→ génère une Mission unique avec:
- id unique (ex: 1.1, 1.2... A.001, A.002 pour acte A)
- story générée à partir du contexte + cible (20+ lignes, règle C3)
- map ASCII générée (position + cible, règle C6)
- goal narratif (règle C4)
- double définition commande (règle C7)
- check adapté au template

### 1.3 Graine déterministe
- Seed fixe (ex: 42) → génération reproductible
- Permet de régénérer exactement les mêmes missions

---

## 2. Organisation par Actes (5 actes narratifs)

| Acte | Thème | Niveau Bloom dominant | Commandes introduites | Nb missions cible |
|------|-------|----------------------|----------------------|-------------------|
| I | Arrivée & exploration | Se souvenir, Comprendre | man, pwd, ls, cd, cat, hostname, whoami, id, date | ~500 |
| II | Organisation & permissions | Appliquer | touch, mkdir, cp, mv, rm, chmod, chown, stat, ls -l, getfacl | ~1000 |
| III | Enquête & surveillance | Analyser | ps, top, systemctl, kill, nice, grep, find, pipe, >, >>, ip, ping, ss | ~1500 |
| IV | Réseau, équipage, sauvegarde | Appliquer/Analyser | ufw, ssh, tar, zip, df, du, mount, useradd, groupadd, passwd, su, sudo | ~1500 |
| V | Durcissement & forteresse | Évaluer, Créer | apt, dpkg, journalctl, fail2ban, iptables, auditctl, chroot, ssh durci | ~500 |

**Progression des compétences**: un étudiant débloque les commandes de l'acte I avant l'acte II, etc. Chaque mission d'un acte N peut utiliser les commandes des actes < N (révision).

---

## 3. Variation Bloom

Chaque commande a plusieurs niveaux Bloom possibles. Le générateur choisit le niveau selon l'acte:
- Acte I: plutôt "Se souvenir"/"Comprendre" (découverte)
- Acte III: "Analyser" (combiner plusieurs commandes)
- Acte V: "Créer" (configurer, durcir)

Exemple pour `grep`:
- Se souvenir: "grep ALERTE fichier" (trouver un mot)
- Comprendre: "grep -c ALERTE fichier" (comprendre le compte)
- Analyser: "grep ALERTE log | grep 10.0.0.66" (croiser les sources)
- Créer: "grep -r ALERTE /securite/ > rapport" (synthétiser)

---

## 4. Cohérence Role Play

### 4.1 Arc narratif par acte
Chaque acte a une trame: I=arrivée, II=organisation, III=enquête pirates, IV=préparation défense, V=durcissement final.

### 4.2 PNJ récurrents
Marie (ingénieur), Jean (sécurité), Sophie (comms), Pierre (méchanicien), Amiral Kowalski. Chaque contexte choisit qui parle selon l'acte et le sujet.

### 4.3 Cibles cohérentes
- Acte I: cibles dans ~/ (cabine) et /communications/
- Acte II: ~/ et /soutes/
- Acte III: /securite/logs/, /salle-des-machines/
- Acte IV: /securite/, /hangar/, réseau
- Acte V: /securite/config/, durcissement

---

## 5. Implémentation technique

### 5.1 Fichiers
- `src/missions/generator.rs` — moteur de génération procédurale
- `src/missions/templates.rs` — templates des ~40 commandes
- `src/missions/acts.rs` — définition des 5 actes
- `src/missions/mod.rs` — load_all_missions() appelle le générateur si GAMESHELL_GEN=5000

### 5.2 Mode de génération
```bash
# 50 missions (défaut, pour tests rapides)
./gameshell.sh

# 5000 missions (génération procédurale)
GAMESHELL_MISSIONS=5000 ./gameshell.sh
```

### 5.3 Persistance
Les missions générées sont mises en cache dans `~/.gameshell/state/missions_cache.json` pour ne pas régénérer à chaque lancement.

---

## 6. Phases d'implémentation

| Phase | Livrable | Tests |
|-------|----------|-------|
| 1 | Templates des 40 commandes + structure actes | compile |
| 2 | Générateur procédural (seed fixe) | 50 missions générées = identiques aux manuelles |
| 3 | Génération 5000 missions | 25/25 tests passent, perf < 2s |
| 4 | Variations Bloom par acte | chaque acte a sa distribution Bloom |
| 5 | Cohérence RP (PNJ, cibles par acte) | revue manuelle de 20 missions aléatoires |
| 6 | Cache + README + commit | push master |

---

## 7. Critères de qualité d'une mission générée

1. ✅ 20+ lignes de story (règle C3)
2. ✅ Objectif narratif sans nommer la commande (règle C4)
3. ✅ Carte ASCII avec position ▶ (règle C6)
4. ✅ Double définition 🚀/💻 (règle C7)
5. ✅ Bloom cohérent avec l'acte (règle C9)
6. ✅ Commande débloquée dans un acte ≤ acte courant
7. ✅ Cible existe dans le vaisseau
8. ✅ Check vérifiable (fichier existe, commande tapée, etc.)
9. ✅ Noms exacts dans le dialogue PNJ si fichier requis (règle C5)
10. ✅ id unique

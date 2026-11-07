# 🚨 RÈGLES STRICTES DE DÉVELOPPEMENT — GameShell Orion-7

Ce fichier définit les règles **non négociables** pour toute modification du dépôt.
Toute régression par rapport à ces règles est un bug critique.

---

## RÈGLE 1 — Isolation du monde de jeu

- Le joueur ne voit JAMAIS le vrai système de fichiers hôte.
- Tous les chemins absolus `/...` sont mappés vers `~/.gameshell/vaisseau/...`.
- `ls /` doit afficher les compartiments du vaisseau (pont, soutes, armes...), jamais `/bin`, `/usr` du vrai système.
- Le prompt affiche `pseudo@orion7:chemin$` avec des chemins propres (`~`, `/pont/...`).

## RÈGLE 2 — Historique des commandes par mission

- Les commandes sont trackées **en mémoire** (`mission_commands: Vec<String>`).
- Ce vecteur est **reset à chaque changement de mission** ET au lancement du jeu.
- `gsh check` ne valide JAMAIS sur l'historique d'une session précédente.
- Test de non-régression: lancer une partie, taper `ls`, faire `gsh check` sur mission 1.1 → doit échouer (il manque `pwd`).

## RÈGLE 3 — Pas de validation automatique

- Une mission ne se termine JAMAIS automatiquement.
- Seul `gsh check` (tapé manuellement par le joueur) peut valider une mission.
- Après `gsh check` réussi, si la mission a un `code_required`, le jeu demande le code en phase 2.

## RÈGLE 4 — Affichage obligatoire

Au lancement et à chaque nouvelle mission, afficher:
1. Les commandes gsh disponibles (`gsh help` auto au start)
2. La mission avec: id, titre, histoire, objectif, **commandes à utiliser** (`required_commands`), XP
3. Le statut: `X/50 | Niv N | XP | skills`

## RÈGLE 4f — Vocabulaire strict: distinguer "commandes shell" et "commandes gsh" (OBLIGATOIRE)

Il existe DEUX types de commandes dans le jeu, jamais confondre:
- **Commandes SHELL** (pwd, ls, cd, cat...) = les vraies commandes Linux que le joueur apprend
- **Commandes GSH** (gsh goal, gsh check, gsh reset, gsh skills, gsh help) = les commandes de gestion DU JEU

Règles de wording:
- Toujours dire "commandes SHELL à utiliser" (ou "outils shell") quand on parle de pwd/ls/cd...
- Toujours préfixer "gsh " pour les commandes du jeu
- Jamais dire juste "commandes" seul quand ça peut prêter à confusion
- Dans gsh help, regrouper clairement: section "COMMANDES DU JEU (gsh)" et mention "les commandes shell (pwd, ls...) s'utilisent normalement"

## RÈGLE 4k — gsh goal = show_mission + compétence de départ = man (OBLIGATOIRE)

- `gsh goal` doit réafficher EXACTEMENT le même cadre que le début de mission (appelle `show_mission()`)
- La syntaxe shell dans gsh help = 2 lignes génériques uniquement:
  `COMMANDE` / `COMMANDE argument` / `COMMANDE -option argument` / `man COMMANDE`
- La première compétence débloquée (mission 1.1) doit être **man**
- `man COMMANDE` est autorisé dans toutes les missions

## RÈGLE 4j — Ne pas donner la réponse (OBLIGATOIRE)

Les missions ne doivent PAS dire explicitement quelle commande utiliser dans l'objectif.
L'objectif (`goal`) doit être **narratif**: "Marie attend votre réponse", "Prouvez votre identité"...
L'étudiant doit LIRE le contexte et DÉDUIRE quelle commande utiliser.
- Interdit: "Affichez votre position (pwd)"
- Autorisé: "Montrez-lui où vous êtes dans le vaisseau"
- La carte montre OÙ on est, jamais QUELLE commande taper
- Toute la story doit être affichée (pas de troncature) — l'étudiant doit tout lire

## RÈGLE 4h — Carte ASCII obligatoire par mission (OBLIGATOIRE)

Chaque mission DOIT avoir un champ `map` contenant une **carte vue de dessus du vaisseau**
en ASCII art avec une flèche (→ ou ▼) pointant vers le compartiment concerné par la mission.
Exemple:
```
  ┌─────────┐  ┌─────────┐  ┌─────────┐
  │  PONT   │  │ CABINES │  │ SOUTES  │
  └────┬────┘  └────┬────┘  └────┬────┘
       │            ▼              │
       │         [VOUS ÊTES ICI]   │
       └──────────→ /communications/messages/dernier_message.txt
```
Cette carte s'affiche dans le cadre de la mission. Interdit de valider une mission sans carte.

## RÈGLE 4i — Palette adaptée terminal fond clair (OBLIGATOIRE)

Le terminal xfce Debian 13 a un fond CLAIR par défaut:
- Texte courant = `.black()` ou `.normal()` (jamais blanc sur fond clair)
- Éléments importants (titres, cadres, commandes) = couleurs vives (vert, jaune, rouge)
- Jamais de texte noir invisible ou blanc sur blanc

## RÈGLE 4g — ASCII art bien formé (OBLIGATOIRE)

Tout ASCII art (vaisseau, cadres) doit être **visuellement reconnaissable**:
- Tester en le collant dans un terminal 80x24
- Les lignes doivent s'aligner correctement (pas de décalage)
- Un vaisseau doit ressembler à un vaisseau (coque, hublots, ailes, propulseurs)

## RÈGLE 4d — Cohérence des analogies role play (OBLIGATOIRE)

Chaque analogie role play doit être **logiquement cohérente** avec l'univers du vaisseau:
- Le hublot montre l'ESPACE, pas l'intérieur → ne jamais dire "regarder par le hublot pour voir la pièce"
- Une analogie doit pouvoir se visualiser concrètement dans le vaisseau
- Tester chaque analogie: "Est-ce que cette action a du sens à bord d'Orion-7?"

## RÈGLE 4e — Textes des commandes gsh cohérents

- `gsh reset` = "réinitialiser la mission en cours" (PAS "le vaisseau")
- Toute description de commande gsh doit dire ce qu'elle fait réellement dans le jeu

## RÈGLE 4c — Minimum 20 lignes de contexte par mission (OBLIGATOIRE)

Chaque mission doit contenir **au moins 20 lignes de texte** de mise en contexte
(champs `description` + `story` combinés, affichés au joueur). Cela inclut:
- La situation narrative (qui parle, pourquoi, l'urgence)
- Le contexte du vaisseau (où on est, ce qui se passe)
- L'objectif clair
- Les enjeux (que se passe-t-il si on échoue?)

Interdit de valider une mission avec moins de 20 lignes de contexte.
Vérification: `wc -l` sur le texte description+story de chaque mission dans les fichiers ch*.rs.

## RÈGLE 4b — Explication double obligatoire: role play PUIS technique

Chaque commande requise par la mission DOIT afficher **deux lignes**:
1. **Définition technique Linux** (ce que fait la commande réellement)
2. **Analogie role play** (l'action équivalente dans le vaisseau Orion-7)

L'ordre d'affichage est TOUJOURS: **role play d'abord, puis technique**.
Exemple:
```
🆕 Commandes à utiliser:
   pwd
     🚀 roleplay : comme regarder par le hublot pour savoir dans quel compartiment du vaisseau vous êtes
     💻 technique: affiche le chemin du répertoire courant
   ls
     🚀 roleplay : comme ouvrir la porte de votre cabine pour voir ce qu'elle contient
     💻 technique: liste les fichiers et dossiers du répertoire courant
```

Ces définitions proviennent de la fonction `cmd_brief()` qui retourne `(technique, roleplay)`.
**Interdit d'afficher une commande sans ses deux définitions.** C'est pédagogique: le joueur associe la commande Linux à une action concrète dans le vaisseau.

## RÈGLE 5 — Pas d'indices automatiques

- `gsh hint` est SUPPRIMÉ. Ne jamais le réintroduire.
- `gsh check` en échec affiche UNIQUEMENT: `❌ Non validé: <liste des cibles manquantes>`. Aucun commentaire d'aide.

## RÈGLE 6 — Parsing robuste

- Toujours utiliser `split_whitespace()` pour parser les commandes (gère espaces/tabs multiples).
- Jamais de `split(' ')`.

## RÈGLE 7 — Commandes interceptées

- `pwd` → affiche le chemin propre (display_path), pas le vrai chemin.
- `hostname` → affiche `orion7`.
- `whoami` → affiche `capitaine`.
- `cd` → met à jour `current_dir` avec résolution + canonicalisation (realpath).

## RÈGLE 8 — Structure du vaisseau

L'arborescence doit ressembler à un vaisseau spatial, pas un serveur Linux:
`/pont/ /cabines/ /soutes/ /reacteur/ /hangar/ /securite/ /communications/ /navigation/ /armes/ /salle-des-machines/ /archives/`

Générée par `vaisseau-gen.sh`. Ne jamais revenir à une arborescence type `/srv/`, `/etc/`, `/home/etudiant/`.

## RÈGLE 9 — Build et test

Avant chaque commit:
```bash
cargo build --release 2>&1 | grep -E "^error"   # doit être vide
bash make-archive.sh                               # régénère gameshell.sh
rm -rf ~/.gameshell && cd /tmp && cp ~/gameshell/gameshell.sh . && echo -e "Test\npwd\nls\ngsh check\nexit" | timeout 10 bash gameshell.sh
# Résultat attendu: ✅ Mission réussie (car pwd ET ls tapés)
```

Test de non-régression critique:
```bash
rm -rf ~/.gameshell && echo -e "Test\nls\ngsh check\nexit" | bash gameshell.sh
# Résultat attendu: ❌ Non validé (pwd manquant) — JAMAIS ✅
```

## RÈGLE 10 — Distribution

- Un seul fichier `gameshell.sh` auto-extractible, 100% offline.
- `./gameshell.sh` installe dans `~/.gameshell/` et lance.
- Aucune dépendance externe, aucun accès réseau requis.

## RÈGLE 4l — Bloom en français + footer 'gsh help' seul (OBLIGATOIRE)

- Les niveaux de Bloom s'affichent en FRANÇAIS: Se souvenir, Comprendre, Appliquer, Analyser, Évaluer, Créer
- Le footer de mission affiche UNIQUEMENT 'gsh help' (pas 'gsh check / gsh goal / gsh help')

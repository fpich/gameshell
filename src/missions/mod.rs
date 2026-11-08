//! Système de missions - Agrège tous les chapitres
pub mod types;
pub mod acts;
pub mod japon;
pub mod chapters {
    pub mod ch1;
    pub mod ch2;
    pub mod ch3;
    pub mod ch4;
    pub mod ch5;
    pub mod ch6;
    pub mod ch7;
    pub mod ch8;
    pub mod ch9;
    pub mod ch10;
    pub mod ch11;
    pub mod ch12;
    pub mod ch13;
    pub mod ch14;
    pub mod ch15;
    pub mod ch16;
    pub mod ch17;
    pub mod ch18;
    pub mod ch19;
    pub mod ch20;
}

pub use types::*;

/// 100 missions EN DUR par campagne (20 chapitres x 5 missions, 5 actes x 20 missions).
/// 100% offline, aucun générateur procédural (règle C1/C14).
/// Campagne 1 = Pirates de l'Espace (facile), 2 = Japon Féodal (moyen).
pub fn load_all_missions() -> Vec<Mission> {
    let campaign: u8 = std::env::var("GAMESHELL_CAMPAIGN").ok().and_then(|v| v.parse().ok()).unwrap_or(1);
    if campaign == 2 {
        return japon::missions();
    }
    let mut missions = Vec::new();
    missions.extend(chapters::ch1::missions());
    missions.extend(chapters::ch2::missions());
    missions.extend(chapters::ch3::missions());
    missions.extend(chapters::ch4::missions());
    missions.extend(chapters::ch5::missions());
    missions.extend(chapters::ch6::missions());
    missions.extend(chapters::ch7::missions());
    missions.extend(chapters::ch8::missions());
    missions.extend(chapters::ch9::missions());
    missions.extend(chapters::ch10::missions());
    missions.extend(chapters::ch11::missions());
    missions.extend(chapters::ch12::missions());
    missions.extend(chapters::ch13::missions());
    missions.extend(chapters::ch14::missions());
    missions.extend(chapters::ch15::missions());
    missions.extend(chapters::ch16::missions());
    missions.extend(chapters::ch17::missions());
    missions.extend(chapters::ch18::missions());
    missions.extend(chapters::ch19::missions());
    missions.extend(chapters::ch20::missions());
    missions
}

#[cfg(test)]

#[cfg(test)]
fn wrap60(l: &str) -> usize {
    // approxime le word wrap de show_mission (width 60)
    if l.trim().is_empty() { return 1; }
    let mut n = 1; let mut cur = 0;
    for w in l.split_whitespace() {
        if cur + w.len() + 1 > 60 && cur > 0 { n += 1; cur = w.len(); } else { cur += w.len() + 1; }
    }
    n
}

#[cfg(test)]
mod conformity_tests {
    //! Tests de conformité (AGENTS.md) — figent les règles C/B/A sur le contenu.
    use super::*;

    const BLOOM_FR: &[&str] = &["Se souvenir", "Comprendre", "Appliquer", "Analyser", "Évaluer", "Créer"];
    const PNJ_BLACKLIST: &[&str] = &["Marie", "Sophie", "Jean", "Pierre", "Kowalski", "Galactica", "Lucas", "Emma", "Hugo", "Léa", "Chloé", "Camille"];
    const SYMBOLIC: &[&str] = &["|", ">", ">>"];

    fn skill_ok(m: &Mission) -> bool {
        match &m.skill {
            None => false,
            Some(s) => {
                let words: Vec<&str> = m.required_commands.iter().flat_map(|c| c.split_whitespace()).collect();
                let base = s.name.split_whitespace().next().unwrap_or(&s.name);
                words.iter().any(|w| *w == base)
                    || (SYMBOLIC.contains(&s.name.as_str()) && m.required_commands.iter().any(|c| c.contains(s.name.as_str())))
            }
        }
    }

    fn check_missions(missions: &[Mission], label: &str) {
        assert!(!missions.is_empty(), "{}: aucune mission", label);
        for m in missions {
            assert!(!m.required_commands.is_empty(), "{} {}: required_commands vide", label, m.id);
            assert!(!m.checks.is_empty(), "{} {}: checks vide", label, m.id);
            assert!(skill_ok(m), "{} {}: skill {:?} n'est pas une commande requise de la mission (règle A15)", label, m.id, m.skill.as_ref().map(|s| s.name.clone()));
            assert!(BLOOM_FR.contains(&m.bloom_level.as_str()), "{} {}: bloom_level {:?} pas en français (règle B8/C9)", label, m.id, m.bloom_level);
            assert!(m.map.contains("VUE DE DESSUS"), "{} {}: pas de carte vue de dessus (règle C6)", label, m.id);
            // Règle B9: carte ASCII bien formée — lignes de la carte alignées (même largeur) et <= 60 car.
            // traits lourds (┏┃┗┛┳┻) comme le cadre; jonction ┻ alignée sous les ┳
            let w0 = m.map.lines().find(|l| l.contains('┏')).map(|l| l.chars().count());
            let mut sy = Vec::new();
            for l in m.map.lines() {
                assert!(l.chars().count() <= 60, "{} {}: ligne de carte trop longue ({} car.)", label, m.id, l.chars().count());
                if l.contains('┏') || l.contains('┃') {
                    if let Some(w) = w0 { assert_eq!(l.chars().count(), w, "{} {}: carte mal alignée (règle B9): {:?}", label, m.id, l); }
                }
                if l.contains('┳') { sy = l.chars().enumerate().filter(|(_, c)| *c == '┳').map(|(i, _)| i).collect(); }
                if l.contains('┻') {
                    let up: Vec<usize> = l.chars().enumerate().filter(|(_, c)| *c == '┻').map(|(i, _)| i).collect();
                    assert_eq!(sy, up, "{} {}: jonction ┻ non alignée sous les ┳ (règle B9)", label, m.id);
                }
            }
            assert!(m.map.contains('▶'), "{} {}: la carte doit marquer la position (▶)", label, m.id);
            // Règle C3: 20+ lignes de contexte AFFICHÉES (wrap à 60 car., comme show_mission)
            let disp = m.story.lines().map(|l| wrap60(l)).sum::<usize>() + m.description.lines().map(|l| wrap60(l)).sum::<usize>();
            assert!(disp >= 18, "{} {}: contexte trop court ({} lignes affichées, règle C3: 20+)", label, m.id, disp);
            let all_text = format!("{} {} {} {} {}", m.title, m.description, m.story, m.goal, m.map);
            for bad in PNJ_BLACKLIST {
                assert!(!all_text.split_whitespace().any(|w| w.trim_matches(|c: char| !c.is_alphabetic()).starts_with(bad) && w.starts_with(bad)),
                    "{} {}: nom PNJ interdit '{}' (règle C13)", label, m.id, bad);
            }
        }
    }

    #[test]
    fn missions_conformes() {
        for camp in [1u8, 2] {
            std::env::set_var("GAMESHELL_CAMPAIGN", camp.to_string());
            let ms = load_all_missions();
            std::env::remove_var("GAMESHELL_CAMPAIGN");
            let label = if camp == 2 { "japon" } else { "espace" };
            assert_eq!(ms.len(), 100, "campagne {}: 100 missions en dur attendues (20 chapitres x 5, 5 actes x 20)", label);
            check_missions(&ms, label);
            // 20 missions par acte (règle C1)
            for act in 1..=5u32 {
                let n = ms.iter().filter(|m| m.act == act).count();
                assert_eq!(n, 20, "{} acte {} doit avoir 20 missions, trouvé {}", label, act, n);
            }
        }
    }

    #[test]
    fn vaisseau_gen_sans_noms_interdits() {
        let content = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/vaisseau-gen.sh")).unwrap();
        for bad in PNJ_BLACKLIST {
            assert!(!content.contains(bad), "vaisseau-gen.sh contient le nom interdit '{}' (règle C13)", bad);
        }
        assert!(!content.contains("Capitaine Fabien"), "vaisseau-gen.sh: nom du joueur en dur interdit (règle A12)");
    }
}

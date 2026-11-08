//! Système de missions - Agrège tous les chapitres
pub mod types;
pub mod acts;
pub mod generator;
pub mod generated;
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
}

pub use types::*;

pub fn load_all_missions() -> Vec<Mission> {
    // Mode 5000 missions EN DUR (compilées dans le binaire, 100% offline, sans LLM)
    if std::env::var("GAMESHELL_MISSIONS").map(|v| v == "all" || v == "500").unwrap_or(false) {
        return generated::generated_missions();
    }
    // Mode génération procédurale runtime si demandé
    if let Ok(n) = std::env::var("GAMESHELL_MISSIONS") {
        if let Ok(count) = n.parse::<usize>() {
            if count > 50 && count != 5000 {
                return generator::generate(count);
            }
        }
    }
    // Mode par défaut: 50 missions manuelles
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
    missions
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
            assert!(m.map.contains("VUE DE DESSUS") || m.map.contains("VUE"), "{} {}: pas de carte vue de dessus (règle C6)", label, m.id);
            assert!(m.story.lines().count() + m.description.lines().count() >= 10, "{} {}: contexte trop court (règle C3)", label, m.id);
            let all_text = format!("{} {} {} {} {}", m.title, m.description, m.story, m.goal, m.map);
            for bad in PNJ_BLACKLIST {
                assert!(!all_text.split_whitespace().any(|w| w.trim_matches(|c: char| !c.is_alphabetic()).starts_with(bad) && w.starts_with(bad)),
                    "{} {}: nom PNJ interdit '{}' (règle C13)", label, m.id, bad);
            }
        }
    }

    #[test]
    fn missions_conformes() {
        // Un seul test séquentiel: la variable d'env GAMESHELL_MISSIONS est globale au process
        let ms = load_all_missions();
        assert_eq!(ms.len(), 50, "50 missions manuelles attendues");
        check_missions(&ms, "manuel");

        std::env::set_var("GAMESHELL_MISSIONS", "all");
        let ms = load_all_missions();
        std::env::remove_var("GAMESHELL_MISSIONS");
        assert_eq!(ms.len(), 500, "500 missions embarquées attendues");
        check_missions(&ms, "genere");
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

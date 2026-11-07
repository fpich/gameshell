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

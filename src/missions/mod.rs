//! Système de missions - Agrège tous les chapitres
pub mod types;
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

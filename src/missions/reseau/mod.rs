//! Campagne 3 — NEXUS RÉSEAU (NOC Neon-9, niveau DIFFICILE) — 100 missions en dur, 5 actes x 20
use crate::missions::types::*;

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

pub fn missions() -> Vec<Mission> {
    let mut missions = Vec::new();
    missions.extend(ch1::missions());
    missions.extend(ch2::missions());
    missions.extend(ch3::missions());
    missions.extend(ch4::missions());
    missions.extend(ch5::missions());
    missions.extend(ch6::missions());
    missions.extend(ch7::missions());
    missions.extend(ch8::missions());
    missions.extend(ch9::missions());
    missions.extend(ch10::missions());
    missions.extend(ch11::missions());
    missions.extend(ch12::missions());
    missions.extend(ch13::missions());
    missions.extend(ch14::missions());
    missions.extend(ch15::missions());
    missions.extend(ch16::missions());
    missions.extend(ch17::missions());
    missions.extend(ch18::missions());
    missions.extend(ch19::missions());
    missions.extend(ch20::missions());
    missions
}

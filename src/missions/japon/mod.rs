//! Campagne 2 — LE JAPON FÉODAL (Château Shiro, niveau MOYEN) — 100 missions en dur, 5 actes x 20
use crate::missions::types::*;

pub mod ch1;
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
pub mod ch2;
pub mod ch20;
pub mod ch3;
pub mod ch4;
pub mod ch5;
pub mod ch6;
pub mod ch7;
pub mod ch8;
pub mod ch9;

pub fn missions() -> Vec<Mission> {
    crate::collect_chapters!(
        japon, ch1, ch2, ch3, ch4, ch5, ch6, ch7, ch8, ch9, ch10, ch11, ch12, ch13, ch14, ch15,
        ch16, ch17, ch18, ch19, ch20,
    )
}

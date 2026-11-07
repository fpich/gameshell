//! Définition des 5 actes narratifs

pub struct Act {
    pub num: u32,
    pub title: &'static str,
    pub situation: &'static str,
}

pub fn acts_definition() -> Vec<Act> {
    vec![
        Act { num: 1, title: "Arrivée à bord", situation: "vous venez d'amarrer, la station est mystérieuse" },
        Act { num: 2, title: "Organisation de la station", situation: "il faut classer et protéger les données avant l'attaque" },
        Act { num: 3, title: "Enquête sur les pirates", situation: "les Corbeaux Noirs ont laissé des traces dans les logs" },
        Act { num: 4, title: "Préparation de la défense", situation: "l'Amiral exige que la station soit prête au combat" },
        Act { num: 5, title: "Durcissement final", situation: "dernière ligne droite: rendre Orion-7 imprenable" },
    ]
}

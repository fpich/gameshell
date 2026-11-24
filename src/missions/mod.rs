//! Système de missions - Agrège tous les chapitres
pub mod japon;
pub mod reseau;
pub mod types;
pub mod chapters {
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
}

pub use types::*;

/// 100 missions EN DUR par campagne (20 chapitres x 5 missions, 5 actes x 20 missions).
/// 100% offline, aucun générateur procédural (règle C1/C14).
/// Campagne 1 = Pirates de l'Espace (facile), 2 = Japon Féodal (moyen), 3 = Nexus Réseau (difficile).
pub fn load_all_missions() -> Vec<Mission> {
    let campaign: u8 = std::env::var("GAMESHELL_CAMPAIGN")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    match campaign {
        2 => return japon::missions(),
        3 => return reseau::missions(),
        _ => {}
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
fn wrap60(l: &str) -> usize {
    // approxime le retour à la ligne de show_mission (≈ 60 colonnes)
    if l.trim().is_empty() {
        return 1;
    }
    let mut n = 1;
    let mut cur = 0;
    for w in l.split_whitespace() {
        if cur + w.len() + 1 > 60 && cur > 0 {
            n += 1;
            cur = w.len();
        } else {
            cur += w.len() + 1;
        }
    }
    n
}

#[cfg(test)]
mod conformity_tests {
    //! Tests de conformité (AGENTS.md) — figent les règles A/B/C sur le contenu des 300 missions.
    use super::*;
    use crate::game::parse;

    const BLOOM_FR: &[&str] = &[
        "Se souvenir",
        "Comprendre",
        "Appliquer",
        "Analyser",
        "Évaluer",
        "Créer",
    ];
    const PNJ_BLACKLIST: &[&str] = &[
        "Marie",
        "Sophie",
        "Jean",
        "Pierre",
        "Kowalski",
        "Galactica",
        "Lucas",
        "Emma",
        "Hugo",
        "Léa",
        "Chloé",
        "Camille",
    ];
    const SYMBOLIC: &[&str] = &["|", ">", ">>"];
    /// mots français qui sont aussi des noms de commande (tolérés dans un objectif)
    const FRENCH_WORDS: &[&str] = &["date", "su", "du"];

    fn all_campaigns() -> Vec<(u8, Vec<Mission>)> {
        let _lock = CAMPAIGN_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        [1u8, 2, 3]
            .iter()
            .map(|c| {
                std::env::set_var("GAMESHELL_CAMPAIGN", c.to_string());
                let ms = load_all_missions();
                std::env::remove_var("GAMESHELL_CAMPAIGN");
                (*c, ms)
            })
            .collect()
    }

    fn label(c: u8) -> &'static str {
        match c {
            2 => "japon",
            3 => "reseau",
            _ => "espace",
        }
    }

    fn skill_ok(m: &Mission) -> bool {
        match &m.skill {
            None => false,
            Some(s) => {
                let words: Vec<&str> = m
                    .required_commands
                    .iter()
                    .flat_map(|c| c.split_whitespace())
                    .collect();
                let base = s.name.split_whitespace().next().unwrap_or(&s.name);
                words.contains(&base)
                    || (SYMBOLIC.contains(&s.name.as_str())
                        && m.required_commands
                            .iter()
                            .any(|c| c.contains(s.name.as_str())))
            }
        }
    }

    fn check_missions(missions: &[Mission], label: &str, errs: &mut Vec<String>) {
        macro_rules! chk { ($c:expr, $($t:tt)*) => { if !$c { errs.push(format!($($t)*)); } } }
        chk!(!missions.is_empty(), "{}: aucune mission", label);
        for m in missions {
            chk!(
                !m.required_commands.is_empty(),
                "{} {}: required_commands vide",
                label,
                m.id
            );
            chk!(!m.checks.is_empty(), "{} {}: checks vide", label, m.id);
            // Invariant A4: chaque check de commande est atteignable par les required_commands
            let parsed: Vec<parse::Parsed> = m
                .required_commands
                .iter()
                .map(|c| parse::parse(c))
                .collect();
            for ck in &m.checks {
                if ck.check_type == "command_executed" {
                    chk!(
                        parse::command_matches(&ck.target, &m.required_commands, &parsed),
                        "{} {}: check '{}' non atteignable par les required_commands",
                        label,
                        m.id,
                        ck.target
                    );
                }
                if ck.check_type == "file_read" {
                    chk!(
                        parse::file_read_matches(&ck.target, &parsed),
                        "{} {}: lecture de '{}' non atteignable",
                        label,
                        m.id,
                        ck.target
                    );
                }
            }
            chk!(
                skill_ok(m),
                "{} {}: skill {:?} n'est pas une commande requise de la mission (règle A15)",
                label,
                m.id,
                m.skill.as_ref().map(|s| s.name.clone())
            );
            chk!(
                BLOOM_FR.contains(&m.bloom_level.as_str()),
                "{} {}: bloom_level {:?} pas en français (règle B8/C9)",
                label,
                m.id,
                m.bloom_level
            );
            // Règle C3: 20+ lignes de contexte AFFICHÉES
            let disp = m.story.lines().map(wrap60).sum::<usize>()
                + m.description.lines().map(wrap60).sum::<usize>();
            chk!(
                disp >= 20,
                "{} {}: contexte trop court ({} lignes affichées, règle C3: 20+)",
                label,
                m.id,
                disp
            );
            let all_text = format!("{} {} {} {}", m.title, m.description, m.story, m.goal);
            for bad in PNJ_BLACKLIST {
                chk!(
                    !all_text.split_whitespace().any(|w| w
                        .trim_matches(|c: char| !c.is_alphabetic())
                        .starts_with(bad)
                        && w.starts_with(bad)),
                    "{} {}: nom PNJ interdit '{}' (règle C13)",
                    label,
                    m.id,
                    bad
                );
            }
            // Règle C4: l'objectif ne nomme jamais la commande (ni option, pipe, parenthèse de syntaxe)
            let goal_words: Vec<String> = m
                .goal
                .split_whitespace()
                .map(|w| {
                    w.trim_matches(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
                        .to_string()
                })
                .collect();
            for p in &parsed {
                for (b, _) in p.effective() {
                    if b.len() >= 2 && !FRENCH_WORDS.contains(&b.as_str()) {
                        chk!(
                            !goal_words.contains(&b),
                            "{} {}: l'objectif nomme la commande '{}' (règle C4): {}",
                            label,
                            m.id,
                            b,
                            m.goal
                        );
                    }
                }
            }
            chk!(
                !m.goal.contains(" | ") && !m.goal.contains("(sudo") && !m.goal.contains(" -l)"),
                "{} {}: syntaxe shell dans l'objectif (règle C4): {}",
                label,
                m.id,
                m.goal
            );
            // Règle C5: le nom exact des fichiers attendus figure dans le dialogue ou l'objectif
            for ck in &m.checks {
                if matches!(ck.check_type.as_str(), "file_exists" | "file_read") {
                    let base = ck
                        .target
                        .trim_end_matches('/')
                        .rsplit('/')
                        .next()
                        .unwrap_or(&ck.target);
                    chk!(m.story.contains(base) || m.goal.contains(base), "{} {}: le nom '{}' n'apparaît ni dans l'histoire ni dans l'objectif (règle C5)", label, m.id, base);
                }
            }
            // P1.1: « echo <toutes les cibles> » et « ls /armes » ne valident jamais un check de commande
            let echo = format!(
                "echo {}",
                m.checks
                    .iter()
                    .map(|c| format!("'{}'", c.target))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            let traps = vec![
                parse::parse(&echo),
                parse::parse("ls /armes"),
                parse::parse("foobar123"),
            ];
            for ck in &m.checks {
                if ck.check_type == "command_executed"
                    && !matches!(
                        ck.target.split_whitespace().next(),
                        Some("ls") | Some("echo")
                    )
                {
                    chk!(
                        !parse::command_matches(&ck.target, &m.required_commands, &traps),
                        "{} {}: '{}' validé par un piège (echo/ls/garbage)",
                        label,
                        m.id,
                        ck.target
                    );
                }
                if ck.check_type == "file_read" {
                    chk!(
                        !parse::file_read_matches(&ck.target, &traps),
                        "{} {}: lecture '{}' validée par echo",
                        label,
                        m.id,
                        ck.target
                    );
                }
            }
        }
    }

    pub static CAMPAIGN_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn missions_conformes() {
        for (camp, ms) in all_campaigns() {
            let label = label(camp);
            assert_eq!(
                ms.len(),
                100,
                "campagne {}: 100 missions en dur attendues (20 chapitres x 5, 5 actes x 20)",
                label
            );
            let mut errs = Vec::new();
            check_missions(&ms, label, &mut errs);
            assert!(
                errs.is_empty(),
                "{} non-conformités:\n{}",
                errs.len(),
                errs.join("\n")
            );
            for act in 1..=5u32 {
                let n = ms.iter().filter(|m| m.act == act).count();
                assert_eq!(
                    n, 20,
                    "{} acte {} doit avoir 20 missions, trouvé {}",
                    label, act, n
                );
            }
            let act1_creer = ms
                .iter()
                .filter(|m| m.act == 1 && m.bloom_level == "Créer")
                .count();
            assert_eq!(
                act1_creer, 0,
                "{}: l'acte 1 ne doit pas contenir de mission 'Créer' (difficulté progressive)",
                label
            );
            let act5_creer = ms
                .iter()
                .filter(|m| m.act == 5 && m.bloom_level == "Créer")
                .count();
            assert!(
                act5_creer >= 1,
                "{}: l'acte 5 doit contenir au moins une mission 'Créer'",
                label
            );
        }
    }

    /// A17: un check d'état système ne doit jamais être déjà vrai dans l'état initial
    #[test]
    fn checks_systeme_jamais_vrais_au_depart() {
        let tmp = std::env::temp_dir().join(format!("gsh_a17_{}", std::process::id()));
        for (camp, ms) in all_campaigns() {
            let etc = tmp.join(format!("etc{}", camp));
            crate::sim::init_system(etc.to_str().unwrap(), camp, true);
            let ctx = crate::sim::Ctx::host(camp, "/", etc.to_str().unwrap());
            for m in &ms {
                for ck in &m.checks {
                    let e = ck.expected.clone().unwrap_or_default();
                    let line_has = |name: &str| {
                        let parts: Vec<&str> = e.split("&&").map(|s| s.trim()).collect();
                        ctx.lines(name)
                            .iter()
                            .any(|l| parts.iter().all(|p| l.contains(p)))
                    };
                    let vacuous = match ck.check_type.as_str() {
                        "user_exists" => crate::sim::users::user_exists(&ctx, &ck.target),
                        "group_exists" => crate::sim::users::group_exists(&ctx, &ck.target),
                        "user_in_group" => crate::sim::users::user_groups(&ctx, &ck.target)
                            .iter()
                            .any(|(_, g)| *g == e),
                        "user_has_password" => crate::sim::users::has_password(&ctx, &ck.target),
                        "package_installed" => crate::sim::admin::installed(&ctx, &ck.target),
                        "process_absent" => !crate::sim::procs::running(&ctx, &ck.target),
                        "sim_contains" => line_has(&ck.target),
                        "sim_not_contains" => !line_has(&ck.target),
                        "permission" | "file_not_exists" => {
                            ck.expected.is_none() && ck.check_type == "permission"
                        }
                        _ => false,
                    };
                    assert!(
                        !vacuous,
                        "{} {}: check {}:{} déjà vrai dans l'état initial (règle A17)",
                        label(camp),
                        m.id,
                        ck.check_type,
                        ck.target
                    );
                }
            }
        }
        let _ = std::fs::remove_dir_all(&tmp);
    }

    /// A12: les fichiers lus / verrouillés par les missions existent dans le monde généré
    #[test]
    fn cibles_existent_dans_le_monde() {
        let root = env!("CARGO_MANIFEST_DIR");
        let tmp = std::env::temp_dir().join(format!("gsh_a12_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&tmp);
        for (camp, ms) in all_campaigns() {
            let (dir, home) = match camp {
                2 => ("monde_japon", "/quartiers/seigneur"),
                3 => ("monde_reseau", "/noc/netrunner"),
                _ => ("vaisseau", "/cabines/capitaine"),
            };
            let world = tmp.join(dir);
            let ok = std::process::Command::new("bash")
                .arg(format!("{}/vaisseau-gen.sh", root))
                .arg(label(camp))
                .arg(&world)
                .env("GAMESHELL_HOME", &tmp)
                .stdout(std::process::Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            assert!(ok, "génération du monde {} impossible", label(camp));
            let mut names: Vec<String> = Vec::new();
            fn walk(p: &std::path::Path, out: &mut Vec<String>) {
                if let Ok(rd) = std::fs::read_dir(p) {
                    for e in rd.flatten() {
                        out.push(e.file_name().to_string_lossy().to_string());
                        if e.path().is_dir() {
                            walk(&e.path(), out);
                        }
                    }
                }
            }
            walk(&world, &mut names);
            let produced: Vec<String> = ms
                .iter()
                .flat_map(|m| {
                    m.required_commands.iter().flat_map(|c| {
                        c.split_whitespace()
                            .map(|w| {
                                w.trim_end_matches('/')
                                    .rsplit('/')
                                    .next()
                                    .unwrap_or(w)
                                    .to_string()
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            for m in &ms {
                for ck in &m.checks {
                    let base = ck
                        .target
                        .trim_end_matches('/')
                        .rsplit('/')
                        .next()
                        .unwrap_or(&ck.target)
                        .to_string();
                    if ck.check_type == "file_read" {
                        assert!(
                            names.contains(&base) || produced.contains(&base),
                            "{} {}: fichier à lire '{}' absent du monde généré (règle A12)",
                            label(camp),
                            m.id,
                            ck.target
                        );
                    }
                    if ck.check_type == "permission" {
                        let p = ck.target.replacen('~', home, 1);
                        assert!(
                            world.join(p.trim_start_matches('/')).exists(),
                            "{} {}: cible de permission '{}' absente du monde (A12)",
                            label(camp),
                            m.id,
                            ck.target
                        );
                    }
                }
            }
        }
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn vaisseau_gen_sans_noms_interdits() {
        // Règle C13: aucun nom contemporain banal dans les mondes générés (3 campagnes)
        for gen in [
            "vaisseau-gen.sh",
            "monde-japon-gen.sh",
            "monde-reseau-gen.sh",
        ] {
            let content =
                std::fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), gen)).unwrap();
            for bad in PNJ_BLACKLIST {
                assert!(
                    !content.contains(bad),
                    "{} contient le nom interdit '{}' (règle C13)",
                    gen,
                    bad
                );
            }
            assert!(
                !content.contains("Capitaine Fabien"),
                "{}: nom du joueur en dur interdit (règle A12)",
                gen
            );
        }
        let brief =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/game/brief.rs"))
                .unwrap();
        for bad in PNJ_BLACKLIST {
            assert!(
                !brief.contains(bad),
                "cmd_brief (game/brief.rs) contient le nom interdit '{}' (règle C13)",
                bad
            );
        }
        assert!(
            !brief.contains("Fabien"),
            "cmd_brief: nom du joueur en dur interdit (règle A12)"
        );
    }

    /// Mojibake UTF-8: aucune chaîne de mission ne doit contenir les séquences de décodage
    /// latin-1 typiques (Ã, Â, â€, ï¿½). Le mojibake est un défaut invisible à la compilation.
    #[test]
    fn stories_sans_mojibake() {
        const MOJIBAKE_MARKERS: &[&str] = &["Ã", "Â", "â€", "Ã§", "Ã©", "ï¿½"];
        for (camp, ms) in all_campaigns() {
            for m in &ms {
                let fields: [(&str, &String); 4] = [
                    ("story", &m.story),
                    ("description", &m.description),
                    ("goal", &m.goal),
                    ("title", &m.title),
                ];
                for (field, text) in fields {
                    for marker in MOJIBAKE_MARKERS {
                        assert!(
                            !text.contains(marker),
                            "{} {}: {} contient la séquence mojibake '{}'",
                            label(camp),
                            m.id,
                            field,
                            marker
                        );
                    }
                }
            }
        }
    }
}

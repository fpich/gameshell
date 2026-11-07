#!/usr/bin/env bash
# Génère src/missions/generated.rs — 5000 missions EN DUR (compilées dans le binaire)
# Usage: ./gen_missions.rs  (depuis la racine du projet)
# Le binaire embarque alors les missions, aucune génération à runtime, 100% offline.

set -e
cd "$(dirname "$0")"
COUNT=5000
OUT="src/missions/generated.rs"

echo "🎲 Génération de $COUNT missions en dur dans $OUT..."

# Le générateur Rust produit le fichier via un petit programme temporaire
cat > /tmp/gen_main.rs << 'EOF'

#[derive(Debug, Clone)]
pub struct Skill { pub command: String, pub name: String, pub description: String }
#[derive(Debug, Clone)]
pub struct MissionCheck { pub check_type: String, pub target: String, pub expected: Option<String> }
#[derive(Debug, Clone)]
pub struct Mission { pub id: String, pub chapter: u32, pub chapter_title: String, pub act: u32, pub title: String, pub description: String, pub story: String, pub goal: String, pub bloom_level: String, pub bloom_question: String, pub required_commands: Vec<String>, pub checks: Vec<MissionCheck>, pub hints: Vec<String>, pub xp_reward: u64, pub skill: Option<Skill>, pub code_required: Option<String>, pub map: String }
;
include!("/home/fabien/gameshell/src/missions/acts.rs");
include!("/home/fabien/gameshell/src/missions/generator.rs");
fn main() {
    let missions = generate(5000);
    let mut out = String::from("//! Missions générées EN DUR — 5000 missions, offline, sans LLM\n");
    out.push_str("use super::types::*;\n\npub fn generated_missions() -> Vec<Mission> {\n    vec![\n");
    for m in &missions {
        out.push_str(&format!(
            "        Mission {{ id: {:?}.into(), chapter: {}, chapter_title: {:?}.into(), act: {}, title: {:?}.into(), description: {:?}.into(), story: {:?}.into(), goal: {:?}.into(), bloom_level: {:?}.into(), bloom_question: {:?}.into(), required_commands: vec![{:?}.into()], checks: vec![MissionCheck {{ check_type: {:?}.into(), target: {:?}.into(), expected: {:?} }}], hints: vec![], xp_reward: {}, code_required: None, skill: Some(Skill {{ command: {:?}.into(), name: {:?}.into(), description: {:?}.into() }}), map: {:?}.into() }},\n",
            m.id, m.chapter, m.chapter_title, m.act, m.title, m.description, m.story, m.goal,
            m.bloom_level, m.bloom_question,
            m.required_commands.first().map(|s| s.as_str()).unwrap_or(""),
            m.checks.first().map(|c| c.check_type.as_str()).unwrap_or(""),
            m.checks.first().map(|c| c.target.as_str()).unwrap_or(""),
            m.checks.first().and_then(|c| c.expected.as_deref()).map(|s| format!("Some({:?}.into())", s)).unwrap_or("None".to_string()),
            m.xp_reward,
            m.skill.as_ref().map(|s| s.command.as_str()).unwrap_or(""),
            m.skill.as_ref().map(|s| s.name.as_str()).unwrap_or(""),
            m.skill.as_ref().map(|s| s.description.as_str()).unwrap_or(""),
            m.map
        ));
    }
    out.push_str("    ]\n}\n");
    std::fs::write("/home/fabien/gameshell/src/missions/generated.rs", out).unwrap();
    eprintln!("✅ {} missions écrites dans src/missions/generated.rs", missions.len());
}
EOF

# Compiler et exécuter le générateur standalone
rustc --edition 2021 -O /tmp/gen_main.rs -o /tmp/gen_missions 2>&1 | grep -E "^error" || true
/tmp/gen_missions
wc -l "$OUT"
echo "✅ Fichier généré: $OUT ($(du -h $OUT | cut -f1))"

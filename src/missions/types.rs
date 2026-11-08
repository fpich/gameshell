//! Types de missions
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub command: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionCheck {
    pub check_type: String,
    pub target: String,
    pub expected: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub id: String,
    pub chapter: u32,
    pub chapter_title: String,
    pub act: u32,
    pub title: String,
    pub description: String,
    pub story: String,
    pub goal: String,               // Objectif en langage naturel (affiché au joueur)
    pub bloom_level: String,
    pub bloom_question: String,
    pub required_commands: Vec<String>,  // Caché, utilisé pour la vérification interne
    pub checks: Vec<MissionCheck>,
    pub hints: Vec<String>,
    pub xp_reward: u64,
    pub skill: Option<Skill>,
    pub code_required: Option<String>,  // Code à entrer en phase 2 de gsh check
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::common::{ApiResource, NamedApiResource, VersionGameIndex};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub base_experience: Option<u32>,
    pub height: u32,
    pub weight: u32,
    pub is_default: bool,
    pub order: u32,

    pub abilities: Vec<PokemonAbility>,
    pub forms: Vec<NamedApiResource>,
    pub game_indices: Vec<VersionGameIndex>,
    pub held_items: Vec<HeldItem>,
    pub location_area_encounters: String,
    pub moves: Vec<PokemonMove>,
    pub species: NamedApiResource,
    pub stats: Vec<PokemonStat>,
    pub types: Vec<PokemonTypeSlot>,

    #[serde(default)]
    pub past_types: Vec<PastType>,

    #[serde(default)]
    pub past_abilities: Vec<PastAbility>,

    /// Explicitly acknowledged but not modeled.
    /// This preserves the full payload without schema explosion.
    pub sprites: Value,

    #[serde(default)]
    pub cries: Option<PokemonCries>,
}

/* ---------- Abilities ---------- */

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: u8,
    pub ability: Option<NamedApiResource>,
}

/* ---------- Held Items ---------- */

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HeldItem {
    pub item: NamedApiResource,
    pub version_details: Vec<HeldItemVersionDetail>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HeldItemVersionDetail {
    pub rarity: u32,
    pub version: NamedApiResource,
}

/* ---------- Moves ---------- */

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub move_: NamedApiResource,
    pub version_group_details: Vec<MoveVersionGroupDetail>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MoveVersionGroupDetail {
    pub level_learned_at: u32,
    pub move_learn_method: NamedApiResource,
    pub version_group: NamedApiResource,
}

/* ---------- Stats ---------- */

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PokemonStat {
    pub base_stat: u32,
    pub effort: u32,
    pub stat: NamedApiResource,
}

/* ---------- Types ---------- */

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PokemonTypeSlot {
    pub slot: u8,

    #[serde(rename = "type")]
    pub ty: NamedApiResource,
}

/* ---------- Past Types ---------- */

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PastType {
    pub generation: NamedApiResource,
    pub types: Vec<PokemonTypeSlot>,
}

/* ---------- Past Abilities ---------- */

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PastAbility {
    pub generation: NamedApiResource,
    pub abilities: Vec<PokemonAbility>,
}

/* ---------- Cries (newer field) ---------- */

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PokemonCries {
    pub latest: String,
    pub legacy: String,
}

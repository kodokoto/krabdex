use serde::{Deserialize, Serialize};

use crate::models::common::{NamedApiResource, Name};

/// PokeAPI Generation resource.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Generation {
    pub id: u32,
    pub name: String,

    pub abilities: Vec<NamedApiResource>,
    pub moves: Vec<NamedApiResource>,
    pub pokemon_species: Vec<NamedApiResource>,
    pub types: Vec<NamedApiResource>,
    pub version_groups: Vec<NamedApiResource>,

    pub main_region: NamedApiResource,
    pub names: Vec<Name>,
}

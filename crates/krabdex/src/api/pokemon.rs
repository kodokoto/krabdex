use crate::{
    client::{PokeApiClient, pagination::page_query},
    error::Result,
    models::{NamedAPIResourceList, Pokemon},
    types::{pagination::PageRequest, identifiers::{PokemonName, PokemonRef}},
};

impl PokeApiClient {
    /// Fetch a Pokemon by numeric id.
    pub async fn pokemon_by_id(&self, id: u32) -> Result<Pokemon> {
        let path = format!("pokemon/{id}");
        self.get_json(&path, None).await
    }

    /// Fetch a Pokemon by validated name (lowercase, hyphenated).
    pub async fn pokemon_by_name(&self, name: PokemonName<'_>) -> Result<Pokemon> {
        let path = format!("pokemon/{}", name.as_str());
        self.get_json(&path, None).await
    }

    /// Fetch a Pokemon by id or name (convenience API).
    pub async fn pokemon(&self, pokemon: PokemonRef<'_>) -> Result<Pokemon> {
        match pokemon {
            PokemonRef::Id(id) => self.pokemon_by_id(id).await,
            PokemonRef::Name(name) => self.pokemon_by_name(name).await,
        }
    }

    /// List Pokemon resources (name+url) with pagination.
    ///
    /// This maps to GET /pokemon?limit=...&offset=...
    pub async fn pokemon_list(&self, page: PageRequest) -> Result<NamedAPIResourceList<Pokemon>> {
        let q = page_query(page);
        self.get_json("pokemon", Some(q)).await
    }
}

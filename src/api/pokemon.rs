use crate::{
    client::PokeApiClient,
    error::Result,
    http::Query,
    models::{common::NamedApiResource, pokemon::Pokemon},
    types::identifiers::{PokemonName, PokemonRef},
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
    pub async fn pokemon_list(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<crate::models::common::Page<NamedApiResource>> {
        let mut q = Query::new();
        q.set("limit", limit.to_string());
        q.set("offset", offset.to_string());

        self.get_json("pokemon", Some(q)).await
    }
}

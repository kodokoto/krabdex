use crate::{
    client::PokeApiClient,
    error::Result,
    http::Query,
    models::{
        common::{NamedApiResource, Page},
        generation::Generation,
    },
    types::identifiers::{GenerationName, GenerationRef},
};

impl PokeApiClient {
    /// Fetch a Generation by numeric id.
    pub async fn generation_by_id(&self, id: u32) -> Result<Generation> {
        let path = format!("generation/{id}");
        self.get_json(&path, None).await
    }

    /// Fetch a Generation by validated name.
    pub async fn generation_by_name(&self, name: GenerationName<'_>) -> Result<Generation> {
        let path = format!("generation/{}", name.as_str());
        self.get_json(&path, None).await
    }

    /// Fetch a Generation by id or name (convenience API).
    pub async fn generation(&self, gen: GenerationRef<'_>) -> Result<Generation> {
        match gen {
            GenerationRef::Id(id) => self.generation_by_id(id).await,
            GenerationRef::Name(name) => self.generation_by_name(name).await,
        }
    }

    /// List generations (name+url) with pagination.
    ///
    /// GET /generation?limit=...&offset=...
    pub async fn generation_list(&self, limit: u32, offset: u32) -> Result<Page<NamedApiResource>> {
        let mut q = Query::new();
        q.set("limit", limit.to_string());
        q.set("offset", offset.to_string());

        self.get_json("generation", Some(q)).await
    }
}

use crate::{
    client::{PokeApiClient, pagination::page_query},
    error::Result,
    models::{
        common::{NamedApiResource, Page},
        generation::Generation,
    },
    types::{
        identifiers::{GenerationName, GenerationRef},
        pagination::PageRequest
    },
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
    pub async fn generation_list(&self, page: PageRequest) -> Result<Page<NamedApiResource>> {
        let q = page_query(page);
        self.get_json("generation", Some(q)).await
    }
}

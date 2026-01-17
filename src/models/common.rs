use serde::{Deserialize, Serialize};

/// Named PokeAPI resource (name + URL).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct NamedApiResource {
    pub name: String,
    pub url: String,
}

/// Unnamed PokeAPI resource (URL only).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ApiResource {
    pub url: String,
}

/// Localized name in a specific language.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Name {
    pub name: String,
    pub language: NamedApiResource,
}

/// Version-specific game index reference.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct VersionGameIndex {
    pub game_index: u32,
    pub version: NamedApiResource,
}

/// Paginated list response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Page<T> {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

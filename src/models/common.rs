use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct NamedApiResource {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ApiResource {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Name {
    pub name: String,
    pub language: NamedApiResource,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct VersionGameIndex {
    pub game_index: u32,
    pub version: NamedApiResource,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Page<T> {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

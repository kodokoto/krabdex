use ::url::Url;
use crate::http::Headers;

/// Configuration values used to construct a `PokeApiClient`.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL of the PokeAPI instance (defaults to `https://pokeapi.co/`).
    pub base_url: Url,
    /// API prefix appended after the base URL (defaults to `api/v2`).
    pub api_prefix: String,
    /// Default headers applied to every request.
    pub default_headers: Headers,
}

impl Default for ClientConfig {
    fn default() -> Self {
        let mut headers = Headers::new();
        headers.insert("accept", "application/json");

        Self {
            base_url: Url::parse("https://pokeapi.co/").expect("valid default base url"),
            api_prefix: "api/v2".to_string(),
            default_headers: headers,
        }
    }
}

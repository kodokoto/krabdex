use ::url::Url;
use crate::http::Headers;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: Url,
    pub api_prefix: String,
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

use ::url::Url;

use crate::{
    client::{config::ClientConfig, client::PokeApiClient},
    error::{Error, Result},
    transport::reqwest_transport::ReqwestTransport
};

/// Builder for constructing a `PokeApiClient` with custom settings.
#[derive(Debug)]
pub struct PokeApiClientBuilder {
    config: ClientConfig,
    timeout: Option<std::time::Duration>,
    user_agent: Option<String>,
}

impl Default for PokeApiClientBuilder {
    fn default() -> Self {
        Self {
            config: ClientConfig::default(),
            timeout: Some(std::time::Duration::from_secs(10)),
            user_agent: Some("krabdex/0.1.0".to_string()),
        }
    }
}

impl PokeApiClientBuilder {
    /// Create a builder with default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a builder from an existing `ClientConfig`.
    pub fn from_config(config: ClientConfig) -> Self {
        Self {
            config,
            timeout: Some(std::time::Duration::from_secs(10)),
            user_agent: Some("krabdex/0.1.0".to_string()),
        }
    }

    /// Override the base URL (defaults to `https://pokeapi.co/`).
    pub fn base_url(mut self, url: Url) -> Self {
        self.config.base_url = url;
        self
    }

    /// Override the API prefix (defaults to `api/v2`).
    pub fn api_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.config.api_prefix = prefix.into();
        self
    }

    /// Add or override a default header applied to every request.
    pub fn default_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.default_headers.insert(key, value);
        self
    }

    /// Set a client-wide timeout.
    pub fn timeout(mut self, d: std::time::Duration) -> Self {
        self.timeout = Some(d);
        self
    }

    /// Set the User-Agent header for all requests.
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// Build a `PokeApiClient` from the accumulated configuration.
    pub fn build(self) -> Result<PokeApiClient> {
        let mut rb = reqwest::Client::builder();

        if let Some(t) = self.timeout {
            rb = rb.timeout(t);
        }
        if let Some(ua) = &self.user_agent {
            rb = rb.user_agent(ua.clone());
        }

        let client = rb
            .build()
            .map_err(|e| Error::Transport { source: Box::new(e) })?;

        let transport = ReqwestTransport::new(client);

        Ok(PokeApiClient {
            config: self.config,
            transport,
        })
    }
}

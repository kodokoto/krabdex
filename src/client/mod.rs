// src/client/mod.rs
pub mod builder;
pub mod config;

use serde::de::DeserializeOwned;

use crate::{
    client::config::ClientConfig,
    error::{Error, Result, classify::classify_http_error},
    http::{HttpRequest, Method, Query, url::join_base},
    transport::Transport,
    transport::reqwest_transport::ReqwestTransport,
};

pub struct PokeApiClient {
    pub(crate) config: ClientConfig,
    pub(crate) transport: ReqwestTransport,
}

impl PokeApiClient {
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }

    pub fn builder() -> builder::PokeApiClientBuilder {
        builder::PokeApiClientBuilder::new()
    }

    /// Core internal helper: build URL, add headers/query, call transport, handle errors, deserialize.
    pub(crate) async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        query: Option<Query>
    ) -> Result<T> {
        let url = join_base(&self.config.base_url, &self.config.api_prefix, path)?;
        let url_string = url.to_string();

        let mut req = HttpRequest::new(Method::Get, url);
        req.headers = self.config.default_headers.clone();
        if let Some(q) = query {
            req.query = q;
        }

        let resp = self.transport.send(req).await?;

        // Non-2xx -> Api error (centralized)
        if !(200..=299).contains(&resp.status) {
            let url = path.to_string();
            return Err(classify_http_error(resp.status, url, &resp));
        }

        serde_json::from_slice::<T>(&resp.body).map_err(|e| Error::Deserialize {
            url: url_string,
            source: e,
        })
    }
}

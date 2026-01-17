pub mod classify;

use thiserror::Error;
use std::{error::Error as StdError, fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // Internal SDK error (bug)
    #[error("internal error: {0}")]
    Internal(&'static str),
    
    // Networing error (TLS, DNS, IO, etc)
    #[error("transport error")]
    Transport {
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },

    // API returned a non-success HTTP status
    #[error("api error: {0}")]
    Api(ApiError),

    // Response body could not be deserialized
    #[error("failed to deserialize response from {url}")]
    Deserialize {
        url: String,
        #[source]
        source: serde_json::Error,
    },

    /// User provided invalid input (local validation failure)
    #[error("invalid argument `{field}`: {reason}")]
    InvalidArgument {
        field: &'static str,
        reason: String,
    },
}

#[derive(Debug)]
pub struct ApiError {
    pub status: u16,
    pub url: String,
    pub kind: ApiErrorKind,
}

#[derive(Debug)]
pub enum ApiErrorKind {
    NotFound {
        resource: &'static str,
        identifier: String,
    },
    RateLimited {
        retry_after: Option<u64>,
    },
    HttpStatus {
        body_snippet: Option<String>,
    },
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ApiErrorKind::NotFound { resource, identifier } => {
                write!(
                    f,
                    "{} `{}` not found (status {})",
                    resource, identifier, self.status
                )
            }
            ApiErrorKind::RateLimited { retry_after } => {
                if let Some(secs) = retry_after {
                    write!(
                        f,
                        "rate limited (retry after {}s) (status {})",
                        secs, self.status
                    )
                } else {
                    write!(f, "rate limited (status {})", self.status)
                }
            }
            ApiErrorKind::HttpStatus { .. } => {
                write!(f, "http error (status {})", self.status)
            }
        }
    }
}

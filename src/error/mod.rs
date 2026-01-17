use thiserror::Error;
use std::error::Error as StdError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // Internal error (bug)
    #[error("internal error: {0}")]
    Internal(&'static str),
    
    // Networing error (TLS, DNS, IO, etc)
    #[error("transport error")]
    Transport {
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
}
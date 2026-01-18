//! krabdex — a typed Rust SDK for PokeAPI.
//!
//! # Quickstart
//! ```no_run
//! use krabdex::{PokeApiClient, types::PokemonName};
//! # #[tokio::main]
//! # async fn main() -> krabdex::Result<()> {
//! let client = PokeApiClient::builder().build()?;
//! let p = client.pokemon_by_name(PokemonName::new("pikachu")?).await?;
//! println!("{}", p.name);
//! # Ok(()) 
//! }
//! ```

pub mod error;
pub(crate) mod http;
pub(crate) mod transport;
pub mod client;
pub mod types;
pub mod models;
pub mod api;

pub use client::PokeApiClient;
pub use error::{Error, Result};

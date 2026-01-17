# krabdex

> A type-safe, async Rust SDK for [PokeAPI](https://pokeapi.co).

## Features
- Async client built on reqwest (rustls).
- Strongly typed models for Pokémon and Generation resources.
- Validated identifiers (`PokemonName`, `GenerationName`) and pagination types (`Limit`, `PageRequest`).
- Error classification for HTTP status codes (including rate limiting).

## Quick start

```bash
cargo add krabdex
```

```rust
use krabdex::{
    PokeApiClient,
    types::{PokemonName, GenerationName, GenerationRef, PokemonRef, Limit, PageRequest},
};

#[tokio::main]
async fn main() -> krabdex::Result<()> {
    // Default client (https://pokeapi.co/api/v2)
    let client = PokeApiClient::new()?;

    // Fetch by id or name
    let pikachu = client.pokemon(PokemonRef::Id(25)).await?;
    let ditto = client.pokemon(PokemonRef::Name(PokemonName::new("ditto")?)).await?;

    // Generations
    let gen1 = client.generation(GenerationRef::Name(GenerationName::new("generation-i")?)).await?;

    // Pagination
    let page = client.pokemon_list(PageRequest::first_page(Limit::new(10)?)).await?;

    println!("pikachu: {:?}", pikachu.name);
    println!("ditto: {:?}", ditto.name);
    println!("gen1: {:?}", gen1.name);
    println!("first page count: {}", page.results.len());
    Ok(())
}
```

## Custom configuration

```rust
use url::Url;
use krabdex::{PokeApiClient, client::builder::PokeApiClientBuilder};

let client = PokeApiClientBuilder::new()
    .base_url(Url::parse("https://pokeapi.co/")?)
    .api_prefix("api/v2")
    .user_agent("krabdex-example/0.1.0")
    .build()?;
```

## Errors

All operations return `krabdex::Result<T>` with rich `krabdex::Error` variants:
- `Transport` for network/TLS/IO issues.
- `Api` for non-2xx responses (NotFound, RateLimited, HttpStatus).
- `Deserialize` when response payloads do not match expected models.
- `InvalidArgument` for local validation failures (e.g., invalid names/limits).

## Development

- Tests: `cargo test`
- Docs: `cargo doc --no-deps --open`

## License

MIT or Apache-2.0

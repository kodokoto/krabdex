use crate::error::{Error, Result};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A validated Pokemon name identifier used for PokeAPI path and query parameters.
///
/// This wrapper ensures the name matches the PokeAPI's expected format
/// (lowercase ASCII letters, digits, or `-`) before being used in requests.
/// The inner value is a borrowed `&str`, so it does not allocate.
pub struct PokemonName<'a>(&'a str);

impl<'a> PokemonName<'a> {
    pub fn new(name: &'a str) -> Result<Self> {
        validate_pokeapi_name(name, "pokemon_name")?;
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
/// A Pokemon reference for API calls, either by numeric id or by name.
pub enum PokemonRef<'a> {
    Id(u32),
    Name(PokemonName<'a>),
}

/// A validated Generation name identifier used for PokeAPI path and query parameters.
///
/// PokeAPI generation names are lowercase ASCII strings
/// like: "generation-i", "generation-ii", etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenerationName<'a>(&'a str);
impl<'a> GenerationName<'a> {
    pub fn new(name: &'a str) -> Result<Self> {
        validate_pokeapi_name(name, "generation_name")?;
        Ok(Self(name))
    }

       pub fn as_str(&self) -> &str {
        self.0
    }
}

/// A Generation reference for API calls, either by numeric id or by name.
#[derive(Debug, Clone, Copy)]
pub enum GenerationRef<'a> {
    Id(u32),
    Name(GenerationName<'a>),
}


fn validate_pokeapi_name(name: &str, field: &'static str) -> Result<()> {
    if name.is_empty() {
        return Err(Error::InvalidArgument {
            field,
            reason: "cannot be empty".into(),
        });
    }

    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(Error::InvalidArgument {
            field,
            reason: "must be lowercase ascii letters, digits, or '-'".into(),
        });
    }

    Ok(())
}

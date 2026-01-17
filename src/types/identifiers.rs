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
        Self::validate(name)?;
        Ok(Self(name))
    }

    pub fn validate(name: &'a str) -> Result<()> {
        if name.is_empty() {
            return Err(Error::InvalidArgument {
                field: "pokemon_name",
                reason: "cannot be empty".into(),
            });
        }

        if !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(Error::InvalidArgument {
                field: "pokemon_name",
                reason: "must be lowercase ascii letters, digits, or '-'".into(),
            });
        }
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
/// A Pokemon identifier for API calls, either by numeric id or by name.
///
/// Use `Id(u32)` when you already have the numeric identifier, or
/// `Name(PokemonName<'_>)` when you want validation of the name format.
pub enum PokemonRef<'a> {
    Id(u32),
    Name(PokemonName<'a>),
}

use crate::error::{Error, Result};

/// Page size used in PokeAPI list endpoints (capped at 100).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limit(u32);

impl Limit {
    pub const DEFAULT: Limit = Limit(20);
    pub const MAX: u32 = 100;

    /// Validate and create a `Limit` (must be > 0 and <= `MAX`).
    pub fn new(v: u32) -> Result<Self> {
        if v == 0 {
            return Err(Error::InvalidArgument { field: "limit", reason: "must be > 0".into() });
        }
        if v > Self::MAX {
            return Err(Error::InvalidArgument { field: "limit", reason: format!("must be <= {}", Self::MAX) });
        }
        Ok(Self(v))
    }

    /// Get the inner `u32` value.
    pub fn get(self) -> u32 { self.0 }
}

/// Offset used in PokeAPI list endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offset(u32);

impl Offset {
    /// Create an `Offset` (no validation, may be 0).
    pub fn new(v: u32) -> Result<Self> {
        Ok(Self(v))
    }
    /// Get the inner `u32` value.
    pub fn get(self) -> u32 { self.0 }
}

/// Pagination parameters for list endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    pub limit: Limit,
    pub offset: Offset,
}

impl PageRequest {
    /// Create a `PageRequest` from limit and offset.
    pub fn new(limit: Limit, offset: Offset) -> Self {
        Self { limit, offset }
    }

    /// Convenience for the first page with a given limit (offset 0).
    pub fn first_page(limit: Limit) -> Self {
        Self { limit, offset: Offset(0) }
    }
}

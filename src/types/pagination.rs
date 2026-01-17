use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limit(u32);

impl Limit {
    pub const DEFAULT: Limit = Limit(20);
    pub const MAX: u32 = 100;

    pub fn new(v: u32) -> Result<Self> {
        if v == 0 {
            return Err(Error::InvalidArgument { field: "limit", reason: "must be > 0".into() });
        }
        if v > Self::MAX {
            return Err(Error::InvalidArgument { field: "limit", reason: format!("must be <= {}", Self::MAX) });
        }
        Ok(Self(v))
    }

    pub fn get(self) -> u32 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offset(u32);

impl Offset {
    pub fn new(v: u32) -> Result<Self> {
        Ok(Self(v))
    }
    pub fn get(self) -> u32 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    pub limit: Limit,
    pub offset: Offset,
}

impl PageRequest {
    pub fn new(limit: Limit, offset: Offset) -> Self {
        Self { limit, offset }
    }

    pub fn first_page(limit: Limit) -> Self {
        Self { limit, offset: Offset(0) }
    }
}
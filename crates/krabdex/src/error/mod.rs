pub mod error;
pub(crate) mod classify;

pub use error::{Error, ApiError, ApiErrorKind, Result};

#[cfg(test)]
mod tests;

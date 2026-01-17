use url::Url;

use crate::error::{Error, Result};

/// Join base + path safely.
/// `path` should be relative like "pokemon/pikachu" (no leading slash).
pub fn join_base(base: &Url, api_prefix: &str, path: &str) -> Result<Url> {
    let api_prefix = api_prefix.trim_matches('/');
    let path = path.trim_matches('/');

    let mut url = base
        .join(&format!("{}/", api_prefix))
        .map_err(|_| Error::Internal("invalid base url join"))?;

    url = url
        .join(path)
        .map_err(|_| Error::Internal("invalid path join"))?;

    Ok(url)
}


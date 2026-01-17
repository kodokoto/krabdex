use crate::error::Result;
use crate::http::{HttpRequest, HttpResponse};

pub(crate) mod reqwest_transport;

// I'm keeping the Transrport trait internal for now, to simplify and avoid async rust pain
#[allow(async_fn_in_trait)]
pub(crate) trait Transport: Send + Sync {
    async fn send(&self, req: HttpRequest) -> Result<HttpResponse>;
}

#[cfg(test)]
mod tests;

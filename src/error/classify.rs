use crate::error::{ApiError, ApiErrorKind, Error};
use crate::http::HttpResponse;

pub fn classify_http_error(
    status: u16,
    url: String,
    resp: &HttpResponse,
) -> Error {
    match status {
        404 => Error::Api(ApiError {
            status,
            url,
            kind: ApiErrorKind::NotFound {
                resource: "resource",
                identifier: "<unknown>".into(),
            },
        }),

        429 => {
            let retry_after = resp
                .headers
                .get("retry-after")
                .and_then(|v| v.parse().ok());

            Error::Api(ApiError {
                status,
                url,
                kind: ApiErrorKind::RateLimited { retry_after },
            })
        }

        _ => {
            let snippet = if resp.body.is_empty() {
                None
            } else {
                Some(
                    String::from_utf8_lossy(&resp.body)
                        .chars()
                        .take(300)
                        .collect(),
                )
            };

            Error::Api(ApiError {
                status,
                url,
                kind: ApiErrorKind::HttpStatus { body_snippet: snippet },
            })
        }
    }
}

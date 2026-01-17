use super::*;
use crate::http::{Headers, HttpResponse};

fn make_resp(status: u16, body: &[u8], headers: &[(&str, &str)]) -> HttpResponse {
    let mut h = Headers::new();
    for (k, v) in headers {
        h.insert(k.to_string(), v.to_string());
    }
    HttpResponse {
        status,
        headers: h,
        body: body.to_vec(),
    }
}

#[test]
fn classify_404_returns_not_found() {
    let resp = make_resp(404, b"", &[]);
    let err = classify::classify_http_error(404, "pokemon/ghost".into(), &resp);

    match err {
        Error::Api(api) => match api.kind {
            ApiErrorKind::NotFound { resource, .. } => assert_eq!(resource, "resource"),
            other => panic!("unexpected api kind: {other:?}"),
        },
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn classify_429_extracts_retry_after() {
    let resp = make_resp(429, b"", &[("retry-after", "5")]);
    let err = classify::classify_http_error(429, "pokemon/overload".into(), &resp);

    match err {
        Error::Api(api) => match api.kind {
            ApiErrorKind::RateLimited { retry_after } => assert_eq!(retry_after, Some(5)),
            other => panic!("unexpected api kind: {other:?}"),
        },
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn classify_generic_includes_snippet() {
    let resp = make_resp(500, b"oops", &[]);
    let err = classify::classify_http_error(500, "pokemon/boom".into(), &resp);

    match err {
        Error::Api(api) => match api.kind {
            ApiErrorKind::HttpStatus { body_snippet } => assert_eq!(body_snippet.as_deref(), Some("oops")),
            other => panic!("unexpected api kind: {other:?}"),
        },
        other => panic!("unexpected error: {other:?}"),
    }
}

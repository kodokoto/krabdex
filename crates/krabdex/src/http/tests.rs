use url::Url;

use crate::{error::Error, http::url::join_base};

#[test]
fn join_base_trims_slashes_and_preserves_segments() {
    let base = Url::parse("https://pokeapi.co/").unwrap();

    let cases = [
        ("/api/v2/", "/pokemon/pikachu/", "https://pokeapi.co/api/v2/pokemon/pikachu"),
        ("api/v2", "pokemon", "https://pokeapi.co/api/v2/pokemon"),
        ("api/v2/", "pokemon/ditto", "https://pokeapi.co/api/v2/pokemon/ditto"),
    ];

    for (api_prefix, path, expected) in cases {
        let url = join_base(&base, api_prefix, path).unwrap();
        assert_eq!(url.as_str(), expected);
    }
}

#[test]
fn join_base_allows_empty_api_prefix() {
    let base = Url::parse("https://pokeapi.co/").unwrap();

    let url = join_base(&base, "", "pokemon").unwrap();

    assert_eq!(url.as_str(), "https://pokeapi.co/pokemon");
}

#[test]
fn join_base_rejects_non_base_urls() {
    let base = Url::parse("mailto:pokeapi").unwrap();

    let err = join_base(&base, "api/v2", "pokemon").unwrap_err();

    assert!(matches!(err, Error::Internal(_)));
}

#[test]
fn headers_insert_and_iter_are_sorted() {
    let mut headers = crate::http::Headers::new();
    headers.insert("z-last", "2");
    headers.insert("a-first", "1");

    assert_eq!(headers.get("a-first"), Some("1"));
    assert_eq!(headers.get("z-last"), Some("2"));

    let collected: Vec<_> = headers.iter().collect();
    assert_eq!(collected, vec![("a-first", "1"), ("z-last", "2")]);
}

#[test]
fn query_overwrites_existing_keys() {
    let mut q = crate::http::Query::new();
    q.set("limit", "10");
    q.set("limit", "20");

    let collected: Vec<_> = q.iter().collect();
    assert_eq!(collected, vec![("limit", "20")]);
}

#[test]
fn http_request_defaults_to_empty_headers_and_query() {
    let url = Url::parse("https://pokeapi.co/ping").unwrap();
    let req = crate::http::HttpRequest::new(crate::http::Method::Get, url.clone());

    assert_eq!(req.method, crate::http::Method::Get);
    assert_eq!(req.url, url);
    assert_eq!(req.headers.iter().count(), 0);
    assert_eq!(req.query.iter().count(), 0);
}

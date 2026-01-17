use krabdex::http::url::join_base;
use url::Url;

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

    assert!(matches!(err, krabdex::error::Error::Internal(_)));
}

use krabdex::types::{GenerationName, Limit, PageRequest, PokemonName};

#[test]
fn pokemon_name_accepts_lowercase_and_hyphen() {
    let n = PokemonName::new("mr-mime").unwrap();
    assert_eq!(n.as_str(), "mr-mime");
}

#[test]
fn pokemon_name_rejects_uppercase() {
    let err = PokemonName::new("Pikachu").unwrap_err();
    assert!(matches!(err, krabdex::Error::InvalidArgument { field: "pokemon_name", .. }));
}

#[test]
fn generation_name_rejects_empty() {
    let err = GenerationName::new("").unwrap_err();
    assert!(matches!(err, krabdex::Error::InvalidArgument { field: "generation_name", .. }));
}

#[test]
fn limit_enforces_bounds() {
    assert!(matches!(Limit::new(0), Err(krabdex::Error::InvalidArgument { field: "limit", .. })));
    assert!(matches!(Limit::new(Limit::MAX + 1), Err(krabdex::Error::InvalidArgument { field: "limit", .. })));
    assert_eq!(Limit::new(10).unwrap().get(), 10);
}

#[test]
fn page_request_first_page_sets_offset_zero() {
    let pr = PageRequest::first_page(Limit::DEFAULT);
    assert_eq!(pr.limit.get(), Limit::DEFAULT.get());
    assert_eq!(pr.offset.get(), 0);
}

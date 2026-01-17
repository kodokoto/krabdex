use httpmock::prelude::*;
use krabdex::{
    types::{GenerationName, GenerationRef, Limit, PageRequest, PokemonRef},
    PokeApiClient,
};
use url::Url;

fn client_with_base(server: &MockServer) -> PokeApiClient {
    PokeApiClient::builder()
        .base_url(Url::parse(&server.base_url()).unwrap())
        .api_prefix("api/v2")
        .build()
        .unwrap()
}

#[tokio::test]
async fn pokemon_by_id_deserializes_minimal_payload() {
    let server = MockServer::start();
    let m = server.mock(|when, then| {
        when.method(GET).path("/api/v2/pokemon/25");
        then.status(200).body(
            r#"{
                "id": 25,
                "name": "pikachu",
                "base_experience": 112,
                "height": 4,
                "weight": 60,
                "is_default": true,
                "order": 35,
                "abilities": [
                    {
                        "is_hidden": false,
                        "slot": 1,
                        "ability": { "name": "static", "url": "https://pokeapi.co/api/v2/ability/9/" }
                    }
                ],
                "forms": [
                    { "name": "pikachu", "url": "https://pokeapi.co/api/v2/pokemon-form/25/" }
                ],
                "game_indices": [],
                "held_items": [],
                "location_area_encounters": "https://pokeapi.co/api/v2/pokemon/25/encounters",
                "moves": [],
                "species": { "name": "pikachu", "url": "https://pokeapi.co/api/v2/pokemon-species/25/" },
                "stats": [
                    { "base_stat": 35, "effort": 0, "stat": { "name": "speed", "url": "https://pokeapi.co/api/v2/stat/6/" } }
                ],
                "types": [
                    { "slot": 1, "type": { "name": "electric", "url": "https://pokeapi.co/api/v2/type/13/" } }
                ],
                "past_types": [],
                "past_abilities": [],
                "sprites": {},
                "cries": null
            }"#,
        );
    });

    let client = client_with_base(&server);
    let pokemon = client.pokemon(PokemonRef::Id(25)).await.unwrap();

    m.assert();
    assert_eq!(pokemon.id, 25);
    assert_eq!(pokemon.name, "pikachu");
    assert_eq!(pokemon.types[0].ty.name, "electric");
}

#[tokio::test]
async fn generation_by_name_deserializes() {
    let server = MockServer::start();
    let m = server.mock(|when, then| {
        when.method(GET).path("/api/v2/generation/generation-i");
        then.status(200).body(
            r#"{
                "id": 1,
                "name": "generation-i",
                "abilities": [],
                "moves": [],
                "pokemon_species": [
                    { "name": "bulbasaur", "url": "https://pokeapi.co/api/v2/pokemon-species/1/" }
                ],
                "types": [],
                "version_groups": [],
                "main_region": { "name": "kanto", "url": "https://pokeapi.co/api/v2/region/1/" },
                "names": [
                    { "name": "Generation I", "language": { "name": "en", "url": "https://pokeapi.co/api/v2/language/9/" } }
                ]
            }"#,
        );
    });

    let client = client_with_base(&server);
    let gen = client
        .generation(GenerationRef::Name(GenerationName::new("generation-i").unwrap()))
        .await
        .unwrap();

    m.assert();
    assert_eq!(gen.id, 1);
    assert_eq!(gen.main_region.name, "kanto");
    assert_eq!(gen.pokemon_species.len(), 1);
}

#[tokio::test]
async fn pokemon_list_sets_limit_and_offset() {
    let server = MockServer::start();
    let m = server.mock(|when, then| {
        when.method(GET)
            .path("/api/v2/pokemon")
            .query_param("limit", "2")
            .query_param("offset", "10");

        then.status(200).body(
            r#"{
                "count": 200,
                "next": "http://example.com/next",
                "previous": null,
                "results": [
                    {"name":"foo","url":"https://pokeapi.co/api/v2/pokemon/1/"},
                    {"name":"bar","url":"https://pokeapi.co/api/v2/pokemon/2/"}
                ]
            }"#,
        );
    });

    let client = client_with_base(&server);
    let page_req = PageRequest::new(Limit::new(2).unwrap(), krabdex::types::Offset::new(10).unwrap());
    let page = client.pokemon_list(page_req).await.unwrap();

    m.assert();
    assert_eq!(page.count, 200);
    assert_eq!(page.results.len(), 2);
    assert_eq!(page.results[0].name, "foo");
}

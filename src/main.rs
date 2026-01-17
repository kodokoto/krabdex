

#[tokio::main]
async fn main() -> krabdex::Result<()> {
    let client = krabdex::PokeApiClient::builder().build()?;
    let p = client.pokemon_by_id(25).await?;
    println!("{}", p.name);
    Ok(())
}
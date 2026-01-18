use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};

use krabdex_mcp::server;

#[tokio::main]
async fn main() -> Result<()> {
    // Build your SDK client once; reused across tool calls.
    let client = krabdex::PokeApiClient::builder()
        .user_agent("krabdex-mcp/0.1.0")
        .build()?;

    // Serve over STDIO (what most local agents want).
    let service = server::KrabdexMcp::new(client).serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

//! krabdex-mcp: MCP server exposing PokeAPI tools powered by the `krabdex` SDK.
//!
//! ## Install
//! ```bash
//! cargo install krabdex-mcp
//! ```
//!
//! ## Codex config (stdio)
//! Add to `~/.codex/config.toml`:
//! ```toml
//! [mcp_servers.krabdex]
//! command = "krabdex-mcp"
//! startup_timeout_sec = 20
//! tool_timeout_sec = 60
//! ```
//!
//! ## Claude Code / claude mcp
//! ```bash
//! claude mcp add --transport stdio --scope local krabdex -- krabdex-mcp
//! ```
//!
//! Tools exposed:
//! - `pokemon.get` (id or name)
//! - `pokemon.list` (limit, offset)
//! - `generation.get` (id or name)
//! - `generation.list` (limit, offset)

pub mod server;

pub use server::KrabdexMcp;

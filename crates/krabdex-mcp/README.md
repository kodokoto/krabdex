# krabdex-mcp

This workspace includes an MCP server utilizing krabdex for making PokeAPI calls.

## Install/build

```bash
cargo install krabdex-mcp
```

## Configure Claude
```bash
claude mcp add --transport stdio --scope local krabdex -- krabdex-mcp
```

## Configure Codex
```bash
codex mcp add krabdex -- krabdex-mcp
```

## Available tools

- `pokemon.get` (args: `id` or `name`)
- `pokemon.list` (args: `limit`, `offset`)
- `generation.get` (args: `id` or `name`)
- `generation.list` (args: `limit`, `offset`)

Example tool call payload:

```json
{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"pokemon.get","arguments":{"name":"pikachu"}}}
```
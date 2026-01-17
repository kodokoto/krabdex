use std::borrow::Cow;

use rmcp::{
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::*,
    tool, tool_handler, tool_router,
    ErrorData as McpError,
};

use schemars::JsonSchema;
use serde::Deserialize;

use krabdex::types::{GenerationName, GenerationRef, Limit, Offset, PageRequest, PokemonName, PokemonRef};
use std::sync::Arc;

#[derive(Clone)]
pub struct KrabdexMcp {
    client: Arc<krabdex::PokeApiClient>,
    tool_router: ToolRouter<Self>,
}

impl KrabdexMcp {
    pub fn new(client: krabdex::PokeApiClient) -> Self {
        Self {
            client: Arc::new(client),
            tool_router: Self::tool_router(),
        }
    }

    fn mcp_err(code: ErrorCode, message: impl Into<String>, data: Option<serde_json::Value>) -> McpError {
        McpError {
            code,
            message: Cow::Owned(message.into()),
            data,
        }
    }

    fn map_sdk_err(&self, e: krabdex::Error) -> McpError {
        // Keep this simple + stable for agents.
        // Put the verbose stuff in `data` rather than a massive message string.
        let (code, msg) = match &e {
            krabdex::Error::InvalidArgument { .. } => (ErrorCode::INVALID_PARAMS, "Invalid arguments"),
            krabdex::Error::Api(api) if api.status == 404 => (ErrorCode::INVALID_REQUEST, "Not found"),
            krabdex::Error::Api(_) => (ErrorCode::INTERNAL_ERROR, "Upstream API error"),
            krabdex::Error::Transport { .. } => (ErrorCode::INTERNAL_ERROR, "Network/transport error"),
            krabdex::Error::Deserialize { .. } => (ErrorCode::INTERNAL_ERROR, "Deserialize error"),
            _ => (ErrorCode::INTERNAL_ERROR, "Unexpected error"),
        };

        Self::mcp_err(
            code,
            msg,
            Some(serde_json::json!({
                "error": format!("{e}"),
                "kind": format!("{:?}", e),
            })),
        )
    }

    fn json_ok<T: serde::Serialize>(value: &T) -> Result<CallToolResult, McpError> {
        let s = serde_json::to_string_pretty(value)
            .map_err(|e| Self::mcp_err(ErrorCode::INTERNAL_ERROR, "Failed to serialize result", Some(serde_json::json!({"error": e.to_string()}))))?;
        Ok(CallToolResult::success(vec![Content::text(s)]))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetByIdOrName {
    /// Numeric id (exclusive with `name`)
    pub id: Option<u32>,
    /// Resource name (exclusive with `id`)
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListArgs {
    /// Page size (default 20, max enforced by SDK types if you wired it that way)
    pub limit: Option<u32>,
    /// Offset into the collection (default 0)
    pub offset: Option<u32>,
}

#[tool_router]
impl KrabdexMcp {
    #[tool(description = "Fetch a Pokemon by id or name. Provide exactly one of {id, name}.")]
    async fn pokemon_get(&self, Parameters(args): Parameters<GetByIdOrName>) -> Result<CallToolResult, McpError> {
        let res = match (args.id, args.name.as_deref()) {
            (Some(id), None) => self.client.pokemon(PokemonRef::Id(id)).await,
            (None, Some(name)) => {
                let name = PokemonName::new(name)
                    .map_err(|e| self.map_sdk_err(e))?;
                self.client.pokemon(PokemonRef::Name(name)).await
            }
            _ => Err(krabdex::Error::InvalidArgument {
                field: "pokemon_get",
                reason: "Provide exactly one of `id` or `name`".into(),
            }),
        };

        let pokemon = res.map_err(|e| self.map_sdk_err(e))?;
        Self::json_ok(&pokemon)
    }

    #[tool(description = "Fetch a Generation by id or name. Provide exactly one of {id, name}.")]
    async fn generation_get(&self, Parameters(args): Parameters<GetByIdOrName>) -> Result<CallToolResult, McpError> {
        let res = match (args.id, args.name.as_deref()) {
            (Some(id), None) => self.client.generation(GenerationRef::Id(id)).await,
            (None, Some(name)) => {
                let name = GenerationName::new(name)
                    .map_err(|e| self.map_sdk_err(e))?;
                self.client.generation(GenerationRef::Name(name)).await
            }
            _ => Err(krabdex::Error::InvalidArgument {
                field: "generation_get",
                reason: "Provide exactly one of `id` or `name`".into(),
            }),
        };

        let gen = res.map_err(|e| self.map_sdk_err(e))?;
        Self::json_ok(&gen)
    }

    #[tool(description = "List Pokemon resources with pagination (limit/offset).")]
    async fn pokemon_list(&self, Parameters(args): Parameters<ListArgs>) -> Result<CallToolResult, McpError> {
        let limit = Limit::new(args.limit.unwrap_or(20)).map_err(|e| self.map_sdk_err(e))?;
        let offset = Offset::new(args.offset.unwrap_or(0)).map_err(|e| self.map_sdk_err(e))?;
        let page = self.client
            .pokemon_list(PageRequest::new(limit, offset))
            .await
            .map_err(|e| self.map_sdk_err(e))?;

        Self::json_ok(&page)
    }

    #[tool(description = "List Generation resources with pagination (limit/offset).")]
    async fn generation_list(&self, Parameters(args): Parameters<ListArgs>) -> Result<CallToolResult, McpError> {
        let limit = Limit::new(args.limit.unwrap_or(20)).map_err(|e| self.map_sdk_err(e))?;
        let offset = Offset::new(args.offset.unwrap_or(0)).map_err(|e| self.map_sdk_err(e))?;
        let page = self.client
            .generation_list(PageRequest::new(limit, offset))
            .await
            .map_err(|e| self.map_sdk_err(e))?;

        Self::json_ok(&page)
    }
}

#[tool_handler]
impl rmcp::ServerHandler for KrabdexMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("PokeAPI tools powered by the krabdex Rust SDK.".into()),
        }
    }
}

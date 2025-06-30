use crate::TIDB_ACCESS_CONFIG;
use gaia_tidb_mcp_common::*;
use mysql::prelude::*;
use rmcp::{
    Error as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::*},
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use std::sync::OnceLock;
use tracing::{error, info};

static SEARCH_TOOL_PROMPT: OnceLock<String> = OnceLock::new();

pub fn set_search_tool_prompt(prompt: String) {
    SEARCH_TOOL_PROMPT.set(prompt).unwrap_or_default();
}

#[derive(Debug, Clone)]
pub struct TidbServer {
    tool_router: ToolRouter<Self>,
}
#[tool_router]
impl TidbServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Perform keyword search in TiDB")]
    async fn search(
        &self,
        Parameters(TidbSearchRequest { query }): Parameters<TidbSearchRequest>,
    ) -> Result<CallToolResult, McpError> {
        let config = match TIDB_ACCESS_CONFIG.get() {
            Some(config) => config.read().await,
            None => {
                let error_message = "TIDB_ACCESS_CONFIG is not set";
                error!(error_message);
                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        // get connection
        info!("Getting connection...");
        let mut conn = config.pool.get_conn().map_err(|e| {
            let error_message = format!("Failed to get connection: {e}");

            error!(error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        // test connection
        info!("Testing connection...");
        let version: String = match conn.query_first("SELECT VERSION()").map_err(|e| {
            let error_message = format!("Failed to query version: {e}");

            error!(error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })? {
            Some(version) => version,
            None => {
                let error_message = "Failed to query version";

                error!(error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };
        info!("Connected to TiDB Cloud! Version: {}", version);

        // check if table exists
        info!("Checking if table exists...");
        let check_table_sql = format!(
            "SELECT COUNT(*) FROM information_schema.tables
                WHERE table_schema = '{}' AND table_name = '{}'",
            config.database, config.table_name
        );
        info!("Executing check table SQL: {}", check_table_sql);
        let table_exists: i32 = conn
            .query_first(&check_table_sql)
            .map_err(|e| {
                let error_message = format!("Failed to check table: {e}");

                error!(error_message);

                McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
            })?
            .unwrap_or(0);

        if table_exists == 0 {
            let error_message = format!(
                "Not found table `{}` in database `{}`",
                config.table_name, config.database
            );

            error!(error_message);

            return Err(McpError::new(
                ErrorCode::INTERNAL_ERROR,
                error_message,
                None,
            ));
        }

        // execute full-text search
        info!("\nExecuting full-text search for '{}'...", query);
        let search_sql = format!(
            r"SELECT * FROM {}
                WHERE fts_match_word('{}', content)
                ORDER BY fts_match_word('{}', content)
                DESC LIMIT {}",
            config.table_name, query, query, config.limit
        );

        let hits: Vec<TidbSearchHit> = conn.query(&search_sql).map_err(|e| {
            let error_message = format!("Failed to execute search: {e}");

            error!(error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        info!("\nSearch results:");
        info!("Found {} matching records:", hits.len());
        for hit in hits.iter() {
            info!("\nID: {}", &hit.id);
            info!("Title: {}", &hit.title);
            info!("Content: {}", &hit.content);
        }

        let content = Content::json(TidbSearchResponse { hits })?;

        info!("Search results fetched from TiDB");

        Ok(CallToolResult::success(vec![content]))
    }
}
#[tool_handler]
impl ServerHandler for TidbServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::default(),
            instructions: Some("A MCP server that performs keyword search in TiDB".into()),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .build(),
            server_info: Implementation::from_build_env(),
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        Ok(ListPromptsResult {
            next_cursor: None,
            prompts: vec![Prompt::new(
                "search",
                Some(
                    "This prompt is for the `search` tool, which takes a query and returns a list of hits",
                ),
                Some(vec![PromptArgument {
                    name: "query".to_string(),
                    description: Some("A user query to search for in TiDB".to_string()),
                    required: Some(true),
                }]),
            )],
        })
    }

    async fn get_prompt(
        &self,
        GetPromptRequestParam { name, arguments }: GetPromptRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        match name.as_str() {
            "search" => {
                let query = arguments
                    .and_then(|json| json.get("query")?.as_str().map(|s| s.to_string()))
                    .ok_or_else(|| {
                        McpError::invalid_params("No query provided to `search` tool", None)
                    })?;

                let prompt = SEARCH_TOOL_PROMPT.get().unwrap();
                let prompt = prompt.replace("{query}", &query);

                Ok(GetPromptResult {
                    description: None,
                    messages: vec![PromptMessage {
                        role: PromptMessageRole::User,
                        content: PromptMessageContent::text(prompt),
                    }],
                })
            }
            _ => {
                let error_message = format!("prompt not found: {name}");
                error!("{error_message}");
                Err(McpError::invalid_params(error_message, None))
            }
        }
    }
}

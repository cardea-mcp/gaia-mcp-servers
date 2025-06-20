use crate::TIDB_ACCESS_CONFIG;
use gaia_tidb_mcp_common::*;
use mysql::prelude::*;
use rmcp::{
    Error as McpError, ServerHandler,
    handler::server::tool::*,
    model::{
        CallToolResult, Content, ErrorCode, Implementation, ServerCapabilities, ServerInfo, Tool,
    },
    tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    result::Result,
    sync::{Arc, OnceLock},
};
use tracing::{error, info};

static SEARCH_TOOL_DESC: OnceLock<String> = OnceLock::new();
static QUERY_PARAM_DESC: OnceLock<String> = OnceLock::new();

pub fn set_search_description(description: String) {
    SEARCH_TOOL_DESC.set(description).unwrap_or_default();
}

pub fn set_query_param_description(description: String) {
    QUERY_PARAM_DESC.set(description).unwrap_or_default();
}

#[derive(Debug, Clone)]
pub struct TidbServer;
impl TidbServer {
    fn search_tool_attr() -> Tool {
        let tool_description = SEARCH_TOOL_DESC
            .get()
            .cloned()
            .unwrap_or_else(|| "Perform a keyword search".to_string());

        let query_description = QUERY_PARAM_DESC
            .get()
            .cloned()
            .unwrap_or_else(|| "the query to search for".to_string());

        // build input schema
        let input_schema = json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": query_description
                }
            },
            "required": ["query"],
            "title": "SearchRequest"
        });

        Tool {
            name: "search".into(),
            description: Some(tool_description.into()),
            input_schema: Arc::new(input_schema.as_object().unwrap().clone()),
            annotations: None,
        }
    }

    async fn search_tool_call(
        context: ToolCallContext<'_, Self>,
    ) -> Result<CallToolResult, McpError> {
        let (__rmcp_tool_receiver, context) = <&Self>::from_tool_call_context_part(context)?;
        let (Parameters(TidbSearchRequest { query }), _context) =
            <Parameters<TidbSearchRequest>>::from_tool_call_context_part(context)?;
        Self::search(__rmcp_tool_receiver, query)
            .await
            .into_call_tool_result()
    }

    async fn search(&self, query: String) -> Result<CallToolResult, McpError> {
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
            let error_message = format!("Failed to get connection: {}", e);

            error!(error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        // test connection
        info!("Testing connection...");
        let version: String = match conn.query_first("SELECT VERSION()").map_err(|e| {
            let error_message = format!("Failed to query version: {}", e);

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
                let error_message = format!("Failed to check table: {}", e);

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
            let error_message = format!("Failed to execute search: {}", e);

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

    fn tool_box() -> &'static ToolBox<TidbServer> {
        static TOOL_BOX: OnceLock<ToolBox<TidbServer>> = OnceLock::new();
        TOOL_BOX.get_or_init(|| {
            let mut tool_box = ToolBox::new();
            tool_box.add(ToolBoxItem::new(
                TidbServer::search_tool_attr(),
                |context| Box::pin(TidbServer::search_tool_call(context)),
            ));
            tool_box
        })
    }
}

#[tool(tool_box)]
impl ServerHandler for TidbServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Gaia TiDB MCP server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: std::env!("CARGO_PKG_NAME").to_string(),
                version: std::env!("CARGO_PKG_VERSION").to_string(),
            },
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TidbSearchRequest {
    query: String,
}

use crate::CONNECTION_CONFIG;
use endpoints::rag::keyword_search::{IndexRequest, IndexResponse, QueryRequest, QueryResponse};
use gaia_kwsearch_mcp_common::{
    CreateIndexRequest, CreateIndexResponse, SearchDocumentsRequest, SearchDocumentsResponse,
};
use rmcp::{
    Error as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::*},
    model::*,
    tool, tool_handler, tool_router,
};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct KeywordSearchServer {
    tool_router: ToolRouter<Self>,
}
#[tool_router]
impl KeywordSearchServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Create an index in the KeywordSearch database")]
    async fn create_index(
        &self,
        Parameters(CreateIndexRequest { index, documents }): Parameters<CreateIndexRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("Creating index in KeywordSearch database");

        // get connection config
        let conn_config = match CONNECTION_CONFIG.get() {
            Some(connection_config) => {
                let conn_config = connection_config.read().await;
                conn_config.clone()
            }
            None => {
                let error_message = "Connection config not found";
                error!("{}", error_message);
                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        // build url
        let base_url = conn_config.base_url.trim_end_matches('/');
        let url = format!("{base_url}/v1/index/create");

        let index_request = IndexRequest {
            index: Some(index),
            documents: documents.into_iter().map(|d| d.into()).collect::<Vec<_>>(),
        };

        let response = reqwest::Client::new()
            .post(&url)
            .json(&index_request)
            .send()
            .await
            .map_err(|e| {
                let error_message = format!("Failed to create index: {e}");

                error!("{}", error_message);

                McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
            })?;

        let index_response = response.json::<IndexResponse>().await.map_err(|e| {
            let error_message = format!("Failed to parse index response: {e}");

            error!("{}", error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        let content = Content::json(CreateIndexResponse::from(index_response))?;

        info!("Index created in KeywordSearch database");

        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Perform a keyword search")]
    async fn search(
        &self,
        Parameters(SearchDocumentsRequest { query }): Parameters<SearchDocumentsRequest>,
    ) -> Result<CallToolResult, McpError> {
        info!("Searching for documents in KeywordSearch database");

        // get connection config
        let conn_config = match CONNECTION_CONFIG.get() {
            Some(connection_config) => {
                let conn_config = connection_config.read().await;
                conn_config.clone()
            }
            None => {
                let error_message = "Connection config not found";
                error!("{}", error_message);
                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        // build url
        let base_url = conn_config.base_url.trim_end_matches('/');
        let url = format!("{base_url}/v1/search");

        let query_request = QueryRequest {
            query,
            top_k: conn_config.limit,
            index: conn_config.index,
        };

        let response = reqwest::Client::new()
            .post(&url)
            .json(&query_request)
            .send()
            .await
            .map_err(|e| {
                let error_message = format!("Failed to search documents: {e}");

                error!("{}", error_message);

                McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
            })?;

        let query_response = response.json::<QueryResponse>().await.map_err(|e| {
            let error_message = format!("Failed to parse query response: {e}");

            error!("{}", error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        let content = Content::json(SearchDocumentsResponse::from(query_response))?;

        info!("Documents searched in KeywordSearch database");

        Ok(CallToolResult::success(vec![content]))
    }
}
#[tool_handler]
impl ServerHandler for KeywordSearchServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::default(),
            instructions: Some("A MCP server that can access the KeywordSearch database".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub base_url: String,
    #[allow(dead_code)]
    pub api_key: Option<String>,
    pub index: String,
    pub limit: usize,
}

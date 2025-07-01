use crate::CONNECTION_CONFIG;
use endpoints::rag::keyword_search::{IndexRequest, IndexResponse, QueryRequest, QueryResponse};
use gaia_kwsearch_mcp_common::{
    CreateIndexRequest, CreateIndexResponse, SearchDocumentsRequest, SearchDocumentsResponse,
};
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
                    description: Some("A user query to search for".to_string()),
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

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub base_url: String,
    #[allow(dead_code)]
    pub api_key: Option<String>,
    pub index: String,
    pub limit: usize,
}

use crate::WEB_SEARCH_CONFIG;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::*},
    model::*,
    schemars, tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

const WEB_SEARCH_URL: &str = "https://api.tavily.com/search";

#[derive(Debug, Clone)]
pub struct WebSearchServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl WebSearchServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Perform a web search for the given query")]
    async fn search(
        &self,
        Parameters(WebSearchRequest { query }): Parameters<WebSearchRequest>,
    ) -> Result<CallToolResult, McpError> {
        let api_key = match std::env::var("TAVILY_API_KEY") {
            Ok(api_key) => api_key,
            Err(_) => {
                return Err(McpError::new(
                    ErrorCode::INVALID_PARAMS,
                    "No API key provided".to_string(),
                    None,
                ));
            }
        };

        let max_results = match WEB_SEARCH_CONFIG.get() {
            Some(config) => config.max_results,
            None => {
                let error_message = "WEB_SEARCH_CONFIG is not initialized";
                error!(error_message);
                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message.to_string(),
                    None,
                ));
            }
        };

        let body = json!({
              "query": query,
              "max_results": max_results,
            //   "search_depth": "advanced",
            //   "chunks_per_source": 3,
            //   "include_raw_content": true,
        });

        let response = reqwest::Client::new()
            .post(WEB_SEARCH_URL)
            .header(AUTHORIZATION, format!("Bearer {api_key}"))
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                let error_message = format!("Failed to perform web search: {e}");
                error!(error_message);
                McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
            })?;

        let response_data = response.json::<serde_json::Value>().await.map_err(|e| {
            let error_message = format!("Failed to parse web search response: {e}");
            error!(error_message);
            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        info!("Web search response:\n{}", response_data);

        let web_search_response = serde_json::from_value::<WebSearchResponse>(response_data)
            .map_err(|e| {
                let error_message = format!("Failed to parse web search response: {e}");
                error!(error_message);
                McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
            })?;

        let mut output = Vec::new();
        for result in web_search_response.results {
            output.push(format!(
                "Title: {}\nURL: {}\nContent: {}",
                result.title, result.url, result.content
            ));
        }

        Ok(CallToolResult::success(vec![Content::text(
            output.join("\n\n"),
        )]))
    }
}

#[tool_handler]
impl ServerHandler for WebSearchServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::LATEST,
            instructions: Some(
                "A MCP server that can perform web searches for the given query".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: std::env!("CARGO_PKG_NAME").to_string(),
                version: std::env!("CARGO_PKG_VERSION").to_string(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct WebSearchConfig {
    pub max_results: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WebSearchRequest {
    #[schemars(description = "The query to search for")]
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WebSearchResponse {
    #[schemars(description = "The query that was used for the search")]
    pub query: String,
    #[schemars(description = "The response time of the search")]
    pub response_time: f64,
    #[schemars(description = "The results of the search")]
    pub results: Vec<WebSearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WebSearchResult {
    #[schemars(description = "The content of the search result")]
    pub content: String,
    #[schemars(description = "The raw content of the search result")]
    pub raw_content: Option<String>,
    #[schemars(description = "The score of the search result")]
    pub score: f64,
    #[schemars(description = "The title of the search result")]
    pub title: String,
    #[schemars(description = "The URL of the search result")]
    pub url: String,
}

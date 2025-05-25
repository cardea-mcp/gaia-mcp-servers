use gaia_elastic_mcp_common::*;
use rmcp::{
    Error as McpError, ServerHandler,
    model::{CallToolResult, Content, ErrorCode, ServerCapabilities, ServerInfo},
    tool,
};
use serde_json::{Value, json};
use tracing::error;

#[derive(Debug, Clone)]
pub struct ElasticSearchServer;
#[tool(tool_box)]
impl ElasticSearchServer {
    #[tool(description = "List all available Elasticsearch indices")]
    async fn list_indices(
        &self,
        #[tool(aggr)] ListIndicesRequest { base_url, api_key }: ListIndicesRequest,
    ) -> Result<CallToolResult, McpError> {
        let base_url = base_url.trim_end_matches('/');
        let url = format!("{}/_cat/indices?v=true&s=index&format=json", base_url);

        let client = reqwest::Client::new();
        let result = match api_key {
            Some(api_key) => {
                client
                    .get(&url)
                    .header("Authorization", api_key)
                    .send()
                    .await
            }
            None => client.get(&url).send().await,
        };

        let response = match result {
            Ok(response) => response,
            Err(e) => {
                let error_message = format!("Failed to list indices: {}", e);

                error!(error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        match response.status().is_success() {
            true => match response.json::<Value>().await {
                Ok(json) => {
                    let indices: Vec<IndexInfo> = serde_json::from_value(json).map_err(|e| {
                        let error_message = format!("Failed to parse indices: {}", e);

                        error!(error_message);

                        McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
                    })?;

                    let content = Content::json(ListIndicesResponse { indices })?;

                    Ok(CallToolResult::success(vec![content]))
                }
                Err(e) => {
                    let error_message = format!("Failed to parse indices: {}", e);

                    error!(error_message);

                    return Err(McpError::new(
                        ErrorCode::INTERNAL_ERROR,
                        error_message,
                        None,
                    ));
                }
            },
            false => {
                let error_message = format!("Failed to list indices: {}", response.status());

                error!(error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        }
    }

    #[tool(
        description = "Get the cluster's index aliases, including filter and routing information. Note that this tool does not return data stream aliases."
    )]
    async fn list_aliases(
        &self,
        #[tool(aggr)] ListAliasesRequest { base_url, api_key }: ListAliasesRequest,
    ) -> Result<CallToolResult, McpError> {
        let base_url = base_url.trim_end_matches('/');
        let url = format!("{}/_cat/aliases?format=json&v=true", base_url);

        let client = reqwest::Client::new();
        let result = match api_key {
            Some(api_key) => {
                client
                    .get(&url)
                    .header("Authorization", api_key)
                    .send()
                    .await
            }
            None => client.get(&url).send().await,
        };

        let response = match result {
            Ok(response) => response,
            Err(e) => {
                let error_message = format!("Failed to list aliases: {}", e);

                error!(error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        match response.status().is_success() {
            true => match response.json::<Value>().await {
                Ok(json) => {
                    let aliases: Vec<AliasInfo> = serde_json::from_value(json).map_err(|e| {
                        let error_message = format!("Failed to parse aliases: {}", e);

                        error!(error_message);

                        McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
                    })?;

                    let content = Content::json(ListAliasesResponse { aliases })?;

                    Ok(CallToolResult::success(vec![content]))
                }
                Err(e) => {
                    let error_message = format!("Failed to parse aliases: {}", e);

                    error!(error_message);

                    return Err(McpError::new(
                        ErrorCode::INTERNAL_ERROR,
                        error_message,
                        None,
                    ));
                }
            },
            false => {
                let error_message = format!("Failed to list aliases: {}", response.status());

                error!(error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        }
    }

    #[tool(description = "Search for documents in an Elasticsearch index")]
    async fn search(
        &self,
        #[tool(aggr)] SearchRequest {
            base_url,
            api_key,
            index,
            query,
            fields,
            size,
        }: SearchRequest,
    ) -> Result<CallToolResult, McpError> {
        let base_url = base_url.trim_end_matches('/');
        let url = format!("{base_url}/{index}/_search");

        let body = match size {
            Some(size) => json!({
                "query": {
                    "multi_match": {
                        "query": query,
                        "fields": fields
                    },
                    "size": size
                }
            }),
            None => json!({
                "query": {
                    "multi_match": {
                        "query": query,
                        "fields": fields
                    },
                }
            }),
        };

        let client = reqwest::Client::new();
        let result = match api_key {
            Some(api_key) => {
                client
                    .post(&url)
                    .header("Authorization", api_key)
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await
            }
            None => {
                client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await
            }
        };

        let response = match result {
            Ok(response) => response,
            Err(e) => {
                let error_message = format!("Failed to search: {}", e);

                error!(error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        match response.status().is_success() {
            true => match response.json::<Value>().await {
                Ok(json) => {
                    tracing::debug!(
                        "search result json:\n{}",
                        serde_json::to_string_pretty(&json).unwrap()
                    );

                    let search_response: SearchResponse =
                        serde_json::from_value(json).map_err(|e| {
                            let error_message = format!("Failed to parse search result: {}", e);

                            error!(error_message);

                            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
                        })?;

                    let content = Content::json(search_response)?;

                    Ok(CallToolResult::success(vec![content]))
                }
                Err(e) => {
                    let error_message = format!("Failed to parse search result: {}", e);

                    error!(error_message);

                    return Err(McpError::new(
                        ErrorCode::INTERNAL_ERROR,
                        error_message,
                        None,
                    ));
                }
            },
            false => {
                let error_message = format!("Failed to search: {}", response.status());

                error!(error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        }
    }
}
#[tool(tool_box)]
impl ServerHandler for ElasticSearchServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A ElasticSearch MCP server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

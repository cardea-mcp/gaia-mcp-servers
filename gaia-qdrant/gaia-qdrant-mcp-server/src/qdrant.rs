use crate::CONNECTION_CONFIG;
use gaia_qdrant_mcp_common::*;
use rmcp::{
    Error as McpError, ServerHandler,
    model::{CallToolResult, Content, ErrorCode, Implementation, ServerCapabilities, ServerInfo},
    tool,
};
use serde_json::{Value, json};
use tracing::error;

#[derive(Debug, Clone)]
pub struct QdrantServer;
#[tool(tool_box)]
impl QdrantServer {
    #[tool(description = "Perform vector search in the Qdrant database")]
    async fn search(
        &self,
        #[tool(aggr)] SearchPointsRequest { vector }: SearchPointsRequest,
    ) -> Result<CallToolResult, McpError> {
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
        let url = format!(
            "{}/collections/{}/points/search",
            base_url, conn_config.collection
        );

        // get api key
        let api_key = conn_config.api_key;

        // build params
        let params = json!({
            "vector": vector,
            "limit": conn_config.limit,
            "with_payload": true,
            "with_vector": true,
            "score_threshold": conn_config.score_threshold,
        });

        let client = reqwest::Client::new();
        let result = match api_key {
            Some(api_key) => {
                client
                    .post(&url)
                    .header("api-key", api_key)
                    .header("Content-Type", "application/json")
                    .json(&params)
                    .send()
                    .await
            }
            None => {
                client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .json(&params)
                    .send()
                    .await
            }
        };

        let response = match result {
            Ok(response) => response,
            Err(e) => {
                let error_message = format!("Failed to search points: {}", e);

                error!("{}", error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        match response.status().is_success() {
            true => match response.json::<Value>().await {
                Ok(json) => match json.get("result") {
                    Some(result) => {
                        let scored_points = result
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|v| ScoredPoint {
                                score: v.get("score").unwrap().as_f64().unwrap(),
                                payload: v
                                    .get("payload")
                                    .unwrap()
                                    .as_object()
                                    .unwrap()
                                    .to_owned()
                                    .into_iter()
                                    .map(|(k, v)| (k.to_string(), v.clone()))
                                    .collect(),
                                vector: v
                                    .get("vector")
                                    .unwrap()
                                    .as_array()
                                    .unwrap()
                                    .to_owned()
                                    .iter()
                                    .map(|v| v.as_f64().unwrap())
                                    .collect::<Vec<f64>>(),
                            })
                            .collect();

                        let content = Content::json(SearchPointsResponse {
                            result: scored_points,
                            time: json.get("time").unwrap().as_f64().unwrap(),
                        })?;

                        Ok(CallToolResult::success(vec![content]))
                    }
                    None => {
                        let error_message =
                            "Failed to search points. The given key 'result' does not exist.";

                        error!("{}", error_message);

                        return Err(McpError::new(
                            ErrorCode::INTERNAL_ERROR,
                            error_message,
                            None,
                        ));
                    }
                },
                Err(e) => {
                    let error_message = format!("Failed to search points: {}", e);

                    error!("{}", error_message);

                    return Err(McpError::new(
                        ErrorCode::INTERNAL_ERROR,
                        error_message,
                        None,
                    ));
                }
            },
            false => {
                let error_message = format!("Failed to search points: {}", response.status());

                error!("{}", error_message);

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
impl ServerHandler for QdrantServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A MCP server that can access the Qdrant database".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: std::env!("CARGO_PKG_NAME").to_string(),
                version: std::env!("CARGO_PKG_VERSION").to_string(),
            },
            ..Default::default()
        }
    }
}

use rmcp::{
    Error as McpError, ServerHandler,
    model::{CallToolResult, Content, ErrorCode, ServerCapabilities, ServerInfo},
    schemars, tool,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct QdrantServer;
#[tool(tool_box)]
impl QdrantServer {
    #[tool(description = "List all collections in the Qdrant database")]
    async fn list_collections(
        &self,
        #[tool(aggr)] ListCollectionsRequest { url, api_key }: ListCollectionsRequest,
    ) -> Result<CallToolResult, McpError> {
        let url = format!("{}/collections", url.trim_end_matches('/'));
        let client = reqwest::Client::new();
        let result = match api_key {
            Some(api_key) => {
                client
                    .get(&url)
                    .header("api-key", api_key)
                    .header("Content-Type", "application/json")
                    .send()
                    .await
            }
            None => {
                client
                    .get(&url)
                    .header("Content-Type", "application/json")
                    .send()
                    .await
            }
        };

        let response = match result {
            Ok(response) => response,
            Err(e) => {
                let error_message = format!("Failed to list collections: {}", e);

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
                    Some(result) => match result.get("collections") {
                        Some(collections) => match collections.as_array() {
                            Some(collections) => {
                                let mut collection_names: Vec<String> = Vec::<String>::new();

                                for collection in collections {
                                    let name = collection.get("name").unwrap().as_str().unwrap();
                                    collection_names.push(name.to_string());
                                }

                                let content = Content::json(ListCollectionsResponse {
                                    collections: collection_names,
                                })?;

                                let res = CallToolResult::success(vec![content]);

                                Ok(res)
                            }
                            None => {
                                let error_message = "[qdrant] The value corresponding to the 'collections' key is not an array.";

                                error!("{}", error_message);

                                return Err(McpError::new(
                                    ErrorCode::INTERNAL_ERROR,
                                    error_message,
                                    None,
                                ));
                            }
                        },
                        None => {
                            let error_message =
                                "[qdrant] The given key 'collections' does not exist.";

                            error!("{}", error_message);

                            return Err(McpError::new(
                                ErrorCode::INTERNAL_ERROR,
                                error_message,
                                None,
                            ));
                        }
                    },
                    None => {
                        let error_message = "[qdrant] The given key 'result' does not exist.";

                        error!("{}", error_message);

                        return Err(McpError::new(
                            ErrorCode::INTERNAL_ERROR,
                            error_message,
                            None,
                        ));
                    }
                },
                Err(e) => {
                    let error_message = format!("[qdrant] Failed to list collections: {}", e);

                    error!("{}", error_message);

                    return Err(McpError::new(
                        ErrorCode::INTERNAL_ERROR,
                        error_message,
                        None,
                    ));
                }
            },
            false => {
                let error_message = "[qdrant] Failed to list collections";

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
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListCollectionsRequest {
    #[schemars(description = "the base URL of the local or remote Qdrant database")]
    pub url: String,
    #[schemars(description = "the API key to use for the Qdrant database")]
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListCollectionsResponse {
    #[schemars(description = "the list of collection names")]
    pub collections: Vec<String>,
}

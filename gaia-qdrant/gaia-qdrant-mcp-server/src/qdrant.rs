use crate::CONNECTION_CONFIG;
use gaia_qdrant_mcp_common::*;
use rmcp::{
    Error as McpError, ServerHandler,
    handler::server::tool::*,
    model::{
        CallToolResult, Content, ErrorCode, Implementation, ServerCapabilities, ServerInfo, Tool,
    },
    tool,
};
use serde_json::{Value, json};
use std::sync::{Arc, OnceLock};
use tracing::error;

static SEARCH_TOOL_DESC: OnceLock<String> = OnceLock::new();
static QUERY_PARAM_DESC: OnceLock<String> = OnceLock::new();

pub fn set_search_tool_description(description: String) {
    SEARCH_TOOL_DESC.set(description).unwrap_or_default();
}

pub fn set_search_tool_param_description(description: String) {
    QUERY_PARAM_DESC.set(description).unwrap_or_default();
}

#[derive(Debug, Clone)]
pub struct QdrantServer;
impl QdrantServer {
    fn search_tool_attr() -> Tool {
        let tool_description = SEARCH_TOOL_DESC
            .get()
            .cloned()
            .unwrap_or_else(|| "Perform vector search in the Qdrant database".to_string());

        let query_description = QUERY_PARAM_DESC
            .get()
            .cloned()
            .unwrap_or_else(|| "The vector to search for in the Qdrant database".to_string());

        // build input schema
        let input_schema = json!({
            "properties": {
                "vector": {
                    "description": query_description,
                    "items": {
                    "format": "float",
                    "type": "number"
                    },
                    "type": "array"
                }
            },
            "required": [
            "vector"
            ],
            "title": "SearchPointsRequest",
            "type": "object"
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
        let (Parameters(SearchPointsRequest { vector }), _context) =
            <Parameters<SearchPointsRequest>>::from_tool_call_context_part(context)?;
        Self::search(__rmcp_tool_receiver, vector)
            .await
            .into_call_tool_result()
    }

    async fn search(&self, vector: Vec<f32>) -> Result<CallToolResult, McpError> {
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

    fn tool_box() -> &'static ToolBox<QdrantServer> {
        static TOOL_BOX: OnceLock<ToolBox<QdrantServer>> = OnceLock::new();
        TOOL_BOX.get_or_init(|| {
            let mut tool_box = ToolBox::new();
            tool_box.add(ToolBoxItem::new(
                QdrantServer::search_tool_attr(),
                |context| Box::pin(QdrantServer::search_tool_call(context)),
            ));
            tool_box
        })
    }
}

// #[tool(tool_box)]
// impl QdrantServer {
//     #[tool(description = "Perform vector search in the Qdrant database")]
//     async fn search(
//         &self,
//         #[tool(aggr)] SearchPointsRequest { vector }: SearchPointsRequest,
//     ) -> Result<CallToolResult, McpError> {
//         // get connection config
//         let conn_config = match CONNECTION_CONFIG.get() {
//             Some(connection_config) => {
//                 let conn_config = connection_config.read().await;
//                 conn_config.clone()
//             }
//             None => {
//                 let error_message = "Connection config not found";
//                 error!("{}", error_message);
//                 return Err(McpError::new(
//                     ErrorCode::INTERNAL_ERROR,
//                     error_message,
//                     None,
//                 ));
//             }
//         };

//         // build url
//         let base_url = conn_config.base_url.trim_end_matches('/');
//         let url = format!(
//             "{}/collections/{}/points/search",
//             base_url, conn_config.collection
//         );

//         // get api key
//         let api_key = conn_config.api_key;

//         // build params
//         let params = json!({
//             "vector": vector,
//             "limit": conn_config.limit,
//             "with_payload": true,
//             "with_vector": true,
//             "score_threshold": conn_config.score_threshold,
//         });

//         let client = reqwest::Client::new();
//         let result = match api_key {
//             Some(api_key) => {
//                 client
//                     .post(&url)
//                     .header("api-key", api_key)
//                     .header("Content-Type", "application/json")
//                     .json(&params)
//                     .send()
//                     .await
//             }
//             None => {
//                 client
//                     .post(&url)
//                     .header("Content-Type", "application/json")
//                     .json(&params)
//                     .send()
//                     .await
//             }
//         };

//         let response = match result {
//             Ok(response) => response,
//             Err(e) => {
//                 let error_message = format!("Failed to search points: {}", e);

//                 error!("{}", error_message);

//                 return Err(McpError::new(
//                     ErrorCode::INTERNAL_ERROR,
//                     error_message,
//                     None,
//                 ));
//             }
//         };

//         match response.status().is_success() {
//             true => match response.json::<Value>().await {
//                 Ok(json) => match json.get("result") {
//                     Some(result) => {
//                         let scored_points = result
//                             .as_array()
//                             .unwrap()
//                             .iter()
//                             .map(|v| ScoredPoint {
//                                 score: v.get("score").unwrap().as_f64().unwrap(),
//                                 payload: v
//                                     .get("payload")
//                                     .unwrap()
//                                     .as_object()
//                                     .unwrap()
//                                     .to_owned()
//                                     .into_iter()
//                                     .map(|(k, v)| (k.to_string(), v.clone()))
//                                     .collect(),
//                                 vector: v
//                                     .get("vector")
//                                     .unwrap()
//                                     .as_array()
//                                     .unwrap()
//                                     .to_owned()
//                                     .iter()
//                                     .map(|v| v.as_f64().unwrap())
//                                     .collect::<Vec<f64>>(),
//                             })
//                             .collect();

//                         let content = Content::json(SearchPointsResponse {
//                             result: scored_points,
//                             time: json.get("time").unwrap().as_f64().unwrap(),
//                         })?;

//                         Ok(CallToolResult::success(vec![content]))
//                     }
//                     None => {
//                         let error_message =
//                             "Failed to search points. The given key 'result' does not exist.";

//                         error!("{}", error_message);

//                         return Err(McpError::new(
//                             ErrorCode::INTERNAL_ERROR,
//                             error_message,
//                             None,
//                         ));
//                     }
//                 },
//                 Err(e) => {
//                     let error_message = format!("Failed to search points: {}", e);

//                     error!("{}", error_message);

//                     return Err(McpError::new(
//                         ErrorCode::INTERNAL_ERROR,
//                         error_message,
//                         None,
//                     ));
//                 }
//             },
//             false => {
//                 let error_message = format!("Failed to search points: {}", response.status());

//                 error!("{}", error_message);

//                 return Err(McpError::new(
//                     ErrorCode::INTERNAL_ERROR,
//                     error_message,
//                     None,
//                 ));
//             }
//         }
//     }
// }
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

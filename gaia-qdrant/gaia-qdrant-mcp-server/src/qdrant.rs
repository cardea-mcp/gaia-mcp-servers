use crate::CONNECTION_CONFIG;
use gaia_qdrant_mcp_common::*;
use rmcp::{
    Error as McpError, ServerHandler,
    model::{CallToolResult, Content, ErrorCode, Implementation, ServerCapabilities, ServerInfo},
    tool,
};
use serde_json::{Value, json};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct QdrantServer;
#[tool(tool_box)]
impl QdrantServer {
    #[tool(description = "Create a new collection in the Qdrant database")]
    async fn create_collection(
        &self,
        #[tool(aggr)] CreateCollectionRequest { name, size }: CreateCollectionRequest,
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
        let url = format!("{}/collections/{}", base_url, name);

        // get api key
        let api_key = conn_config.api_key;

        // build params
        let params = json!({
            "vectors": {
                "size": size,
                "distance": "Cosine",
                "on_disk": true,
            }
        });

        // create client
        let client = reqwest::Client::new();
        let result = match api_key {
            Some(api_key) => {
                client
                    .put(&url)
                    .header("api-key", api_key)
                    .header("Content-Type", "application/json")
                    .json(&params)
                    .send()
                    .await
            }
            None => {
                client
                    .put(&url)
                    .header("Content-Type", "application/json")
                    .json(&params)
                    .send()
                    .await
            }
        };

        // handle response
        let response = match result {
            Ok(response) => response,
            Err(e) => {
                let error_message = format!("Failed to create collection: {}", e);

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
                        let content = Content::json(CreateCollectionResponse {
                            result: result.as_bool().unwrap(),
                            time: json.get("time").unwrap().as_f64().unwrap(),
                        })?;

                        Ok(CallToolResult::success(vec![content]))
                    }
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
                    let error_message = format!("Failed to create collection: {}", e);

                    error!("{}", error_message);

                    return Err(McpError::new(
                        ErrorCode::INTERNAL_ERROR,
                        error_message,
                        None,
                    ));
                }
            },
            false => {
                let error_message = format!("Failed to create collection: {}", response.status());

                error!("{}", error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        }
    }

    #[tool(description = "List all collections in the Qdrant database")]
    async fn list_collections(&self) -> Result<CallToolResult, McpError> {
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
        let url = format!("{}/collections", base_url);

        // get api key
        let api_key = conn_config.api_key;

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
                                    time: json.get("time").unwrap().as_f64().unwrap(),
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

    #[tool(description = "Check if a collection exists in the Qdrant database")]
    async fn collection_exists(
        &self,
        #[tool(aggr)] CollectionExistsRequest { name }: CollectionExistsRequest,
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
        let url = format!("{}/collections/{}/exists", base_url, name);

        // get api key
        let api_key = conn_config.api_key;

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
                let error_message = format!("Failed to check if collection exists: {}", e);

                error!("{}", error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        info!("response: {:?}", &response);

        match response.status().is_success() {
            true => match response.json::<Value>().await {
                Ok(json) => match json.get("result") {
                    Some(result) => match result.get("exists") {
                        Some(exists) => {
                            let content = Content::json(CollectionExistsResponse {
                                result: exists.as_bool().unwrap(),
                            })?;

                            Ok(CallToolResult::success(vec![content]))
                        }
                        None => {
                            let error_message = "Failed to check if collection exists. The given key 'exists' does not exist.";

                            error!("{}", error_message);

                            return Err(McpError::new(
                                ErrorCode::INTERNAL_ERROR,
                                error_message,
                                None,
                            ));
                        }
                    },
                    None => {
                        let error_message = "Failed to check if collection exists. The given key 'result' does not exist.";

                        error!("{}", error_message);

                        return Err(McpError::new(
                            ErrorCode::INTERNAL_ERROR,
                            error_message,
                            None,
                        ));
                    }
                },
                Err(e) => {
                    let error_message = format!("Failed to check if collection exists: {}", e);

                    error!("{}", error_message);

                    return Err(McpError::new(
                        ErrorCode::INTERNAL_ERROR,
                        error_message,
                        None,
                    ));
                }
            },
            false => {
                let error_message = "Failed to check if collection exists";

                error!("{}", error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        }
    }

    #[tool(description = "Delete a collection in the Qdrant database")]
    async fn delete_collection(
        &self,
        #[tool(aggr)] DeleteCollectionRequest { name }: DeleteCollectionRequest,
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
        let url = format!("{}/collections/{}", base_url, name);

        // get api key
        let api_key = conn_config.api_key;

        let client = reqwest::Client::new();
        let result = match api_key {
            Some(api_key) => {
                client
                    .delete(&url)
                    .header("api-key", api_key)
                    .header("Content-Type", "application/json")
                    .send()
                    .await
            }
            None => {
                client
                    .delete(&url)
                    .header("Content-Type", "application/json")
                    .send()
                    .await
            }
        };

        let response = match result {
            Ok(response) => response,
            Err(e) => {
                let error_message = format!("Failed to delete collection: {}", e);

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
                        let content = Content::json(DeleteCollectionResponse {
                            result: result.as_bool().unwrap(),
                            time: json.get("time").unwrap().as_f64().unwrap(),
                        })?;

                        Ok(CallToolResult::success(vec![content]))
                    }
                    None => {
                        let error_message =
                            "Failed to delete collection. The given key 'result' does not exist.";

                        error!("{}", error_message);

                        return Err(McpError::new(
                            ErrorCode::INTERNAL_ERROR,
                            error_message,
                            None,
                        ));
                    }
                },
                Err(e) => {
                    let error_message = format!("Failed to delete collection: {}", e);

                    error!("{}", error_message);

                    return Err(McpError::new(
                        ErrorCode::INTERNAL_ERROR,
                        error_message,
                        None,
                    ));
                }
            },
            false => {
                let error_message = "Failed to delete collection";

                error!("{}", error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        }
    }

    #[tool(description = "Upsert points into a collection in the Qdrant database")]
    async fn upsert_points(
        &self,
        #[tool(aggr)] UpsertPointsRequest { name, points }: UpsertPointsRequest,
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
        let url = format!("{}/collections/{}/points", base_url, name);

        // get api key
        let api_key = conn_config.api_key;

        let params = json!({
            "points": points,
        });

        let client = reqwest::Client::new();
        let result = match api_key {
            Some(api_key) => {
                client
                    .put(&url)
                    .header("api-key", api_key)
                    .header("Content-Type", "application/json")
                    .json(&params)
                    .send()
                    .await
            }
            None => {
                client
                    .put(&url)
                    .header("Content-Type", "application/json")
                    .json(&params)
                    .send()
                    .await
            }
        };

        let response = match result {
            Ok(response) => response,
            Err(e) => {
                let error_message = format!("Failed to upsert points: {}", e);

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
                        let content = Content::json(UpsertPointsResponse {
                            status: result.get("status").unwrap().as_str().unwrap().to_string(),
                            time: json.get("time").unwrap().as_f64().unwrap(),
                        })?;

                        Ok(CallToolResult::success(vec![content]))
                    }
                    None => {
                        let error_message =
                            "Failed to upsert points. The given key 'result' does not exist.";

                        error!("{}", error_message);

                        return Err(McpError::new(
                            ErrorCode::INTERNAL_ERROR,
                            error_message,
                            None,
                        ));
                    }
                },
                Err(e) => {
                    let error_message = format!("Failed to upsert points: {}", e);

                    error!("{}", error_message);

                    return Err(McpError::new(
                        ErrorCode::INTERNAL_ERROR,
                        error_message,
                        None,
                    ));
                }
            },
            false => {
                let error_message = "Failed to upsert points";

                error!("{}", error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        }
    }

    #[tool(description = "Search for points in a collection in the Qdrant database")]
    async fn search_points(
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

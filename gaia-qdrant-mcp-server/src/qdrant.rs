use gaia_qdrant_common::*;
use rmcp::{
    Error as McpError, ServerHandler,
    model::{CallToolResult, Content, ErrorCode, ServerCapabilities, ServerInfo},
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
        #[tool(aggr)] CreateCollectionRequest {
            base_url,
            api_key,
            name,
            size,
        }: CreateCollectionRequest,
    ) -> Result<CallToolResult, McpError> {
        let base_url = base_url.trim_end_matches('/');
        let url = format!("{}/collections/{}", base_url, name);

        let params = json!({
            "vectors": {
                "size": size,
                "distance": "Cosine",
                "on_disk": true,
            }
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
    async fn list_collections(
        &self,
        #[tool(aggr)] ListCollectionsRequest { base_url, api_key }: ListCollectionsRequest,
    ) -> Result<CallToolResult, McpError> {
        let base_url = base_url.trim_end_matches('/');
        let url = format!("{}/collections", base_url);

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
        #[tool(aggr)] CollectionExistsRequest {
            base_url,
            api_key,
            name,
        }: CollectionExistsRequest,
    ) -> Result<CallToolResult, McpError> {
        let base_url = base_url.trim_end_matches('/');
        let url = format!("{}/collections/{}/exists", base_url, name);

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
        #[tool(aggr)] DeleteCollectionRequest {
            base_url,
            api_key,
            name,
        }: DeleteCollectionRequest,
    ) -> Result<CallToolResult, McpError> {
        let base_url = base_url.trim_end_matches('/');
        let url = format!("{}/collections/{}", base_url, name);

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

    // fn create_client(url: &str, api_key: Option<String>) -> Result<Qdrant, McpError> {
    //     let api_key = api_key.unwrap_or_default();

    //     // create qdrant client
    //     let client = Qdrant::from_url(&url)
    //         .api_key(api_key)
    //         .build()
    //         .map_err(|e| {
    //             McpError::new(
    //                 ErrorCode::INTERNAL_ERROR,
    //                 format!("Failed to create Qdrant client: {}", e),
    //                 None,
    //             )
    //         })?;

    //     Ok(client)
    // }

    #[tool(description = "Upsert points into a collection in the Qdrant database")]
    async fn upsert_points(
        &self,
        #[tool(aggr)] UpsertPointsRequest {
            base_url,
            api_key,
            name,
            points,
        }: UpsertPointsRequest,
    ) -> Result<CallToolResult, McpError> {
        let base_url = base_url.trim_end_matches('/');
        let url = format!("{}/collections/{}/points", base_url, name);

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
        #[tool(aggr)] SearchPointsRequest {
            base_url,
            api_key,
            name,
            vector,
            limit,
            score_threshold,
        }: SearchPointsRequest,
    ) -> Result<CallToolResult, McpError> {
        let base_url = base_url.trim_end_matches('/');
        let url = format!("{}/collections/{}/points/search", base_url, name);

        let params = json!({
            "vector": vector,
            "limit": limit,
            "with_payload": true,
            "with_vector": true,
            "score_threshold": score_threshold.unwrap_or(0.0),
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
            ..Default::default()
        }
    }
}

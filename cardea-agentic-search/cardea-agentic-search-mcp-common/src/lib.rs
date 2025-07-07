use mysql_common::prelude::FromRow;
use rmcp::schemars;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchRequest {
    #[schemars(description = "The query to search for")]
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SearchResponse {
    #[schemars(description = "The result of the search operation")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, FromRow)]
pub struct TidbSearchHit {
    #[schemars(description = "the id of the tidb server")]
    pub id: i32,
    #[schemars(description = "the title of the tidb server")]
    pub title: String,
    #[schemars(description = "the content of the tidb server")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct QdrantSearchHit {
    #[schemars(description = "the score of the point")]
    pub score: f64,
    #[schemars(description = "the payload of the point")]
    pub payload: HashMap<String, Value>,
    #[schemars(description = "the vector of the point")]
    pub vector: Vec<f64>,
}

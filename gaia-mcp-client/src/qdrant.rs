use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DeleteCollectionResponse {
    #[schemars(description = "if the collection was deleted")]
    pub result: bool,
    #[schemars(description = "the time it took to delete the collection")]
    pub time: f64,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CollectionExistsResponse {
    #[schemars(description = "if the collection exists")]
    pub result: bool,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct UpsertPointsResponse {
    #[schemars(
        description = "the status of the upsert operation. Allowed values: 'acknowledged', 'completed'"
    )]
    pub status: String,
    #[schemars(description = "the time it took to upsert the points")]
    pub time: f64,
}
impl From<rmcp::model::CallToolResult> for UpsertPointsResponse {
    fn from(result: rmcp::model::CallToolResult) -> Self {
        let content = result.content[0].as_text().unwrap().text.as_ref();
        let response = serde_json::from_str::<UpsertPointsResponse>(content).unwrap();
        response
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Point {
    #[schemars(description = "the id of the point")]
    pub id: u64,
    #[schemars(description = "the payload of the point")]
    pub payload: Map<String, Value>,
    #[schemars(description = "the vector of the point")]
    pub vector: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SearchPointsResponse {
    #[schemars(description = "the results of the search")]
    pub result: Vec<ScoredPoint>,
    #[schemars(description = "the time it took to search the points")]
    pub time: f64,
}
impl From<rmcp::model::CallToolResult> for SearchPointsResponse {
    fn from(result: rmcp::model::CallToolResult) -> Self {
        let content = result.content[0].as_text().unwrap().text.as_ref();
        let response = serde_json::from_str::<SearchPointsResponse>(content).unwrap();
        response
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ScoredPoint {
    #[schemars(description = "the score of the point")]
    pub score: f64,
    #[schemars(description = "the payload of the point")]
    pub payload: HashMap<String, Value>,
    #[schemars(description = "the vector of the point")]
    pub vector: Vec<f64>,
}

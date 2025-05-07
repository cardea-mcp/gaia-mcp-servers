use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListIndicesRequest {
    pub base_url: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListIndicesResponse {
    pub indices: Vec<IndexInfo>,
}

impl From<rmcp::model::CallToolResult> for ListIndicesResponse {
    fn from(result: rmcp::model::CallToolResult) -> Self {
        let content = result.content[0].as_text().unwrap().text.as_ref();
        serde_json::from_str::<ListIndicesResponse>(content).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct IndexInfo {
    /// current health status
    pub health: String,
    /// open/close status
    pub status: String,
    /// index name
    pub index: String,
    /// uuid
    pub uuid: String,
    /// number of primary shards
    pub pri: String,
    /// number of replica shards
    pub rep: String,
    /// available docs
    #[serde(rename = "docs.count")]
    pub docs_count: String,
    /// deleted docs
    #[serde(rename = "docs.deleted")]
    pub docs_deleted: String,
    /// store size
    #[serde(rename = "store.size")]
    pub store_size: String,
    /// primary shard size
    #[serde(rename = "pri.store.size")]
    pub pri_store_size: String,
    /// dataset size
    #[serde(rename = "dataset.size")]
    pub dataset_size: String,
}

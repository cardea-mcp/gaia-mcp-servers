use endpoints::rag::keyword_search::SearchHit;
use mysql_common::prelude::FromRow;
use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TidbSearchRequest {
    #[schemars(description = "The query to search for")]
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct TidbSearchResponse {
    #[schemars(description = "the hits of the tidb server")]
    pub hits: Vec<TidbSearchHit>,
}
impl From<rmcp::model::CallToolResult> for TidbSearchResponse {
    fn from(result: rmcp::model::CallToolResult) -> Self {
        let content = result.content[0].as_text().unwrap().text.as_ref();
        serde_json::from_str::<TidbSearchResponse>(content).unwrap()
    }
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
impl From<TidbSearchHit> for SearchHit {
    fn from(value: TidbSearchHit) -> Self {
        SearchHit {
            title: value.title,
            content: value.content,
            score: 0.0,
        }
    }
}

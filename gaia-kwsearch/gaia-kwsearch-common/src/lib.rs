use endpoints::keyword_search::{
    DocumentInput, DocumentResult, IndexResponse, QueryResponse, SearchHit,
};
use rmcp::{model::CallToolResult, schemars};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CreateIndexRequest {
    #[schemars(description = "the base URL of the local or remote KeywordSearch database")]
    pub base_url: String,
    // #[schemars(description = "the API key to use for the KeywordSearch database")]
    // pub api_key: Option<String>,
    #[schemars(description = "the name of the index to create")]
    pub name: Option<String>,
    #[schemars(description = "the documents to index")]
    pub documents: Vec<KwDocumentInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct KwDocumentInput {
    #[schemars(description = "the content of the document")]
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "the title of the document")]
    pub title: Option<String>,
}
impl From<KwDocumentInput> for DocumentInput {
    fn from(value: KwDocumentInput) -> Self {
        Self {
            content: value.content,
            title: value.title,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CreateIndexResponse {
    #[schemars(description = "the name of the index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[schemars(description = "the results of the indexing operation")]
    pub results: Vec<KwDocumentResult>,
}
impl From<IndexResponse> for CreateIndexResponse {
    fn from(value: IndexResponse) -> Self {
        Self {
            index_name: value.index_name,
            results: value.results.into_iter().map(|r| r.into()).collect(),
        }
    }
}
impl From<CallToolResult> for CreateIndexResponse {
    fn from(value: CallToolResult) -> Self {
        let content = value.content[0].as_text().unwrap().text.as_ref();
        let response = serde_json::from_str::<CreateIndexResponse>(content).unwrap();
        response
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct KwDocumentResult {
    #[schemars(description = "the filename of the document")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[schemars(description = "the status of the indexing operation")]
    pub status: String,
    #[schemars(description = "the error of the indexing operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
impl From<DocumentResult> for KwDocumentResult {
    fn from(value: DocumentResult) -> Self {
        Self {
            filename: value.filename,
            status: value.status,
            error: value.error,
        }
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchDocumentsRequest {
    #[schemars(description = "the base URL of the local or remote KeywordSearch database")]
    pub base_url: String,
    // #[schemars(description = "the API key to use for the KeywordSearch database")]
    // pub api_key: Option<String>,
    #[schemars(description = "the index to search")]
    pub index_name: String,
    #[schemars(description = "the query to search for")]
    pub query: String,
    #[schemars(description = "the number of results to return")]
    pub limit: usize,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SearchDocumentsResponse {
    #[schemars(description = "the results of the search operation")]
    pub hits: Vec<KwSearchHit>,
}
impl From<QueryResponse> for SearchDocumentsResponse {
    fn from(value: QueryResponse) -> Self {
        Self {
            hits: value.hits.into_iter().map(|h| h.into()).collect(),
        }
    }
}
impl From<CallToolResult> for SearchDocumentsResponse {
    fn from(value: CallToolResult) -> Self {
        let content = value.content[0].as_text().unwrap().text.as_ref();
        let response = serde_json::from_str::<SearchDocumentsResponse>(content).unwrap();
        response
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct KwSearchHit {
    #[schemars(description = "the title of the document")]
    pub title: String,
    #[schemars(description = "the content of the document")]
    pub content: String,
    #[schemars(description = "the score of the document")]
    pub score: f64,
}
impl From<SearchHit> for KwSearchHit {
    fn from(value: SearchHit) -> Self {
        Self {
            title: value.title,
            content: value.content,
            score: value.score,
        }
    }
}

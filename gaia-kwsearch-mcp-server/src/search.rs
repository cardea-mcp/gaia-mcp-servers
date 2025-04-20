use endpoints::keyword_search::{
    DocumentInput, DocumentResult, IndexRequest, IndexResponse, QueryRequest, QueryResponse,
    SearchHit,
};
use rmcp::{
    Error as McpError, ServerHandler,
    model::{CallToolResult, Content, ErrorCode, ServerCapabilities, ServerInfo},
    schemars, tool,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct KeywordSearchServer;
#[tool(tool_box)]
impl ServerHandler for KeywordSearchServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A MCP server that can access the KeywordSearch database".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
#[tool(tool_box)]
impl KeywordSearchServer {
    #[tool(description = "Create an index in the KeywordSearch database")]
    async fn create_index(
        &self,
        #[tool(aggr)] CreateIndexRequest {
            base_url,
            // api_key,
            // name,
            documents,
        }: CreateIndexRequest,
    ) -> Result<CallToolResult, McpError> {
        info!("Creating index in KeywordSearch database");

        let base_url = base_url.trim_end_matches('/');
        let url = format!("{}/v1/index/create", base_url);

        let index_request = IndexRequest {
            documents: documents.into_iter().map(|d| d.into()).collect::<Vec<_>>(),
        };

        let response = reqwest::Client::new()
            .post(&url)
            .json(&index_request)
            .send()
            .await
            .map_err(|e| {
                let error_message = format!("Failed to create index: {}", e);

                error!("{}", error_message);

                McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
            })?;

        let index_response = response.json::<IndexResponse>().await.map_err(|e| {
            let error_message = format!("Failed to parse index response: {}", e);

            error!("{}", error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        let content = Content::json(CreateIndexResponse::from(index_response))?;

        info!("Index created in KeywordSearch database");

        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Search for documents in the KeywordSearch database")]
    async fn search_documents(
        &self,
        #[tool(aggr)] SearchDocumentsRequest {
            base_url,
            // api_key,
            index_name,
            query,
            limit,
        }: SearchDocumentsRequest,
    ) -> Result<CallToolResult, McpError> {
        info!("Searching for documents in KeywordSearch database");

        let base_url = base_url.trim_end_matches('/');
        let url = format!("{}/v1/search", base_url);

        let query_request = QueryRequest {
            query,
            top_k: limit,
            index: index_name,
        };

        let response = reqwest::Client::new()
            .post(&url)
            .json(&query_request)
            .send()
            .await
            .map_err(|e| {
                let error_message = format!("Failed to search documents: {}", e);

                error!("{}", error_message);

                McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
            })?;

        let query_response = response.json::<QueryResponse>().await.map_err(|e| {
            let error_message = format!("Failed to parse query response: {}", e);

            error!("{}", error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        let content = Content::json(SearchDocumentsResponse::from(query_response))?;

        info!("Documents searched in KeywordSearch database");

        Ok(CallToolResult::success(vec![content]))
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CreateIndexRequest {
    #[schemars(description = "the base URL of the local or remote KeywordSearch database")]
    pub base_url: String,
    // #[schemars(description = "the API key to use for the KeywordSearch database")]
    // pub api_key: Option<String>,
    // #[schemars(description = "the name of the index to create")]
    // pub name: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct KwDocumentResult {
    #[schemars(description = "the filename of the document")]
    pub filename: String,
    #[schemars(description = "the status of the indexing operation")]
    pub status: String,
    #[schemars(description = "the error of the indexing operation")]
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

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct KwSearchHit {
    #[schemars(description = "the title of the document")]
    pub title: String,
    #[schemars(description = "the content of the document")]
    pub content: String,
    #[schemars(description = "the score of the document")]
    pub score: f32,
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

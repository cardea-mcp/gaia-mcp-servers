use rmcp::{
    ServerHandler,
    model::{ServerCapabilities, ServerInfo},
    schemars, tool,
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchRequest {
    #[schemars(description = "the query to search for")]
    pub query: String,
}

#[derive(Debug, Clone)]
pub struct SearchServer;
#[tool(tool_box)]
impl SearchServer {
    #[tool(description = "Search the web for information")]
    fn search(&self, #[tool(aggr)] SearchRequest { query }: SearchRequest) -> String {
        format!("Searching for: {}", query)
    }
}

#[tool(tool_box)]
impl ServerHandler for SearchServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A search agent to perform vector search and keyword search".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

use gaia_github_mcp_common::GetStarCountResponse;
use octocrab::OctocrabBuilder;
use rmcp::{
    Error as McpError, ServerHandler,
    model::{CallToolResult, Content, ErrorCode, Implementation, ServerCapabilities, ServerInfo},
    tool,
};
use tracing::error;

#[derive(Debug, Clone)]
pub struct GithubServer;
#[tool(tool_box)]
impl GithubServer {
    #[tool(description = "Get the star count of a Github repository")]
    async fn get_star_count(
        &self,
        #[tool(param)]
        #[schemars(description = "The owner of the Github repository")]
        owner: String,
        #[tool(param)]
        #[schemars(description = "The name of the Github repository")]
        repo: String,
    ) -> Result<CallToolResult, McpError> {
        let octocrab = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            OctocrabBuilder::new()
                .personal_token(token)
                .build()
                .map_err(|e| {
                    let error_message = format!("Failed to build Octocrab: {}", e);
                    error!("{}", error_message);
                    McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
                })?
        } else {
            OctocrabBuilder::new().build().map_err(|e| {
                let error_message = format!("Failed to build Octocrab: {}", e);
                error!("{}", error_message);
                McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
            })?
        };

        let repo = octocrab.repos(owner, repo).get().await.map_err(|e| {
            let error_message = format!("Failed to get repository: {}", e);
            error!("{}", error_message);
            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        let response = GetStarCountResponse {
            count: repo.stargazers_count.unwrap_or(0),
        };

        let content = Content::json(response)?;

        Ok(CallToolResult::success(vec![content]))
    }
}
#[tool(tool_box)]
impl ServerHandler for GithubServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Gaia Github MCP server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: std::env!("CARGO_PKG_NAME").to_string(),
                version: std::env!("CARGO_PKG_VERSION").to_string(),
            },
            ..Default::default()
        }
    }
}

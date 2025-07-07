use cardea_github_mcp_common::GetStarCountResponse;
use clap::{Parser, ValueEnum};
use rmcp::{
    model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation},
    service::ServiceExt,
    transport::{SseClientTransport, StreamableHttpClientTransport},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8008";

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Sse,
    StreamHttp,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Cardea Github MCP client")]
struct Args {
    /// Transport type to use
    #[arg(short, long, value_enum, default_value = "stream-http")]
    transport: TransportType,
    /// The owner of the Github repository
    #[arg(short, long, required = true)]
    owner: String,
    /// The name of the Github repository
    #[arg(short, long, required = true)]
    repo: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("info,{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer().with_line_number(true))
        .init();

    let cli = Args::parse();

    let owner = &cli.owner;
    let repo = &cli.repo;

    match cli.transport {
        TransportType::Sse => {
            let url = format!("http://{SOCKET_ADDR}/sse");
            tracing::info!("Connecting to Github MCP server via sse: {}", url);

            let transport = SseClientTransport::start(url).await?;
            let client_info = ClientInfo {
                protocol_version: Default::default(),
                capabilities: ClientCapabilities::default(),
                client_info: Implementation {
                    name: "test sse client".to_string(),
                    version: "0.1.0".to_string(),
                },
            };
            let service = client_info.serve(transport).await.inspect_err(|e| {
                tracing::error!("client error: {:?}", e);
            })?;

            // Initialize
            let server_info = service.peer_info();
            tracing::info!("Connected to server: {server_info:#?}");

            // List available tools
            let tools = service.list_all_tools().await?;
            tracing::info!(
                "Available tools:\n{}",
                serde_json::to_string_pretty(&tools)?
            );

            // request param
            let request_param = CallToolRequestParam {
                name: "get_star_count".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "owner".to_string(),
                        serde_json::Value::String(owner.to_string()),
                    ),
                    (
                        "repo".to_string(),
                        serde_json::Value::String(repo.to_string()),
                    ),
                ])),
            };

            // call tool
            let tool_result = service.call_tool(request_param).await?;
            let response = GetStarCountResponse::from(tool_result);

            // parse tool result
            tracing::info!(
                "star count response:\n{}",
                serde_json::to_string_pretty(&response)?
            );

            service.cancel().await?;
        }
        TransportType::StreamHttp => {
            let url = format!("http://{SOCKET_ADDR}/mcp");
            tracing::info!(
                "Connecting to Cardea Github MCP server via stream-http: {}",
                url
            );

            let transport = StreamableHttpClientTransport::from_uri(url);
            let client_info = ClientInfo {
                protocol_version: Default::default(),
                capabilities: ClientCapabilities::default(),
                client_info: Implementation {
                    name: "test stream-http client".to_string(),
                    version: "0.0.1".to_string(),
                },
            };
            let service = client_info.serve(transport).await.inspect_err(|e| {
                tracing::error!("client error: {:?}", e);
            })?;

            // Initialize
            let server_info = service.peer_info();
            tracing::info!("Connected to server: {server_info:#?}");

            // List available tools
            let tools = service.list_all_tools().await?;
            tracing::info!(
                "Available tools:\n{}",
                serde_json::to_string_pretty(&tools)?
            );

            // request param
            let request_param = CallToolRequestParam {
                name: "get_star_count".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "owner".to_string(),
                        serde_json::Value::String(owner.to_string()),
                    ),
                    (
                        "repo".to_string(),
                        serde_json::Value::String(repo.to_string()),
                    ),
                ])),
            };

            // call tool
            let tool_result = service.call_tool(request_param).await?;
            let response = GetStarCountResponse::from(tool_result);

            // parse tool result
            tracing::info!(
                "star count response:\n{}",
                serde_json::to_string_pretty(&response)?
            );

            service.cancel().await?;
        }
    }

    Ok(())
}

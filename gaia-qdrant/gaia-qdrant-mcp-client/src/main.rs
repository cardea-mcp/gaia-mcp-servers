use clap::{Parser, ValueEnum};
use gaia_qdrant_mcp_common::SearchPointsResponse;
use rmcp::{
    model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation},
    service::ServiceExt,
    transport::{
        ConfigureCommandExt, SseClientTransport, StreamableHttpClientTransport, TokioChildProcess,
    },
};
use tokio::process::Command;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8003";
const QDRANT_BASE_URL: &str = "http://127.0.0.1:6333";

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Stdio,
    Sse,
    StreamHttp,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Gaia Qdrant MCP client")]
struct Args {
    /// Transport type to use (tcp or stdio)
    #[arg(short, long, value_enum, default_value = "stream-http")]
    transport: TransportType,
    /// The name of the collection to use
    #[arg(short, long, required = true)]
    collection: String,
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

    // create a mcp client
    match cli.transport {
        TransportType::StreamHttp => {
            let url = format!("http://{SOCKET_ADDR}/mcp");
            tracing::info!(
                "Connecting to Gaia Qdrant MCP server via stream-http: {}",
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
            let mcp_client = client_info.serve(transport).await.inspect_err(|e| {
                tracing::error!("client error: {:?}", e);
            })?;

            // Initialize
            let server_info = mcp_client.peer_info();
            tracing::info!("Connected to server: {server_info:#?}");

            // List tools
            let tools = mcp_client.list_tools(Default::default()).await?;
            tracing::info!(
                "Available tools:\n{}",
                serde_json::to_string_pretty(&tools)?
            );

            // * search points
            let search_points = CallToolRequestParam {
                name: "search".into(),
                arguments: Some(serde_json::Map::from_iter([(
                    "vector".to_string(),
                    serde_json::Value::Array(
                        vec![0.2, 0.1, 0.9, 0.7]
                            .into_iter()
                            .map(|v| serde_json::Value::from(v))
                            .collect(),
                    ),
                )])),
            };
            let tool_result = mcp_client.peer().call_tool(search_points).await?;
            let response = SearchPointsResponse::from(tool_result);
            tracing::info!("search points response:\n{:?}", &response);

            mcp_client.cancel().await?;
        }
        TransportType::Sse => {
            let url = format!("http://{SOCKET_ADDR}/sse");
            tracing::info!("Connecting to Gaia Qdrant MCP server via sse: {}", url);

            let transport = SseClientTransport::start(url).await?;
            let client_info = ClientInfo {
                protocol_version: Default::default(),
                capabilities: ClientCapabilities::default(),
                client_info: Implementation {
                    name: "test sse client".to_string(),
                    version: "0.1.0".to_string(),
                },
            };
            let mcp_client = client_info.serve(transport).await.inspect_err(|e| {
                tracing::error!("client error: {:?}", e);
            })?;

            // Initialize
            let server_info = mcp_client.peer_info();
            tracing::info!("Connected to server: {server_info:#?}");

            // List available tools
            let tools = mcp_client.peer().list_tools(Default::default()).await?;
            tracing::info!(
                "Available tools:\n{}",
                serde_json::to_string_pretty(&tools)?
            );

            // * search points
            let search_points = CallToolRequestParam {
                name: "search".into(),
                arguments: Some(serde_json::Map::from_iter([(
                    "vector".to_string(),
                    serde_json::Value::Array(
                        vec![0.2, 0.1, 0.9, 0.7]
                            .into_iter()
                            .map(|v| serde_json::Value::from(v))
                            .collect(),
                    ),
                )])),
            };
            let tool_result = mcp_client.peer().call_tool(search_points).await?;
            let response = SearchPointsResponse::from(tool_result);
            tracing::info!("search points response:\n{:?}", &response);

            mcp_client.cancel().await?;
        }
        TransportType::Stdio => {
            tracing::info!("Connecting to MCP server via stdio");

            // build command
            let cmd = Command::new("./target/release/gaia-qdrant-mcp-server").configure(|cmd| {
                cmd.arg("--base-url")
                    .arg(QDRANT_BASE_URL)
                    .arg("--transport")
                    .arg("stdio")
                    .arg("--collection")
                    .arg(cli.collection);
            });

            // start mcp server
            let transport = TokioChildProcess::new(cmd)?;

            // create mcp client
            let mcp_client = ().serve(transport).await?;
            tracing::info!("Connected to server");

            // Initialize
            let server_info = mcp_client.peer_info();
            tracing::info!("Connected to server: {server_info:#?}");

            // * search points
            let search_points = CallToolRequestParam {
                name: "search".into(),
                arguments: Some(serde_json::Map::from_iter([(
                    "vector".to_string(),
                    serde_json::Value::Array(
                        vec![0.2, 0.1, 0.9, 0.7]
                            .into_iter()
                            .map(|v| serde_json::Value::from(v))
                            .collect(),
                    ),
                )])),
            };
            let tool_result = mcp_client.call_tool(search_points).await?;
            let response = SearchPointsResponse::from(tool_result);
            tracing::info!("search points response:\n{:?}", &response);

            mcp_client.cancel().await?;
        }
    };

    Ok(())
}

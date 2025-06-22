use clap::{Parser, ValueEnum};
use gaia_tidb_mcp_common::TidbSearchResponse;
use rmcp::{
    model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation},
    service::ServiceExt,
    transport::{SseClientTransport, StreamableHttpClientTransport},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8007";

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Sse,
    StreamHttp,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Gaia TiDB MCP client")]
struct Args {
    /// Transport type to use
    #[arg(short, long, value_enum, default_value = "stream-http")]
    transport: TransportType,
    /// query
    #[arg(long)]
    query: String,
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

    let client_version = env!("CARGO_PKG_VERSION");
    let client_name = env!("CARGO_PKG_NAME");

    match cli.transport {
        TransportType::Sse => {
            let url = format!("http://{SOCKET_ADDR}/sse");
            tracing::info!("Connecting to Gaia TiDB MCP server via sse: {}", url);

            let transport = SseClientTransport::start(url).await?;
            let client_info = ClientInfo {
                protocol_version: Default::default(),
                capabilities: ClientCapabilities::default(),
                client_info: Implementation {
                    name: client_name.to_string(),
                    version: client_version.to_string(),
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

            // create request param
            let request_param = CallToolRequestParam {
                name: "search".into(),
                arguments: Some(serde_json::Map::from_iter([(
                    "query".to_string(),
                    serde_json::Value::from(cli.query.clone()),
                )])),
            };

            let tool_result = service.call_tool(request_param).await?;
            tracing::info!(
                "search response:\n{}",
                serde_json::to_string_pretty(&tool_result)?
            );

            // parse tool result
            let search_result = TidbSearchResponse::from(tool_result);
            tracing::info!(
                "search_result:\n{}",
                serde_json::to_string_pretty(&search_result)?
            );

            service.cancel().await?;
        }
        TransportType::StreamHttp => {
            let url = format!("http://{SOCKET_ADDR}/mcp");
            tracing::info!(
                "Connecting to Gaia TiDB MCP server via stream-http: {}",
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

            // List tools
            let tools = service.list_all_tools().await?;
            tracing::info!("Available tools: {tools:#?}");

            // create request param
            let request_param = CallToolRequestParam {
                name: "search".into(),
                arguments: Some(serde_json::Map::from_iter([(
                    "query".to_string(),
                    serde_json::Value::from(cli.query.clone()),
                )])),
            };

            let tool_result = service.call_tool(request_param).await?;
            tracing::info!(
                "search response:\n{}",
                serde_json::to_string_pretty(&tool_result)?
            );

            // parse tool result
            let search_result = TidbSearchResponse::from(tool_result);
            tracing::info!(
                "search_result:\n{}",
                serde_json::to_string_pretty(&search_result)?
            );

            service.cancel().await?;
        }
    }

    Ok(())
}

use clap::{Parser, ValueEnum};
use rmcp::{
    model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation},
    service::ServiceExt,
    transport::SseClientTransport,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8007";

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Sse,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Gaia TiDB MCP client")]
struct Args {
    /// Transport type to use (sse)
    #[arg(long, value_enum, default_value = "sse")]
    transport: TransportType,
    /// host
    #[arg(long)]
    tidb_host: String,
    /// port
    #[arg(long)]
    tidb_port: u16,
    /// username
    #[arg(long)]
    tidb_username: String,
    /// password
    #[arg(long)]
    tidb_password: String,
    /// database
    #[arg(long)]
    tidb_database: String,
    /// table name
    #[arg(long)]
    tidb_table_name: String,
    /// limit
    #[arg(long, default_value = "10")]
    tidb_limit: u64,
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
            tracing::info!("Connecting to TiDB MCP server via sse");

            let url = format!("http://{SOCKET_ADDR}/sse");

            let transport = SseClientTransport::start(url).await?;
            let client_info = ClientInfo {
                protocol_version: Default::default(),
                capabilities: ClientCapabilities::default(),
                client_info: Implementation {
                    name: client_name.to_string(),
                    version: client_version.to_string(),
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

            let request_param = CallToolRequestParam {
                name: "search".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "host".to_string(),
                        serde_json::Value::String(cli.tidb_host.clone()),
                    ),
                    ("port".to_string(), serde_json::Value::from(cli.tidb_port)),
                    (
                        "username".to_string(),
                        serde_json::Value::from(cli.tidb_username.clone()),
                    ),
                    (
                        "password".to_string(),
                        serde_json::Value::from(cli.tidb_password.clone()),
                    ),
                    (
                        "database".to_string(),
                        serde_json::Value::from(cli.tidb_database.clone()),
                    ),
                    (
                        "table_name".to_string(),
                        serde_json::Value::from(cli.tidb_table_name.clone()),
                    ),
                    ("limit".to_string(), serde_json::Value::from(cli.tidb_limit)),
                    (
                        "query".to_string(),
                        serde_json::Value::from(cli.query.clone()),
                    ),
                ])),
            };

            let tool_result = mcp_client.peer().call_tool(request_param).await?;
            tracing::info!(
                "search response:\n{}",
                serde_json::to_string_pretty(&tool_result)?
            );

            mcp_client.cancel().await?;
        }
    }

    Ok(())
}

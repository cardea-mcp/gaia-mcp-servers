use clap::{Parser, ValueEnum};
use rmcp::{model::CallToolRequestParam, service::ServiceExt, transport::TokioChildProcess};
use tokio::process::Command;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8002";

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Tcp,
    Stdio,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Gaia Weather MCP client")]
struct Args {
    /// Transport type to use (tcp or stdio)
    #[arg(short, long, value_enum, default_value = "tcp")]
    transport: TransportType,
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

    match cli.transport {
        TransportType::Tcp => {
            tracing::info!("Connecting to MCP server via tcp");

            // connect to mcp server
            let stream = tokio::net::TcpSocket::new_v4()?
                .connect(SOCKET_ADDR.parse()?)
                .await?;

            // create a mcp client
            let mcp_client = ().serve(stream).await?;

            // List available tools
            let tools = mcp_client.peer().list_tools(Default::default()).await?;
            tracing::info!("{}", serde_json::to_string_pretty(&tools)?);

            // request param
            let request_param = CallToolRequestParam {
                name: "get_current_weather".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "location".to_string(),
                        serde_json::Value::String("Beijing".to_string()),
                    ),
                    (
                        "unit".to_string(),
                        serde_json::Value::String("celsius".to_string()),
                    ),
                    (
                        "api_key".to_string(),
                        serde_json::Value::String(
                            std::env::var("OPENWEATHERMAP_API_KEY")
                                .unwrap_or_else(|_| "".to_string()),
                        )
                        .into(),
                    ),
                ])),
            };

            // Call the sum tool
            let sum_result = mcp_client.peer().call_tool(request_param).await?;

            tracing::info!("Sum result: {}", serde_json::to_string_pretty(&sum_result)?);
        }
        TransportType::Stdio => {
            tracing::info!("Connecting to MCP server via stdio");

            let transport = TokioChildProcess::new(&mut Command::new(
                "./target/release/gaia-weather-mcp-server-stdio",
            ))?;

            let mcp_client = ().serve(transport).await?;

            // Initialize
            let server_info = mcp_client.peer_info();
            tracing::info!("Connected to server: {server_info:#?}");

            // List available tools
            let tools = mcp_client.peer().list_tools(Default::default()).await?;
            tracing::info!("{}", serde_json::to_string_pretty(&tools)?);

            // request param
            let request_param = CallToolRequestParam {
                name: "get_current_weather".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "location".to_string(),
                        serde_json::Value::String("Beijing".to_string()),
                    ),
                    (
                        "unit".to_string(),
                        serde_json::Value::String("celsius".to_string()),
                    ),
                    (
                        "api_key".to_string(),
                        serde_json::Value::String(
                            std::env::var("OPENWEATHERMAP_API_KEY")
                                .unwrap_or_else(|_| "".to_string()),
                        )
                        .into(),
                    ),
                ])),
            };

            // Call the sum tool
            let sum_result = mcp_client.peer().call_tool(request_param).await?;

            tracing::info!("Sum result: {}", serde_json::to_string_pretty(&sum_result)?);

            mcp_client.cancel().await?;
        }
    }

    Ok(())
}

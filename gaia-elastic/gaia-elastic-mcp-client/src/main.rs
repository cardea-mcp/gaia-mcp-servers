use clap::{Parser, ValueEnum};
use rmcp::{
    model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation},
    service::ServiceExt,
    transport::TokioChildProcess,
};
use tokio::process::Command;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Stdio,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Gaia Weather MCP client")]
struct Args {
    /// Transport type to use (stdio)
    #[arg(short, long, value_enum, default_value = "stdio")]
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
        TransportType::Stdio => {
            tracing::info!("Connecting to ElasticSearch MCP server via stdio");

            // Start server
            let mcp_client = ()
                .serve(TokioChildProcess::new(
                    Command::new("npx")
                        .arg("-y")
                        .arg("@elastic/mcp-server-elasticsearch@0.1.1"),
                )?)
                .await?;

            tracing::info!("Connected to server");

            // Initialize
            let server_info = mcp_client.peer_info();
            tracing::info!("Connected to server: {server_info:#?}");

            // List available tools
            let tools = mcp_client.peer().list_tools(Default::default()).await?;
            tracing::info!(
                "Available tools:\n{}",
                serde_json::to_string_pretty(&tools)?
            );

            // // request param
            // let request_param = CallToolRequestParam {
            //     name: "get_current_weather".into(),
            //     arguments: Some(serde_json::Map::from_iter([
            //         (
            //             "location".to_string(),
            //             serde_json::Value::String("Beijing".to_string()),
            //         ),
            //         (
            //             "unit".to_string(),
            //             serde_json::Value::String("celsius".to_string()),
            //         ),
            //         (
            //             "api_key".to_string(),
            //             serde_json::Value::String(
            //                 std::env::var("OPENWEATHERMAP_API_KEY")
            //                     .unwrap_or_else(|_| "".to_string()),
            //             )
            //             .into(),
            //         ),
            //     ])),
            // };

            // // Call the sum tool
            // let weather_result = mcp_client.peer().call_tool(request_param).await?;

            // tracing::info!(
            //     "Weather result: {}",
            //     serde_json::to_string_pretty(&weather_result)?
            // );

            mcp_client.cancel().await?;
        }
    }

    Ok(())
}

use clap::{Parser, ValueEnum};
use gaia_elastic_mcp_common::*;
use rmcp::{
    model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation},
    service::ServiceExt,
    transport::{SseTransport, TokioChildProcess},
};
use tokio::process::Command;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8006";

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Stdio,
    Sse,
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
        TransportType::Sse => {
            tracing::info!("Connecting to ElasticSearch MCP server via sse");

            let url = format!("http://{SOCKET_ADDR}/sse");

            let transport = SseTransport::start(url).await?;
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

            let mut api_key = std::env::var("ES_API_KEY").unwrap();
            api_key = format!("ApiKey {}", api_key);
            tracing::info!("api_key: {}", api_key);

            // * list indices
            {
                // request param
                let request_param = CallToolRequestParam {
                    name: "list_indices".into(),
                    arguments: Some(serde_json::Map::from_iter([
                        (
                            "base_url".to_string(),
                            serde_json::Value::String("http://127.0.0.1:9200".to_string()),
                        ),
                        (
                            "api_key".to_string(),
                            serde_json::Value::String(api_key.clone()),
                        ),
                    ])),
                };
                // call tool
                let tool_result = mcp_client.peer().call_tool(request_param).await?;

                // parse tool result
                let indices = ListIndicesResponse::from(tool_result);
                tracing::info!("indices:\n{}", serde_json::to_string_pretty(&indices)?);
            }

            // * get aliases
            {
                // request param
                let request_param = CallToolRequestParam {
                    name: "get_aliases".into(),
                    arguments: Some(serde_json::Map::from_iter([
                        (
                            "base_url".to_string(),
                            serde_json::Value::String("http://127.0.0.1:9200".to_string()),
                        ),
                        (
                            "api_key".to_string(),
                            serde_json::Value::String(api_key.clone()),
                        ),
                    ])),
                };

                // call tool
                let tool_result = mcp_client.peer().call_tool(request_param).await?;

                // parse tool result
                let aliases = GetAliasesResponse::from(tool_result);
                tracing::info!("aliases:\n{}", serde_json::to_string_pretty(&aliases)?);
            }

            mcp_client.cancel().await?;
        }
    }

    Ok(())
}

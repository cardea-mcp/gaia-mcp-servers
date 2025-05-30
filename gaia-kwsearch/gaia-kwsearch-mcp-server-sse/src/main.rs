mod search;

use clap::Parser;
use gaia_kwsearch_mcp_common::ConnectionConfig;
use once_cell::sync::OnceCell;
use rmcp::transport::sse_server::SseServer;
use search::KeywordSearchServer;
use tokio::sync::RwLock;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8005";

static CONNECTION_CONFIG: OnceCell<RwLock<ConnectionConfig>> = OnceCell::new();

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The base URL of the kw-search-server
    #[arg(long, default_value = "http://127.0.0.1:12306")]
    base_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer().with_line_number(true))
        .init();

    let args = Args::parse();

    let connection_config = ConnectionConfig {
        base_url: args.base_url,
        api_key: None,
    };

    CONNECTION_CONFIG
        .set(RwLock::new(connection_config))
        .unwrap();

    tracing::info!("Starting Gaia KeywordSearch MCP server on {}", SOCKET_ADDR);

    let ct = SseServer::serve(SOCKET_ADDR.parse()?)
        .await?
        .with_service(|| KeywordSearchServer);

    tokio::signal::ctrl_c().await?;
    ct.cancel();

    Ok(())
}

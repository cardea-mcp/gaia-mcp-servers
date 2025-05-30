mod search;

use clap::Parser;
use gaia_kwsearch_mcp_common::ConnectionConfig;
use once_cell::sync::OnceCell;
use rmcp::{ServiceExt, transport::stdio};
use search::KeywordSearchServer;
use tokio::sync::RwLock;
use tracing_subscriber::{self, EnvFilter};

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
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_line_number(true)
        .init();

    let args = Args::parse();

    let connection_config = ConnectionConfig {
        base_url: args.base_url,
        api_key: None,
    };

    CONNECTION_CONFIG
        .set(RwLock::new(connection_config))
        .unwrap();

    tracing::info!("Starting Gaia Keyword Search MCP server in stdio mode");

    // Create an instance of our counter router
    let service = KeywordSearchServer.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}

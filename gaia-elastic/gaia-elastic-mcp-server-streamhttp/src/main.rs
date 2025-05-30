mod elastic;

use clap::Parser;
use elastic::ElasticSearchServer;
use gaia_elastic_mcp_common::ConnectionConfig;
use once_cell::sync::OnceCell;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use tokio::sync::RwLock;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8006";

static CONNECTION_CONFIG: OnceCell<RwLock<ConnectionConfig>> = OnceCell::new();

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The base URL of the Elasticsearch server
    #[arg(long, default_value = "http://127.0.0.1:9200")]
    base_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let api_key = match std::env::var("ES_API_KEY") {
        Ok(api_key) => format!("ApiKey {}", api_key),
        Err(_) => {
            let err_message = "The environment variable `ES_API_KEY` is not set";

            tracing::error!("{}", err_message);

            return Err(anyhow::anyhow!(err_message));
        }
    };

    let args = Args::parse();

    let connection_config = ConnectionConfig {
        base_url: args.base_url,
        api_key: Some(api_key),
    };

    CONNECTION_CONFIG
        .set(RwLock::new(connection_config))
        .unwrap();

    tracing::info!("Starting Gaia Elastic MCP server on {}", SOCKET_ADDR);

    let service = StreamableHttpService::new(
        || ElasticSearchServer,
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let router = axum::Router::new().nest_service("/mcp", service);
    let tcp_listener = tokio::net::TcpListener::bind(SOCKET_ADDR).await?;
    let _ = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await;

    Ok(())
}

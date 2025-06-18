mod tidb;

use clap::{Parser, ValueEnum};
use once_cell::sync::OnceCell;
use rmcp::transport::{
    sse_server::SseServer,
    streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
};
use rustls::crypto::{CryptoProvider, ring::default_provider};
use std::path::PathBuf;
use tidb::{TidbAccessConfig, TidbServer, set_search_description};
use tokio::sync::RwLock as TokioRwLock;
use tracing::{error, info};
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

pub static SSL_CA_PATH: OnceCell<PathBuf> = OnceCell::new();
pub static TIDB_ACCESS_CONFIG: OnceCell<TokioRwLock<TidbAccessConfig>> = OnceCell::new();

const DEFAULT_SOCKET_ADDR: &str = "127.0.0.1:8007";

#[derive(Parser, Debug)]
#[command(author, version, about = "Gaia TiDB MCP server")]
struct Args {
    /// Path to the SSL CA certificate. On macOS, this is typically
    /// `/etc/ssl/cert.pem`. On Debian/Ubuntu/Arch Linux, it's typically
    /// `/etc/ssl/certs/ca-certificates.crt`.
    #[arg(long, required = true)]
    ssl_ca: PathBuf,
    /// Socket address to bind to
    #[arg(short, long, default_value = DEFAULT_SOCKET_ADDR)]
    socket_addr: String,
    /// Transport type to use (sse or stream-http)
    #[arg(short, long, value_enum, default_value = "stream-http")]
    transport: TransportType,
    /// Database name
    #[arg(long, required = true)]
    database: String,
    /// Table name
    #[arg(long, required = true)]
    table_name: String,
    /// Maximum number of query results to return
    #[arg(long, default_value = "10")]
    limit: u64,
    /// The description for the search tool
    #[arg(long, default_value = "Perform keyword search in TiDB")]
    search_tool_desc: String,
}

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Sse,
    StreamHttp,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("info,{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer().with_line_number(true))
        .init();

    CryptoProvider::install_default(default_provider()).map_err(|e| {
        let err_msg = format!("Failed to install default crypto provider: {:?}", e);

        error!("{}", err_msg);

        anyhow::anyhow!(err_msg)
    })?;

    let args = Args::parse();

    let config = TidbAccessConfig {
        database: args.database,
        table_name: args.table_name,
        limit: args.limit,
        ssl_ca_path: args.ssl_ca,
    };

    TIDB_ACCESS_CONFIG
        .set(TokioRwLock::new(config))
        .map_err(|_| anyhow::anyhow!("Failed to set TIDB_ACCESS_CONFIG"))?;

    // Set the search tool description from CLI
    set_search_description(args.search_tool_desc);

    info!("Starting Gaia TiDB MCP server on {}", args.socket_addr);

    match args.transport {
        TransportType::StreamHttp => {
            let service = StreamableHttpService::new(
                || TidbServer,
                LocalSessionManager::default().into(),
                Default::default(),
            );

            let router = axum::Router::new().nest_service("/mcp", service);
            let tcp_listener = tokio::net::TcpListener::bind(args.socket_addr).await?;
            let _ = axum::serve(tcp_listener, router)
                .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
                .await;
        }
        TransportType::Sse => {
            let ct = SseServer::serve(args.socket_addr.parse()?)
                .await?
                .with_service(|| TidbServer);

            tokio::signal::ctrl_c().await?;
            ct.cancel();
        }
    }

    Ok(())
}

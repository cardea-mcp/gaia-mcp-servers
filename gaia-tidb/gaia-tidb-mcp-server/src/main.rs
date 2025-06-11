mod tidb;

use clap::{Parser, ValueEnum};
use once_cell::sync::OnceCell;
use rmcp::transport::{
    sse_server::SseServer,
    streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
};
use rustls::crypto::{CryptoProvider, ring::default_provider};
use std::path::PathBuf;
use tidb::TidbServer;
use tracing::{error, info};
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

pub static SSL_CA_PATH: OnceCell<PathBuf> = OnceCell::new();

const DEFAULT_SOCKET_ADDR: &str = "127.0.0.1:8007";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the SSL CA certificate. On macOS, this is typically
    /// `/etc/ssl/cert.pem`. On Debian/Ubuntu/Arch Linux, it's typically
    /// `/etc/ssl/certs/ca-certificates.crt`.
    #[arg(long)]
    ssl_ca: PathBuf,
    /// Socket address to bind to
    #[arg(short, long, default_value = DEFAULT_SOCKET_ADDR)]
    socket_addr: String,
    /// Transport type to use (sse or stream-http)
    #[arg(short, long, value_enum, default_value = "stream-http")]
    transport: TransportType,
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
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    CryptoProvider::install_default(default_provider()).map_err(|e| {
        let err_msg = format!("Failed to install default crypto provider: {:?}", e);

        error!("{}", err_msg);

        anyhow::anyhow!(err_msg)
    })?;

    let args = Args::parse();

    info!("Starting Gaia TiDB MCP server on {}", args.socket_addr);

    SSL_CA_PATH
        .set(args.ssl_ca)
        .map_err(|_| anyhow::anyhow!("Failed to set SSL CA path"))?;

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

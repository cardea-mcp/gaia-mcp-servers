mod tidb;

use clap::Parser;
use once_cell::sync::OnceCell;
use rmcp::transport::sse_server::SseServer;
use rustls::crypto::CryptoProvider;
use rustls::crypto::ring::default_provider;
use std::path::PathBuf;
use tidb::TidbServer;
use tracing::{error, info};
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

pub static SSL_CA_PATH: OnceCell<PathBuf> = OnceCell::new();

const SOCKET_ADDR: &str = "127.0.0.1:8007";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the SSL CA certificate. On macOS, this is typically
    /// `/etc/ssl/cert.pem`. On Debian/Ubuntu/Arch Linux, it's typically
    /// `/etc/ssl/certs/ca-certificates.crt`.
    #[arg(long)]
    ssl_ca: PathBuf,
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

    CryptoProvider::install_default(default_provider()).map_err(|e| {
        let err_msg = format!("Failed to install default crypto provider: {:?}", e);

        error!("{}", err_msg);

        anyhow::anyhow!(err_msg)
    })?;

    let args = Args::parse();

    info!("Starting Gaia TiDB MCP server on {}", SOCKET_ADDR);

    SSL_CA_PATH
        .set(args.ssl_ca)
        .map_err(|_| anyhow::anyhow!("Failed to set SSL CA path"))?;

    let ct = SseServer::serve(SOCKET_ADDR.parse()?)
        .await?
        .with_service(|| TidbServer);

    tokio::signal::ctrl_c().await?;
    ct.cancel();

    Ok(())
}

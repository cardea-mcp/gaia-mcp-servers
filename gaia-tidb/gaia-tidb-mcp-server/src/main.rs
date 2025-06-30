mod tidb;

use anyhow::{anyhow, bail};
use clap::{Parser, ValueEnum};
use mysql::*;
use once_cell::sync::OnceCell;
use rmcp::transport::{
    sse_server::SseServer,
    streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
};
use rustls::crypto::{CryptoProvider, ring::default_provider};
use std::{env, path::PathBuf};
use tidb::{TidbServer, set_search_tool_prompt};
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
    /// The prompt for the `search` mcp tool
    #[arg(
        long,
        default_value = "Please extract 3 to 5 keywords from my question, separated by spaces. Then, try to return a tool call that invokes the keyword search tool.\n\nMy question is: {query}"
    )]
    search_tool_prompt: String,
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
        let err_msg = format!("Failed to install default crypto provider: {e:?}");
        error!("{}", err_msg);
        anyhow!(err_msg)
    })?;

    let args = Args::parse();

    // parse host
    let host = match env::var("TIDB_HOST") {
        Ok(host) => host,
        Err(e) => {
            let error_message = format!("Failed to get TIDB_HOST: {e}");
            error!(error_message);
            bail!(error_message);
        }
    };

    // parse port
    let port: u16 = match env::var("TIDB_PORT") {
        Ok(port) => match port.parse() {
            Ok(port) => port,
            Err(e) => {
                let error_message = format!("Failed to parse TIDB_PORT: {e}");
                error!(error_message);
                bail!(error_message);
            }
        },
        Err(e) => {
            let error_message = format!("Failed to get TIDB_PORT: {e}");
            error!(error_message);
            bail!(error_message);
        }
    };

    // parse username
    let username = match env::var("TIDB_USERNAME") {
        Ok(username) => username,
        Err(e) => {
            let error_message = format!("Failed to get TIDB_USERNAME: {e}");
            error!(error_message);
            bail!(error_message);
        }
    };

    // parse password
    let password = match env::var("TIDB_PASSWORD") {
        Ok(password) => password,
        Err(e) => {
            let error_message = format!("Failed to get TIDB_PASSWORD: {e}");
            error!(error_message);
            bail!(error_message);
        }
    };

    // create connection options
    info!("Creating connection options for TiDB Cloud...");
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(host))
        .tcp_port(port)
        .user(Some(username))
        .pass(Some(password))
        .db_name(Some(args.database.clone()))
        .ssl_opts(Some(
            SslOpts::default().with_root_cert_path(Some(args.ssl_ca)),
        ));

    // create connection pool
    info!("Creating connection pool...");
    let pool = Pool::new(opts).map_err(|e| {
        let error_message = format!("Failed to create connection pool: {e}");
        error!(error_message);
        anyhow!(error_message)
    })?;

    let config = TidbAccessConfig {
        pool,
        database: args.database,
        table_name: args.table_name,
        limit: args.limit,
    };

    TIDB_ACCESS_CONFIG
        .set(TokioRwLock::new(config))
        .map_err(|_| anyhow::anyhow!("Failed to set TIDB_ACCESS_CONFIG"))?;

    // Set the search tool prompt from CLI
    set_search_tool_prompt(args.search_tool_prompt);

    info!("Starting Gaia TiDB MCP server on {}", args.socket_addr);

    match args.transport {
        TransportType::StreamHttp => {
            let service = StreamableHttpService::new(
                || Ok(TidbServer::new()),
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
                .with_service(TidbServer::new);

            tokio::signal::ctrl_c().await?;
            ct.cancel();
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct TidbAccessConfig {
    pub pool: Pool,
    pub database: String,
    pub table_name: String,
    pub limit: u64,
}

mod search;

use clap::{Parser, ValueEnum};
use gaia_kwsearch_mcp_common::ConnectionConfig;
use once_cell::sync::OnceCell;
use rmcp::{
    ServiceExt,
    transport::{
        sse_server::SseServer,
        stdio,
        streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
    },
};
use search::KeywordSearchServer;
use tokio::sync::RwLock;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_SOCKET_ADDR: &str = "127.0.0.1:8005";

static CONNECTION_CONFIG: OnceCell<RwLock<ConnectionConfig>> = OnceCell::new();

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The base URL of the kw-search-server
    #[arg(long, default_value = "http://127.0.0.1:12306")]
    base_url: String,
    /// Socket address to bind to
    #[arg(short, long, default_value = DEFAULT_SOCKET_ADDR)]
    socket_addr: String,
    /// Transport type to use
    #[arg(short, long, value_enum, default_value = "stream-http")]
    transport: TransportType,
}

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Stdio,
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

    tracing::info!(
        "Starting Gaia KeywordSearch MCP server on {}",
        args.socket_addr
    );

    match args.transport {
        TransportType::StreamHttp => {
            let service = StreamableHttpService::new(
                || KeywordSearchServer,
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
                .with_service(|| KeywordSearchServer);

            tokio::signal::ctrl_c().await?;
            ct.cancel();
        }
        TransportType::Stdio => {
            // Create an instance of our counter router
            let service = KeywordSearchServer.serve(stdio()).await.inspect_err(|e| {
                tracing::error!("serving error: {:?}", e);
            })?;

            service.waiting().await?;
        }
    }

    Ok(())
}

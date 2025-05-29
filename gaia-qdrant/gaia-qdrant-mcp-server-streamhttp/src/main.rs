mod qdrant;

use qdrant::QdrantServer;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8003";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Gaia Qdrant MCP server on {}", SOCKET_ADDR);

    let service = StreamableHttpService::new(
        || QdrantServer,
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

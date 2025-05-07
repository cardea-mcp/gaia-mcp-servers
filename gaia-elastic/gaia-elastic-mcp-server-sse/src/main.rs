mod elastic;

use elastic::ElasticSearchServer;
use rmcp::transport::sse_server::SseServer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8006";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer().with_line_number(true))
        .init();

    tracing::info!("Starting Gaia Elastic MCP server on {}", SOCKET_ADDR);

    let ct = SseServer::serve(SOCKET_ADDR.parse()?)
        .await?
        .with_service(|| ElasticSearchServer);

    tokio::signal::ctrl_c().await?;
    ct.cancel();

    Ok(())
}

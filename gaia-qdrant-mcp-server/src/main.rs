mod qdrant;

use qdrant::*;
use rmcp::serve_server;

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

    let tcp_listener = tokio::net::TcpListener::bind(SOCKET_ADDR).await?;
    println!("Gaia Qdrant MCP Server is listening on {}", SOCKET_ADDR);

    while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
        // spawn a new task to handle the connection
        tokio::spawn(async move {
            // create a mcp server
            let mcp_server = serve_server(QdrantServer, stream).await?;

            // wait for the connection to be closed
            mcp_server.waiting().await?;

            anyhow::Ok(())
        });
    }
    Ok(())
}

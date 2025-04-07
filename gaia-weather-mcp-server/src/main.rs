mod weather;

use rmcp::serve_server;
use weather::*;

const SOCKET_ADDR: &str = "127.0.0.1:8002";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tcp_listener = tokio::net::TcpListener::bind(SOCKET_ADDR).await?;
    println!("Gaia Weather MCP Server is listening on {}", SOCKET_ADDR);

    while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
        // spawn a new task to handle the connection
        tokio::spawn(async move {
            // create a mcp server
            let mcp_server = serve_server(WeatherServer, stream).await?;

            // wait for the connection to be closed
            mcp_server.waiting().await?;

            anyhow::Ok(())
        });
    }
    Ok(())
}

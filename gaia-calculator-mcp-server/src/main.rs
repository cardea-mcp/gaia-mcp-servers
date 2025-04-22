mod calculator;

use calculator::Calculator;
use rmcp::ServiceExt;

const SOCKET_ADDR: &str = "127.0.0.1:8001";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tcp_listener = tokio::net::TcpListener::bind(SOCKET_ADDR).await?;
    println!("Calculator MCP server is listening on {}", SOCKET_ADDR);

    while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
        // spawn a new task to handle the connection
        tokio::spawn(async move {
            // create a mcp server
            let mcp_server = Calculator.serve(stream).await?;

            // wait for the connection to be closed
            mcp_server.waiting().await?;

            anyhow::Ok(())
        });
    }
    Ok(())
}

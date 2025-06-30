mod qdrant;

use clap::{Parser, ValueEnum};
use once_cell::sync::OnceCell;
use qdrant::{QdrantServer, set_search_tool_prompt};
use rmcp::{
    ServiceExt,
    transport::{
        sse_server::SseServer,
        stdio,
        streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
    },
};
use tokio::sync::RwLock;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_SOCKET_ADDR: &str = "127.0.0.1:8003";

static CONNECTION_CONFIG: OnceCell<RwLock<ConnectionConfig>> = OnceCell::new();

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The base URL of the Qdrant database
    #[arg(long, default_value = "http://127.0.0.1:6333")]
    base_url: String,
    /// The API key to use for the Qdrant database
    #[arg(long)]
    api_key: Option<String>,
    /// Socket address to bind to
    #[arg(short, long, default_value = DEFAULT_SOCKET_ADDR)]
    socket_addr: String,
    /// Transport type to use
    #[arg(short, long, value_enum, default_value = "stream-http")]
    transport: TransportType,
    /// Name of the collection to search
    #[arg(long, required = true)]
    collection: String,
    /// Maximum number of results to return
    #[arg(long, default_value = "10")]
    limit: u64,
    /// Score threshold for the results
    #[arg(long, default_value = "0.5")]
    score_threshold: f32,
    /// The prompt for the `search` mcp tool
    #[arg(
        long,
        default_value = "Perform vector search with the input vector. Return a tool call that invokes the vector search tool.\n\nThe input vector is: [0.0,0.0,0.0,0.0]"
    )]
    search_tool_prompt: String,
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
        api_key: args.api_key,
        collection: args.collection,
        limit: args.limit,
        score_threshold: args.score_threshold,
    };

    CONNECTION_CONFIG
        .set(RwLock::new(connection_config))
        .unwrap();

    // Set the search tool prompt from CLI
    set_search_tool_prompt(args.search_tool_prompt);

    tracing::info!("Starting Gaia Qdrant MCP server on {}", args.socket_addr);

    match args.transport {
        TransportType::StreamHttp => {
            let service = StreamableHttpService::new(
                || Ok(QdrantServer::new()),
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
                .with_service(|| QdrantServer::new());

            tokio::signal::ctrl_c().await?;
            ct.cancel();
        }
        TransportType::Stdio => {
            // Create an instance of our counter router
            let service = QdrantServer::new().serve(stdio()).await.inspect_err(|e| {
                tracing::error!("serving error: {:?}", e);
            })?;

            service.waiting().await?;
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub collection: String,
    pub limit: u64,
    pub score_threshold: f32,
}

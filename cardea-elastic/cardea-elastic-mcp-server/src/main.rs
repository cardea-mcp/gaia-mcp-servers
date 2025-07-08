mod elastic;

use clap::{Parser, ValueEnum};
use elastic::ElasticSearchServer;
use once_cell::sync::OnceCell;
use rmcp::transport::{
    sse_server::SseServer,
    streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
};
use tokio::sync::RwLock;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_SOCKET_ADDR: &str = "127.0.0.1:8006";

static CONNECTION_CONFIG: OnceCell<RwLock<ConnectionConfig>> = OnceCell::new();

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The base URL of the Elasticsearch server
    #[arg(long, default_value = "http://127.0.0.1:9200")]
    base_url: String,
    /// Socket address to bind to
    #[arg(short, long, default_value = DEFAULT_SOCKET_ADDR)]
    socket_addr: String,
    /// Transport type to use
    #[arg(short, long, value_enum, default_value = "stream-http")]
    transport: TransportType,
    /// Index to search
    #[arg(long)]
    index: String,
    /// Name of fields to search
    #[arg(long, value_delimiter = ',', default_value = "title,content")]
    fields: Vec<String>,
    /// Maximum number of query results to return
    #[arg(long, default_value = "10")]
    size: u64,
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
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();

    let api_key = match std::env::var("ES_API_KEY") {
        Ok(api_key) => format!("ApiKey {api_key}"),
        Err(_) => {
            let err_message = "The environment variable `ES_API_KEY` is not set";

            tracing::error!("{}", err_message);

            return Err(anyhow::anyhow!(err_message));
        }
    };

    let args = Args::parse();

    let connection_config = ConnectionConfig {
        base_url: args.base_url,
        api_key: Some(api_key),
        index: args.index,
        fields: args.fields,
        size: args.size,
    };

    CONNECTION_CONFIG
        .set(RwLock::new(connection_config))
        .unwrap();

    tracing::info!("Starting Cardea Elastic MCP server on {}", args.socket_addr);

    match args.transport {
        TransportType::StreamHttp => {
            let service = StreamableHttpService::new(
                || Ok(ElasticSearchServer::new()),
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
                .with_service(ElasticSearchServer::new);

            tokio::signal::ctrl_c().await?;
            ct.cancel();
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    /// index name
    pub index: String,
    /// name of fields to search
    pub fields: Vec<String>,
    /// number of results to return
    pub size: u64,
}

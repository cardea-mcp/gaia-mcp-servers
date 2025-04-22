use clap::{Parser, ValueEnum};
use gaia_qdrant_common::{self as qdrant, Point};
use rmcp::{model::CallToolRequestParam, service::ServiceExt, transport::TokioChildProcess};
use serde_json::json;
use tokio::process::Command;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8003";
const QDRANT_BASE_URL: &str = "http://127.0.0.1:6333";
const QDRANT_COLLECTION_NAME: &str = "mcp-test";

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Tcp,
    Stdio,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Gaia Qdrant MCP client")]
struct Args {
    /// Transport type to use (tcp or stdio)
    #[arg(short, long, value_enum, default_value = "tcp")]
    transport: TransportType,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("info,{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer().with_line_number(true))
        .init();

    let cli = Args::parse();

    // create a mcp client
    match cli.transport {
        TransportType::Tcp => {
            tracing::info!("Connecting to MCP server via tcp");

            // connect to mcp server
            let stream = tokio::net::TcpSocket::new_v4()?
                .connect(SOCKET_ADDR.parse()?)
                .await?;

            // create a mcp client
            let mcp_client = ().serve(stream).await?;

            // List available tools
            let tools = mcp_client.peer().list_tools(Default::default()).await?;
            tracing::info!("{}", serde_json::to_string_pretty(&tools)?);

            // * list collections
            let list_collections = CallToolRequestParam {
                name: "list_collections".into(),
                arguments: Some(serde_json::Map::from_iter([(
                    "base_url".to_string(),
                    serde_json::Value::String(QDRANT_BASE_URL.into()),
                )])),
            };
            let res = mcp_client.peer().call_tool(list_collections).await?;
            tracing::info!("collections:\n{}", serde_json::to_string_pretty(&res)?);

            // * check if collection exists
            let collection_exists = CallToolRequestParam {
                name: "collection_exists".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "base_url".to_string(),
                        serde_json::Value::String(QDRANT_BASE_URL.into()),
                    ),
                    (
                        "name".to_string(),
                        serde_json::Value::String(QDRANT_COLLECTION_NAME.into()),
                    ),
                ])),
            };
            let res = mcp_client.peer().call_tool(collection_exists).await?;
            tracing::info!(
                "collection exists:\n{}",
                serde_json::to_string_pretty(&res)?
            );

            if !res.is_error.unwrap() {
                let content = res.content[0].as_text().unwrap().text.as_ref();
                let response =
                    serde_json::from_str::<qdrant::CollectionExistsResponse>(content).unwrap();
                let exists = response.result;
                tracing::info!("Exists? {}", exists);

                if exists {
                    // * delete collection
                    let delete_collection = CallToolRequestParam {
                        name: "delete_collection".into(),
                        arguments: Some(serde_json::Map::from_iter([
                            (
                                "base_url".to_string(),
                                serde_json::Value::String(QDRANT_BASE_URL.into()),
                            ),
                            (
                                "name".to_string(),
                                serde_json::Value::String(QDRANT_COLLECTION_NAME.into()),
                            ),
                        ])),
                    };
                    let res = mcp_client.peer().call_tool(delete_collection).await?;
                    tracing::info!(
                        "delete collection:\n{}",
                        serde_json::to_string_pretty(&res)?
                    );
                }
            }

            // * create collection
            let create_collection = CallToolRequestParam {
                name: "create_collection".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "base_url".to_string(),
                        serde_json::Value::String(QDRANT_BASE_URL.into()),
                    ),
                    (
                        "name".to_string(),
                        serde_json::Value::String(QDRANT_COLLECTION_NAME.into()),
                    ),
                    ("size".to_string(), serde_json::Value::from(4)),
                ])),
            };
            let res = mcp_client.peer().call_tool(create_collection).await?;
            tracing::info!(
                "create collection:\n{}",
                serde_json::to_string_pretty(&res)?
            );

            // * upsert points
            let mut points = Vec::<Point>::new();
            points.push(Point {
                id: 1,
                vector: vec![0.05, 0.61, 0.76, 0.74],
                payload: json!({"city": "Berlin"}).as_object().unwrap().to_owned(),
            });
            points.push(Point {
                id: 2,
                vector: vec![0.19, 0.81, 0.75, 0.11],
                payload: json!({"city": "London"}).as_object().unwrap().to_owned(),
            });
            points.push(Point {
                id: 3,
                vector: vec![0.36, 0.55, 0.47, 0.94],
                payload: json!({"city": "Moscow"}).as_object().unwrap().to_owned(),
            });
            points.push(Point {
                id: 4,
                vector: vec![0.18, 0.01, 0.85, 0.80],
                payload: json!({"city": "New York"}).as_object().unwrap().to_owned(),
            });
            points.push(Point {
                id: 5,
                vector: vec![0.24, 0.18, 0.22, 0.44],
                payload: json!({"city": "Beijing"}).as_object().unwrap().to_owned(),
            });
            points.push(Point {
                id: 6,
                vector: vec![0.35, 0.08, 0.11, 0.44],
                payload: json!({"city": "Mumbai"}).as_object().unwrap().to_owned(),
            });

            let upsert_points = CallToolRequestParam {
                name: "upsert_points".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "base_url".to_string(),
                        serde_json::Value::String(QDRANT_BASE_URL.into()),
                    ),
                    (
                        "name".to_string(),
                        serde_json::Value::String(QDRANT_COLLECTION_NAME.into()),
                    ),
                    (
                        "points".to_string(),
                        serde_json::Value::Array(
                            points
                                .into_iter()
                                .map(|p| serde_json::to_value(p).unwrap())
                                .collect(),
                        ),
                    ),
                ])),
            };
            let tool_result = mcp_client.peer().call_tool(upsert_points).await?;
            let response = qdrant::UpsertPointsResponse::from(tool_result);
            tracing::info!("upsert points response:\n{:?}", &response);

            // * search points
            let search_points = CallToolRequestParam {
                name: "search_points".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "base_url".to_string(),
                        serde_json::Value::String(QDRANT_BASE_URL.into()),
                    ),
                    (
                        "name".to_string(),
                        serde_json::Value::String(QDRANT_COLLECTION_NAME.into()),
                    ),
                    (
                        "vector".to_string(),
                        serde_json::Value::Array(
                            vec![0.2, 0.1, 0.9, 0.7]
                                .into_iter()
                                .map(|v| serde_json::Value::from(v))
                                .collect(),
                        ),
                    ),
                    ("limit".to_string(), serde_json::Value::from(2)),
                ])),
            };
            let tool_result = mcp_client.peer().call_tool(search_points).await?;
            let response = qdrant::SearchPointsResponse::from(tool_result);
            // let results = response.result;
            tracing::info!("search points response:\n{:?}", &response);
        }
        TransportType::Stdio => {
            tracing::info!("Connecting to MCP server via stdio");

            let transport = TokioChildProcess::new(&mut Command::new(
                "./target/release/gaia-qdrant-mcp-server-stdio",
            ))?;

            let mcp_client = ().serve(transport).await?;
            tracing::info!("Connected to server");

            // Initialize
            let server_info = mcp_client.peer_info();
            tracing::info!("Connected to server: {server_info:#?}");

            // List available tools
            let tools = mcp_client.peer().list_tools(Default::default()).await?;
            tracing::info!("{}", serde_json::to_string_pretty(&tools)?);

            // * list collections
            let list_collections = CallToolRequestParam {
                name: "list_collections".into(),
                arguments: Some(serde_json::Map::from_iter([(
                    "base_url".to_string(),
                    serde_json::Value::String(QDRANT_BASE_URL.into()),
                )])),
            };
            let res = mcp_client.peer().call_tool(list_collections).await?;
            tracing::info!("collections:\n{}", serde_json::to_string_pretty(&res)?);

            // * check if collection exists
            let collection_exists = CallToolRequestParam {
                name: "collection_exists".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "base_url".to_string(),
                        serde_json::Value::String(QDRANT_BASE_URL.into()),
                    ),
                    (
                        "name".to_string(),
                        serde_json::Value::String(QDRANT_COLLECTION_NAME.into()),
                    ),
                ])),
            };
            let res = mcp_client.peer().call_tool(collection_exists).await?;
            tracing::info!(
                "collection exists:\n{}",
                serde_json::to_string_pretty(&res)?
            );

            if !res.is_error.unwrap() {
                let content = res.content[0].as_text().unwrap().text.as_ref();
                let response =
                    serde_json::from_str::<qdrant::CollectionExistsResponse>(content).unwrap();
                let exists = response.result;
                tracing::info!("Exists? {}", exists);

                if exists {
                    // * delete collection
                    let delete_collection = CallToolRequestParam {
                        name: "delete_collection".into(),
                        arguments: Some(serde_json::Map::from_iter([
                            (
                                "base_url".to_string(),
                                serde_json::Value::String(QDRANT_BASE_URL.into()),
                            ),
                            (
                                "name".to_string(),
                                serde_json::Value::String(QDRANT_COLLECTION_NAME.into()),
                            ),
                        ])),
                    };
                    let res = mcp_client.peer().call_tool(delete_collection).await?;
                    tracing::info!(
                        "delete collection:\n{}",
                        serde_json::to_string_pretty(&res)?
                    );
                }
            }

            // * create collection
            let create_collection = CallToolRequestParam {
                name: "create_collection".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "base_url".to_string(),
                        serde_json::Value::String(QDRANT_BASE_URL.into()),
                    ),
                    (
                        "name".to_string(),
                        serde_json::Value::String(QDRANT_COLLECTION_NAME.into()),
                    ),
                    ("size".to_string(), serde_json::Value::from(4)),
                ])),
            };
            let res = mcp_client.peer().call_tool(create_collection).await?;
            tracing::info!(
                "create collection:\n{}",
                serde_json::to_string_pretty(&res)?
            );

            // * upsert points
            let mut points = Vec::<Point>::new();
            points.push(Point {
                id: 1,
                vector: vec![0.05, 0.61, 0.76, 0.74],
                payload: json!({"city": "Berlin"}).as_object().unwrap().to_owned(),
            });
            points.push(Point {
                id: 2,
                vector: vec![0.19, 0.81, 0.75, 0.11],
                payload: json!({"city": "London"}).as_object().unwrap().to_owned(),
            });
            points.push(Point {
                id: 3,
                vector: vec![0.36, 0.55, 0.47, 0.94],
                payload: json!({"city": "Moscow"}).as_object().unwrap().to_owned(),
            });
            points.push(Point {
                id: 4,
                vector: vec![0.18, 0.01, 0.85, 0.80],
                payload: json!({"city": "New York"}).as_object().unwrap().to_owned(),
            });
            points.push(Point {
                id: 5,
                vector: vec![0.24, 0.18, 0.22, 0.44],
                payload: json!({"city": "Beijing"}).as_object().unwrap().to_owned(),
            });
            points.push(Point {
                id: 6,
                vector: vec![0.35, 0.08, 0.11, 0.44],
                payload: json!({"city": "Mumbai"}).as_object().unwrap().to_owned(),
            });

            let upsert_points = CallToolRequestParam {
                name: "upsert_points".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "base_url".to_string(),
                        serde_json::Value::String(QDRANT_BASE_URL.into()),
                    ),
                    (
                        "name".to_string(),
                        serde_json::Value::String(QDRANT_COLLECTION_NAME.into()),
                    ),
                    (
                        "points".to_string(),
                        serde_json::Value::Array(
                            points
                                .into_iter()
                                .map(|p| serde_json::to_value(p).unwrap())
                                .collect(),
                        ),
                    ),
                ])),
            };
            let tool_result = mcp_client.peer().call_tool(upsert_points).await?;
            let response = qdrant::UpsertPointsResponse::from(tool_result);
            tracing::info!("upsert points response:\n{:?}", &response);

            // * search points
            let search_points = CallToolRequestParam {
                name: "search_points".into(),
                arguments: Some(serde_json::Map::from_iter([
                    (
                        "base_url".to_string(),
                        serde_json::Value::String(QDRANT_BASE_URL.into()),
                    ),
                    (
                        "name".to_string(),
                        serde_json::Value::String(QDRANT_COLLECTION_NAME.into()),
                    ),
                    (
                        "vector".to_string(),
                        serde_json::Value::Array(
                            vec![0.2, 0.1, 0.9, 0.7]
                                .into_iter()
                                .map(|v| serde_json::Value::from(v))
                                .collect(),
                        ),
                    ),
                    ("limit".to_string(), serde_json::Value::from(2)),
                ])),
            };
            let tool_result = mcp_client.peer().call_tool(search_points).await?;
            let response = qdrant::SearchPointsResponse::from(tool_result);
            // let results = response.result;
            tracing::info!("search points response:\n{:?}", &response);

            mcp_client.cancel().await?;
        }
    };

    Ok(())
}

use clap::Parser;
use gaia_qdrant_common::{self as qdrant, Point};
use rmcp::model::CallToolRequestParam;
use rmcp::serve_client;
use serde_json::json;
use std::net::SocketAddr;

// const MCP_SERVER_ADDR: &str = "127.0.0.1:8003";

#[derive(Debug, Parser)]
#[command(version = env!("CARGO_PKG_VERSION"), about = "Gaia MCP Client")]
struct Cli {
    /// Host address of the target MCP Server
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    /// Port of the target MCP Server
    #[arg(long, default_value = "8001")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // parse the command line arguments
    let cli = Cli::parse();

    // mcp server address
    let mcp_server_addr: SocketAddr = format!("{}:{}", cli.host, cli.port).parse()?;

    // connect to mcp server
    let stream = tokio::net::TcpSocket::new_v4()?
        .connect(mcp_server_addr)
        .await?;
    // create a mcp client
    let mcp_client = serve_client((), stream).await?;

    // list tools
    let tools = mcp_client.peer().list_tools(Default::default()).await?;
    println!("{}", serde_json::to_string_pretty(&tools)?);

    // print name of all tools
    for (idx, tool) in tools.tools.iter().enumerate() {
        println!("{}: {}", idx, tool.name);
        for (k, v) in tool.input_schema.iter() {
            if k == "properties" {
                let map =
                    serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(v.clone())
                        .unwrap();
                for (k, v) in map.iter() {
                    if v.is_string() {
                        println!("{}: {}", k, v.as_str().unwrap());
                    } else if v.is_object() {
                        let map = serde_json::from_value::<
                            serde_json::Map<String, serde_json::Value>,
                        >(v.clone())
                        .unwrap();
                        for (k, v) in map.iter() {
                            if v.is_string() {
                                println!("{}: {}", k, v.as_str().unwrap());
                            }
                        }
                    }
                }
            } else if k == "required" {
                serde_json::from_value::<Vec<String>>(v.clone())
                    .unwrap()
                    .iter()
                    .for_each(|v| {
                        println!("{:?}", v);
                    });
            } else if k == "definitions" {
                println!("{}: {}", k, v);
            }
        }
    }

    // calculator
    {
        // // call a tool
        // let tool_sum = CallToolRequestParam {
        //     name: "sum".into(),
        //     arguments: Some(serde_json::Map::from_iter([
        //         ("a".to_string(), serde_json::Value::Number(1.into())),
        //         ("b".to_string(), serde_json::Value::Number(2.into())),
        //     ])),
        // };
        // let res = mcp_client.peer().call_tool(tool_sum).await?;
        // println!("{}", serde_json::to_string_pretty(&res)?);
    }

    // qdrant
    {
        // * list collections
        let list_collections = CallToolRequestParam {
            name: "list_collections".into(),
            arguments: Some(serde_json::Map::from_iter([(
                "base_url".to_string(),
                serde_json::Value::String("http://127.0.0.1:6333".into()),
            )])),
        };
        let res = mcp_client.peer().call_tool(list_collections).await?;
        println!("collections:\n{}", serde_json::to_string_pretty(&res)?);

        // * check if collection exists
        let collection_exists = CallToolRequestParam {
            name: "collection_exists".into(),
            arguments: Some(serde_json::Map::from_iter([
                (
                    "base_url".to_string(),
                    serde_json::Value::String("http://127.0.0.1:6333".into()),
                ),
                (
                    "name".to_string(),
                    serde_json::Value::String("mcp-test".into()),
                ),
            ])),
        };
        let res = mcp_client.peer().call_tool(collection_exists).await?;
        println!(
            "collection exists:\n{}",
            serde_json::to_string_pretty(&res)?
        );

        if !res.is_error.unwrap() {
            let content = res.content[0].as_text().unwrap().text.as_ref();
            let response =
                serde_json::from_str::<qdrant::CollectionExistsResponse>(content).unwrap();
            let exists = response.result;
            println!("Exists? {}", exists);

            if exists {
                // * delete collection
                let delete_collection = CallToolRequestParam {
                    name: "delete_collection".into(),
                    arguments: Some(serde_json::Map::from_iter([
                        (
                            "base_url".to_string(),
                            serde_json::Value::String("http://127.0.0.1:6333".into()),
                        ),
                        (
                            "name".to_string(),
                            serde_json::Value::String("mcp-test".into()),
                        ),
                    ])),
                };
                let res = mcp_client.peer().call_tool(delete_collection).await?;
                println!(
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
                    serde_json::Value::String("http://127.0.0.1:6333".into()),
                ),
                (
                    "name".to_string(),
                    serde_json::Value::String("mcp-test".into()),
                ),
                ("size".to_string(), serde_json::Value::from(4)),
            ])),
        };
        let res = mcp_client.peer().call_tool(create_collection).await?;
        println!(
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
                    serde_json::Value::String("http://127.0.0.1:6333".into()),
                ),
                (
                    "name".to_string(),
                    serde_json::Value::String("mcp-test".into()),
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
        println!("upsert points response:\n{:?}", &response);

        // * search points
        let search_points = CallToolRequestParam {
            name: "search_points".into(),
            arguments: Some(serde_json::Map::from_iter([
                (
                    "base_url".to_string(),
                    serde_json::Value::String("http://127.0.0.1:6333".into()),
                ),
                (
                    "name".to_string(),
                    serde_json::Value::String("mcp-test".into()),
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
        println!("search points response:\n{:?}", &response);
    }

    // print server info
    let info = mcp_client.peer_info();
    println!("server info:\n{}", serde_json::to_string_pretty(&info)?);

    Ok(())
}

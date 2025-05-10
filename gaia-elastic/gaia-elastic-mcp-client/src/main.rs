use clap::{Parser, ValueEnum};
use gaia_elastic_mcp_common::*;
use rmcp::{
    model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation},
    service::ServiceExt,
    transport::{SseTransport, TokioChildProcess},
};
use serde_json::json;
use tokio::process::Command;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SOCKET_ADDR: &str = "127.0.0.1:8006";

#[derive(Debug, Clone, ValueEnum)]
enum TransportType {
    Stdio,
    Sse,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Gaia Weather MCP client")]
struct Args {
    /// Transport type to use (sse)
    #[arg(short, long, value_enum, default_value = "sse")]
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

    match cli.transport {
        TransportType::Stdio => {
            tracing::info!("Connecting to ElasticSearch MCP server via stdio");

            // Start server
            let mcp_client = ()
                .serve(TokioChildProcess::new(
                    Command::new("npx")
                        .arg("-y")
                        .arg("@elastic/mcp-server-elasticsearch@0.1.1"),
                )?)
                .await?;

            tracing::info!("Connected to server");

            // Initialize
            let server_info = mcp_client.peer_info();
            tracing::info!("Connected to server: {server_info:#?}");

            // List available tools
            let tools = mcp_client.peer().list_tools(Default::default()).await?;
            tracing::info!(
                "Available tools:\n{}",
                serde_json::to_string_pretty(&tools)?
            );

            // // request param
            // let request_param = CallToolRequestParam {
            //     name: "get_current_weather".into(),
            //     arguments: Some(serde_json::Map::from_iter([
            //         (
            //             "location".to_string(),
            //             serde_json::Value::String("Beijing".to_string()),
            //         ),
            //         (
            //             "unit".to_string(),
            //             serde_json::Value::String("celsius".to_string()),
            //         ),
            //         (
            //             "api_key".to_string(),
            //             serde_json::Value::String(
            //                 std::env::var("OPENWEATHERMAP_API_KEY")
            //                     .unwrap_or_else(|_| "".to_string()),
            //             )
            //             .into(),
            //         ),
            //     ])),
            // };

            // // Call the sum tool
            // let weather_result = mcp_client.peer().call_tool(request_param).await?;

            // tracing::info!(
            //     "Weather result: {}",
            //     serde_json::to_string_pretty(&weather_result)?
            // );

            mcp_client.cancel().await?;
        }
        TransportType::Sse => {
            tracing::info!("Connecting to ElasticSearch MCP server via sse");

            let url = format!("http://{SOCKET_ADDR}/sse");

            let transport = SseTransport::start(url).await?;
            let client_info = ClientInfo {
                protocol_version: Default::default(),
                capabilities: ClientCapabilities::default(),
                client_info: Implementation {
                    name: "test sse client".to_string(),
                    version: "0.1.0".to_string(),
                },
            };
            let mcp_client = client_info.serve(transport).await.inspect_err(|e| {
                tracing::error!("client error: {:?}", e);
            })?;

            // Initialize
            let server_info = mcp_client.peer_info();
            tracing::info!("Connected to server: {server_info:#?}");

            // List available tools
            let tools = mcp_client.peer().list_tools(Default::default()).await?;
            tracing::info!(
                "Available tools:\n{}",
                serde_json::to_string_pretty(&tools)?
            );

            let mut api_key = std::env::var("ES_API_KEY").unwrap();
            api_key = format!("ApiKey {}", api_key);
            tracing::info!("api_key: {}", api_key);

            let index_name = "paris";

            // create index
            {
                let url = format!("http://127.0.0.1:9200/{index_name}");

                let data = json!({
                    "settings": {
                        "number_of_shards": 1,
                        "number_of_replicas": 1
                    },
                    "mappings": {
                        "properties": {
                            "title": {
                                "type": "text"
                            },
                            "content": {
                                "type": "text"
                            },
                            "created": {
                                "type": "date"
                            },
                            "doc_id": {
                                "type": "keyword"
                            },
                            "chunk_id": {
                                "type": "text"
                            },
                            "chunk_index": {
                                "type": "integer"
                            }
                        }
                    }
                });

                let response = reqwest::Client::new()
                    .put(&url)
                    .header("Authorization", api_key.clone())
                    .header("Content-Type", "application/json")
                    .json(&data)
                    .send()
                    .await?;

                let response_body = response.json::<serde_json::Value>().await?;

                tracing::info!(
                    "Create index response: {}",
                    serde_json::to_string_pretty(&response_body)?
                );
            }

            // add documents
            {
                let url = format!("http://127.0.0.1:9200/{index_name}/_bulk");

                let data = r#"
{"index":{}}
{"title":"Paris","content":"Paris, city and capital of France, situated in the north-central part of the country.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-01","chunk_index":1}
{"index":{}}
{"title":"Paris","content":"People were living on the site of the present-day city, located along the Seine River some 233 miles (375 km) upstream from the river’s mouth on the","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-02","chunk_index":2}
{"index":{}}
{"title":"Paris","content":"English Channel (La Manche), by about 7600 BCE.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-03","chunk_index":3}
{"index":{}}
{"title":"Paris","content":"The modern city has spread from the island (the Île de la Cité) and far beyond both banks of the Seine.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-04","chunk_index":4}
{"index":{}}
{"title":"Paris","content":"Paris occupies a central position in the rich agricultural region known as the Paris Basin, and it constitutes one of eight départements of the Île-de","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-05","chunk_index":5}
{"index":{}}
{"title":"Paris","content":"-France administrative region. It is by far the country’s most important centre of commerce and culture.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-06","chunk_index":6}
{"index":{}}
{"title":"Paris","content":"Area city, 41 square miles (105 square km); metropolitan area, 890 square miles (2,300 square km).","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-07","chunk_index":7}
{"index":{}}
{"title":"Paris","content":"Pop. (2020 est.) city, 2,145,906; (2020 est.) urban agglomeration, 10,858,874.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-08","chunk_index":8}
{"index":{}}
{"title":"Paris","content":"For centuries Paris has been one of the world’s most important and attractive cities.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-09","chunk_index":9}
{"index":{}}
{"title":"Paris","content":"It is appreciated for the opportunities it offers for business and commerce, for study, for culture, and for entertainment; its gastronomy, haute","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-10","chunk_index":10}
{"index":{}}
{"title":"Paris","content":"couture, painting, literature, and intellectual community especially enjoy an enviable reputation.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-11","chunk_index":11}
{"index":{}}
{"title":"Paris","content":"Its sobriquet “the City of Light” (“la Ville Lumière”), earned during the Enlightenment, remains appropriate, for Paris has retained its importance as","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-12","chunk_index":12}
{"index":{}}
{"title":"Paris","content":"a centre for education and intellectual pursuits.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-13","chunk_index":13}
{"index":{}}
{"title":"Paris","content":"Paris’s site at a crossroads of both water and land routes significant not only to France but also to Europe has had a continuing influence on its","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-14","chunk_index":14}
{"index":{}}
{"title":"Paris","content":"growth.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-15","chunk_index":15}
{"index":{}}
{"title":"Paris","content":"Under Roman administration, in the 1st century BCE, the original site on the Île de la Cité was designated the capital of the Parisii tribe and","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-16","chunk_index":16}
{"index":{}}
{"title":"Paris","content":"territory. The Frankish king Clovis I had taken Paris from the Gauls by 494 CE and later made his capital there.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-17","chunk_index":17}
{"index":{}}
{"title":"Paris","content":"Under Hugh Capet (ruled 987–996) and the Capetian dynasty the preeminence of Paris was firmly established, and Paris became the political and cultural","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-18","chunk_index":18}
{"index":{}}
{"title":"Paris","content":"hub as modern France took shape.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-19","chunk_index":19}
{"index":{}}
{"title":"Paris","content":"France has long been a highly centralized country, and Paris has come to be identified with a powerful central state, drawing to itself much of the","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-20","chunk_index":20}
{"index":{}}
{"title":"Paris","content":"talent and vitality of the provinces.","created":"2025-05-08","doc_id":"paris-001","chunk_id":"paris-001-21","chunk_index":21}
"#;

                let response = reqwest::Client::new()
                    .post(&url)
                    .header("Authorization", api_key.clone())
                    .header("Content-Type", "application/json")
                    .body(data)
                    .send()
                    .await?;

                let response_body = response.json::<serde_json::Value>().await?;

                tracing::info!(
                    "Add documents response: {}",
                    serde_json::to_string_pretty(&response_body)?
                );
            }

            // * list indices
            {
                // request param
                let request_param = CallToolRequestParam {
                    name: "list_indices".into(),
                    arguments: Some(serde_json::Map::from_iter([
                        (
                            "base_url".to_string(),
                            serde_json::Value::String("http://127.0.0.1:9200".to_string()),
                        ),
                        (
                            "api_key".to_string(),
                            serde_json::Value::String(api_key.clone()),
                        ),
                    ])),
                };
                // call tool
                let tool_result = mcp_client.peer().call_tool(request_param).await?;

                // parse tool result
                let indices = ListIndicesResponse::from(tool_result);
                tracing::info!("indices:\n{}", serde_json::to_string_pretty(&indices)?);
            }

            // * list aliases
            {
                // request param
                let request_param = CallToolRequestParam {
                    name: "list_aliases".into(),
                    arguments: Some(serde_json::Map::from_iter([
                        (
                            "base_url".to_string(),
                            serde_json::Value::String("http://127.0.0.1:9200".to_string()),
                        ),
                        (
                            "api_key".to_string(),
                            serde_json::Value::String(api_key.clone()),
                        ),
                    ])),
                };

                // call tool
                let tool_result = mcp_client.peer().call_tool(request_param).await?;

                // parse tool result
                let aliases = ListAliasesResponse::from(tool_result);
                tracing::info!("aliases:\n{}", serde_json::to_string_pretty(&aliases)?);
            }

            // * search
            {
                // query
                let query = "What is the location of Paris, France along the Seine river?";

                // fields
                let fields = vec!["title", "content"];

                // request param
                let request_param = CallToolRequestParam {
                    name: "search".into(),
                    arguments: Some(serde_json::Map::from_iter([
                        (
                            "base_url".to_string(),
                            serde_json::Value::String("http://127.0.0.1:9200".to_string()),
                        ),
                        (
                            "api_key".to_string(),
                            serde_json::Value::String(api_key.clone()),
                        ),
                        (
                            "index".to_string(),
                            serde_json::Value::String(index_name.to_string()),
                        ),
                        (
                            "query".to_string(),
                            serde_json::Value::String(query.to_string()),
                        ),
                        (
                            "fields".to_string(),
                            serde_json::Value::Array(
                                fields
                                    .iter()
                                    .map(|f| serde_json::Value::String(f.to_string()))
                                    .collect(),
                            ),
                        ),
                    ])),
                };

                // call tool
                let tool_result = mcp_client.peer().call_tool(request_param).await?;

                // parse tool result
                let search_result = SearchResponse::from(tool_result);
                tracing::info!(
                    "search_result:\n{}",
                    serde_json::to_string_pretty(&search_result)?
                );
            }

            // delete index
            {
                let url = format!("http://127.0.0.1:9200/{index_name}");

                let response = reqwest::Client::new()
                    .delete(&url)
                    .header("Authorization", api_key.clone())
                    .send()
                    .await?;

                let response_body = response.json::<serde_json::Value>().await?;

                tracing::info!(
                    "Delete index response: {}",
                    serde_json::to_string_pretty(&response_body)?
                );
            }

            mcp_client.cancel().await?;
        }
    }

    Ok(())
}

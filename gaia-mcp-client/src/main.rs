// use rmcp::model::CallToolRequestParam;
use rmcp::serve_client;
use std::net::SocketAddr;

const MCP_SERVER_ADDR: &str = "127.0.0.1:8001";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // mcp server address
    let mcp_server_addr: SocketAddr = MCP_SERVER_ADDR.parse()?;

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

    // print server info
    let info = mcp_client.peer_info();
    println!("{}", serde_json::to_string_pretty(&info)?);

    Ok(())
}

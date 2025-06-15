# Gaia TiDB Full Text Search MCP Server

## Quick Start

### Build mcp server and client

Let's build mcp server and client by running the following commands:

```bash
# build mcp server
cargo build --package gaia-tidb-mcp-server --release

# build mcp client
cargo build --package gaia-tidb-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

### Start mcp server

> [!IMPORTANT]
>
> Before running the mcp server, you need to start the TiDB Cloud server.

The CLI options of the mcp server are as follows:

```bash
Usage: gaia-tidb-mcp-server [OPTIONS] --ssl-ca <SSL_CA> --database <DATABASE> --table-name <TABLE_NAME>

Options:
      --ssl-ca <SSL_CA>            Path to the SSL CA certificate. On macOS, this is typically `/etc/ssl/cert.pem`. On Debian/Ubuntu/Arch Linux, it's typically `/etc/ssl/certs/ca-certificates.crt`
  -s, --socket-addr <SOCKET_ADDR>  Socket address to bind to [default: 127.0.0.1:8007]
  -t, --transport <TRANSPORT>      Transport type to use (sse or stream-http) [default: stream-http] [possible values: sse, stream-http]
      --database <DATABASE>        Database name
      --table-name <TABLE_NAME>    Table name
      --limit <LIMIT>              Maximum number of query results to return [default: 10]
  -h, --help                       Print help
  -V, --version                    Print version
```

Now, let's start the mcp server by running the following command:

```bash
# set the SSL CA path on macOS
export SSL_CA_PATH=/etc/ssl/cert.pem
# set the SSL CA path on Debian/Ubuntu/Arch Linux
export SSL_CA_PATH=/etc/ssl/certs/ca-certificates.crt

# set connection parameters
export TIDB_HOST=<your-tidb-host>
export TIDB_PORT=<your-tidb-port>
export TIDB_USERNAME=<your-tidb-username>
export TIDB_PASSWORD=<your-tidb-password>

# run the mcp server
./target/release/gaia-tidb-mcp-server --transport stream-http --ssl-ca $SSL_CA_PATH --database <your-tidb-database> --table-name <your-table-name>
```

> [!IMPORTANT]
> Connections to TiDB Serverless clusters with public endpoint require TLS. Learn more about [secure connection settings](https://docs.pingcap.com/tidbcloud/secure-connections-to-serverless-clusters/).

If start successfully, you will see the following output:

```bash
Starting Gaia TiDB MCP server on 127.0.0.1:8007
```

### Run mcp client

To demonstrate the mcp server, we can use the mcp client to interact with the mcp server. The CLI options of the mcp client are as follows:

```bash
Usage: gaia-tidb-mcp-client [OPTIONS] --query <QUERY>

Options:
      --transport <TRANSPORT>  Transport type to use [default: stream-http] [possible values: sse, stream-http]
      --query <QUERY>          query
  -h, --help                   Print help
  -V, --version                Print version
```

Now, let's run the mcp client by running the following command:

```bash
# run the mcp client
./target/release/gaia-tidb-mcp-client --transport stream-http --query <your-query-text>
```

If start successfully, you will see a response similar to the following output. The output is different depending on the transport type you used. The output of `stream-http` is shown below.

<details><summary>Expand to view the output</summary>

```console
2025-06-15T09:09:04.116185Z  INFO gaia_tidb_mcp_client: 100: Connecting to Gaia TiDB MCP server via stream-http: http://127.0.0.1:8007/mcp
2025-06-15T09:09:04.151349Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "gaia-tidb-mcp-server", version: "0.4.0" }, instructions: Some("A TiDB MCP server") })
2025-06-15T09:09:04.151383Z  INFO gaia_tidb_mcp_client: 120: Connected to server: Some(
    InitializeResult {
        protocol_version: ProtocolVersion(
            "2025-03-26",
        ),
        capabilities: ServerCapabilities {
            experimental: None,
            logging: None,
            completions: None,
            prompts: None,
            resources: None,
            tools: Some(
                ToolsCapability {
                    list_changed: None,
                },
            ),
        },
        server_info: Implementation {
            name: "gaia-tidb-mcp-server",
            version: "0.4.0",
        },
        instructions: Some(
            "A TiDB MCP server",
        ),
    },
)
2025-06-15T09:09:04.158072Z  INFO gaia_tidb_mcp_client: 124: Available tools: ListToolsResult {
    next_cursor: None,
    tools: [
        Tool {
            name: "search",
            description: Some(
                "Perform a keyword search",
            ),
            input_schema: {
                "properties": Object {
                    "query": Object {
                        "description": String("the query to search for"),
                        "type": String("string"),
                    },
                },
                "required": Array [
                    String("query"),
                ],
                "title": String("TidbSearchRequest"),
                "type": String("object"),
            },
            annotations: None,
        },
    ],
}
2025-06-15T09:09:16.273464Z  INFO gaia_tidb_mcp_client: 136: search response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"hits\":[{\"id\":1,\"title\":\"イヤホン bluetooth ワイヤレスイヤホン\",\"content\":\"イヤホン bluetooth ワイヤレスイヤホン\"},{\"id\":6,\"title\":\"Lightweight Bluetooth Earbuds with 48 Hours Playtime\",\"content\":\"Lightweight Bluetooth Earbuds with 48 Hours Playtime\"},{\"id\":3,\"title\":\"ワイヤレス ヘッドホン Bluetooth 5.3 65時間再生 ヘッドホン 40mm HD\",\"content\":\"ワイヤレス ヘッドホン Bluetooth 5.3 65時間再生 ヘッドホン 40mm HD\"}]}"
    }
  ],
  "isError": false
}
2025-06-15T09:09:16.273501Z  INFO gaia_tidb_mcp_client: 143: search_result:
{
  "hits": [
    {
      "id": 1,
      "title": "イヤホン bluetooth ワイヤレスイヤホン",
      "content": "イヤホン bluetooth ワイヤレスイヤホン"
    },
    {
      "id": 6,
      "title": "Lightweight Bluetooth Earbuds with 48 Hours Playtime",
      "content": "Lightweight Bluetooth Earbuds with 48 Hours Playtime"
    },
    {
      "id": 3,
      "title": "ワイヤレス ヘッドホン Bluetooth 5.3 65時間再生 ヘッドホン 40mm HD",
      "content": "ワイヤレス ヘッドホン Bluetooth 5.3 65時間再生 ヘッドホン 40mm HD"
    }
  ]
}
2025-06-15T09:09:16.274167Z  INFO rmcp::service: 625: task cancelled
2025-06-15T09:09:16.274490Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

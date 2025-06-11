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
Usage: gaia-tidb-mcp-server [OPTIONS] --ssl-ca <SSL_CA>

Options:
      --ssl-ca <SSL_CA>            Path to the SSL CA certificate. On macOS, this is typically `/etc/ssl/cert.pem`. On Debian/Ubuntu/Arch Linux, it's typically `/etc/ssl/certs/ca-certificates.crt`
  -s, --socket-addr <SOCKET_ADDR>  Socket address to bind to [default: 127.0.0.1:8007]
  -t, --transport <TRANSPORT>      Transport type to use (sse or stream-http) [default: stream-http] [possible values: sse, stream-http]
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

# run the mcp server (stream-http)
./target/release/gaia-tidb-mcp-server --transport stream-http --ssl-ca $SSL_CA_PATH

# run the mcp server (sse)
./target/release/gaia-tidb-mcp-server --transport sse --ssl-ca $SSL_CA_PATH
```

> [!IMPORTANT]
> Connections to TiDB Serverless clusters with public endpoint require TLS. Learn more about [secure connection settings](https://docs.pingcap.com/tidbcloud/secure-connections-to-serverless-clusters/).

If start successfully, you will see the following output:

```bash
Starting Gaia TiDB MCP server on 127.0.0.1:8007
```

### Run mcp client

The CLI options of the mcp client are as follows:

```bash
Usage: gaia-tidb-mcp-client [OPTIONS] --tidb-database <TIDB_DATABASE> --tidb-table-name <TIDB_TABLE_NAME> --query <QUERY>

Options:
      --transport <TRANSPORT>
          Transport type to use (sse) [default: sse] [possible values: sse, stream-http]
      --tidb-database <TIDB_DATABASE>
          database
      --tidb-table-name <TIDB_TABLE_NAME>
          table name
      --tidb-limit <TIDB_LIMIT>
          limit [default: 10]
      --query <QUERY>
          query
  -h, --help
          Print help
  -V, --version
          Print version
```

Now, let's run the mcp client by running the following command:

```bash
export DATABASE=<your-tidb-database>
export TABLE_NAME=<your-tidb-table-name>
export LIMIT=<your-tidb-limit>
export QUERY=<your-query>

# run the mcp client (stream-http)
./target/release/gaia-tidb-mcp-client --tidb-database $DATABASE \
    --tidb-table-name $TABLE_NAME \
    --tidb-limit $LIMIT \
    --query $QUERY \
    --transport stream-http

# run the mcp client (sse)
./target/release/gaia-tidb-mcp-client --tidb-database $DATABASE \
    --tidb-table-name $TABLE_NAME \
    --tidb-limit $LIMIT \
    --query $QUERY \
    --transport sse
```

If start successfully, you will see a response similar to the following output. The output is different depending on the transport type you used. The output of `stream-http` is shown below.

<details><summary>Expand to view the output</summary>

```console
2025-06-11T08:52:48.630379Z  INFO gaia_tidb_mcp_client: 120: Connecting to Gaia TiDB MCP server via stream-http: http://127.0.0.1:8007/mcp
2025-06-11T08:52:48.650413Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A TiDB MCP server") })
2025-06-11T08:52:48.650447Z  INFO gaia_tidb_mcp_client: 140: Connected to server: Some(
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
            name: "rmcp",
            version: "0.1.5",
        },
        instructions: Some(
            "A TiDB MCP server",
        ),
    },
)
2025-06-11T08:52:48.652772Z  INFO gaia_tidb_mcp_client: 144: Available tools: ListToolsResult {
    next_cursor: None,
    tools: [
        Tool {
            name: "search",
            description: Some(
                "Search for documents in a TiDB database",
            ),
            input_schema: {
                "properties": Object {
                    "database": Object {
                        "description": String("the database of the tidb server"),
                        "type": String("string"),
                    },
                    "limit": Object {
                        "description": String("the number of rows to return"),
                        "format": String("uint64"),
                        "minimum": Number(0.0),
                        "nullable": Bool(true),
                        "type": String("integer"),
                    },
                    "query": Object {
                        "description": String("the query to search for"),
                        "type": String("string"),
                    },
                    "table_name": Object {
                        "description": String("the table name to search in"),
                        "type": String("string"),
                    },
                },
                "required": Array [
                    String("database"),
                    String("query"),
                    String("table_name"),
                ],
                "title": String("TidbSearchRequest"),
                "type": String("object"),
            },
            annotations: None,
        },
    ],
}
2025-06-11T08:53:01.132500Z  INFO gaia_tidb_mcp_client: 167: search response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"hits\":[{\"id\":6,\"title\":\"Lightweight Bluetooth Earbuds with 48 Hours Playtime\",\"content\":\"Lightweight Bluetooth Earbuds with 48 Hours Playtime\"},{\"id\":1,\"title\":\"イヤホン bluetooth ワイヤレスイヤホン\",\"content\":\"イヤホン bluetooth ワイヤレスイヤホン\"},{\"id\":7,\"title\":\"True Wireless Noise Cancelling Earbuds - Compatible with Apple & Android, Built-in Microphone\",\"content\":\"True Wireless Noise Cancelling Earbuds - Compatible with Apple & Android, Built-in Microphone\"},{\"id\":3,\"title\":\"ワイヤレス ヘッドホン Bluetooth 5.3 65時間再生 ヘッドホン 40mm HD\",\"content\":\"ワイヤレス ヘッドホン Bluetooth 5.3 65時間再生 ヘッドホン 40mm HD\"}]}"
    }
  ],
  "isError": false
}
2025-06-11T08:53:01.132742Z  INFO gaia_tidb_mcp_client: 174: search_result:
{
  "hits": [
    {
      "id": 6,
      "title": "Lightweight Bluetooth Earbuds with 48 Hours Playtime",
      "content": "Lightweight Bluetooth Earbuds with 48 Hours Playtime"
    },
    {
      "id": 1,
      "title": "イヤホン bluetooth ワイヤレスイヤホン",
      "content": "イヤホン bluetooth ワイヤレスイヤホン"
    },
    {
      "id": 7,
      "title": "True Wireless Noise Cancelling Earbuds - Compatible with Apple & Android, Built-in Microphone",
      "content": "True Wireless Noise Cancelling Earbuds - Compatible with Apple & Android, Built-in Microphone"
    },
    {
      "id": 3,
      "title": "ワイヤレス ヘッドホン Bluetooth 5.3 65時間再生 ヘッドホン 40mm HD",
      "content": "ワイヤレス ヘッドホン Bluetooth 5.3 65時間再生 ヘッドホン 40mm HD"
    }
  ]
}
2025-06-11T08:53:01.132781Z  INFO rmcp::service: 625: task cancelled
2025-06-11T08:53:01.132808Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

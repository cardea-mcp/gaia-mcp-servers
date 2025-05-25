# Gaia TiDB Full Text Search MCP Server

## Quick Start

### Build and run gaia-tidb-mcp-server (sse)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (tcp)
cargo build --package gaia-tidb-mcp-server-sse --release

# build mcp client
cargo build --package gaia-tidb-mcp-client --release
```

**Note** that, before running the mcp server, you need to start the TiDB Cloud server.

Now, let's start the mcp server by running the following command:

```bash
# set the SSL CA path on macOS
export SSL_CA_PATH=/etc/ssl/cert.pem

# set the SSL CA path on Debian/Ubuntu/Arch Linux
export SSL_CA_PATH=/etc/ssl/certs/ca-certificates.crt

# run the mcp server
./target/release/gaia-tidb-mcp-server-sse --ssl-ca $SSL_CA_PATH
```

> [!NOTE]
> Connections to TiDB Serverless clusters with public endpoint require TLS. Learn more about [secure connection settings](https://docs.pingcap.com/tidbcloud/secure-connections-to-serverless-clusters/).

If start successfully, you will see the following output:

```bash
Starting Gaia TiDB MCP server on 127.0.0.1:8007
```

Now, let's run the mcp client by running the following command:

```bash
export HOST=<your-tidb-host>
export PORT=<your-tidb-port>
export USERNAME=<your-tidb-username>
export PASSWORD=<your-tidb-password>
export DATABASE=<your-tidb-database>
export TABLE_NAME=<your-tidb-table-name>
export LIMIT=<your-tidb-limit>
export QUERY=<your-query>

# run the mcp client
./target/release/gaia-tidb-mcp-client --tidb-host $HOST \
    --tidb-port $PORT \
    --tidb-username $USERNAME \
    --tidb-password $PASSWORD \
    --tidb-database $DATABASE \
    --tidb-table-name $TABLE_NAME \
    --tidb-limit $LIMIT \
    --query $QUERY \
    --transport sse
```

If start successfully, you will see a response similar to the following output:

<details><summary>Expand to view the output</summary>

```bash
2025-05-25T09:36:21.285636Z  INFO gaia_tidb_mcp_client: 72: Connecting to TiDB MCP server via sse
2025-05-25T09:36:21.307840Z  INFO serve_inner: rmcp::service: 519: Service initialized as client peer_info=InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A TiDB MCP server") }
2025-05-25T09:36:21.307898Z  INFO gaia_tidb_mcp_client: 91: Connected to server: InitializeResult {
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
}
2025-05-25T09:36:21.313286Z  INFO gaia_tidb_mcp_client: 95: Available tools:
{
  "tools": [
    {
      "name": "search",
      "description": "Search for documents in a TiDB database",
      "inputSchema": {
        "properties": {
          "database": {
            "description": "the database of the tidb server",
            "type": "string"
          },
          "host": {
            "description": "the host of the tidb server",
            "type": "string"
          },
          "limit": {
            "description": "the limit of the tidb server",
            "format": "uint64",
            "minimum": 0.0,
            "nullable": true,
            "type": "integer"
          },
          "password": {
            "description": "the password of the tidb server",
            "type": "string"
          },
          "port": {
            "description": "the port of the tidb server",
            "format": "uint16",
            "minimum": 0.0,
            "type": "integer"
          },
          "query": {
            "description": "the query of the tidb server",
            "type": "string"
          },
          "table_name": {
            "description": "the table name of the tidb server",
            "type": "string"
          },
          "username": {
            "description": "the username of the tidb server",
            "type": "string"
          }
        },
        "required": [
          "database",
          "host",
          "password",
          "port",
          "query",
          "table_name",
          "username"
        ],
        "title": "TidbSearchRequest",
        "type": "object"
      }
    }
  ]
}
2025-05-25T09:36:55.952875Z  INFO gaia_tidb_mcp_client: 133: search response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"hits\":[{\"id\":11,\"title\":\"无线消噪耳机\",\"content\":\"无线消噪耳机-黑色 手势触控蓝牙降噪 主动降噪头戴式耳机（智能降噪 长久续航）\"},{\"id\":14,\"title\":\"无线蓝牙耳机\",\"content\":\"无线蓝牙耳机超长续航42小时快速充电 流光金属耳机\"},{\"id\":12,\"title\":\"专业版USB7.1声道游戏耳机\",\"content\":\"专业版USB7.1声道游戏耳机电竞耳麦头戴式电脑网课办公麦克风带线控\"}]}"
    }
  ],
  "isError": false
}
2025-05-25T09:36:55.952935Z  INFO rmcp::service: 603: task cancelled
2025-05-25T09:36:55.952951Z  INFO rmcp::service: 789: serve finished quit_reason=Cancelled
```

</details>

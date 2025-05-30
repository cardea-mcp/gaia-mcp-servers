# Gaia TiDB Full Text Search MCP Server

## Quick Start

### Build and run gaia-tidb-mcp-server (StreamableHttp)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (streamablehttp)
cargo build --package gaia-tidb-mcp-server-streamhttp --release

# build mcp client
cargo build --package gaia-tidb-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

> [!IMPORTANT]
>
> Before running the mcp server, you need to start the TiDB Cloud server.

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
./target/release/gaia-tidb-mcp-server-sse --ssl-ca $SSL_CA_PATH
```

> [!IMPORTANT]
> Connections to TiDB Serverless clusters with public endpoint require TLS. Learn more about [secure connection settings](https://docs.pingcap.com/tidbcloud/secure-connections-to-serverless-clusters/).

If start successfully, you will see the following output:

```bash
Starting Gaia TiDB MCP server on 127.0.0.1:8007
```

Now, let's run the mcp client by running the following command:

```bash
export DATABASE=<your-tidb-database>
export TABLE_NAME=<your-tidb-table-name>
export LIMIT=<your-tidb-limit>
export QUERY=<your-query>

# run the mcp client
./target/release/gaia-tidb-mcp-client --tidb-database $DATABASE \
    --tidb-table-name $TABLE_NAME \
    --tidb-limit $LIMIT \
    --query $QUERY \
    --transport stream-http
```

If start successfully, you will see a response similar to the following output:

<details><summary>Expand to view the output</summary>

```console
2025-05-29T08:10:56.785064Z  INFO gaia_tidb_mcp_client: 120: Connecting to Gaia TiDB MCP server via stream-http: http://127.0.0.1:8007/mcp
2025-05-29T08:10:56.805574Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A TiDB MCP server") })
2025-05-29T08:10:56.805612Z  INFO gaia_tidb_mcp_client: 140: Connected to server: Some(
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
2025-05-29T08:10:56.813375Z  INFO gaia_tidb_mcp_client: 144: Available tools: ListToolsResult {
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
2025-05-29T08:11:25.579990Z  INFO gaia_tidb_mcp_client: 167: search response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"hits\":[{\"id\":11,\"title\":\"无线消噪耳机\",\"content\":\"无线消噪耳机-黑色 手势触控蓝牙降噪 主动降噪头戴式耳机（智能降噪 长久续航）\"},{\"id\":14,\"title\":\"无线蓝牙耳机\",\"content\":\"无线蓝牙耳机超长续航42小时快速充电 流光金属耳机\"},{\"id\":12,\"title\":\"专业版USB7.1声道游戏耳机\",\"content\":\"专业版USB7.1声道游戏耳机电竞耳麦头戴式电脑网课办公麦克风带线控\"}]}"
    }
  ],
  "isError": false
}
2025-05-29T08:11:25.580044Z  INFO gaia_tidb_mcp_client: 174: search_result:
{
  "hits": [
    {
      "id": 11,
      "title": "无线消噪耳机",
      "content": "无线消噪耳机-黑色 手势触控蓝牙降噪 主动降噪头戴式耳机（智能降噪 长久续航）"
    },
    {
      "id": 14,
      "title": "无线蓝牙耳机",
      "content": "无线蓝牙耳机超长续航42小时快速充电 流光金属耳机"
    },
    {
      "id": 12,
      "title": "专业版USB7.1声道游戏耳机",
      "content": "专业版USB7.1声道游戏耳机电竞耳麦头戴式电脑网课办公麦克风带线控"
    }
  ]
}
2025-05-29T08:11:25.580123Z  INFO rmcp::service: 625: task cancelled
2025-05-29T08:11:25.580229Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

### Build and run gaia-tidb-mcp-server (sse)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (tcp)
cargo build --package gaia-tidb-mcp-server-sse --release

# build mcp client
cargo build --package gaia-tidb-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

> [!IMPORTANT]
>
> Before running the mcp server, you need to start the TiDB Cloud server.

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
./target/release/gaia-tidb-mcp-server-sse --ssl-ca $SSL_CA_PATH
```

> [!IMPORTANT]
> Connections to TiDB Serverless clusters with public endpoint require TLS. Learn more about [secure connection settings](https://docs.pingcap.com/tidbcloud/secure-connections-to-serverless-clusters/).

If start successfully, you will see the following output:

```bash
Starting Gaia TiDB MCP server on 127.0.0.1:8007
```

Now, let's run the mcp client by running the following command:

```bash
export DATABASE=<your-tidb-database>
export TABLE_NAME=<your-tidb-table-name>
export LIMIT=<your-tidb-limit>
export QUERY=<your-query>

# run the mcp client
./target/release/gaia-tidb-mcp-client --tidb-database $DATABASE \
    --tidb-table-name $TABLE_NAME \
    --tidb-limit $LIMIT \
    --query $QUERY \
    --transport sse
```

If start successfully, you will see a response similar to the following output:

<details><summary>Expand to view the output</summary>

```console
2025-05-29T07:54:57.284265Z  INFO gaia_tidb_mcp_client: 57: Connecting to Gaia TiDB MCP server via sse: http://127.0.0.1:8007/sse
2025-05-29T07:54:57.307704Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A TiDB MCP server") })
2025-05-29T07:54:57.307798Z  INFO gaia_tidb_mcp_client: 74: Connected to server: Some(
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
2025-05-29T07:54:57.313695Z  INFO gaia_tidb_mcp_client: 78: Available tools:
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
          "limit": {
            "description": "the number of rows to return",
            "format": "uint64",
            "minimum": 0.0,
            "nullable": true,
            "type": "integer"
          },
          "query": {
            "description": "the query to search for",
            "type": "string"
          },
          "table_name": {
            "description": "the table name to search in",
            "type": "string"
          }
        },
        "required": [
          "database",
          "query",
          "table_name"
        ],
        "title": "TidbSearchRequest",
        "type": "object"
      }
    }
  ]
}
2025-05-29T07:55:26.317373Z  INFO gaia_tidb_mcp_client: 104: search response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"hits\":[{\"id\":11,\"title\":\"无线消噪耳机\",\"content\":\"无线消噪耳机-黑色 手势触控蓝牙降噪 主动降噪头戴式耳机（智能降噪 长久续航）\"},{\"id\":14,\"title\":\"无线蓝牙耳机\",\"content\":\"无线蓝牙耳机超长续航42小时快速充电 流光金属耳机\"},{\"id\":12,\"title\":\"专业版USB7.1声道游戏耳机\",\"content\":\"专业版USB7.1声道游戏耳机电竞耳麦头戴式电脑网课办公麦克风带线控\"}]}"
    }
  ],
  "isError": false
}
2025-05-29T07:55:26.317419Z  INFO gaia_tidb_mcp_client: 111: search_result:
{
  "hits": [
    {
      "id": 11,
      "title": "无线消噪耳机",
      "content": "无线消噪耳机-黑色 手势触控蓝牙降噪 主动降噪头戴式耳机（智能降噪 长久续航）"
    },
    {
      "id": 14,
      "title": "无线蓝牙耳机",
      "content": "无线蓝牙耳机超长续航42小时快速充电 流光金属耳机"
    },
    {
      "id": 12,
      "title": "专业版USB7.1声道游戏耳机",
      "content": "专业版USB7.1声道游戏耳机电竞耳麦头戴式电脑网课办公麦克风带线控"
    }
  ]
}
2025-05-29T07:55:26.317529Z  INFO rmcp::service: 625: task cancelled
2025-05-29T07:55:26.317632Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

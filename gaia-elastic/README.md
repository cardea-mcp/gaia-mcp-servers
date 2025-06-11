# Gaia Elastic MCP Server

## Quick Start

### Build mcp server and client

Let's build mcp server and client by running the following commands:

```bash
# build mcp server
cargo build --package gaia-elastic-mcp-server --release

# build mcp client
cargo build --package gaia-elastic-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

### Start mcp server

> [!IMPORTANT]
>
> Before running the mcp server, you need to start the Elasticsearch server. If you don't have an Elasticsearch server running, you can refer to [Run Elasticsearch locally](https://www.elastic.co/docs/solutions/search/run-elasticsearch-locally#local-dev-quick-start) or use the following command to start one:
>
> ```bash
> curl -fsSL https://elastic.co/start-local | sh
> ```
>
> The Elasticsearch server will be started on `http://localhost:9200`. And you can get the API key from the output of the command, remember it! You can pass it to the mcp server while invoking the mcp tools.

The CLI options of the mcp server are as follows:

```bash
Usage: gaia-elastic-mcp-server [OPTIONS]

Options:
      --base-url <BASE_URL>        The base URL of the Elasticsearch server [default: http://127.0.0.1:9200]
  -s, --socket-addr <SOCKET_ADDR>  Socket address to bind to [default: 127.0.0.1:8006]
  -t, --transport <TRANSPORT>      Transport type to use (sse or stream-http) [default: stream-http] [possible values: sse, stream-http]
  -h, --help                       Print help
  -V, --version                    Print version
```

Now, let's start the mcp server:

```bash
export ES_API_KEY=<your-api-key>

# run mcp server (stream-http)
./target/release/gaia-elastic-mcp-server --transport stream-http

# run mcp server (sse)
./target/release/gaia-elastic-mcp-server --transport sse

# run mcp server (stdio)
./target/release/gaia-elastic-mcp-server --transport stdio
```

If start successfully, you will see the following output:

```bash
Starting Gaia Elastic MCP server on 127.0.0.1:8006
```

### Run mcp client

The CLI options of the mcp client are as follows:

```bash
Usage: gaia-elastic-mcp-client [OPTIONS] --index <INDEX>

Options:
  -t, --transport <TRANSPORT>  Transport type to use [default: stream-http] [possible values: stdio, sse, stream-http]
  -i, --index <INDEX>          The name of the index to use
  -h, --help                   Print help
  -V, --version                Print version
```

Now, let's run the mcp client by running the following command:

```bash
export ES_API_KEY=<your-api-key>

# run mcp client (stream-http)
./target/release/gaia-elastic-mcp-client --transport stream-http --index test01

# run mcp client (sse)
./target/release/gaia-elastic-mcp-client --transport sse --index test01

# run mcp client (stdio)
./target/release/gaia-elastic-mcp-client --transport stdio --index test01
```

If start successfully, you will see the following output. The output is different depending on the transport type you used. The output of `stream-http` is shown below.

<details><summary>Expand to view the output</summary>

```console
2025-06-11T08:31:28.318557Z  INFO gaia_elastic_mcp_client: 318: Connecting to Gaia Qdrant MCP server via stream-http: http://127.0.0.1:8006/mcp
2025-06-11T08:31:28.342863Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A ElasticSearch MCP server") })
2025-06-11T08:31:28.342981Z  INFO gaia_elastic_mcp_client: 338: Connected to server: Some(
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
            "A ElasticSearch MCP server",
        ),
    },
)
2025-06-11T08:31:28.346000Z  INFO gaia_elastic_mcp_client: 342: Available tools:
{
  "tools": [
    {
      "name": "list_indices",
      "description": "List all available Elasticsearch indices",
      "inputSchema": {
        "title": "EmptyObject",
        "type": "object"
      }
    },
    {
      "name": "search",
      "description": "Search for documents in an Elasticsearch index",
      "inputSchema": {
        "properties": {
          "fields": {
            "description": "name of fields to search",
            "items": {
              "type": "string"
            },
            "type": "array"
          },
          "index": {
            "description": "index name",
            "type": "string"
          },
          "query": {
            "description": "user query",
            "type": "string"
          },
          "size": {
            "description": "number of results to return",
            "format": "uint64",
            "minimum": 0.0,
            "nullable": true,
            "type": "integer"
          }
        },
        "required": [
          "fields",
          "index",
          "query"
        ],
        "title": "SearchRequest",
        "type": "object"
      }
    },
    {
      "name": "list_aliases",
      "description": "Get the cluster's index aliases, including filter and routing information. Note that this tool does not return data stream aliases.",
      "inputSchema": {
        "title": "EmptyObject",
        "type": "object"
      }
    }
  ]
}
2025-06-11T08:31:29.045960Z  INFO gaia_elastic_mcp_client: 393: Create index response: {
  "acknowledged": true,
  "index": "test03",
  "shards_acknowledged": true
}
2025-06-11T08:31:29.428760Z  INFO gaia_elastic_mcp_client: 458: Add documents response: {
  "errors": false,
  "items": [
    {
      "index": {
        "_id": "6MwdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 0,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "6cwdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 1,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "6swdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 2,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "68wdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 3,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "7MwdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 4,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "7cwdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 5,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "7swdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 6,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "78wdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 7,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "8MwdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 8,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "8cwdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 9,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "8swdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 10,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "88wdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 11,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "9MwdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 12,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "9cwdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 13,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "9swdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 14,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "98wdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 15,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "-MwdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 16,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "-cwdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 17,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "-swdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 18,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "-8wdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 19,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    },
    {
      "index": {
        "_id": "_MwdXpcB4-JOtvuIoIFh",
        "_index": "test03",
        "_primary_term": 1,
        "_seq_no": 20,
        "_shards": {
          "failed": 0,
          "successful": 1,
          "total": 2
        },
        "_version": 1,
        "result": "created",
        "status": 201
      }
    }
  ],
  "took": 0
}
2025-06-11T08:31:29.717728Z  INFO gaia_elastic_mcp_client: 476: indices:
{
  "indices": [
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-default.alerts-default-000001",
      "uuid": "zpmCR1CuSbmqsVpzSwUwAA",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-ml.anomaly-detection-health.alerts-default-000001",
      "uuid": "odkEnOstTUWjy2YFt49MNQ",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-ml.anomaly-detection.alerts-default-000001",
      "uuid": "ZyxmlJhtTwW4OtfzSaMxgg",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-observability.apm.alerts-default-000001",
      "uuid": "DJgJRHcfSiKg762eX9cHIQ",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-observability.logs.alerts-default-000001",
      "uuid": "gPoRGtIsQi-Dqlg1d11bMA",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-observability.metrics.alerts-default-000001",
      "uuid": "HWSktmlRT1CKJe448WIo-Q",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-observability.slo.alerts-default-000001",
      "uuid": "q3P45sOPT7-xXkHmVDuWAw",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-observability.threshold.alerts-default-000001",
      "uuid": "czi-egGwQ9O3yjOxH4gRKg",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-observability.uptime.alerts-default-000001",
      "uuid": "B8Pc0if9RbGuoBeATTBQIg",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-security.alerts-default-000001",
      "uuid": "TtPcxTt-QL-0R0pVWzwUbQ",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-stack.alerts-default-000001",
      "uuid": "H0SFkkeZQlOpCQFjhajj3w",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "green",
      "status": "open",
      "index": ".internal.alerts-transform.health.alerts-default-000001",
      "uuid": "0vJqTt9-TsueBRcw_qJktg",
      "pri": "1",
      "rep": "0",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "yellow",
      "status": "open",
      "index": "paris02",
      "uuid": "y3Ad2pEbQAOM5OfBCGNxaQ",
      "pri": "1",
      "rep": "1",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "249b",
      "pri.store.size": "249b",
      "dataset.size": "249b"
    },
    {
      "health": "yellow",
      "status": "open",
      "index": "paris03",
      "uuid": "r0k4cP6ZS0eZeiqwmci6Fg",
      "pri": "1",
      "rep": "1",
      "docs.count": "45",
      "docs.deleted": "0",
      "store.size": "34.4kb",
      "pri.store.size": "34.4kb",
      "dataset.size": "34.4kb"
    },
    {
      "health": "yellow",
      "status": "open",
      "index": "test03",
      "uuid": "9_SNtDXeQAuMDzTzTzEThA",
      "pri": "1",
      "rep": "1",
      "docs.count": "0",
      "docs.deleted": "0",
      "store.size": "227b",
      "pri.store.size": "227b",
      "dataset.size": "227b"
    },
    {
      "health": "yellow",
      "status": "open",
      "index": "xibei-01",
      "uuid": "UA_f68qET_-a9oydU_WsXw",
      "pri": "1",
      "rep": "1",
      "docs.count": "16",
      "docs.deleted": "0",
      "store.size": "83.9kb",
      "pri.store.size": "83.9kb",
      "dataset.size": "83.9kb"
    },
    {
      "health": "yellow",
      "status": "open",
      "index": "xibei-02",
      "uuid": "J1w3Qo1OTg2DuuyUz7dNaw",
      "pri": "1",
      "rep": "1",
      "docs.count": "16",
      "docs.deleted": "0",
      "store.size": "83.6kb",
      "pri.store.size": "83.6kb",
      "dataset.size": "83.6kb"
    }
  ]
}
2025-06-11T08:31:30.002659Z  INFO gaia_elastic_mcp_client: 492: aliases:
{
  "aliases": [
    {
      "alias": ".alerts-transform.health.alerts-default",
      "index": ".internal.alerts-transform.health.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".alerts-ml.anomaly-detection.alerts-default",
      "index": ".internal.alerts-ml.anomaly-detection.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".alerts-observability.slo.alerts-default",
      "index": ".internal.alerts-observability.slo.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".kibana_security_session",
      "index": ".kibana_security_session_1",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".kibana_entities-definitions",
      "index": ".kibana_entities-definitions-1",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".alerts-observability.apm.alerts-default",
      "index": ".internal.alerts-observability.apm.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".alerts-default.alerts-default",
      "index": ".internal.alerts-default.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".alerts-observability.metrics.alerts-default",
      "index": ".internal.alerts-observability.metrics.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".kibana_usage_counters",
      "index": ".kibana_usage_counters_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".kibana_usage_counters_9.0.0",
      "index": ".kibana_usage_counters_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".alerts-ml.anomaly-detection-health.alerts-default",
      "index": ".internal.alerts-ml.anomaly-detection-health.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".alerts-security.alerts-default",
      "index": ".internal.alerts-security.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".siem-signals-default",
      "index": ".internal.alerts-security.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "false"
    },
    {
      "alias": ".kibana",
      "index": ".kibana_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".kibana_9.0.0",
      "index": ".kibana_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".kibana_ingest",
      "index": ".kibana_ingest_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".kibana_ingest_9.0.0",
      "index": ".kibana_ingest_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".alerts-stack.alerts-default",
      "index": ".internal.alerts-stack.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".kibana_security_solution",
      "index": ".kibana_security_solution_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".kibana_security_solution_9.0.0",
      "index": ".kibana_security_solution_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".alerts-observability.logs.alerts-default",
      "index": ".internal.alerts-observability.logs.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".kibana_task_manager",
      "index": ".kibana_task_manager_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".kibana_task_manager_9.0.0",
      "index": ".kibana_task_manager_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".alerts-observability.uptime.alerts-default",
      "index": ".internal.alerts-observability.uptime.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".kibana_alerting_cases",
      "index": ".kibana_alerting_cases_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".kibana_alerting_cases_9.0.0",
      "index": ".kibana_alerting_cases_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".security",
      "index": ".security-7",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".alerts-observability.threshold.alerts-default",
      "index": ".internal.alerts-observability.threshold.alerts-default-000001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "true"
    },
    {
      "alias": ".kibana_analytics",
      "index": ".kibana_analytics_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    },
    {
      "alias": ".kibana_analytics_9.0.0",
      "index": ".kibana_analytics_9.0.0_001",
      "filter": "-",
      "routing.index": "-",
      "routing.search": "-",
      "is_write_index": "-"
    }
  ]
}
2025-06-11T08:31:30.280734Z  INFO gaia_elastic_mcp_client: 532: search_result:
{
  "took": 1,
  "timed_out": false,
  "_shards": {
    "total": 1,
    "successful": 1,
    "skipped": 0,
    "failed": 0
  },
  "hits": {
    "hits": [
      {
        "_index": "test03",
        "_score": 7.955199,
        "_source": {
          "chunk_id": "paris-001-02",
          "chunk_index": 2,
          "content": "People were living on the site of the present-day city, located along the Seine River some 233 miles (375 km) upstream from the river’s mouth on the",
          "created": "2025-05-08",
          "doc_id": "paris-001",
          "title": "Paris"
        }
      },
      {
        "_index": "test03",
        "_score": 5.2056227,
        "_source": {
          "chunk_id": "paris-001-06",
          "chunk_index": 6,
          "content": "-France administrative region. It is by far the country’s most important centre of commerce and culture.",
          "created": "2025-05-08",
          "doc_id": "paris-001",
          "title": "Paris"
        }
      },
      {
        "_index": "test03",
        "_score": 4.762345,
        "_source": {
          "chunk_id": "paris-001-01",
          "chunk_index": 1,
          "content": "Paris, city and capital of France, situated in the north-central part of the country.",
          "created": "2025-05-08",
          "doc_id": "paris-001",
          "title": "Paris"
        }
      },
      {
        "_index": "test03",
        "_score": 4.071006,
        "_source": {
          "chunk_id": "paris-001-04",
          "chunk_index": 4,
          "content": "The modern city has spread from the island (the Île de la Cité) and far beyond both banks of the Seine.",
          "created": "2025-05-08",
          "doc_id": "paris-001",
          "title": "Paris"
        }
      },
      {
        "_index": "test03",
        "_score": 3.3300545,
        "_source": {
          "chunk_id": "paris-001-05",
          "chunk_index": 5,
          "content": "Paris occupies a central position in the rich agricultural region known as the Paris Basin, and it constitutes one of eight départements of the Île-de",
          "created": "2025-05-08",
          "doc_id": "paris-001",
          "title": "Paris"
        }
      },
      {
        "_index": "test03",
        "_score": 3.2725017,
        "_source": {
          "chunk_id": "paris-001-20",
          "chunk_index": 20,
          "content": "France has long been a highly centralized country, and Paris has come to be identified with a powerful central state, drawing to itself much of the",
          "created": "2025-05-08",
          "doc_id": "paris-001",
          "title": "Paris"
        }
      },
      {
        "_index": "test03",
        "_score": 3.2078598,
        "_source": {
          "chunk_id": "paris-001-18",
          "chunk_index": 18,
          "content": "Under Hugh Capet (ruled 987–996) and the Capetian dynasty the preeminence of Paris was firmly established, and Paris became the political and cultural",
          "created": "2025-05-08",
          "doc_id": "paris-001",
          "title": "Paris"
        }
      },
      {
        "_index": "test03",
        "_score": 2.8103137,
        "_source": {
          "chunk_id": "paris-001-09",
          "chunk_index": 9,
          "content": "For centuries Paris has been one of the world’s most important and attractive cities.",
          "created": "2025-05-08",
          "doc_id": "paris-001",
          "title": "Paris"
        }
      },
      {
        "_index": "test03",
        "_score": 2.7980127,
        "_source": {
          "chunk_id": "paris-001-10",
          "chunk_index": 10,
          "content": "It is appreciated for the opportunities it offers for business and commerce, for study, for culture, and for entertainment; its gastronomy, haute",
          "created": "2025-05-08",
          "doc_id": "paris-001",
          "title": "Paris"
        }
      },
      {
        "_index": "test03",
        "_score": 2.6930192,
        "_source": {
          "chunk_id": "paris-001-12",
          "chunk_index": 12,
          "content": "Its sobriquet “the City of Light” (“la Ville Lumière”), earned during the Enlightenment, remains appropriate, for Paris has retained its importance as",
          "created": "2025-05-08",
          "doc_id": "paris-001",
          "title": "Paris"
        }
      }
    ]
  }
}
2025-06-11T08:31:30.702927Z  INFO gaia_elastic_mcp_client: 550: Delete index response: {
  "acknowledged": true
}
2025-06-11T08:31:30.702978Z  INFO rmcp::service: 625: task cancelled
2025-06-11T08:31:30.703015Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

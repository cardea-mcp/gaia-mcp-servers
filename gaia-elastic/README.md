# Gaia Elastic MCP Server

## Quick Start

### Build and run gaia-elastic-mcp-server (SSE)

Let's build mcp server and client by running the following commands:


```bash
# build mcp server (sse)
cargo build --package gaia-elastic-mcp-server-sse --release

# build mcp client
cargo build --package gaia-elastic-mcp-client --release
```

**Note** that, before running the mcp server, you need to start the Elasticsearch server. If you don't have an Elasticsearch server running, you can refer to [Run Elasticsearch locally](https://www.elastic.co/docs/solutions/search/run-elasticsearch-locally#local-dev-quick-start) or use the following command to start one:

```bash
curl -fsSL https://elastic.co/start-local | sh
```

The Elasticsearch server will be started on `http://localhost:9200`. And you can get the API key from the output of the command, remember it! You can pass it to the mcp server while invoking the mcp tools.

**Note** that, To run the mcp client, you need to set the `ES_API_KEY` environment variable with the following command. The mcp client will use this key to access the Elasticsearch server.

```bash
export ES_API_KEY=<your-api-key>
```

Now, let's start the mcp server (sse) by running the following command:

```bash
./target/release/gaia-elastic-mcp-server
```

If start successfully, you will see the following output:

```bash
Starting Gaia Elastic MCP server on 127.0.0.1:8006
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client
./target/release/gaia-elastic-mcp-client --transport sse
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>


```console
2025-05-10T10:05:19.134409Z  INFO gaia_elastic_mcp_client: 101: Connecting to ElasticSearch MCP server via sse
2025-05-10T10:05:19.155704Z  INFO serve_inner: rmcp::service: 531: Service initialized as client peer_info=InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A ElasticSearch MCP server") }
2025-05-10T10:05:19.155832Z  INFO gaia_elastic_mcp_client: 120: Connected to server: InitializeResult {
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
}
2025-05-10T10:05:19.163204Z  INFO gaia_elastic_mcp_client: 124: Available tools:
{
  "tools": [
    {
      "name": "search",
      "description": "Search for documents in an Elasticsearch index",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "type": "string"
          },
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
          }
        },
        "required": [
          "base_url",
          "fields",
          "index",
          "query"
        ],
        "title": "SearchRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "list_indices",
      "description": "List all available Elasticsearch indices",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "type": "string"
          }
        },
        "required": [
          "base_url"
        ],
        "title": "ListIndicesRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "list_aliases",
      "description": "Get the cluster's index aliases, including filter and routing information. Note that this tool does not return data stream aliases.",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "type": "string"
          }
        },
        "required": [
          "base_url"
        ],
        "title": "ListAliasesRequest",
        "type": "object"
      },
      "annotations": null
    }
  ]
}
2025-05-10T10:05:20.753512Z  INFO gaia_elastic_mcp_client: 178: Create index response: {
  "acknowledged": true,
  "index": "paris",
  "shards_acknowledged": true
}
2025-05-10T10:05:21.042373Z  INFO gaia_elastic_mcp_client: 243: Add documents response: {
  "errors": false,
  "items": [
    {
      "index": {
        "_id": "GcSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "GsSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "G8SouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "HMSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "HcSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "HsSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "H8SouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "IMSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "IcSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "IsSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "I8SouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "JMSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "JcSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "JsSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "J8SouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "KMSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "KcSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "KsSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "K8SouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "LMSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
        "_id": "LcSouZYBObs4NrjdD6ti",
        "_index": "paris",
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
2025-05-10T10:05:21.380345Z  INFO gaia_elastic_mcp_client: 270: indices:
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
      "index": "paris",
      "uuid": "Ps8RmGvqTzqae2OJ9-2UPg",
      "pri": "1",
      "rep": "1",
      "docs.count": "21",
      "docs.deleted": "0",
      "store.size": "11.3kb",
      "pri.store.size": "11.3kb",
      "dataset.size": "11.3kb"
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
    }
  ]
}
2025-05-10T10:05:21.662613Z  INFO gaia_elastic_mcp_client: 295: aliases:
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
2025-05-10T10:05:21.995055Z  INFO gaia_elastic_mcp_client: 343: search_result:
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
        "_index": "paris",
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
        "_index": "paris",
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
        "_index": "paris",
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
        "_index": "paris",
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
        "_index": "paris",
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
        "_index": "paris",
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
        "_index": "paris",
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
        "_index": "paris",
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
        "_index": "paris",
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
        "_index": "paris",
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
2025-05-10T10:05:22.687215Z  INFO gaia_elastic_mcp_client: 361: Delete index response: {
  "acknowledged": true
}
2025-05-10T10:05:22.687305Z  INFO rmcp::service: 587: task cancelled
2025-05-10T10:05:22.687320Z  INFO rmcp::service: 755: serve finished quit_reason=Cancelled
```

</details>

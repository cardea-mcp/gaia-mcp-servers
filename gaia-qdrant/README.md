# Gaia Qdrant MCP Server

## Quick Start

### Build and run gaia-qdrant-mcp-server (StreamableHttp)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (streamablehttp)
cargo build --package gaia-qdrant-mcp-server-streamhttp --release

# build mcp client
cargo build --package gaia-qdrant-mcp-client --release
```

**Note** that, before running the mcp server, you need to start the Qdrant server. If you don't have a Qdrant server running, you can start one by running the following command:

```bash
docker pull qdrant/qdrant:latest

docker run -p 6333:6333 -p 6334:6334 \
    -v $(pwd)/qdrant_storage:/qdrant/storage:z \
    qdrant/qdrant
```

Now, let's start the mcp server (tcp) by running the following command:

```bash
# run mcp server (tcp)
./target/release/gaia-qdrant-mcp-server-streamhttp
```

If start successfully, you will see the following output:

```bash
Gaia Qdrant MCP Server is listening on 127.0.0.1:8003
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client
./target/release/gaia-qdrant-mcp-client --transport stream-http
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-05-29T08:57:29.107379Z  INFO gaia_qdrant_mcp_client: 49: Connecting to Gaia Qdrant MCP server via stream-http: http://127.0.0.1:8003/mcp
2025-05-29T08:57:29.122312Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can access the Qdrant database") })
2025-05-29T08:57:29.122369Z  INFO gaia_qdrant_mcp_client: 69: Connected to server: Some(
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
            "A MCP server that can access the Qdrant database",
        ),
    },
)
2025-05-29T08:57:29.129443Z  INFO gaia_qdrant_mcp_client: 73: Available tools: ListToolsResult {
    next_cursor: None,
    tools: [
        Tool {
            name: "upsert_points",
            description: Some(
                "Upsert points into a collection in the Qdrant database",
            ),
            input_schema: {
                "definitions": Object {
                    "Point": Object {
                        "properties": Object {
                            "id": Object {
                                "description": String("the id of the point"),
                                "format": String("uint64"),
                                "minimum": Number(0.0),
                                "type": String("integer"),
                            },
                            "payload": Object {
                                "additionalProperties": Bool(true),
                                "description": String("the payload of the point"),
                                "type": String("object"),
                            },
                            "vector": Object {
                                "description": String("the vector of the point"),
                                "items": Object {
                                    "format": String("float"),
                                    "type": String("number"),
                                },
                                "type": String("array"),
                            },
                        },
                        "required": Array [
                            String("id"),
                            String("payload"),
                            String("vector"),
                        ],
                        "type": String("object"),
                    },
                },
                "properties": Object {
                    "api_key": Object {
                        "description": String("the API key to use for the Qdrant database"),
                        "nullable": Bool(true),
                        "type": String("string"),
                    },
                    "base_url": Object {
                        "description": String("the base URL of the local or remote Qdrant database"),
                        "type": String("string"),
                    },
                    "name": Object {
                        "description": String("the name of the collection to upsert points into"),
                        "type": String("string"),
                    },
                    "points": Object {
                        "description": String("the points to upsert"),
                        "items": Object {
                            "$ref": String("#/components/schemas/Point"),
                        },
                        "type": String("array"),
                    },
                },
                "required": Array [
                    String("base_url"),
                    String("name"),
                    String("points"),
                ],
                "title": String("UpsertPointsRequest"),
                "type": String("object"),
            },
            annotations: None,
        },
        Tool {
            name: "create_collection",
            description: Some(
                "Create a new collection in the Qdrant database",
            ),
            input_schema: {
                "properties": Object {
                    "api_key": Object {
                        "description": String("the API key to use for the Qdrant database"),
                        "nullable": Bool(true),
                        "type": String("string"),
                    },
                    "base_url": Object {
                        "description": String("the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333"),
                        "type": String("string"),
                    },
                    "name": Object {
                        "description": String("the name of the collection to create"),
                        "type": String("string"),
                    },
                    "size": Object {
                        "description": String("the size of the vectors in the collection"),
                        "format": String("uint64"),
                        "minimum": Number(0.0),
                        "type": String("integer"),
                    },
                },
                "required": Array [
                    String("base_url"),
                    String("name"),
                    String("size"),
                ],
                "title": String("CreateCollectionRequest"),
                "type": String("object"),
            },
            annotations: None,
        },
        Tool {
            name: "collection_exists",
            description: Some(
                "Check if a collection exists in the Qdrant database",
            ),
            input_schema: {
                "properties": Object {
                    "api_key": Object {
                        "description": String("the API key to use for the Qdrant database"),
                        "nullable": Bool(true),
                        "type": String("string"),
                    },
                    "base_url": Object {
                        "description": String("the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333"),
                        "type": String("string"),
                    },
                    "name": Object {
                        "description": String("the name of the collection to check"),
                        "type": String("string"),
                    },
                },
                "required": Array [
                    String("base_url"),
                    String("name"),
                ],
                "title": String("CollectionExistsRequest"),
                "type": String("object"),
            },
            annotations: None,
        },
        Tool {
            name: "delete_collection",
            description: Some(
                "Delete a collection in the Qdrant database",
            ),
            input_schema: {
                "properties": Object {
                    "api_key": Object {
                        "description": String("the API key to use for the Qdrant database"),
                        "nullable": Bool(true),
                        "type": String("string"),
                    },
                    "base_url": Object {
                        "description": String("the base URL of the local or remote Qdrant database"),
                        "type": String("string"),
                    },
                    "name": Object {
                        "description": String("the name of the collection to delete"),
                        "type": String("string"),
                    },
                },
                "required": Array [
                    String("base_url"),
                    String("name"),
                ],
                "title": String("DeleteCollectionRequest"),
                "type": String("object"),
            },
            annotations: None,
        },
        Tool {
            name: "search_points",
            description: Some(
                "Search for points in a collection in the Qdrant database",
            ),
            input_schema: {
                "properties": Object {
                    "api_key": Object {
                        "description": String("the API key to use for the Qdrant database"),
                        "nullable": Bool(true),
                        "type": String("string"),
                    },
                    "base_url": Object {
                        "description": String("the base URL of the local or remote Qdrant database"),
                        "type": String("string"),
                    },
                    "limit": Object {
                        "description": String("the number of results to return"),
                        "format": String("uint64"),
                        "minimum": Number(0.0),
                        "type": String("integer"),
                    },
                    "name": Object {
                        "description": String("the name of the collection to search"),
                        "type": String("string"),
                    },
                    "score_threshold": Object {
                        "description": String("the score threshold for the results"),
                        "format": String("float"),
                        "nullable": Bool(true),
                        "type": String("number"),
                    },
                    "vector": Object {
                        "description": String("the vector to search for"),
                        "items": Object {
                            "format": String("float"),
                            "type": String("number"),
                        },
                        "type": String("array"),
                    },
                },
                "required": Array [
                    String("base_url"),
                    String("limit"),
                    String("name"),
                    String("vector"),
                ],
                "title": String("SearchPointsRequest"),
                "type": String("object"),
            },
            annotations: None,
        },
        Tool {
            name: "list_collections",
            description: Some(
                "List all collections in the Qdrant database",
            ),
            input_schema: {
                "properties": Object {
                    "api_key": Object {
                        "description": String("the API key to use for the Qdrant database"),
                        "nullable": Bool(true),
                        "type": String("string"),
                    },
                    "base_url": Object {
                        "description": String("the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333"),
                        "type": String("string"),
                    },
                },
                "required": Array [
                    String("base_url"),
                ],
                "title": String("ListCollectionsRequest"),
                "type": String("object"),
            },
            annotations: None,
        },
    ],
}
2025-05-29T08:57:29.158998Z  INFO gaia_qdrant_mcp_client: 84: collections:
{
  "content": [
    {
      "type": "text",
      "text": "{\"collections\":[\"paris\",\"paris-01\",\"paris-test-04\",\"paris-02\",\"paris-test-05\",\"paris-03\",\"paris-test-03\",\"mcp-test\"],\"time\":0.000643042}"
    }
  ],
  "isError": false
}
2025-05-29T08:57:29.174798Z  INFO gaia_qdrant_mcp_client: 101: collection exists:
{
  "content": [
    {
      "type": "text",
      "text": "{\"result\":true}"
    }
  ],
  "isError": false
}
2025-05-29T08:57:29.174841Z  INFO gaia_qdrant_mcp_client: 111: Exists? true
2025-05-29T08:57:29.259799Z  INFO gaia_qdrant_mcp_client: 129: delete collection:
{
  "content": [
    {
      "type": "text",
      "text": "{\"result\":true,\"time\":0.06854275}"
    }
  ],
  "isError": false
}
2025-05-29T08:57:29.466645Z  INFO gaia_qdrant_mcp_client: 152: create collection:
{
  "content": [
    {
      "type": "text",
      "text": "{\"result\":true,\"time\":0.184629417}"
    }
  ],
  "isError": false
}
2025-05-29T08:57:29.494120Z  INFO gaia_qdrant_mcp_client: 214: upsert points response:
UpsertPointsResponse { status: "acknowledged", time: 0.002446333 }
2025-05-29T08:57:29.505634Z  INFO gaia_qdrant_mcp_client: 243: search points response:
SearchPointsResponse { result: [ScoredPoint { score: 0.99248314, payload: {"city": String("New York")}, vector: [0.15240015, 0.008466675, 0.7196674, 0.677334] }, ScoredPoint { score: 0.89463294, payload: {"city": String("Berlin")}, vector: [0.04082755, 0.4980961, 0.62057877, 0.60424775] }], time: 0.000954291 }
2025-05-29T08:57:29.505695Z  INFO rmcp::service: 625: task cancelled
2025-05-29T08:57:29.505755Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

### Build and run gaia-qdrant-mcp-server (sse)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (sse)
cargo build --package gaia-qdrant-mcp-server-sse --release

# build mcp client
cargo build --package gaia-qdrant-mcp-client --release
```

**Note** that, before running the mcp server, you need to start the Qdrant server. If you don't have a Qdrant server running, you can start one by running the following command:

```bash
docker pull qdrant/qdrant:latest

docker run -p 6333:6333 -p 6334:6334 \
    -v $(pwd)/qdrant_storage:/qdrant/storage:z \
    qdrant/qdrant
```

Now, let's start the mcp server (sse) by running the following command:

```bash
# run mcp server (sse)
./target/release/gaia-qdrant-mcp-server-sse
```

If start successfully, you will see the following output:

```bash
Gaia Qdrant MCP Server is listening on 127.0.0.1:8003
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client
./target/release/gaia-qdrant-mcp-client --transport sse
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```bash
2025-04-27T02:50:48.413818Z  INFO gaia_qdrant_mcp_client: 48: Connecting to Gaia Qdrant MCP server via sse: http://127.0.0.1:8003/sse
2025-04-27T02:50:48.445851Z  INFO serve_inner: rmcp::service: 531: Service initialized as client peer_info=InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can access the Qdrant database") }
2025-04-27T02:50:48.445909Z  INFO gaia_qdrant_mcp_client: 65: Connected to server: InitializeResult {
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
        "A MCP server that can access the Qdrant database",
    ),
}
2025-04-27T02:50:48.448703Z  INFO gaia_qdrant_mcp_client: 69: Available tools:
{
  "tools": [
    {
      "name": "collection_exists",
      "description": "Check if a collection exists in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333",
            "type": "string"
          },
          "name": {
            "description": "the name of the collection to check",
            "type": "string"
          }
        },
        "required": [
          "base_url",
          "name"
        ],
        "title": "CollectionExistsRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "list_collections",
      "description": "List all collections in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333",
            "type": "string"
          }
        },
        "required": [
          "base_url"
        ],
        "title": "ListCollectionsRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "search_points",
      "description": "Search for points in a collection in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database",
            "type": "string"
          },
          "limit": {
            "description": "the number of results to return",
            "format": "uint64",
            "minimum": 0.0,
            "type": "integer"
          },
          "name": {
            "description": "the name of the collection to search",
            "type": "string"
          },
          "score_threshold": {
            "description": "the score threshold for the results",
            "format": "float",
            "type": [
              "number",
              "null"
            ]
          },
          "vector": {
            "description": "the vector to search for",
            "items": {
              "format": "float",
              "type": "number"
            },
            "type": "array"
          }
        },
        "required": [
          "base_url",
          "limit",
          "name",
          "vector"
        ],
        "title": "SearchPointsRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "upsert_points",
      "description": "Upsert points into a collection in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "definitions": {
          "Point": {
            "properties": {
              "id": {
                "description": "the id of the point",
                "format": "uint64",
                "minimum": 0.0,
                "type": "integer"
              },
              "payload": {
                "additionalProperties": true,
                "description": "the payload of the point",
                "type": "object"
              },
              "vector": {
                "description": "the vector of the point",
                "items": {
                  "format": "float",
                  "type": "number"
                },
                "type": "array"
              }
            },
            "required": [
              "id",
              "payload",
              "vector"
            ],
            "type": "object"
          }
        },
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database",
            "type": "string"
          },
          "name": {
            "description": "the name of the collection to upsert points into",
            "type": "string"
          },
          "points": {
            "description": "the points to upsert",
            "items": {
              "$ref": "#/definitions/Point"
            },
            "type": "array"
          }
        },
        "required": [
          "base_url",
          "name",
          "points"
        ],
        "title": "UpsertPointsRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "create_collection",
      "description": "Create a new collection in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333",
            "type": "string"
          },
          "name": {
            "description": "the name of the collection to create",
            "type": "string"
          },
          "size": {
            "description": "the size of the vectors in the collection",
            "format": "uint64",
            "minimum": 0.0,
            "type": "integer"
          }
        },
        "required": [
          "base_url",
          "name",
          "size"
        ],
        "title": "CreateCollectionRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "delete_collection",
      "description": "Delete a collection in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database",
            "type": "string"
          },
          "name": {
            "description": "the name of the collection to delete",
            "type": "string"
          }
        },
        "required": [
          "base_url",
          "name"
        ],
        "title": "DeleteCollectionRequest",
        "type": "object"
      },
      "annotations": null
    }
  ]
}
2025-04-27T02:50:48.478322Z  INFO gaia_qdrant_mcp_client: 83: collections:
{
  "content": [
    {
      "type": "text",
      "text": "{\"collections\":[\"paris-test-03\",\"paris-test-04\",\"paris-test-05\",\"paris\"],\"time\":0.000015583}"
    }
  ],
  "isError": false
}
2025-04-27T02:50:48.528367Z  INFO gaia_qdrant_mcp_client: 100: collection exists:
{
  "content": [
    {
      "type": "text",
      "text": "{\"result\":false}"
    }
  ],
  "isError": false
}
2025-04-27T02:50:48.528376Z  INFO gaia_qdrant_mcp_client: 110: Exists? false
2025-04-27T02:50:48.708608Z  INFO gaia_qdrant_mcp_client: 151: create collection:
{
  "content": [
    {
      "type": "text",
      "text": "{\"result\":true,\"time\":0.171189208}"
    }
  ],
  "isError": false
}
2025-04-27T02:50:48.723148Z  INFO gaia_qdrant_mcp_client: 213: upsert points response:
UpsertPointsResponse { status: "acknowledged", time: 0.002321667 }
2025-04-27T02:50:48.741243Z  INFO gaia_qdrant_mcp_client: 242: search points response:
SearchPointsResponse { result: [ScoredPoint { score: 0.99248314, payload: {"city": String("New York")}, vector: [0.15240015, 0.008466675, 0.7196674, 0.677334] }, ScoredPoint { score: 0.89463294, payload: {"city": String("Berlin")}, vector: [0.04082755, 0.4980961, 0.62057877, 0.60424775] }], time: 0.003648459 }
2025-04-27T02:50:48.741293Z  INFO rmcp::service: 587: task cancelled
2025-04-27T02:50:48.741296Z  INFO rmcp::service: 755: serve finished quit_reason=Cancelled
```

</details>

### Build and run gaia-qdrant-mcp-server (stdio)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (stdio)
cargo build --package gaia-qdrant-mcp-server-stdio --release

# build mcp client
cargo build --package gaia-qdrant-mcp-client --release
```

**Note** that, before running the mcp server, you need to start the Qdrant server. If you don't have a Qdrant server running, you can start one by running the following command:

```bash
docker pull qdrant/qdrant:latest

docker run -p 6333:6333 -p 6334:6334 \
    -v $(pwd)/qdrant_storage:/qdrant/storage:z \
    qdrant/qdrant
```

Now, let's run the mcp client by running the following command. The mcp client will start and call the mcp server.

```bash
# run mcp client
./target/release/gaia-qdrant-mcp-client --transport stdio
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-04-23T06:25:43.010207Z  INFO gaia_qdrant_mcp_client: 227: Connecting to MCP server via stdio
2025-04-23T06:25:43.641505Z  INFO gaia_qdrant_mcp_server_stdio: 17: Starting Gaia Qdrant MCP server in stdio mode
2025-04-23T06:25:43.643518Z  INFO rmcp::handler::server: 214: client initialized
2025-04-23T06:25:43.643560Z  INFO serve_inner: rmcp::service: 531: Service initialized as client peer_info=InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can access the Qdrant database") }
2025-04-23T06:25:43.643665Z  INFO gaia_qdrant_mcp_client: 234: Connected to server
2025-04-23T06:25:43.643645Z  INFO serve_inner: rmcp::service: 533: Service initialized as server peer_info=InitializeRequestParam { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ClientCapabilities { experimental: None, roots: None, sampling: None }, client_info: Implementation { name: "rmcp", version: "0.1.5" } }
2025-04-23T06:25:43.643670Z  INFO gaia_qdrant_mcp_client: 238: Connected to server: InitializeResult {
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
        "A MCP server that can access the Qdrant database",
    ),
}
2025-04-23T06:25:43.643881Z DEBUG rmcp::service: 593: new event evt=PeerMessage(Request(JsonRpcRequest { jsonrpc: JsonRpcVersion2_0, id: Number(1), request: ListToolsRequest(RequestOptionalParam { method: ListToolsRequestMethod, params: Some(PaginatedRequestParam { cursor: None }), extensions: Extensions }) }))
2025-04-23T06:25:43.643895Z  INFO rmcp::service: 659: received request id=1 request=ListToolsRequest(RequestOptionalParam { method: ListToolsRequestMethod, params: Some(PaginatedRequestParam { cursor: None }), extensions: Extensions })
2025-04-23T06:25:43.644229Z  INFO rmcp::service: 677: response message id=1 result=ListToolsResult(ListToolsResult { next_cursor: None, tools: [Tool { name: "collection_exists", description: Some("Check if a collection exists in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333"), "type": String("string")}, "name": Object {"description": String("the name of the collection to check"), "type": String("string")}}, "required": Array [String("base_url"), String("name")], "title": String("CollectionExistsRequest"), "type": String("object")}, annotations: None }, Tool { name: "upsert_points", description: Some("Upsert points into a collection in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "definitions": Object {"Point": Object {"properties": Object {"id": Object {"description": String("the id of the point"), "format": String("uint64"), "minimum": Number(0.0), "type": String("integer")}, "payload": Object {"additionalProperties": Bool(true), "description": String("the payload of the point"), "type": String("object")}, "vector": Object {"description": String("the vector of the point"), "items": Object {"format": String("float"), "type": String("number")}, "type": String("array")}}, "required": Array [String("id"), String("payload"), String("vector")], "type": String("object")}}, "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database"), "type": String("string")}, "name": Object {"description": String("the name of the collection to upsert points into"), "type": String("string")}, "points": Object {"description": String("the points to upsert"), "items": Object {"$ref": String("#/definitions/Point")}, "type": String("array")}}, "required": Array [String("base_url"), String("name"), String("points")], "title": String("UpsertPointsRequest"), "type": String("object")}, annotations: None }, Tool { name: "search_points", description: Some("Search for points in a collection in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database"), "type": String("string")}, "limit": Object {"description": String("the number of results to return"), "format": String("uint64"), "minimum": Number(0.0), "type": String("integer")}, "name": Object {"description": String("the name of the collection to search"), "type": String("string")}, "score_threshold": Object {"description": String("the score threshold for the results"), "format": String("float"), "type": Array [String("number"), String("null")]}, "vector": Object {"description": String("the vector to search for"), "items": Object {"format": String("float"), "type": String("number")}, "type": String("array")}}, "required": Array [String("base_url"), String("limit"), String("name"), String("vector")], "title": String("SearchPointsRequest"), "type": String("object")}, annotations: None }, Tool { name: "create_collection", description: Some("Create a new collection in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333"), "type": String("string")}, "name": Object {"description": String("the name of the collection to create"), "type": String("string")}, "size": Object {"description": String("the size of the vectors in the collection"), "format": String("uint64"), "minimum": Number(0.0), "type": String("integer")}}, "required": Array [String("base_url"), String("name"), String("size")], "title": String("CreateCollectionRequest"), "type": String("object")}, annotations: None }, Tool { name: "list_collections", description: Some("List all collections in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333"), "type": String("string")}}, "required": Array [String("base_url")], "title": String("ListCollectionsRequest"), "type": String("object")}, annotations: None }, Tool { name: "delete_collection", description: Some("Delete a collection in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database"), "type": String("string")}, "name": Object {"description": String("the name of the collection to delete"), "type": String("string")}}, "required": Array [String("base_url"), String("name")], "title": String("DeleteCollectionRequest"), "type": String("object")}, annotations: None }] })
2025-04-23T06:25:43.644477Z DEBUG rmcp::service: 593: new event evt=ToSink(Response(JsonRpcResponse { jsonrpc: JsonRpcVersion2_0, id: Number(1), result: ListToolsResult(ListToolsResult { next_cursor: None, tools: [Tool { name: "collection_exists", description: Some("Check if a collection exists in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333"), "type": String("string")}, "name": Object {"description": String("the name of the collection to check"), "type": String("string")}}, "required": Array [String("base_url"), String("name")], "title": String("CollectionExistsRequest"), "type": String("object")}, annotations: None }, Tool { name: "upsert_points", description: Some("Upsert points into a collection in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "definitions": Object {"Point": Object {"properties": Object {"id": Object {"description": String("the id of the point"), "format": String("uint64"), "minimum": Number(0.0), "type": String("integer")}, "payload": Object {"additionalProperties": Bool(true), "description": String("the payload of the point"), "type": String("object")}, "vector": Object {"description": String("the vector of the point"), "items": Object {"format": String("float"), "type": String("number")}, "type": String("array")}}, "required": Array [String("id"), String("payload"), String("vector")], "type": String("object")}}, "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database"), "type": String("string")}, "name": Object {"description": String("the name of the collection to upsert points into"), "type": String("string")}, "points": Object {"description": String("the points to upsert"), "items": Object {"$ref": String("#/definitions/Point")}, "type": String("array")}}, "required": Array [String("base_url"), String("name"), String("points")], "title": String("UpsertPointsRequest"), "type": String("object")}, annotations: None }, Tool { name: "search_points", description: Some("Search for points in a collection in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database"), "type": String("string")}, "limit": Object {"description": String("the number of results to return"), "format": String("uint64"), "minimum": Number(0.0), "type": String("integer")}, "name": Object {"description": String("the name of the collection to search"), "type": String("string")}, "score_threshold": Object {"description": String("the score threshold for the results"), "format": String("float"), "type": Array [String("number"), String("null")]}, "vector": Object {"description": String("the vector to search for"), "items": Object {"format": String("float"), "type": String("number")}, "type": String("array")}}, "required": Array [String("base_url"), String("limit"), String("name"), String("vector")], "title": String("SearchPointsRequest"), "type": String("object")}, annotations: None }, Tool { name: "create_collection", description: Some("Create a new collection in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333"), "type": String("string")}, "name": Object {"description": String("the name of the collection to create"), "type": String("string")}, "size": Object {"description": String("the size of the vectors in the collection"), "format": String("uint64"), "minimum": Number(0.0), "type": String("integer")}}, "required": Array [String("base_url"), String("name"), String("size")], "title": String("CreateCollectionRequest"), "type": String("object")}, annotations: None }, Tool { name: "list_collections", description: Some("List all collections in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333"), "type": String("string")}}, "required": Array [String("base_url")], "title": String("ListCollectionsRequest"), "type": String("object")}, annotations: None }, Tool { name: "delete_collection", description: Some("Delete a collection in the Qdrant database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"api_key": Object {"description": String("the API key to use for the Qdrant database"), "type": Array [String("string"), String("null")]}, "base_url": Object {"description": String("the base URL of the local or remote Qdrant database"), "type": String("string")}, "name": Object {"description": String("the name of the collection to delete"), "type": String("string")}}, "required": Array [String("base_url"), String("name")], "title": String("DeleteCollectionRequest"), "type": String("object")}, annotations: None }] }) }))
2025-04-23T06:25:43.645048Z  INFO gaia_qdrant_mcp_client: 242: {
  "tools": [
    {
      "name": "collection_exists",
      "description": "Check if a collection exists in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333",
            "type": "string"
          },
          "name": {
            "description": "the name of the collection to check",
            "type": "string"
          }
        },
        "required": [
          "base_url",
          "name"
        ],
        "title": "CollectionExistsRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "upsert_points",
      "description": "Upsert points into a collection in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "definitions": {
          "Point": {
            "properties": {
              "id": {
                "description": "the id of the point",
                "format": "uint64",
                "minimum": 0.0,
                "type": "integer"
              },
              "payload": {
                "additionalProperties": true,
                "description": "the payload of the point",
                "type": "object"
              },
              "vector": {
                "description": "the vector of the point",
                "items": {
                  "format": "float",
                  "type": "number"
                },
                "type": "array"
              }
            },
            "required": [
              "id",
              "payload",
              "vector"
            ],
            "type": "object"
          }
        },
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database",
            "type": "string"
          },
          "name": {
            "description": "the name of the collection to upsert points into",
            "type": "string"
          },
          "points": {
            "description": "the points to upsert",
            "items": {
              "$ref": "#/definitions/Point"
            },
            "type": "array"
          }
        },
        "required": [
          "base_url",
          "name",
          "points"
        ],
        "title": "UpsertPointsRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "search_points",
      "description": "Search for points in a collection in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database",
            "type": "string"
          },
          "limit": {
            "description": "the number of results to return",
            "format": "uint64",
            "minimum": 0.0,
            "type": "integer"
          },
          "name": {
            "description": "the name of the collection to search",
            "type": "string"
          },
          "score_threshold": {
            "description": "the score threshold for the results",
            "format": "float",
            "type": [
              "number",
              "null"
            ]
          },
          "vector": {
            "description": "the vector to search for",
            "items": {
              "format": "float",
              "type": "number"
            },
            "type": "array"
          }
        },
        "required": [
          "base_url",
          "limit",
          "name",
          "vector"
        ],
        "title": "SearchPointsRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "create_collection",
      "description": "Create a new collection in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333",
            "type": "string"
          },
          "name": {
            "description": "the name of the collection to create",
            "type": "string"
          },
          "size": {
            "description": "the size of the vectors in the collection",
            "format": "uint64",
            "minimum": 0.0,
            "type": "integer"
          }
        },
        "required": [
          "base_url",
          "name",
          "size"
        ],
        "title": "CreateCollectionRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "list_collections",
      "description": "List all collections in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database, e.g. http://127.0.0.1:6333",
            "type": "string"
          }
        },
        "required": [
          "base_url"
        ],
        "title": "ListCollectionsRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "delete_collection",
      "description": "Delete a collection in the Qdrant database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "api_key": {
            "description": "the API key to use for the Qdrant database",
            "type": [
              "string",
              "null"
            ]
          },
          "base_url": {
            "description": "the base URL of the local or remote Qdrant database",
            "type": "string"
          },
          "name": {
            "description": "the name of the collection to delete",
            "type": "string"
          }
        },
        "required": [
          "base_url",
          "name"
        ],
        "title": "DeleteCollectionRequest",
        "type": "object"
      },
      "annotations": null
    }
  ]
}
2025-04-23T06:25:43.645392Z DEBUG rmcp::service: 593: new event evt=PeerMessage(Request(JsonRpcRequest { jsonrpc: JsonRpcVersion2_0, id: Number(2), request: CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "list_collections", arguments: Some({"base_url": String("http://127.0.0.1:6333")}) }, extensions: Extensions }) }))
2025-04-23T06:25:43.645404Z  INFO rmcp::service: 659: received request id=2 request=CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "list_collections", arguments: Some({"base_url": String("http://127.0.0.1:6333")}) }, extensions: Extensions })
2025-04-23T06:25:43.652263Z DEBUG reqwest::connect: 625: starting new connection: http://127.0.0.1:6333/
2025-04-23T06:25:43.652304Z DEBUG reqwest::connect: 501: proxy(http://127.0.0.1:7890) intercepts 'http://127.0.0.1:6333/'
2025-04-23T06:25:43.652412Z DEBUG hyper_util::client::legacy::connect::http: 769: connecting to 127.0.0.1:7890
2025-04-23T06:25:43.652672Z DEBUG hyper_util::client::legacy::connect::http: 772: connected to 127.0.0.1:7890
```

</details>

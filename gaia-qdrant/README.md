# Gaia Qdrant MCP Server

## Quick Start

Let's build mcp server and client by running the following commands:

```bash
# build mcp server
cargo build --package gaia-qdrant-mcp-server --release

# build mcp client
cargo build --package gaia-qdrant-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

> [!IMPORTANT]
>
> Before running the mcp server, you need to start the Qdrant server. If you don't have a Qdrant server running, you can start one by running the following command:
>
> ```bash
> docker pull qdrant/qdrant:latest
>
> docker run -p 6333:6333 -p 6334:6334 \
>    -v $(pwd)/qdrant_storage:/qdrant/storage:z \
>    qdrant/qdrant
> ```
>

The CLI options of the mcp server are as follows:

```bash
Usage: gaia-qdrant-mcp-server [OPTIONS]

Options:
      --base-url <BASE_URL>        The base URL of the Qdrant database [default: http://127.0.0.1:6333]
      --api-key <API_KEY>          The API key to use for the Qdrant database
  -s, --socket-addr <SOCKET_ADDR>  Socket address to bind to [default: 127.0.0.1:8003]
  -t, --transport <TRANSPORT>      Transport type to use (sse or stream-http) [default: stream-http] [possible values: stdio, sse, stream-http]
  -h, --help                       Print help
  -V, --version                    Print version
```

Now, let's start the mcp server:

```bash
# run mcp server (stream-http)
./target/release/gaia-qdrant-mcp-server --transport stream-http

# run mcp server (sse)
./target/release/gaia-qdrant-mcp-server --transport sse

# run mcp server (stdio)
./target/release/gaia-qdrant-mcp-server --transport stdio
```

If start successfully, you will see the following output:

```bash
Gaia Qdrant MCP Server is listening on 127.0.0.1:8003
```

The CLI options of the mcp client are as follows:

```bash
Usage: gaia-qdrant-mcp-client [OPTIONS] --collection <COLLECTION>

Options:
  -t, --transport <TRANSPORT>    Transport type to use (tcp or stdio) [default: stream-http] [possible values: stdio, sse, stream-http]
  -c, --collection <COLLECTION>  The name of the collection to use
  -h, --help                     Print help
  -V, --version                  Print version
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client (stream-http)
./target/release/gaia-qdrant-mcp-client --transport stream-http --collection test01

# run mcp client (sse)
./target/release/gaia-qdrant-mcp-client --transport sse --collection test01

# run mcp client (stdio)
./target/release/gaia-qdrant-mcp-client --transport stdio --collection test01
```

If start successfully, you will see the following output. The output is different depending on the transport type you used. The output of `stream-http` is shown below.

<details><summary>Expand to view the output</summary>

```console
2025-06-11T07:24:06.500762Z  INFO gaia_qdrant_mcp_client: 50: Connecting to Gaia Qdrant MCP server via stream-http: http://127.0.0.1:8003/mcp
2025-06-11T07:24:06.513298Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can access the Qdrant database") })
2025-06-11T07:24:06.513322Z  INFO gaia_qdrant_mcp_client: 70: Connected to server: Some(
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
2025-06-11T07:24:06.515112Z  INFO gaia_qdrant_mcp_client: 74: Available tools: ListToolsResult {
    next_cursor: None,
    tools: [
        Tool {
            name: "create_collection",
            description: Some(
                "Create a new collection in the Qdrant database",
            ),
            input_schema: {
                "properties": Object {
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
                    String("name"),
                    String("size"),
                ],
                "title": String("CreateCollectionRequest"),
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
                "title": String("EmptyObject"),
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
                    "name": Object {
                        "description": String("the name of the collection to delete"),
                        "type": String("string"),
                    },
                },
                "required": Array [
                    String("name"),
                ],
                "title": String("DeleteCollectionRequest"),
                "type": String("object"),
            },
            annotations: None,
        },
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
                    String("name"),
                    String("points"),
                ],
                "title": String("UpsertPointsRequest"),
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
                    "name": Object {
                        "description": String("the name of the collection to check"),
                        "type": String("string"),
                    },
                },
                "required": Array [
                    String("name"),
                ],
                "title": String("CollectionExistsRequest"),
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
                    String("limit"),
                    String("name"),
                    String("vector"),
                ],
                "title": String("SearchPointsRequest"),
                "type": String("object"),
            },
            annotations: None,
        },
    ],
}
2025-06-11T07:24:06.530841Z  INFO gaia_qdrant_mcp_client: 82: collections:
{
  "content": [
    {
      "type": "text",
      "text": "{\"collections\":[\"paris-test-03\",\"test01\",\"mcp-test\",\"paris-01\",\"paris\",\"paris-test-05\",\"xibei-01\",\"paris-03\",\"paris-test-04\",\"paris-02\"],\"time\":0.000369875}"
    }
  ],
  "isError": false
}
2025-06-11T07:24:06.555066Z  INFO gaia_qdrant_mcp_client: 93: collection exists:
{
  "content": [
    {
      "type": "text",
      "text": "{\"result\":true}"
    }
  ],
  "isError": false
}
2025-06-11T07:24:06.555103Z  INFO gaia_qdrant_mcp_client: 103: Exists? true
2025-06-11T07:24:06.612071Z  INFO gaia_qdrant_mcp_client: 121: delete collection:
{
  "content": [
    {
      "type": "text",
      "text": "{\"result\":true,\"time\":0.048636042}"
    }
  ],
  "isError": false
}
2025-06-11T07:24:06.830689Z  INFO gaia_qdrant_mcp_client: 140: create collection:
{
  "content": [
    {
      "type": "text",
      "text": "{\"result\":true,\"time\":0.199637375}"
    }
  ],
  "isError": false
}
2025-06-11T07:24:06.846357Z  INFO gaia_qdrant_mcp_client: 202: upsert points response:
UpsertPointsResponse { status: "acknowledged", time: 0.000574917 }
2025-06-11T07:24:06.853712Z  INFO gaia_qdrant_mcp_client: 226: search points response:
SearchPointsResponse { result: [ScoredPoint { score: 0.99248314, payload: {"city": String("New York")}, vector: [0.15240015, 0.008466675, 0.7196674, 0.677334] }, ScoredPoint { score: 0.89463294, payload: {"city": String("Berlin")}, vector: [0.04082755, 0.4980961, 0.62057877, 0.60424775] }], time: 0.0008795 }
2025-06-11T07:24:06.853744Z  INFO rmcp::service: 625: task cancelled
2025-06-11T07:24:06.853785Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

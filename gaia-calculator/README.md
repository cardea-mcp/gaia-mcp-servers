# Gaia Calculator MCP Server

## Quick Start

### Build and run gaia-calculator-mcp-server (StreamableHttp)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (streamablehttp)
cargo build --package gaia-calculator-mcp-server-streamhttp --release

# build mcp client
cargo build --package gaia-calculator-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

Now, let's start the mcp server (streamablehttp) by running the following command:

```bash
# run mcp server (streamablehttp)
./target/release/gaia-calculator-mcp-server
```

If start successfully, you will see the following output:

```bash
Calculator MCP server is listening on 127.0.0.1:8001
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client
./target/release/gaia-calculator-mcp-client --transport stream-http
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-05-29T04:28:57.019045Z  INFO gaia_calculator_mcp_client: 143: Connecting to Gaia Calculator MCP server via stream-http: http://127.0.0.1:8001/mcp
2025-05-29T04:28:57.022345Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A simple calculator") })
2025-05-29T04:28:57.022385Z  INFO gaia_calculator_mcp_client: 163: Connected to server: Some(
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
            "A simple calculator",
        ),
    },
)
2025-05-29T04:28:57.023462Z  INFO gaia_calculator_mcp_client: 167: Available tools: ListToolsResult {
    next_cursor: None,
    tools: [
        Tool {
            name: "sub",
            description: Some(
                "Calculate the difference of two numbers",
            ),
            input_schema: {
                "properties": Object {
                    "a": Object {
                        "description": String("the left hand side number"),
                        "format": String("int32"),
                        "type": String("integer"),
                    },
                    "b": Object {
                        "description": String("the right hand side number"),
                        "format": String("int32"),
                        "type": String("integer"),
                    },
                },
                "required": Array [
                    String("a"),
                    String("b"),
                ],
                "title": String("__SUBToolCallParam"),
                "type": String("object"),
            },
            annotations: None,
        },
        Tool {
            name: "sum",
            description: Some(
                "Calculate the sum of two numbers",
            ),
            input_schema: {
                "properties": Object {
                    "a": Object {
                        "description": String("the left hand side number"),
                        "format": String("int32"),
                        "type": String("integer"),
                    },
                    "b": Object {
                        "description": String("the right hand side number"),
                        "format": String("int32"),
                        "type": String("integer"),
                    },
                },
                "required": Array [
                    String("a"),
                    String("b"),
                ],
                "title": String("SumRequest"),
                "type": String("object"),
            },
            annotations: None,
        },
    ],
}
2025-05-29T04:28:57.024426Z  INFO gaia_calculator_mcp_client: 180: Sum result: {
  "content": [
    {
      "type": "text",
      "text": "3"
    }
  ],
  "isError": false
}
2025-05-29T04:28:57.024471Z  INFO rmcp::service: 625: task cancelled
2025-05-29T04:28:57.024494Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

### Build and run gaia-calculator-mcp-server (stdio)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (tcp)
cargo build --package gaia-calculator-mcp-server-stdio --release

# build mcp client
cargo build --package gaia-calculator-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

Now, let's run the mcp client by running the following command. The mcp client will start and call the mcp server.

```bash
# run mcp client
./target/release/gaia-calculator-mcp-client --transport stdio
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-04-23T05:49:07.876387Z  INFO gaia_calculator_mcp_client: 66: Connecting to MCP server via stdio
2025-04-23T05:49:08.462289Z  INFO gaia_calculator_mcp_server_stdio: Starting MCP server
2025-04-23T05:49:08.464503Z  INFO rmcp::handler::server: client initialized
2025-04-23T05:49:08.464531Z  INFO serve_inner: rmcp::service: 531: Service initialized as client peer_info=InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A simple calculator") }
2025-04-23T05:49:08.464565Z  INFO gaia_calculator_mcp_client: 76: Connected to server: InitializeResult {
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
        "A simple calculator",
    ),
}
2025-04-23T05:49:08.464556Z  INFO serve_inner: rmcp::service: Service initialized as server peer_info=InitializeRequestParam { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ClientCapabilities { experimental: None, roots: None, sampling: None }, client_info: Implementation { name: "rmcp", version: "0.1.5" } }
2025-04-23T05:49:08.464930Z DEBUG rmcp::service: new event evt=PeerMessage(Request(JsonRpcRequest { jsonrpc: JsonRpcVersion2_0, id: Number(1), request: ListToolsRequest(RequestOptionalParam { method: ListToolsRequestMethod, params: Some(PaginatedRequestParam { cursor: None }), extensions: Extensions }) }))
2025-04-23T05:49:08.464976Z  INFO rmcp::service: received request id=1 request=ListToolsRequest(RequestOptionalParam { method: ListToolsRequestMethod, params: Some(PaginatedRequestParam { cursor: None }), extensions: Extensions })
2025-04-23T05:49:08.465142Z  INFO rmcp::service: response message id=1 result=ListToolsResult(ListToolsResult { next_cursor: None, tools: [Tool { name: "sub", description: Some("Calculate the difference of two numbers"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"a": Object {"description": String("the left hand side number"), "format": String("int32"), "type": String("integer")}, "b": Object {"description": String("the right hand side number"), "format": String("int32"), "type": String("integer")}}, "required": Array [String("a"), String("b")], "title": String("__SUBToolCallParam"), "type": String("object")}, annotations: None }, Tool { name: "sum", description: Some("Calculate the sum of two numbers"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"a": Object {"description": String("the left hand side number"), "format": String("int32"), "type": String("integer")}, "b": Object {"description": String("the right hand side number"), "format": String("int32"), "type": String("integer")}}, "required": Array [String("a"), String("b")], "title": String("SumRequest"), "type": String("object")}, annotations: None }] })
2025-04-23T05:49:08.465211Z DEBUG rmcp::service: new event evt=ToSink(Response(JsonRpcResponse { jsonrpc: JsonRpcVersion2_0, id: Number(1), result: ListToolsResult(ListToolsResult { next_cursor: None, tools: [Tool { name: "sub", description: Some("Calculate the difference of two numbers"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"a": Object {"description": String("the left hand side number"), "format": String("int32"), "type": String("integer")}, "b": Object {"description": String("the right hand side number"), "format": String("int32"), "type": String("integer")}}, "required": Array [String("a"), String("b")], "title": String("__SUBToolCallParam"), "type": String("object")}, annotations: None }, Tool { name: "sum", description: Some("Calculate the sum of two numbers"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"a": Object {"description": String("the left hand side number"), "format": String("int32"), "type": String("integer")}, "b": Object {"description": String("the right hand side number"), "format": String("int32"), "type": String("integer")}}, "required": Array [String("a"), String("b")], "title": String("SumRequest"), "type": String("object")}, annotations: None }] }) }))
2025-04-23T05:49:08.465378Z  INFO gaia_calculator_mcp_client: 80: {
  "tools": [
    {
      "name": "sub",
      "description": "Calculate the difference of two numbers",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "a": {
            "description": "the left hand side number",
            "format": "int32",
            "type": "integer"
          },
          "b": {
            "description": "the right hand side number",
            "format": "int32",
            "type": "integer"
          }
        },
        "required": [
          "a",
          "b"
        ],
        "title": "__SUBToolCallParam",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "sum",
      "description": "Calculate the sum of two numbers",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "a": {
            "description": "the left hand side number",
            "format": "int32",
            "type": "integer"
          },
          "b": {
            "description": "the right hand side number",
            "format": "int32",
            "type": "integer"
          }
        },
        "required": [
          "a",
          "b"
        ],
        "title": "SumRequest",
        "type": "object"
      },
      "annotations": null
    }
  ]
}
2025-04-23T05:49:08.465476Z DEBUG rmcp::service: new event evt=PeerMessage(Request(JsonRpcRequest { jsonrpc: JsonRpcVersion2_0, id: Number(2), request: CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "sum", arguments: Some({"a": Number(1), "b": Number(2)}) }, extensions: Extensions }) }))
2025-04-23T05:49:08.465489Z  INFO rmcp::service: received request id=2 request=CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "sum", arguments: Some({"a": Number(1), "b": Number(2)}) }, extensions: Extensions })
2025-04-23T05:49:08.465507Z  INFO rmcp::service: response message id=2 result=CallToolResult(CallToolResult { content: [Annotated { raw: Text(RawTextContent { text: "3" }), annotations: None }], is_error: Some(false) })
2025-04-23T05:49:08.465516Z DEBUG rmcp::service: new event evt=ToSink(Response(JsonRpcResponse { jsonrpc: JsonRpcVersion2_0, id: Number(2), result: CallToolResult(CallToolResult { content: [Annotated { raw: Text(RawTextContent { text: "3" }), annotations: None }], is_error: Some(false) }) }))
2025-04-23T05:49:08.465636Z  INFO gaia_calculator_mcp_client: 93: Sum result: {
  "content": [
    {
      "type": "text",
      "text": "3"
    }
  ],
  "isError": false
}
2025-04-23T05:49:08.465656Z  INFO rmcp::service: 587: task cancelled
2025-04-23T05:49:08.465665Z  INFO rmcp::service: 755: serve finished quit_reason=Cancelled
```

</details>

### Build and run gaia-calculator-mcp-server (sse)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (sse)
cargo build --package gaia-calculator-mcp-server-sse --release

# build mcp client
cargo build --package gaia-calculator-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

Now, let's start the mcp server (sse) by running the following command:

```bash
# run mcp server (sse)
./target/release/gaia-calculator-mcp-server-sse
```

If start successfully, you will see the following output:

```bash
Starting Gaia Calculator MCP server on 127.0.0.1:8001
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client
./target/release/gaia-calculator-mcp-client --transport sse
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-04-25T09:16:51.105070Z  INFO gaia_calculator_mcp_client: 44: Connecting to MCP server via sse: http://127.0.0.1:8001/sse
2025-04-25T09:16:51.122807Z  INFO serve_inner: rmcp::service: 531: Service initialized as client peer_info=InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A simple calculator") }
2025-04-25T09:16:51.122856Z  INFO gaia_calculator_mcp_client: 61: Connected to server: InitializeResult {
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
        "A simple calculator",
    ),
}
2025-04-25T09:16:51.127883Z  INFO gaia_calculator_mcp_client: 65: Available tools: {
  "tools": [
    {
      "name": "sub",
      "description": "Calculate the difference of two numbers",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "a": {
            "description": "the left hand side number",
            "format": "int32",
            "type": "integer"
          },
          "b": {
            "description": "the right hand side number",
            "format": "int32",
            "type": "integer"
          }
        },
        "required": [
          "a",
          "b"
        ],
        "title": "__SUBToolCallParam",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "sum",
      "description": "Calculate the sum of two numbers",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "a": {
            "description": "the left hand side number",
            "format": "int32",
            "type": "integer"
          },
          "b": {
            "description": "the right hand side number",
            "format": "int32",
            "type": "integer"
          }
        },
        "required": [
          "a",
          "b"
        ],
        "title": "SumRequest",
        "type": "object"
      },
      "annotations": null
    }
  ]
}
2025-04-25T09:16:51.131202Z  INFO gaia_calculator_mcp_client: 78: Sum result: {
  "content": [
    {
      "type": "text",
      "text": "3"
    }
  ],
  "isError": false
}
```

</details>

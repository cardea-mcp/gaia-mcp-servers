# Gaia Calculator MCP Server

## Quick Start

### Build and run gaia-calculator-mcp-server

Let's build mcp server and client by running the following commands:

```bash
# build mcp server
cargo build --package gaia-calculator-mcp-server --release

# build mcp client
cargo build --package gaia-calculator-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

Now, let's start the mcp server. You can choose to start the server with different transport types by specifying the `--transport` CLI option. The default transport type is `stream-http`. In addition, you can also specify the socket address to bind to by specifying the `--socket-addr` CLI option. The default socket address is `127.0.0.1:8001`.

```bash
# run mcp server (stream-http)
./target/release/gaia-calculator-mcp-server --transport stream-http

# run mcp server (sse)
./target/release/gaia-calculator-mcp-server --transport sse

# run mcp server (stdio)
./target/release/gaia-calculator-mcp-server --transport stdio
```

If start successfully, you will see the following output:

```bash
Calculator MCP server is listening on 127.0.0.1:8001
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client (stream-http)
./target/release/gaia-calculator-mcp-client --transport stream-http

# run mcp client (sse)
./target/release/gaia-calculator-mcp-client --transport sse

# run mcp client (stdio)
./target/release/gaia-calculator-mcp-client --transport stdio
```

If start successfully, you will see the following output. The output is different depending on the transport type you used. The output of `stream-http` is shown below.

<details><summary>Expand to view the output</summary>

```console
2025-06-11T03:49:39.463041Z  INFO gaia_calculator_mcp_client: 114: Connecting to Gaia Calculator MCP server via stream-http: http://127.0.0.1:8001/mcp
2025-06-11T03:49:39.480584Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A simple calculator") })
2025-06-11T03:49:39.480605Z  INFO gaia_calculator_mcp_client: 134: Connected to server: Some(
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
2025-06-11T03:49:39.481931Z  INFO gaia_calculator_mcp_client: 138: Available tools: ListToolsResult {
    next_cursor: None,
    tools: [
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
    ],
}
2025-06-11T03:49:39.482298Z  INFO gaia_calculator_mcp_client: 151: Sum result: {
  "content": [
    {
      "type": "text",
      "text": "3"
    }
  ],
  "isError": false
}
2025-06-11T03:49:39.482310Z  INFO rmcp::service: 625: task cancelled
2025-06-11T03:49:39.482320Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

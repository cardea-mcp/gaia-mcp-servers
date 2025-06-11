# Gaia Weather MCP Server

## Quick Start

### Build mcp server and client

Let's build mcp server and client by running the following commands:

```bash
# build mcp server
cargo build --package gaia-weather-mcp-server --release

# build mcp client
cargo build --package gaia-weather-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

### Start mcp server

> [!IMPORTANT]
>
> The mcp server will use the `OPENWEATHERMAP_API_KEY` environment variable to get the weather data from [OpenWeatherMap.org](https://openweathermap.org/). If you don't have an API key, you **SHOULD** apply one from [OpenWeatherMap.org](https://openweathermap.org/) and set it by running the following command:
>
> ```bash
> export OPENWEATHERMAP_API_KEY=<your-api-key>
> ```

Now, let's start the mcp server. You can choose to start the server with different transport types by specifying the `--transport` CLI option. The default transport type is `stream-http`. In addition, you can also specify the socket address to bind to by specifying the `--socket-addr` CLI option. The default socket address is `127.0.0.1:8002`.

```bash
# run mcp server (stream-http)
./target/release/gaia-weather-mcp-server --transport stream-http

# run mcp server (sse)
./target/release/gaia-weather-mcp-server --transport sse

# run mcp server (stdio)
./target/release/gaia-weather-mcp-server --transport stdio
```

If start successfully, you will see the following output:

```bash
Gaia Weather MCP Server is listening on 127.0.0.1:8002
```

### Run mcp client

Now, let's run the mcp client by running the following command:

```bash
# run mcp client (stream-http)
./target/release/gaia-weather-mcp-client --transport stream-http

# run mcp client (sse)
./target/release/gaia-weather-mcp-client --transport sse

# run mcp client (stdio)
./target/release/gaia-weather-mcp-client --transport stdio
```

If start successfully, you will see the following output. The output is different depending on the transport type you used. The output of `stream-http` is shown below.

<details><summary>Expand to view the output</summary>

```console
2025-06-11T06:37:03.579902Z  INFO gaia_weather_mcp_client: 151: Connecting to Gaia Weather MCP server via stream-http: http://127.0.0.1:8002/mcp
2025-06-11T06:37:03.592281Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can get the weather for a given city") })
2025-06-11T06:37:03.592321Z  INFO gaia_weather_mcp_client: 171: Connected to server: Some(
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
            "A MCP server that can get the weather for a given city",
        ),
    },
)
2025-06-11T06:37:03.593352Z  INFO gaia_weather_mcp_client: 175: Available tools: {
  "tools": [
    {
      "name": "get_current_weather",
      "description": "Get the weather for a given city",
      "inputSchema": {
        "definitions": {
          "TemperatureUnit": {
            "enum": [
              "celsius",
              "fahrenheit"
            ],
            "type": "string"
          }
        },
        "properties": {
          "api_key": {
            "default": null,
            "description": "the OpenWeatherMap API key to use. If not provided, the server will use the OPENWEATHERMAP_API_KEY environment variable.",
            "nullable": true,
            "type": "string"
          },
          "location": {
            "description": "the city to get the weather for, e.g., 'Beijing', 'New York', 'Tokyo'",
            "type": "string"
          },
          "unit": {
            "$ref": "#/components/schemas/TemperatureUnit",
            "description": "the unit to use for the temperature, e.g., 'celsius', 'fahrenheit'"
          }
        },
        "required": [
          "location",
          "unit"
        ],
        "title": "GetWeatherRequest",
        "type": "object"
      }
    }
  ]
}
2025-06-11T06:37:04.246599Z  INFO gaia_weather_mcp_client: 203: Weather result: {
  "content": [
    {
      "type": "text",
      "text": "{\"temperature\":34.94,\"unit\":\"celsius\"}"
    }
  ],
  "isError": false
}
2025-06-11T06:37:04.246688Z  INFO rmcp::service: 625: task cancelled
2025-06-11T06:37:04.246840Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

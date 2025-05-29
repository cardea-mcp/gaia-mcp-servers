# Gaia Weather MCP Server

## Quick Start

### Build and run gaia-weather-mcp-server (StreamableHttp)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (streamablehttp)
cargo build --package gaia-weather-mcp-server-streamhttp --release

# build mcp client
cargo build --package gaia-weather-mcp-client --release
```

**Note** that the mcp server will use the `OPENWEATHERMAP_API_KEY` environment variable to get the weather data from [OpenWeatherMap.org](https://openweathermap.org/). If you don't have an API key, you **SHOULD** apply one from [OpenWeatherMap.org](https://openweathermap.org/) and set it by running the following command:

```bash
export OPENWEATHERMAP_API_KEY=<your-api-key>
```

Now, let's start the mcp server (streamablehttp) by running the following command:

```bash
# run mcp server (streamablehttp)
./target/release/gaia-weather-mcp-server
```

If start successfully, you will see the following output:

```bash
Gaia Weather MCP Server is listening on 127.0.0.1:8002
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client
./target/release/gaia-weather-mcp-client --transport stream-http
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-05-29T07:26:01.874406Z  INFO gaia_weather_mcp_client: 198: Connecting to Gaia Weather MCP server via stream-http: http://127.0.0.1:8002/mcp
2025-05-29T07:26:01.891987Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can get the weather for a given city") })
2025-05-29T07:26:01.892015Z  INFO gaia_weather_mcp_client: 218: Connected to server: Some(
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
2025-05-29T07:26:01.897628Z  INFO gaia_weather_mcp_client: 222: Available tools: {
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
2025-05-29T07:26:02.750843Z  INFO gaia_weather_mcp_client: 250: Weather result: {
  "content": [
    {
      "type": "text",
      "text": "{\"temperature\":31.94,\"unit\":\"celsius\"}"
    }
  ],
  "isError": false
}
2025-05-29T07:26:02.750926Z  INFO rmcp::service: 625: task cancelled
2025-05-29T07:26:02.751031Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

### Build and run gaia-weather-mcp-server (stdio)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (stdio)
cargo build --package gaia-weather-mcp-server-stdio --release

# build mcp client
cargo build --package gaia-weather-mcp-client --release
```

**Note** that the mcp server will use the `OPENWEATHERMAP_API_KEY` environment variable to get the weather data from [OpenWeatherMap.org](https://openweathermap.org/). If you don't have an API key, you **SHOULD** apply one from [OpenWeatherMap.org](https://openweathermap.org/) and set it by running the following command:

```bash
export OPENWEATHERMAP_API_KEY=<your-api-key>
```

Now, let's run the mcp client by running the following command. The mcp client will start and call the mcp server.

```bash
# run mcp client
./target/release/gaia-weather-mcp-client --transport stdio
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-04-23T06:06:50.773006Z  INFO gaia_weather_mcp_client: 80: Connecting to MCP server via stdio
2025-04-23T06:06:51.111970Z  INFO gaia_weather_mcp_server_stdio: Starting Gaia Weather MCP server in stdio mode
2025-04-23T06:06:51.113783Z  INFO rmcp::handler::server: client initialized
2025-04-23T06:06:51.113784Z  INFO serve_inner: rmcp::service: 531: Service initialized as client peer_info=InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can get the weather for a given city") }
2025-04-23T06:06:51.113840Z  INFO gaia_weather_mcp_client: 90: Connected to server: InitializeResult {
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
                2025-04-23T06:06:51.113860Z  INFO serve_inner: rmcp::service: Service initialized as server peer_info=InitializeRequestParam { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ClientCapabilities { experimental: None, roots: None, sampling: None }, client_info: Implementation { name: "rmcp", version: "0.1.5" } }
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
}
2025-04-23T06:06:51.114131Z DEBUG rmcp::service: new event evt=PeerMessage(Request(JsonRpcRequest { jsonrpc: JsonRpcVersion2_0, id: Number(1), request: ListToolsRequest(RequestOptionalParam { method: ListToolsRequestMethod, params: Some(PaginatedRequestParam { cursor: None }), extensions: Extensions }) }))
2025-04-23T06:06:51.114139Z  INFO rmcp::service: received request id=1 request=ListToolsRequest(RequestOptionalParam { method: ListToolsRequestMethod, params: Some(PaginatedRequestParam { cursor: None }), extensions: Extensions })
2025-04-23T06:06:51.114228Z  INFO rmcp::service: response message id=1 result=ListToolsResult(ListToolsResult { next_cursor: None, tools: [Tool { name: "get_current_weather", description: Some("Get the weather for a given city"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "definitions": Object {"TemperatureUnit": Object {"enum": Array [String("celsius"), String("fahrenheit")], "type": String("string")}}, "properties": Object {"api_key": Object {"default": Null, "description": String("the OpenWeatherMap API key to use. If not provided, the server will use the OPENWEATHERMAP_API_KEY environment variable."), "type": Array [String("string"), String("null")]}, "location": Object {"description": String("the city to get the weather for, e.g., 'Beijing', 'New York', 'Tokyo'"), "type": String("string")}, "unit": Object {"allOf": Array [Object {"$ref": String("#/definitions/TemperatureUnit")}], "description": String("the unit to use for the temperature, e.g., 'celsius', 'fahrenheit'")}}, "required": Array [String("location"), String("unit")], "title": String("GetWeatherRequest"), "type": String("object")}, annotations: None }] })
2025-04-23T06:06:51.114273Z DEBUG rmcp::service: new event evt=ToSink(Response(JsonRpcResponse { jsonrpc: JsonRpcVersion2_0, id: Number(1), result: ListToolsResult(ListToolsResult { next_cursor: None, tools: [Tool { name: "get_current_weather", description: Some("Get the weather for a given city"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "definitions": Object {"TemperatureUnit": Object {"enum": Array [String("celsius"), String("fahrenheit")], "type": String("string")}}, "properties": Object {"api_key": Object {"default": Null, "description": String("the OpenWeatherMap API key to use. If not provided, the server will use the OPENWEATHERMAP_API_KEY environment variable."), "type": Array [String("string"), String("null")]}, "location": Object {"description": String("the city to get the weather for, e.g., 'Beijing', 'New York', 'Tokyo'"), "type": String("string")}, "unit": Object {"allOf": Array [Object {"$ref": String("#/definitions/TemperatureUnit")}], "description": String("the unit to use for the temperature, e.g., 'celsius', 'fahrenheit'")}}, "required": Array [String("location"), String("unit")], "title": String("GetWeatherRequest"), "type": String("object")}, annotations: None }] }) }))
2025-04-23T06:06:51.114418Z  INFO gaia_weather_mcp_client: 94: {
  "tools": [
    {
      "name": "get_current_weather",
      "description": "Get the weather for a given city",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
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
            "type": [
              "string",
              "null"
            ]
          },
          "location": {
            "description": "the city to get the weather for, e.g., 'Beijing', 'New York', 'Tokyo'",
            "type": "string"
          },
          "unit": {
            "allOf": [
              {
                "$ref": "#/definitions/TemperatureUnit"
              }
            ],
            "description": "the unit to use for the temperature, e.g., 'celsius', 'fahrenheit'"
          }
        },
        "required": [
          "location",
          "unit"
        ],
        "title": "GetWeatherRequest",
        "type": "object"
      },
      "annotations": null
    }
  ]
}
2025-04-23T06:06:51.114541Z DEBUG rmcp::service: new event evt=PeerMessage(Request(JsonRpcRequest { jsonrpc: JsonRpcVersion2_0, id: Number(2), request: CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "get_current_weather", arguments: Some({"api_key": String("09a55b004ce2f065b903015e3284de35"), "location": String("Beijing"), "unit": String("celsius")}) }, extensions: Extensions }) }))
2025-04-23T06:06:51.114549Z  INFO rmcp::service: received request id=2 request=CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "get_current_weather", arguments: Some({"api_key": String("09a55b004ce2f065b903015e3284de35"), "location": String("Beijing"), "unit": String("celsius")}) }, extensions: Extensions })
2025-04-23T06:06:51.114561Z  INFO gaia_weather_mcp_server_stdio::weather: getting geocode for Beijing
2025-04-23T06:06:51.119731Z DEBUG reqwest::connect: starting new connection: http://api.openweathermap.org/
2025-04-23T06:06:51.119779Z DEBUG reqwest::connect: proxy(http://127.0.0.1:7890) intercepts 'http://api.openweathermap.org/'
2025-04-23T06:06:51.119811Z DEBUG hyper_util::client::legacy::connect::http: connecting to 127.0.0.1:7890
2025-04-23T06:06:51.120052Z DEBUG hyper_util::client::legacy::connect::http: connected to 127.0.0.1:7890
2025-04-23T06:06:51.619376Z  INFO gaia_weather_mcp_server_stdio::weather: getting weather for Beijing at 39.906217 116.3912757
2025-04-23T06:06:51.619684Z DEBUG reqwest::connect: starting new connection: http://api.openweathermap.org/
2025-04-23T06:06:51.619694Z DEBUG reqwest::connect: proxy(http://127.0.0.1:7890) intercepts 'http://api.openweathermap.org/'
2025-04-23T06:06:51.619701Z DEBUG hyper_util::client::legacy::connect::http: connecting to 127.0.0.1:7890
2025-04-23T06:06:51.619881Z DEBUG hyper_util::client::legacy::connect::http: connected to 127.0.0.1:7890
2025-04-23T06:06:51.913855Z  INFO gaia_weather_mcp_server_stdio::weather: temperature: 26.91
2025-04-23T06:06:51.913887Z  INFO gaia_weather_mcp_server_stdio::weather: description: clear sky
2025-04-23T06:06:51.913961Z  INFO rmcp::service: response message id=2 result=CallToolResult(CallToolResult { content: [Annotated { raw: Text(RawTextContent { text: "{\"temperature\":26.91,\"unit\":\"celsius\"}" }), annotations: None }], is_error: Some(false) })
2025-04-23T06:06:51.913977Z DEBUG rmcp::service: new event evt=ToSink(Response(JsonRpcResponse { jsonrpc: JsonRpcVersion2_0, id: Number(2), result: CallToolResult(CallToolResult { content: [Annotated { raw: Text(RawTextContent { text: "{\"temperature\":26.91,\"unit\":\"celsius\"}" }), annotations: None }], is_error: Some(false) }) }))
2025-04-23T06:06:51.914232Z  INFO gaia_weather_mcp_client: 122: Sum result: {
  "content": [
    {
      "type": "text",
      "text": "{\"temperature\":26.91,\"unit\":\"celsius\"}"
    }
  ],
  "isError": false
}
2025-04-23T06:06:51.914278Z  INFO rmcp::service: 587: task cancelled
2025-04-23T06:06:51.914284Z  INFO rmcp::service: 755: serve finished quit_reason=Cancelled
```

</details>

### Build and run gaia-weather-mcp-server (sse)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (sse)
cargo build --package gaia-weather-mcp-server-sse --release

# build mcp client
cargo build --package gaia-weather-mcp-client --release
```

**Note** that the mcp server will use the `OPENWEATHERMAP_API_KEY` environment variable to get the weather data from [OpenWeatherMap.org](https://openweathermap.org/). If you don't have an API key, you **SHOULD** apply one from [OpenWeatherMap.org](https://openweathermap.org/) and set it by running the following command:

```bash
export OPENWEATHERMAP_API_KEY=<your-api-key>
```

Now, let's start the mcp server (sse) by running the following command:

```bash
# run mcp server (sse)
./target/release/gaia-weather-mcp-server-sse
```

If start successfully, you will see the following output:

```bash
Starting Gaia Weather MCP server on 127.0.0.1:8002
```

Now, let's run the mcp client by running the following command. The mcp client will start and call the mcp server.

```bash
# run mcp client
./target/release/gaia-weather-mcp-client --transport sse
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-04-27T02:22:41.600277Z  INFO gaia_weather_mcp_client: 43: Connecting to Gaia Weather MCP server via sse: http://127.0.0.1:8002/sse
2025-04-27T02:22:41.634839Z  INFO serve_inner: rmcp::service: 531: Service initialized as client peer_info=InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can get the weather for a given city") }
2025-04-27T02:22:41.634931Z  INFO gaia_weather_mcp_client: 60: Connected to server: InitializeResult {
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
}
2025-04-27T02:22:41.637600Z  INFO gaia_weather_mcp_client: 64: Available tools: {
  "tools": [
    {
      "name": "get_current_weather",
      "description": "Get the weather for a given city",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
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
            "type": [
              "string",
              "null"
            ]
          },
          "location": {
            "description": "the city to get the weather for, e.g., 'Beijing', 'New York', 'Tokyo'",
            "type": "string"
          },
          "unit": {
            "allOf": [
              {
                "$ref": "#/definitions/TemperatureUnit"
              }
            ],
            "description": "the unit to use for the temperature, e.g., 'celsius', 'fahrenheit'"
          }
        },
        "required": [
          "location",
          "unit"
        ],
        "title": "GetWeatherRequest",
        "type": "object"
      },
      "annotations": null
    }
  ]
}
2025-04-27T02:22:42.462484Z  INFO gaia_weather_mcp_client: 92: Weather result: {
  "content": [
    {
      "type": "text",
      "text": "{\"temperature\":17.94,\"unit\":\"celsius\"}"
    }
  ],
  "isError": false
}
```

</details>

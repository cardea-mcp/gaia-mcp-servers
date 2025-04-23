# Gaia Keyword Search MCP Server

## Quick Start

### Build and run gaia-kwsearch-mcp-server (TCP)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (tcp)
cargo build --package gaia-kwsearch-mcp-server --release

# build mcp client
cargo build --package gaia-kwsearch-mcp-client --release
```

**Note** that, before running the mcp server, you need to start the kw-search server. If you don't have a kw-search server running, you can start one by running the following commands to download the binary and start the server:

<details><summary>Expand to view the commands</summary>

```bash
export VERSION=0.1.1

# macOS on Apple Silicon
curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/${VERSION}/server-assistant-aarch64-apple-darwin.tar.gz
tar -xvzf server-assistant-aarch64-apple-darwin.tar.gz

# macOS on Intel
curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/${VERSION}/server-assistant-x86_64-apple-darwin.tar.gz
tar -xvzf server-assistant-x86_64-apple-darwin.tar.gz

# Linux
curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/${VERSION}/server-assistant-x86_64-unknown-linux-gnu.tar.gz
tar -xvzf server-assistant-x86_64-unknown-linux-gnu.tar.gz

# start kw-search-server on default port 12306
./kw-search-server
```

</details>

Now, let's start the mcp server (tcp) by running the following command:

```bash
# run mcp server (tcp)
./target/release/gaia-kwsearch-mcp-server
```

If start successfully, you will see the following output:

```bash
Keyword Search MCP server is listening on 127.0.0.1:8005
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client
./target/release/gaia-kwsearch-mcp-client --transport tcp
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-04-23T06:39:34.166566Z  INFO gaia_kwsearch_mcp_client: 40: Connecting to MCP server via tcp
2025-04-23T06:39:34.168316Z  INFO serve_inner: rmcp::service: 531: Service initialized as client peer_info=InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can access the KeywordSearch database") }
2025-04-23T06:39:34.168363Z  INFO gaia_kwsearch_mcp_client: 52: Connected to server: InitializeResult {
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
        "A MCP server that can access the KeywordSearch database",
    ),
}
2025-04-23T06:39:34.169304Z  INFO gaia_kwsearch_mcp_client: 56: {
  "tools": [
    {
      "name": "create_index",
      "description": "Create an index in the KeywordSearch database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "definitions": {
          "KwDocumentInput": {
            "properties": {
              "content": {
                "description": "the content of the document",
                "type": "string"
              },
              "title": {
                "description": "the title of the document",
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "required": [
              "content"
            ],
            "type": "object"
          }
        },
        "properties": {
          "base_url": {
            "description": "the base URL of the local or remote KeywordSearch database",
            "type": "string"
          },
          "documents": {
            "description": "the documents to index",
            "items": {
              "$ref": "#/definitions/KwDocumentInput"
            },
            "type": "array"
          },
          "name": {
            "description": "the name of the index to create",
            "type": [
              "string",
              "null"
            ]
          }
        },
        "required": [
          "base_url",
          "documents"
        ],
        "title": "CreateIndexRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "search_documents",
      "description": "Search for documents in the KeywordSearch database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "base_url": {
            "description": "the base URL of the local or remote KeywordSearch database",
            "type": "string"
          },
          "index_name": {
            "description": "the index to search",
            "type": "string"
          },
          "limit": {
            "description": "the number of results to return",
            "format": "uint",
            "minimum": 0.0,
            "type": "integer"
          },
          "query": {
            "description": "the query to search for",
            "type": "string"
          }
        },
        "required": [
          "base_url",
          "index_name",
          "limit",
          "query"
        ],
        "title": "SearchDocumentsRequest",
        "type": "object"
      },
      "annotations": null
    }
  ]
}
```

</details>

### Build and run gaia-kwsearch-mcp-server (stdio)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (stdio)
cargo build --package gaia-kwsearch-mcp-server-stdio --release

# build mcp client
cargo build --package gaia-kwsearch-mcp-client --release
```

**Note** that, before running the mcp server, you need to start the kw-search server. If you don't have a kw-search server running, you can start one by running the following commands to download the binary and start the server:

<details><summary>Expand to view the commands</summary>

```bash
export VERSION=0.1.1

# macOS on Apple Silicon
curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/${VERSION}/server-assistant-aarch64-apple-darwin.tar.gz
tar -xvzf server-assistant-aarch64-apple-darwin.tar.gz

# macOS on Intel
curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/${VERSION}/server-assistant-x86_64-apple-darwin.tar.gz
tar -xvzf server-assistant-x86_64-apple-darwin.tar.gz

# Linux
curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/${VERSION}/server-assistant-x86_64-unknown-linux-gnu.tar.gz
tar -xvzf server-assistant-x86_64-unknown-linux-gnu.tar.gz

# start kw-search-server on default port 12306
./kw-search-server
```

</details>

Now, let's run the mcp client by running the following command. The mcp client will start and call the mcp server.

```bash
# run mcp client
./target/release/gaia-kwsearch-mcp-client --transport stdio
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-04-23T06:41:27.063191Z  INFO gaia_kwsearch_mcp_client: 59: Connecting to MCP server via stdio
2025-04-23T06:41:27.755600Z  INFO gaia_kwsearch_mcp_server_stdio: 17: Starting Gaia Keyword Search MCP server in stdio mode
2025-04-23T06:41:27.757543Z  INFO rmcp::handler::server: 214: client initialized
2025-04-23T06:41:27.757611Z  INFO serve_inner: rmcp::service: 531: Service initialized as client peer_info=InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can access the KeywordSearch database") }
2025-04-23T06:41:27.757681Z  INFO gaia_kwsearch_mcp_client: 66: Connected to server
2025-04-23T06:41:27.757685Z  INFO gaia_kwsearch_mcp_client: 70: Connected to server: InitializeResult {
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
        "A MCP server that can access the KeywordSearch database",
    ),
}
2025-04-23T06:41:27.757674Z  INFO serve_inner: rmcp::service: 533: Service initialized as server peer_info=InitializeRequestParam { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ClientCapabilities { experimental: None, roots: None, sampling: None }, client_info: Implementation { name: "rmcp", version: "0.1.5" } }
2025-04-23T06:41:27.757970Z DEBUG rmcp::service: 593: new event evt=PeerMessage(Request(JsonRpcRequest { jsonrpc: JsonRpcVersion2_0, id: Number(1), request: ListToolsRequest(RequestOptionalParam { method: ListToolsRequestMethod, params: Some(PaginatedRequestParam { cursor: None }), extensions: Extensions }) }))
2025-04-23T06:41:27.757985Z  INFO rmcp::service: 659: received request id=1 request=ListToolsRequest(RequestOptionalParam { method: ListToolsRequestMethod, params: Some(PaginatedRequestParam { cursor: None }), extensions: Extensions })
2025-04-23T06:41:27.758180Z  INFO rmcp::service: 677: response message id=1 result=ListToolsResult(ListToolsResult { next_cursor: None, tools: [Tool { name: "create_index", description: Some("Create an index in the KeywordSearch database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "definitions": Object {"KwDocumentInput": Object {"properties": Object {"content": Object {"description": String("the content of the document"), "type": String("string")}, "title": Object {"description": String("the title of the document"), "type": Array [String("string"), String("null")]}}, "required": Array [String("content")], "type": String("object")}}, "properties": Object {"base_url": Object {"description": String("the base URL of the local or remote KeywordSearch database"), "type": String("string")}, "documents": Object {"description": String("the documents to index"), "items": Object {"$ref": String("#/definitions/KwDocumentInput")}, "type": String("array")}, "name": Object {"description": String("the name of the index to create"), "type": Array [String("string"), String("null")]}}, "required": Array [String("base_url"), String("documents")], "title": String("CreateIndexRequest"), "type": String("object")}, annotations: None }, Tool { name: "search_documents", description: Some("Search for documents in the KeywordSearch database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"base_url": Object {"description": String("the base URL of the local or remote KeywordSearch database"), "type": String("string")}, "index_name": Object {"description": String("the index to search"), "type": String("string")}, "limit": Object {"description": String("the number of results to return"), "format": String("uint"), "minimum": Number(0.0), "type": String("integer")}, "query": Object {"description": String("the query to search for"), "type": String("string")}}, "required": Array [String("base_url"), String("index_name"), String("limit"), String("query")], "title": String("SearchDocumentsRequest"), "type": String("object")}, annotations: None }] })
2025-04-23T06:41:27.758274Z DEBUG rmcp::service: 593: new event evt=ToSink(Response(JsonRpcResponse { jsonrpc: JsonRpcVersion2_0, id: Number(1), result: ListToolsResult(ListToolsResult { next_cursor: None, tools: [Tool { name: "create_index", description: Some("Create an index in the KeywordSearch database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "definitions": Object {"KwDocumentInput": Object {"properties": Object {"content": Object {"description": String("the content of the document"), "type": String("string")}, "title": Object {"description": String("the title of the document"), "type": Array [String("string"), String("null")]}}, "required": Array [String("content")], "type": String("object")}}, "properties": Object {"base_url": Object {"description": String("the base URL of the local or remote KeywordSearch database"), "type": String("string")}, "documents": Object {"description": String("the documents to index"), "items": Object {"$ref": String("#/definitions/KwDocumentInput")}, "type": String("array")}, "name": Object {"description": String("the name of the index to create"), "type": Array [String("string"), String("null")]}}, "required": Array [String("base_url"), String("documents")], "title": String("CreateIndexRequest"), "type": String("object")}, annotations: None }, Tool { name: "search_documents", description: Some("Search for documents in the KeywordSearch database"), input_schema: {"$schema": String("http://json-schema.org/draft-07/schema#"), "properties": Object {"base_url": Object {"description": String("the base URL of the local or remote KeywordSearch database"), "type": String("string")}, "index_name": Object {"description": String("the index to search"), "type": String("string")}, "limit": Object {"description": String("the number of results to return"), "format": String("uint"), "minimum": Number(0.0), "type": String("integer")}, "query": Object {"description": String("the query to search for"), "type": String("string")}}, "required": Array [String("base_url"), String("index_name"), String("limit"), String("query")], "title": String("SearchDocumentsRequest"), "type": String("object")}, annotations: None }] }) }))
2025-04-23T06:41:27.758522Z  INFO gaia_kwsearch_mcp_client: 74: {
  "tools": [
    {
      "name": "create_index",
      "description": "Create an index in the KeywordSearch database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "definitions": {
          "KwDocumentInput": {
            "properties": {
              "content": {
                "description": "the content of the document",
                "type": "string"
              },
              "title": {
                "description": "the title of the document",
                "type": [
                  "string",
                  "null"
                ]
              }
            },
            "required": [
              "content"
            ],
            "type": "object"
          }
        },
        "properties": {
          "base_url": {
            "description": "the base URL of the local or remote KeywordSearch database",
            "type": "string"
          },
          "documents": {
            "description": "the documents to index",
            "items": {
              "$ref": "#/definitions/KwDocumentInput"
            },
            "type": "array"
          },
          "name": {
            "description": "the name of the index to create",
            "type": [
              "string",
              "null"
            ]
          }
        },
        "required": [
          "base_url",
          "documents"
        ],
        "title": "CreateIndexRequest",
        "type": "object"
      },
      "annotations": null
    },
    {
      "name": "search_documents",
      "description": "Search for documents in the KeywordSearch database",
      "inputSchema": {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "properties": {
          "base_url": {
            "description": "the base URL of the local or remote KeywordSearch database",
            "type": "string"
          },
          "index_name": {
            "description": "the index to search",
            "type": "string"
          },
          "limit": {
            "description": "the number of results to return",
            "format": "uint",
            "minimum": 0.0,
            "type": "integer"
          },
          "query": {
            "description": "the query to search for",
            "type": "string"
          }
        },
        "required": [
          "base_url",
          "index_name",
          "limit",
          "query"
        ],
        "title": "SearchDocumentsRequest",
        "type": "object"
      },
      "annotations": null
    }
  ]
}
2025-04-23T06:41:27.758696Z DEBUG rmcp::service: 593: new event evt=PeerMessage(Request(JsonRpcRequest { jsonrpc: JsonRpcVersion2_0, id: Number(2), request: CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "create_index", arguments: Some({"base_url": String("http://127.0.0.1:12306"), "documents": Array [Object {"content": String("Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers."), "title": String("section 1")}, Object {"content": String("The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies."), "title": String("section 2")}], "name": String("mcp-test")}) }, extensions: Extensions }) }))
2025-04-23T06:41:27.758745Z  INFO rmcp::service: 659: received request id=2 request=CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "create_index", arguments: Some({"base_url": String("http://127.0.0.1:12306"), "documents": Array [Object {"content": String("Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers."), "title": String("section 1")}, Object {"content": String("The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies."), "title": String("section 2")}], "name": String("mcp-test")}) }, extensions: Extensions })
2025-04-23T06:41:27.758791Z  INFO gaia_kwsearch_mcp_server_stdio::search: 36: Creating index in KeywordSearch database
2025-04-23T06:41:27.758817Z  INFO gaia_kwsearch_mcp_server_stdio::search: 40: URL to create index: http://127.0.0.1:12306/v1/index/create
2025-04-23T06:41:27.767276Z DEBUG reqwest::connect: 625: starting new connection: http://127.0.0.1:12306/
2025-04-23T06:41:27.767320Z DEBUG reqwest::connect: 501: proxy(http://127.0.0.1:7890) intercepts 'http://127.0.0.1:12306/'
2025-04-23T06:41:27.767429Z DEBUG hyper_util::client::legacy::connect::http: 769: connecting to 127.0.0.1:7890
2025-04-23T06:41:27.767761Z DEBUG hyper_util::client::legacy::connect::http: 772: connected to 127.0.0.1:7890
2025-04-23T06:41:27.981232Z  INFO gaia_kwsearch_mcp_server_stdio::search: 70: Index created in KeywordSearch database
2025-04-23T06:41:27.981909Z  INFO rmcp::service: 677: response message id=2 result=CallToolResult(CallToolResult { content: [Annotated { raw: Text(RawTextContent { text: "{\"index_name\":\"mcp-test\",\"results\":[{\"filename\":\"section 1\",\"status\":\"indexed\"},{\"filename\":\"section 2\",\"status\":\"indexed\"}]}" }), annotations: None }], is_error: Some(false) })
2025-04-23T06:41:27.981984Z DEBUG rmcp::service: 593: new event evt=ToSink(Response(JsonRpcResponse { jsonrpc: JsonRpcVersion2_0, id: Number(2), result: CallToolResult(CallToolResult { content: [Annotated { raw: Text(RawTextContent { text: "{\"index_name\":\"mcp-test\",\"results\":[{\"filename\":\"section 1\",\"status\":\"indexed\"},{\"filename\":\"section 2\",\"status\":\"indexed\"}]}" }), annotations: None }], is_error: Some(false) }) }))
2025-04-23T06:41:27.983287Z  INFO gaia_kwsearch_mcp_client: 116: create index response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"index_name\":\"mcp-test\",\"results\":[{\"filename\":\"section 1\",\"status\":\"indexed\"},{\"filename\":\"section 2\",\"status\":\"indexed\"}]}"
    }
  ],
  "isError": false
}
2025-04-23T06:41:27.983365Z  INFO gaia_kwsearch_mcp_client: 122: create index response:
CreateIndexResponse { index_name: Some("mcp-test"), results: [KwDocumentResult { filename: Some("section 1"), status: "indexed", error: None }, KwDocumentResult { filename: Some("section 2"), status: "indexed", error: None }] }
2025-04-23T06:41:27.983579Z DEBUG rmcp::service: 593: new event evt=PeerMessage(Request(JsonRpcRequest { jsonrpc: JsonRpcVersion2_0, id: Number(3), request: CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "search_documents", arguments: Some({"base_url": String("http://127.0.0.1:12306"), "index_name": String("mcp-test"), "limit": Number(2), "query": String("What's Gaianet?")}) }, extensions: Extensions }) }))
2025-04-23T06:41:27.983620Z  INFO rmcp::service: 659: received request id=3 request=CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "search_documents", arguments: Some({"base_url": String("http://127.0.0.1:12306"), "index_name": String("mcp-test"), "limit": Number(2), "query": String("What's Gaianet?")}) }, extensions: Extensions })
2025-04-23T06:41:27.983676Z  INFO gaia_kwsearch_mcp_server_stdio::search: 86: Searching for documents in KeywordSearch database
2025-04-23T06:41:27.984408Z DEBUG reqwest::connect: 625: starting new connection: http://127.0.0.1:12306/
2025-04-23T06:41:27.984433Z DEBUG reqwest::connect: 501: proxy(http://127.0.0.1:7890) intercepts 'http://127.0.0.1:12306/'
2025-04-23T06:41:27.984449Z DEBUG hyper_util::client::legacy::connect::http: 769: connecting to 127.0.0.1:7890
2025-04-23T06:41:27.984833Z DEBUG hyper_util::client::legacy::connect::http: 772: connected to 127.0.0.1:7890
2025-04-23T06:41:28.001320Z  INFO gaia_kwsearch_mcp_server_stdio::search: 120: Documents searched in KeywordSearch database
2025-04-23T06:41:28.001342Z  INFO rmcp::service: 677: response message id=3 result=CallToolResult(CallToolResult { content: [Annotated { raw: Text(RawTextContent { text: "{\"hits\":[{\"title\":\"section 1\",\"content\":\"Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.\",\"score\":0.25017098},{\"title\":\"section 2\",\"content\":\"The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.\",\"score\":0.18627283}]}" }), annotations: None }], is_error: Some(false) })
2025-04-23T06:41:28.001392Z DEBUG rmcp::service: 593: new event evt=ToSink(Response(JsonRpcResponse { jsonrpc: JsonRpcVersion2_0, id: Number(3), result: CallToolResult(CallToolResult { content: [Annotated { raw: Text(RawTextContent { text: "{\"hits\":[{\"title\":\"section 1\",\"content\":\"Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.\",\"score\":0.25017098},{\"title\":\"section 2\",\"content\":\"The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.\",\"score\":0.18627283}]}" }), annotations: None }], is_error: Some(false) }) }))
2025-04-23T06:41:28.001602Z  INFO gaia_kwsearch_mcp_client: 145: search documents response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"hits\":[{\"title\":\"section 1\",\"content\":\"Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.\",\"score\":0.25017098},{\"title\":\"section 2\",\"content\":\"The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.\",\"score\":0.18627283}]}"
    }
  ],
  "isError": false
}
2025-04-23T06:41:28.001632Z  INFO gaia_kwsearch_mcp_client: 150: search documents response:
SearchDocumentsResponse { hits: [KwSearchHit { title: "section 1", content: "Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.", score: 0.25017098 }, KwSearchHit { title: "section 2", content: "The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.", score: 0.18627283 }] }
2025-04-23T06:41:28.001658Z  INFO rmcp::service: 587: task cancelled
2025-04-23T06:41:28.001665Z  INFO rmcp::service: 755: serve finished quit_reason=Cancelled
```

</details>

# Gaia Keyword Search MCP Server

## Quick Start

### Build and run gaia-kwsearch-mcp-server (StreamableHttp)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (tcp)
cargo build --package gaia-kwsearch-mcp-server-streamhttp --release

# build mcp client
cargo build --package gaia-kwsearch-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

> [!IMPORTANT]
>
> Before running the mcp server, you need to start the kw-search server. If you don't have a kw-search server running, you can start one by running the following commands to download the binary and start the server:
>
> <details><summary>Expand to view the commands</summary>
>
> ```bash
> export VERSION=0.2.0
>
> # macOS on Apple Silicon
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-apple-aarch64-darwin.tar.gz
> tar -xvzf kw-search-server-apple-aarch64-darwin.tar.gz kw-search-server
>
> # macOS on Intel
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-apple-x86_64-darwin.tar.gz
> tar -xvzf kw-search-server-apple-x86_64-darwin.tar.gz kw-search-server
>
> # Linux (x86_64)
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-linux-x86_64-unknown-gnu.tar.gz
> tar -xvzf kw-search-server-linux-x86_64-unknown-gnu.tar.gz kw-search-server
>
> # Linux (aarch64)
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-linux-aarch64-unknown-gnu.tar.gz
> tar -xvzf kw-search-server-linux-aarch64-unknown-gnu.tar.gz kw-search-server
>
> # start kw-search-server on default port 12306
> ./kw-search-server
> ```
>
> </details>

Now, let's start the mcp server (streamable-http) by running the following command:

```bash
# run mcp server (streamable-http)
./target/release/gaia-kwsearch-mcp-server-streamhttp --base-url http://127.0.0.1:12306
```

If start successfully, you will see the following output:

```bash
Keyword Search MCP server is listening on 127.0.0.1:8005
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client
./target/release/gaia-kwsearch-mcp-client --transport stream-http
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-05-30T08:01:09.769067Z  INFO gaia_kwsearch_mcp_client: 225: Connecting to Gaia KeywordSearch MCP server via stream-http: http://127.0.0.1:8005/mcp
2025-05-30T08:01:09.786646Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can access the KeywordSearch database") })
2025-05-30T08:01:09.786685Z  INFO gaia_kwsearch_mcp_client: 245: Connected to server: Some(
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
            "A MCP server that can access the KeywordSearch database",
        ),
    },
)
2025-05-30T08:01:09.800521Z  INFO gaia_kwsearch_mcp_client: 249: {
  "tools": [
    {
      "name": "search_documents",
      "description": "Search for documents in the KeywordSearch database",
      "inputSchema": {
        "properties": {
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
          "index_name",
          "limit",
          "query"
        ],
        "title": "SearchDocumentsRequest",
        "type": "object"
      }
    },
    {
      "name": "create_index",
      "description": "Create an index in the KeywordSearch database",
      "inputSchema": {
        "definitions": {
          "KwDocumentInput": {
            "properties": {
              "content": {
                "description": "the content of the document",
                "type": "string"
              },
              "title": {
                "description": "the title of the document",
                "nullable": true,
                "type": "string"
              }
            },
            "required": [
              "content"
            ],
            "type": "object"
          }
        },
        "properties": {
          "documents": {
            "description": "the documents to index",
            "items": {
              "$ref": "#/components/schemas/KwDocumentInput"
            },
            "type": "array"
          },
          "name": {
            "description": "the name of the index to create",
            "nullable": true,
            "type": "string"
          }
        },
        "required": [
          "documents"
        ],
        "title": "CreateIndexRequest",
        "type": "object"
      }
    }
  ]
}
2025-05-30T08:01:10.031413Z  INFO gaia_kwsearch_mcp_client: 287: create index response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"index_name\":\"mcp-test\",\"results\":[{\"filename\":\"section 1\",\"status\":\"indexed\"},{\"filename\":\"section 2\",\"status\":\"indexed\"}]}"
    }
  ],
  "isError": false
}
2025-05-30T08:01:10.031779Z  INFO gaia_kwsearch_mcp_client: 293: create index response:
CreateIndexResponse { index_name: Some("mcp-test"), results: [KwDocumentResult { filename: Some("section 1"), status: "indexed", error: None }, KwDocumentResult { filename: Some("section 2"), status: "indexed", error: None }] }
2025-05-30T08:01:10.049876Z  INFO gaia_kwsearch_mcp_client: 309: search documents response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"hits\":[{\"title\":\"section 1\",\"content\":\"Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.\",\"score\":0.2501709759235382},{\"title\":\"section 2\",\"content\":\"The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.\",\"score\":0.18627282977104187}]}"
    }
  ],
  "isError": false
}
2025-05-30T08:01:10.049934Z  INFO gaia_kwsearch_mcp_client: 314: search documents response:
SearchDocumentsResponse { hits: [KwSearchHit { title: "section 1", content: "Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.", score: 0.2501709759235382 }, KwSearchHit { title: "section 2", content: "The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.", score: 0.18627282977104187 }] }
2025-05-30T08:01:10.050051Z  INFO rmcp::service: 625: task cancelled
2025-05-30T08:01:10.050101Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
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

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

> [!IMPORTANT]
>
> Before running the mcp server, you need to start the kw-search server. If you don't have a kw-search server running, you can start one by running the following commands to download the binary and start the server:
>
> <details><summary>Expand to view the commands</summary>
>
> ```bash
> export VERSION=0.2.0
>
> # macOS on Apple Silicon
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-apple-aarch64-darwin.tar.gz
> tar -xvzf kw-search-server-apple-aarch64-darwin.tar.gz kw-search-server
>
> # macOS on Intel
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-apple-x86_64-darwin.tar.gz
> tar -xvzf kw-search-server-apple-x86_64-darwin.tar.gz kw-search-server
>
> # Linux (x86_64)
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-linux-x86_64-unknown-gnu.tar.gz
> tar -xvzf kw-search-server-linux-x86_64-unknown-gnu.tar.gz kw-search-server
>
> # Linux (aarch64)
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-linux-aarch64-unknown-gnu.tar.gz
> tar -xvzf kw-search-server-linux-aarch64-unknown-gnu.tar.gz kw-search-server
>
> # start kw-search-server on default port 12306
> ./kw-search-server
> ```
>
> </details>

Now, let's run the mcp client by running the following command. The mcp client will start and call the mcp server.

```bash
# run mcp client
./target/release/gaia-kwsearch-mcp-client --transport stdio
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-05-30T07:55:26.164602Z  INFO gaia_kwsearch_mcp_client: 137: Connecting to MCP server via stdio
2025-05-30T07:55:26.171942Z  INFO gaia_kwsearch_mcp_server_stdio: 42: Starting Gaia Keyword Search MCP server in stdio mode
2025-05-30T07:55:26.172609Z  INFO rmcp::handler::server: 209: client initialized
2025-05-30T07:55:26.172627Z  INFO serve_inner: rmcp::service: 543: Service initialized as server peer_info=Some(InitializeRequestParam { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ClientCapabilities { experimental: None, roots: None, sampling: None }, client_info: Implementation { name: "rmcp", version: "0.1.5" } })
2025-05-30T07:55:26.172636Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can access the KeywordSearch database") })
2025-05-30T07:55:26.172863Z  INFO gaia_kwsearch_mcp_client: 145: Connected to server
2025-05-30T07:55:26.172866Z  INFO gaia_kwsearch_mcp_client: 149: Connected to server: Some(
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
            "A MCP server that can access the KeywordSearch database",
        ),
    },
)
2025-05-30T07:55:26.173321Z DEBUG rmcp::service: 715: received request id=1 request=ListToolsRequest(RequestOptionalParam { method: ListToolsRequestMethod, params: Some(PaginatedRequestParam { cursor: None }), extensions: Extensions })
2025-05-30T07:55:26.173386Z DEBUG rmcp::service: 733: response message id=1 result=ListToolsResult(ListToolsResult { next_cursor: None, tools: [Tool { name: "search_documents", description: Some("Search for documents in the KeywordSearch database"), input_schema: {"properties": Object {"index_name": Object {"description": String("the index to search"), "type": String("string")}, "limit": Object {"description": String("the number of results to return"), "format": String("uint"), "minimum": Number(0.0), "type": String("integer")}, "query": Object {"description": String("the query to search for"), "type": String("string")}}, "required": Array [String("index_name"), String("limit"), String("query")], "title": String("SearchDocumentsRequest"), "type": String("object")}, annotations: None }, Tool { name: "create_index", description: Some("Create an index in the KeywordSearch database"), input_schema: {"definitions": Object {"KwDocumentInput": Object {"properties": Object {"content": Object {"description": String("the content of the document"), "type": String("string")}, "title": Object {"description": String("the title of the document"), "nullable": Bool(true), "type": String("string")}}, "required": Array [String("content")], "type": String("object")}}, "properties": Object {"documents": Object {"description": String("the documents to index"), "items": Object {"$ref": String("#/components/schemas/KwDocumentInput")}, "type": String("array")}, "name": Object {"description": String("the name of the index to create"), "nullable": Bool(true), "type": String("string")}}, "required": Array [String("documents")], "title": String("CreateIndexRequest"), "type": String("object")}, annotations: None }] })
2025-05-30T07:55:26.173536Z  INFO gaia_kwsearch_mcp_client: 153: {
  "tools": [
    {
      "name": "search_documents",
      "description": "Search for documents in the KeywordSearch database",
      "inputSchema": {
        "properties": {
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
          "index_name",
          "limit",
          "query"
        ],
        "title": "SearchDocumentsRequest",
        "type": "object"
      }
    },
    {
      "name": "create_index",
      "description": "Create an index in the KeywordSearch database",
      "inputSchema": {
        "definitions": {
          "KwDocumentInput": {
            "properties": {
              "content": {
                "description": "the content of the document",
                "type": "string"
              },
              "title": {
                "description": "the title of the document",
                "nullable": true,
                "type": "string"
              }
            },
            "required": [
              "content"
            ],
            "type": "object"
          }
        },
        "properties": {
          "documents": {
            "description": "the documents to index",
            "items": {
              "$ref": "#/components/schemas/KwDocumentInput"
            },
            "type": "array"
          },
          "name": {
            "description": "the name of the index to create",
            "nullable": true,
            "type": "string"
          }
        },
        "required": [
          "documents"
        ],
        "title": "CreateIndexRequest",
        "type": "object"
      }
    }
  ]
}
2025-05-30T07:55:26.173627Z DEBUG rmcp::service: 715: received request id=2 request=CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "create_index", arguments: Some({"documents": Array [Object {"content": String("Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers."), "title": String("section 1")}, Object {"content": String("The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies."), "title": String("section 2")}], "name": String("mcp-test")}) }, extensions: Extensions })
2025-05-30T07:55:26.173665Z  INFO gaia_kwsearch_mcp_server_stdio::search: 32: Creating index in KeywordSearch database
2025-05-30T07:55:26.178560Z DEBUG reqwest::connect: 753: starting new connection: http://127.0.0.1:12306/
2025-05-30T07:55:26.178570Z DEBUG reqwest::connect: 617: proxy(http://127.0.0.1:7890/) intercepts 'http://127.0.0.1:12306/'
2025-05-30T07:55:26.178580Z DEBUG hyper_util::client::legacy::connect::http: 768: connecting to 127.0.0.1:7890
2025-05-30T07:55:26.178863Z DEBUG hyper_util::client::legacy::connect::http: 771: connected to 127.0.0.1:7890
2025-05-30T07:55:26.382091Z  INFO gaia_kwsearch_mcp_server_stdio::search: 83: Index created in KeywordSearch database
2025-05-30T07:55:26.382185Z DEBUG rmcp::service: 733: response message id=2 result=CallToolResult(CallToolResult { content: [Annotated { raw: Text(RawTextContent { text: "{\"index_name\":\"mcp-test\",\"results\":[{\"filename\":\"section 1\",\"status\":\"indexed\"},{\"filename\":\"section 2\",\"status\":\"indexed\"}]}" }), annotations: None }], is_error: Some(false) })
2025-05-30T07:55:26.383060Z  INFO gaia_kwsearch_mcp_client: 191: create index response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"index_name\":\"mcp-test\",\"results\":[{\"filename\":\"section 1\",\"status\":\"indexed\"},{\"filename\":\"section 2\",\"status\":\"indexed\"}]}"
    }
  ],
  "isError": false
}
2025-05-30T07:55:26.383119Z  INFO gaia_kwsearch_mcp_client: 197: create index response:
CreateIndexResponse { index_name: Some("mcp-test"), results: [KwDocumentResult { filename: Some("section 1"), status: "indexed", error: None }, KwDocumentResult { filename: Some("section 2"), status: "indexed", error: None }] }
2025-05-30T07:55:26.383440Z DEBUG rmcp::service: 715: received request id=3 request=CallToolRequest(Request { method: CallToolRequestMethod, params: CallToolRequestParam { name: "search_documents", arguments: Some({"index_name": String("mcp-test"), "limit": Number(2), "query": String("Gaianet")}) }, extensions: Extensions })
2025-05-30T07:55:26.383510Z  INFO gaia_kwsearch_mcp_server_stdio::search: 97: Searching for documents in KeywordSearch database
2025-05-30T07:55:26.384437Z DEBUG reqwest::connect: 753: starting new connection: http://127.0.0.1:12306/
2025-05-30T07:55:26.384464Z DEBUG reqwest::connect: 617: proxy(http://127.0.0.1:7890/) intercepts 'http://127.0.0.1:12306/'
2025-05-30T07:55:26.384497Z DEBUG hyper_util::client::legacy::connect::http: 768: connecting to 127.0.0.1:7890
2025-05-30T07:55:26.384992Z DEBUG hyper_util::client::legacy::connect::http: 771: connected to 127.0.0.1:7890
2025-05-30T07:55:26.394667Z  INFO gaia_kwsearch_mcp_server_stdio::search: 149: Documents searched in KeywordSearch database
2025-05-30T07:55:26.394704Z DEBUG rmcp::service: 733: response message id=3 result=CallToolResult(CallToolResult { content: [Annotated { raw: Text(RawTextContent { text: "{\"hits\":[{\"title\":\"section 1\",\"content\":\"Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.\",\"score\":0.2501709759235382},{\"title\":\"section 2\",\"content\":\"The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.\",\"score\":0.18627282977104187}]}" }), annotations: None }], is_error: Some(false) })
2025-05-30T07:55:26.395010Z  INFO gaia_kwsearch_mcp_client: 213: search documents response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"hits\":[{\"title\":\"section 1\",\"content\":\"Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.\",\"score\":0.2501709759235382},{\"title\":\"section 2\",\"content\":\"The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.\",\"score\":0.18627282977104187}]}"
    }
  ],
  "isError": false
}
2025-05-30T07:55:26.395139Z  INFO gaia_kwsearch_mcp_client: 218: search documents response:
SearchDocumentsResponse { hits: [KwSearchHit { title: "section 1", content: "Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.", score: 0.2501709759235382 }, KwSearchHit { title: "section 2", content: "The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.", score: 0.18627282977104187 }] }
2025-05-30T07:55:26.395275Z  INFO rmcp::service: 625: task cancelled
2025-05-30T07:55:26.395286Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

### Build and run gaia-kwsearch-mcp-server (sse)

Let's build mcp server and client by running the following commands:

```bash
# build mcp server (sse)
cargo build --package gaia-kwsearch-mcp-server-sse --release

# build mcp client
cargo build --package gaia-kwsearch-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

> [!IMPORTANT]
>
> Before running the mcp server, you need to start the kw-search server. If you don't have a kw-search server running, you can start one by running the following commands to download the binary and start the server:
>
> <details><summary>Expand to view the commands</summary>
>
> ```bash
> export VERSION=0.2.0
>
> # macOS on Apple Silicon
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-apple-aarch64-darwin.tar.gz
> tar -xvzf kw-search-server-apple-aarch64-darwin.tar.gz kw-search-server
>
> # macOS on Intel
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-apple-x86_64-darwin.tar.gz
> tar -xvzf kw-search-server-apple-x86_64-darwin.tar.gz kw-search-server
>
> # Linux (x86_64)
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-linux-x86_64-unknown-gnu.tar.gz
> tar -xvzf kw-search-server-linux-x86_64-unknown-gnu.tar.gz kw-search-server
>
> # Linux (aarch64)
> curl -LO https://github.com/LlamaEdge/kw-search-server/releases/download/{$VERSION}/kw-search-server-linux-aarch64-unknown-gnu.tar.gz
> tar -xvzf kw-search-server-linux-aarch64-unknown-gnu.tar.gz kw-search-server
>
> # start kw-search-server on default port 12306
> ./kw-search-server
> ```
>
> </details>

Now, let's start the mcp server (sse) by running the following command:

```bash
# run mcp server (sse)
./target/release/gaia-kwsearch-mcp-server-sse --base-url http://127.0.0.1:12306
```

If start successfully, you will see the following output:

```bash
Starting Gaia KeywordSearch MCP server on 127.0.0.1:8005
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client
./target/release/gaia-kwsearch-mcp-client --transport sse
```

If start successfully, you will see the following output:

<details><summary>Expand to view the output</summary>

```console
2025-05-30T07:51:25.960392Z  INFO gaia_kwsearch_mcp_client: 44: Connecting to Gaia KeywordSearch MCP server via sse: http://127.0.0.1:8005/sse
2025-05-30T07:51:25.989098Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "rmcp", version: "0.1.5" }, instructions: Some("A MCP server that can access the KeywordSearch database") })
2025-05-30T07:51:25.989182Z  INFO gaia_kwsearch_mcp_client: 64: Connected to server: Some(
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
            "A MCP server that can access the KeywordSearch database",
        ),
    },
)
2025-05-30T07:51:25.993457Z  INFO gaia_kwsearch_mcp_client: 68: Available tools: {
  "tools": [
    {
      "name": "create_index",
      "description": "Create an index in the KeywordSearch database",
      "inputSchema": {
        "definitions": {
          "KwDocumentInput": {
            "properties": {
              "content": {
                "description": "the content of the document",
                "type": "string"
              },
              "title": {
                "description": "the title of the document",
                "nullable": true,
                "type": "string"
              }
            },
            "required": [
              "content"
            ],
            "type": "object"
          }
        },
        "properties": {
          "documents": {
            "description": "the documents to index",
            "items": {
              "$ref": "#/components/schemas/KwDocumentInput"
            },
            "type": "array"
          },
          "name": {
            "description": "the name of the index to create",
            "nullable": true,
            "type": "string"
          }
        },
        "required": [
          "documents"
        ],
        "title": "CreateIndexRequest",
        "type": "object"
      }
    },
    {
      "name": "search_documents",
      "description": "Search for documents in the KeywordSearch database",
      "inputSchema": {
        "properties": {
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
          "index_name",
          "limit",
          "query"
        ],
        "title": "SearchDocumentsRequest",
        "type": "object"
      }
    }
  ]
}
2025-05-30T07:51:26.021983Z  INFO gaia_kwsearch_mcp_client: 105: create index response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"results\":[]}"
    }
  ],
  "isError": false
}
2025-05-30T07:51:26.022002Z  INFO gaia_kwsearch_mcp_client: 111: create index response:
CreateIndexResponse { index_name: None, results: [] }
2025-05-30T07:51:26.035858Z  INFO gaia_kwsearch_mcp_client: 127: search documents response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"hits\":[{\"title\":\"section 1\",\"content\":\"Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.\",\"score\":0.2501709759235382},{\"title\":\"section 2\",\"content\":\"The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.\",\"score\":0.18627282977104187}]}"
    }
  ],
  "isError": false
}
2025-05-30T07:51:26.035907Z  INFO gaia_kwsearch_mcp_client: 132: search documents response:
SearchDocumentsResponse { hits: [KwSearchHit { title: "section 1", content: "Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.", score: 0.2501709759235382 }, KwSearchHit { title: "section 2", content: "The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.", score: 0.18627282977104187 }] }
2025-05-30T07:51:26.035971Z  INFO rmcp::service: 625: task cancelled
2025-05-30T07:51:26.035996Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

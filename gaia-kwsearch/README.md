# Gaia Keyword Search MCP Server

## Quick Start

### Build mcp server and client

Let's build mcp server and client by running the following commands:

```bash
# build mcp server
cargo build --package gaia-kwsearch-mcp-server --release

# build mcp client
cargo build --package gaia-kwsearch-mcp-client --release
```

> [!NOTE]
> The mcp client is ONLY used for verifying and demonstrating mcp servers.

### Start mcp server

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

The CLI options of the mcp server are as follows:

```bash
Usage: gaia-kwsearch-mcp-server [OPTIONS] --index <INDEX>

Options:
      --base-url <BASE_URL>        The base URL of the kw-search-server [default: http://127.0.0.1:12306]
  -s, --socket-addr <SOCKET_ADDR>  Socket address to bind to [default: 127.0.0.1:8005]
  -t, --transport <TRANSPORT>      Transport type to use [default: stream-http] [possible values: stdio, sse, stream-http]
      --index <INDEX>              Index to search
      --limit <LIMIT>              Maximum number of query results to return [default: 10]
  -h, --help                       Print help
  -V, --version                    Print version
```

Now, let's start the mcp server:

```bash
# run mcp server
./target/release/gaia-kwsearch-mcp-server --transport stream-http --index test01
```

If start successfully, you will see the following output:

```bash
Starting Gaia KeywordSearch MCP server on 127.0.0.1:8005
```

### Run mcp client

To demonstrate the mcp server, we can use the mcp client to interact with the mcp server. The CLI options of the mcp client are as follows:

```bash
Usage: gaia-kwsearch-mcp-client [OPTIONS] --index <INDEX>

Options:
  -t, --transport <TRANSPORT>  Transport type to use [default: stream-http] [possible values: stdio, sse, stream-http]
  -i, --index <INDEX>          The name of the index to use
  -h, --help                   Print help
  -V, --version                Print version
```

Now, let's run the mcp client by running the following command:

```bash
# run mcp client
./target/release/gaia-kwsearch-mcp-client --transport stream-http --index test01 --limit 2
```

If start successfully, you will see the following output returned by the mcp client. The output is different depending on the transport type you used. The output of `stream-http` is shown below.

<details><summary>Expand to view the output</summary>

```console
2025-06-15T08:57:40.141883Z  INFO gaia_kwsearch_mcp_client: 227: Connecting to Gaia KeywordSearch MCP server via stream-http: http://127.0.0.1:8005/mcp
2025-06-15T08:57:40.154481Z  INFO serve_inner: rmcp::service: 541: Service initialized as client peer_info=Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: None, completions: None, prompts: None, resources: None, tools: Some(ToolsCapability { list_changed: None }) }, server_info: Implementation { name: "gaia-kwsearch-mcp-server", version: "0.4.0" }, instructions: Some("A MCP server that can access the KeywordSearch database") })
2025-06-15T08:57:40.154531Z  INFO gaia_kwsearch_mcp_client: 247: Connected to server: Some(
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
            name: "gaia-kwsearch-mcp-server",
            version: "0.4.0",
        },
        instructions: Some(
            "A MCP server that can access the KeywordSearch database",
        ),
    },
)
2025-06-15T08:57:40.160663Z  INFO gaia_kwsearch_mcp_client: 251: {
  "tools": [
    {
      "name": "search",
      "description": "Perform a keyword search",
      "inputSchema": {
        "properties": {
          "query": {
            "description": "the query to search for",
            "type": "string"
          }
        },
        "required": [
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
          "index": {
            "description": "the name of the index to create",
            "type": "string"
          }
        },
        "required": [
          "documents",
          "index"
        ],
        "title": "CreateIndexRequest",
        "type": "object"
      }
    }
  ]
}
2025-06-15T08:57:40.168113Z  INFO gaia_kwsearch_mcp_client: 289: create index response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"results\":[]}"
    }
  ],
  "isError": false
}
2025-06-15T08:57:40.168137Z  INFO gaia_kwsearch_mcp_client: 295: create index response:
CreateIndexResponse { index_name: None, results: [] }
2025-06-15T08:57:40.194073Z  INFO gaia_kwsearch_mcp_client: 307: search documents response:
{
  "content": [
    {
      "type": "text",
      "text": "{\"hits\":[{\"title\":\"section 1\",\"content\":\"Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.\",\"score\":0.2501709759235382},{\"title\":\"section 2\",\"content\":\"The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.\",\"score\":0.18627282977104187}]}"
    }
  ],
  "isError": false
}
2025-06-15T08:57:40.194098Z  INFO gaia_kwsearch_mcp_client: 312: search response:
SearchDocumentsResponse { hits: [KwSearchHit { title: "section 1", content: "Gaianet is revolutionizing the AI landscape with a distributed AI infrastructure that seeks to decentralize the dominance of major players such as OpenAI, Google, and Anthropic. By leveraging a network of edge-computing nodes owned by individuals around the world, Gaianet enables hosting of both open-source and finely-tuned models. This infrastructure is designed to cater to diverse AI demands, offering a scalable alternative to traditional centralized servers.", score: 0.2501709759235382 }, KwSearchHit { title: "section 2", content: "The inception of Gaianet is driven by the necessity to address key issues in the current AI industry: censorship and bias in AI outputs, lack of privacy for user data, and the high costs associated with accessing and developing on centralized AI models. These challenges have restricted the dissemination of unbiased information, compromised data security, and erected barriers to innovation and broader application of AI technologies.", score: 0.18627282977104187 }] }
2025-06-15T08:57:40.194122Z  INFO rmcp::service: 625: task cancelled
2025-06-15T08:57:40.194185Z  INFO rmcp::service: 811: serve finished quit_reason=Cancelled
```

</details>

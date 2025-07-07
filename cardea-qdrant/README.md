# Cardea Qdrant MCP Server

## Quick Start

### Build

Let's build mcp server by running the following commands:

```bash
# build mcp server
cargo build --package cardea-qdrant-mcp-server --release
```

### Run

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
Usage: cardea-qdrant-mcp-server [OPTIONS] --collection <COLLECTION>

Options:
  -s, --socket-addr <SOCKET_ADDR>
          Socket address to bind to [default: 127.0.0.1:8003]
  -t, --transport <TRANSPORT>
          Transport type to use [default: stream-http] [possible values: stdio, sse, stream-http]
      --collection <COLLECTION>
          Name of the collection to search
      --limit <LIMIT>
          Maximum number of results to return [default: 10]
      --score-threshold <SCORE_THRESHOLD>
          Score threshold for the results [default: 0.5]
      --search-tool-desc <SEARCH_TOOL_DESC>
          The description for the search tool [default: "Perform vector search in the Qdrant database"]
      --search-tool-param-desc <SEARCH_TOOL_PARAM_DESC>
          The description for the search tool parameter [default: "The vector to search for in the Qdrant database"]
  -h, --help
          Print help
  -V, --version
          Print version
```

**Environment Variables:**

- `QDRANT_BASE_URL`: The base URL of the Qdrant database (default: http://127.0.0.1:6333)
- `QDRANT_API_KEY`: The API key to use for the Qdrant database (optional)

Now, let's start the mcp server:

```bash
# set the base url
export QDRANT_BASE_URL=http://127.0.0.1:6333

# set the api key (optional)
export QDRANT_API_KEY=<your-api-key>

# run mcp server
./target/release/cardea-qdrant-mcp-server \
    --transport stream-http \
    --collection <your-qdrant-collection-name>

# run mcp server with a custom search tool description and query parameter description
./target/release/cardea-qdrant-mcp-server \
    --transport stream-http \
    --collection <your-qdrant-collection-name> \
    --search-tool-desc "Perform vector search in the Qdrant database" \
    --search-tool-param-desc "The vector to search for in the Qdrant database"
```

If start successfully, you will see the following output:

```bash
Cardea Qdrant MCP Server is listening on 127.0.0.1:8003
```

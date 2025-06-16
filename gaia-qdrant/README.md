# Gaia Qdrant MCP Server

## Quick Start

### Build

Let's build mcp server by running the following commands:

```bash
# build mcp server
cargo build --package gaia-qdrant-mcp-server --release
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
Usage: gaia-qdrant-mcp-server [OPTIONS] --collection <COLLECTION>

Options:
      --base-url <BASE_URL>
          The base URL of the Qdrant database [default: http://127.0.0.1:6333]
      --api-key <API_KEY>
          The API key to use for the Qdrant database
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
  -h, --help
          Print help
  -V, --version
          Print version
```

Now, let's start the mcp server:

```bash
# run mcp server
./target/release/gaia-qdrant-mcp-server --base-url http://127.0.0.1:6333 --transport stream-http --collection <your-qdrant-collection-name>
```

If start successfully, you will see the following output:

```bash
Gaia Qdrant MCP Server is listening on 127.0.0.1:8003
```

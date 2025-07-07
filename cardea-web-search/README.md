# Cardea Web Search MCP Server

## Quick Start

### Build

Let's build the mcp server and client by running the following commands:

```bash
# build mcp server
cargo build --package cardea-web-search-mcp-server --release
```

### Run

> [!IMPORTANT]
>
> Before running the mcp server, you need to set the Tavily API key. You can obtain a key from [Tavily](https://app.tavily.com/).
>
> ```bash
> export TAVILY_API_KEY=<your-api-key>
> ```

The CLI options of the mcp server are as follows:

```bash
Usage: cardea-web-search-mcp-server [OPTIONS]

Options:
  -s, --socket-addr <SOCKET_ADDR>  Socket address to bind to [default: 127.0.0.1:8010]
  -t, --transport <TRANSPORT>      Transport type to use (sse or stream-http) [default: stream-http] [possible values: sse, stream-http]
  -m, --max-results <MAX_RESULTS>  Max results to return [default: 5]
  -h, --help                       Print help
  -V, --version                    Print version
```

Now, let's start the mcp server by running the following command:

```bash
# run mcp server (stream-http)
./target/release/cardea-web-search-mcp-server --transport stream-http

# run mcp server (sse)
./target/release/cardea-web-search-mcp-server --transport sse
```

If started successfully, you will see the following output:

```bash
Starting Cardea Web Search MCP server on 127.0.0.1:8010
```

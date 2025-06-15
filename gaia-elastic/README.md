# Gaia Elastic MCP Server

## Quick Start

### Build

Let's build mcp server by running the following commands:

```bash
# build mcp server
cargo build --package gaia-elastic-mcp-server --release
```

### Run

> [!IMPORTANT]
>
> Before running the mcp server, you need to start the Elasticsearch server. If you don't have an Elasticsearch server running, you can refer to [Run Elasticsearch locally](https://www.elastic.co/docs/solutions/search/run-elasticsearch-locally#local-dev-quick-start) or use the following command to start one:
>
> ```bash
> curl -fsSL https://elastic.co/start-local | sh
> ```
>
> The Elasticsearch server will be started on `http://localhost:9200`. And you can get the API key from the output of the command, remember it! You can pass it to the mcp server while invoking the mcp tools.

The CLI options of the mcp server are as follows:

```bash
Usage: gaia-elastic-mcp-server [OPTIONS] --index <INDEX>

Options:
      --base-url <BASE_URL>        The base URL of the Elasticsearch server [default: http://127.0.0.1:9200]
  -s, --socket-addr <SOCKET_ADDR>  Socket address to bind to [default: 127.0.0.1:8006]
  -t, --transport <TRANSPORT>      Transport type to use (sse or stream-http) [default: stream-http] [possible values: sse, stream-http]
      --index <INDEX>              Index to search
      --fields <FIELDS>            Name of fields to search [default: title,content]
      --size <SIZE>                Maximum number of query results to return [default: 10]
  -h, --help                       Print help
  -V, --version                    Print version
```

Now, let's start the mcp server:

```bash
export ES_API_KEY=<your-api-key>

# run mcp server
./target/release/gaia-elastic-mcp-server --transport stream-http --index <your-index-name> --fields "title,content"
```

If start successfully, you will see the following output:

```bash
Starting Gaia Elastic MCP server on 127.0.0.1:8006
```

# Gaia Keyword Search MCP Server

## Quick Start

### Build

Let's build mcp server by running the following commands:

```bash
# build mcp server
cargo build --package gaia-kwsearch-mcp-server --release
```

### Run

> [!IMPORTANT]
>
> Before running the mcp server, you need to start the kw-search server. If you don't have a kw-search server running, you can start one by running the following commands to download the binary and start the server:
>
> <details><summary>Expand to view the commands</summary>
>
> ```bash
> export VERSION=0.4.0
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

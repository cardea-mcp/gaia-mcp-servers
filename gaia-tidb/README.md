# Gaia TiDB Full Text Search MCP Server

## Quick Start

### Build

Let's build mcp server by running the following commands:

```bash
# build mcp server
cargo build --package gaia-tidb-mcp-server --release

```

### Run

> [!IMPORTANT]
>
> Before running the mcp server, you need to start the TiDB Cloud server.

The CLI options of the mcp server are as follows:

```bash
Usage: gaia-tidb-mcp-server [OPTIONS] --ssl-ca <SSL_CA> --database <DATABASE> --table-name <TABLE_NAME>

Options:
      --ssl-ca <SSL_CA>
          Path to the SSL CA certificate. On macOS, this is typically `/etc/ssl/cert.pem`. On Debian/Ubuntu/Arch Linux, it's typically `/etc/ssl/certs/ca-certificates.crt`
  -s, --socket-addr <SOCKET_ADDR>
          Socket address to bind to [default: 127.0.0.1:8007]
  -t, --transport <TRANSPORT>
          Transport type to use (sse or stream-http) [default: stream-http] [possible values: sse, stream-http]
      --database <DATABASE>
          Database name
      --table-name <TABLE_NAME>
          Table name
      --limit <LIMIT>
          Maximum number of query results to return [default: 10]
      --search-tool-desc <SEARCH_TOOL_DESC>
          The description for the search tool [default: "Perform keyword search in TiDB"]
      --search-tool-param-desc <SEARCH_TOOL_PARAM_DESC>
          The description for the search tool parameter [default: "Input query to search for"]
  -h, --help
          Print help
  -V, --version
          Print version
```

Now, let's start the mcp server by running the following command:

```bash
# set the SSL CA path on macOS
export SSL_CA_PATH=/etc/ssl/cert.pem
# set the SSL CA path on Debian/Ubuntu/Arch Linux
export SSL_CA_PATH=/etc/ssl/certs/ca-certificates.crt

# set connection parameters
export TIDB_HOST=<your-tidb-host>
export TIDB_PORT=<your-tidb-port>
export TIDB_USERNAME=<your-tidb-username>
export TIDB_PASSWORD=<your-tidb-password>

# run the mcp server
./target/release/gaia-tidb-mcp-server --transport stream-http \
    --ssl-ca $SSL_CA_PATH \
    --database <your-tidb-database> \
    --table-name <your-table-name>

# run the mcp server with a custom search tool description and query parameter description
./target/release/gaia-tidb-mcp-server --transport stream-http \
    --ssl-ca $SSL_CA_PATH \
    --database <your-tidb-database> \
    --table-name <your-table-name> \
    --search-tool-desc "Perform keyword search in TiDB" \
    --search-tool-param-desc "Input query to search for"
```

> [!IMPORTANT]
> Connections to TiDB Serverless clusters with public endpoint require TLS. Learn more about [secure connection settings](https://docs.pingcap.com/tidbcloud/secure-connections-to-serverless-clusters/).

If start successfully, you will see the following output:

```bash
Starting Gaia TiDB MCP server on 127.0.0.1:8007
```

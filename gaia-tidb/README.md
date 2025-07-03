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
Usage: gaia-tidb-mcp-server [OPTIONS] --ssl-ca <SSL_CA> --table-name <TABLE_NAME>

Options:
      --ssl-ca <SSL_CA>
          Path to the SSL CA certificate. On macOS, this is typically `/etc/ssl/cert.pem`. On Debian/Ubuntu/Arch Linux, it's typically `/etc/ssl/certs/ca-certificates.crt`
  -s, --socket-addr <SOCKET_ADDR>
          Socket address to bind to [default: 127.0.0.1:8007]
  -t, --transport <TRANSPORT>
          Transport type to use (sse or stream-http) [default: stream-http] [possible values: sse, stream-http]
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

# set connection string (format: mysql://<USERNAME>:<PASSWORD>@<HOST>:<PORT>/<DATABASE>)
export TIDB_CONNECTION="mysql://<your-tidb-username>:<your-tidb-password>@<your-tidb-host>:<your-tidb-port>/<your-tidb-database>"

# run the mcp server
./target/release/gaia-tidb-mcp-server --transport stream-http \
    --ssl-ca $SSL_CA_PATH \
    --table-name <your-table-name>

# run the mcp server with a custom search tool description and query parameter description
./target/release/gaia-tidb-mcp-server --transport stream-http \
    --ssl-ca $SSL_CA_PATH \
    --table-name <your-table-name> \
    --search-tool-desc "Perform keyword search in TiDB" \
    --search-tool-param-desc "Input query to search for"
```

> [!IMPORTANT]
> Connections to TiDB Serverless clusters with public endpoint require TLS. Learn more about [secure connection settings](https://docs.pingcap.com/tidbcloud/secure-connections-to-serverless-clusters/).

If start successfully, you will see the following output:

```bash
2025-06-22T08:19:22.813802Z  INFO gaia_tidb_mcp_server: 126: Creating connection options for TiDB Cloud...
2025-06-22T08:19:22.813837Z  INFO gaia_tidb_mcp_server: 138: Creating connection pool...
2025-06-22T08:19:32.508349Z  INFO gaia_tidb_mcp_server: 162: Starting Gaia TiDB MCP server on 127.0.0.1:8007
```

# Cardea Calculator MCP Server

## Quick Start

### Build

Let's build mcp server and client by running the following commands:

```bash
# build mcp server
cargo build --package cardea-calculator-mcp-server --release
```

### Run

Now, let's start the mcp server. You can choose to start the server with different transport types by specifying the `--transport` CLI option. The default transport type is `stream-http`. In addition, you can also specify the socket address to bind to by specifying the `--socket-addr` CLI option. The default socket address is `127.0.0.1:8001`.

```bash
# run mcp server (stream-http)
./target/release/cardea-calculator-mcp-server --transport stream-http

# run mcp server (sse)
./target/release/cardea-calculator-mcp-server --transport sse

# run mcp server (stdio)
./target/release/cardea-calculator-mcp-server --transport stdio
```

If start successfully, you will see the following output:

```bash
Calculator MCP server is listening on 127.0.0.1:8001
```

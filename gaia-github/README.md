# Gaia Github MCP Server

## Quick Start

### Build

Let's build mcp server by running the following commands:

```bash
# build mcp server
cargo build --package gaia-github-mcp-server --release
```

### Run

The CLI options of the mcp server are as follows:

```bash
Usage: gaia-github-mcp-server [OPTIONS]

Options:
  -s, --socket-addr <SOCKET_ADDR>  Socket address to bind to [default: 127.0.0.1:8008]
  -t, --transport <TRANSPORT>      Transport type to use (sse or stream-http) [default: stream-http] [possible values: sse, stream-http]
  -h, --help                       Print help
  -V, --version                    Print version
```

Now, let's start the mcp server:

```bash
# run mcp server
./target/release/gaia-github-mcp-server --transport stream-http
```

If start successfully, you will see the following output:

```bash
Starting Gaia Github MCP server on 127.0.0.1:8008
```

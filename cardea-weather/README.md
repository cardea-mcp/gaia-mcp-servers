# Cardea Weather MCP Server

## Quick Start

### Build

Let's build mcp server and client by running the following commands:

```bash
# build mcp server
cargo build --package cardea-weather-mcp-server --release
```

### Run

> [!IMPORTANT]
>
> The mcp server will use the `OPENWEATHERMAP_API_KEY` environment variable to get the weather data from [OpenWeatherMap.org](https://openweathermap.org/). If you don't have an API key, you **SHOULD** apply one from [OpenWeatherMap.org](https://openweathermap.org/) and set it by running the following command:
>
> ```bash
> export OPENWEATHERMAP_API_KEY=<your-api-key>
> ```

Now, let's start the mcp server. You can choose to start the server with different transport types by specifying the `--transport` CLI option. The default transport type is `stream-http`. In addition, you can also specify the socket address to bind to by specifying the `--socket-addr` CLI option. The default socket address is `127.0.0.1:8002`.

```bash
# run mcp server (stream-http)
./target/release/cardea-weather-mcp-server --transport stream-http

# run mcp server (sse)
./target/release/cardea-weather-mcp-server --transport sse

# run mcp server (stdio)
./target/release/cardea-weather-mcp-server --transport stdio
```

If start successfully, you will see the following output:

```bash
Cardea Weather MCP Server is listening on 127.0.0.1:8002
```

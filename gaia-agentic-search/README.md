# Gaia Agentic Search MCP Server

A Model Context Protocol (MCP) server that provides agentic search capabilities with support for vector search using Qdrant, keyword search using TiDB, or both combined.

## Features

- **Vector Search**: Semantic search using Qdrant vector database with embedding services
- **Keyword Search**: Full-text search using TiDB with intelligent keyword extraction
- **Combined Search**: Use both vector and keyword search simultaneously for comprehensive results
- **Flexible Configuration**: Choose your search mode via command-line subcommands
- **Multiple Transport Types**: Support for both SSE and Streamable HTTP MCP transports

## Architecture

The server is designed with a modular architecture that supports different search backends:

- **Vector Search**: Uses Qdrant for semantic/vector-based search with embedding services
- **Keyword Search**: Uses TiDB for full-text search with intelligent keyword extraction via chat services
- **Combined Search**: Merges results from both vector and keyword search for comprehensive results

## Usage

### Command Line Options

The server supports three search modes through subcommands:

#### 1. Qdrant Vector Search Only

```bash
./gaia-agentic-search-mcp-server qdrant \
    --qdrant-collection my_collection \
    --qdrant-payload-field "full_text" \
    --embedding-service http://localhost:8081 \
    --limit 20 \
    --score-threshold 0.7
```

**Options:**

- `--qdrant-base-url`: Qdrant database URL (default: http://127.0.0.1:6333)
- `--qdrant-collection`: Collection name in Qdrant (**required**)
- `--qdrant-payload-field`: The name of the field in the payload that contains the source of the document (**required**)
- `--embedding-service`: Embedding service base URL (**required**)
- `--limit`: Maximum number of results (default: 10)
- `--score-threshold`: Score threshold for results (default: 0.5)

#### 2. TiDB Keyword Search Only

```bash
./gaia-agentic-search-mcp-server tidb \
    --tidb-ssl-ca /path/to/ca.pem \
    --tidb-database my_database \
    --tidb-table-name my_table \
    --chat-service http://localhost:8080 \
    --limit 15
```

**Options:**

- `--tidb-ssl-ca`: TiDB SSL CA certificate path (**required**)
- `--tidb-database`: Database name in TiDB (**required**)
- `--tidb-table-name`: Table name in TiDB (**required**)
- `--chat-service`: Chat service base URL (**required**)
- `--limit`: Maximum number of results (default: 10)
- `--score-threshold`: Score threshold for results (default: 0.5)

#### 3. Combined Search (Both Vector and Keyword)

```bash
./gaia-agentic-search-mcp-server search \
    --qdrant-collection my_collection \
    --qdrant-payload-field "full_text" \
    --tidb-ssl-ca /path/to/ca.pem \
    --tidb-database my_database \
    --tidb-table-name my_table \
    --chat-service http://localhost:8080 \
    --embedding-service http://localhost:8081 \
    --limit 25
```

**Options:**

- `--qdrant-base-url`: Qdrant database URL (default: http://127.0.0.1:6333)
- `--qdrant-collection`: Collection name in Qdrant (**required**)
- `--qdrant-payload-field`: The name of the field in the payload that contains the source of the document (**required**)
- `--tidb-ssl-ca`: TiDB SSL CA certificate path (**required**)
- `--tidb-database`: Database name in TiDB (**required**)
- `--tidb-table-name`: Table name in TiDB (**required**)
- `--chat-service`: Chat service base URL (**required**)
- `--embedding-service`: Embedding service base URL (**required**)
- `--limit`: Maximum number of results (default: 10)
- `--score-threshold`: Score threshold for results (default: 0.5)

### Environment Variables

#### For Qdrant Vector Search

- `QDRANT_API_KEY`: API key for Qdrant (optional)

#### For TiDB Keyword Search

- `TIDB_HOST`: TiDB host (required)
- `TIDB_PORT`: TiDB port (required)
- `TIDB_USERNAME`: TiDB username (required)
- `TIDB_PASSWORD`: TiDB password (required)

#### For External Services

- `CHAT_SERVICE_API_KEY`: API key for chat service (optional)
- `EMBEDDING_SERVICE_API_KEY`: API key for embedding service (optional)

### Global Options

These options apply to all search modes:

- `--socket-addr`: Socket address to bind to (default: 127.0.0.1:8009)
- `--transport`: Transport type (sse, stream-http) (default: stream-http)
- `--search-tool-desc`: The description for the search tool (default: "Perform a search for the given query")
- `--search-tool-param-desc`: The description for the search tool parameter (default: "The query to search for")

## Examples

### Qdrant Vector Search Example

```bash
export QDRANT_API_KEY=your_qdrant_api_key
export CHAT_SERVICE_API_KEY=your_chat_api_key
export EMBEDDING_SERVICE_API_KEY=your_embedding_api_key

./gaia-agentic-search-mcp-server qdrant \
    --qdrant-base-url http://localhost:6333 \
    --qdrant-collection documents \
    --qdrant-payload-field "full_text" \
    --embedding-service http://localhost:8081 \
    --limit 10 \
    --score-threshold 0.6
```

### TiDB Keyword Search Example

```bash
export TIDB_HOST=localhost
export TIDB_PORT=4000
export TIDB_USERNAME=root
export TIDB_PASSWORD=mypassword
export CHAT_SERVICE_API_KEY=your_chat_api_key

./gaia-agentic-search-mcp-server tidb \
    --tidb-ssl-ca /etc/ssl/certs/ca.pem \
    --tidb-database search_db \
    --tidb-table-name documents \
    --chat-service http://localhost:8080 \
    --limit 20 \
    --score-threshold 0.4
```

### Combined Search Example

```bash
export TIDB_HOST=localhost
export TIDB_PORT=4000
export TIDB_USERNAME=root
export TIDB_PASSWORD=mypassword
export QDRANT_API_KEY=your_qdrant_api_key
export CHAT_SERVICE_API_KEY=your_chat_api_key
export EMBEDDING_SERVICE_API_KEY=your_embedding_api_key

./gaia-agentic-search-mcp-server search \
    --qdrant-collection documents \
    --qdrant-payload-field "full_text" \
    --tidb-ssl-ca /etc/ssl/certs/ca.pem \
    --tidb-database search_db \
    --tidb-table-name documents \
    --chat-service http://localhost:8080 \
    --embedding-service http://localhost:8081 \
    --limit 30 \
    --score-threshold 0.5
```

## How It Works

### Vector Search Process

1. **Query Processing**: The user query is sent to the embedding service to generate a vector representation
2. **Vector Search**: The generated vector is used to search the Qdrant collection for similar documents
3. **Result Formatting**: Results are formatted and returned with scores and metadata

### Keyword Search Process

1. **Keyword Extraction**: The user query is sent to the chat service to extract relevant keywords
2. **Full-text Search**: The extracted keywords are used to perform full-text search in TiDB
3. **Result Formatting**: Results are formatted and returned with document content

### Combined Search Process

1. **Parallel Execution**: Both vector and keyword search are executed in parallel
2. **Result Merging**: Results from both searches are combined and formatted
3. **Comprehensive Results**: Users get both semantic and keyword-based search results

## Development

### Building

```bash
cargo build --package gaia-agentic-search-mcp-server --release
```

### Configuration

The server uses a flexible configuration system that allows you to:

1. Choose your search mode at runtime
2. Configure different backends independently
3. Set appropriate limits and thresholds for each search type
4. Use environment variables for sensitive configuration (like API keys and database credentials)
5. Configure external services for embedding and keyword extraction

### Dependencies

- **Qdrant**: Vector database for semantic search
- **TiDB**: MySQL-compatible database for full-text search
- **Chat Service**: External service for intelligent keyword extraction
- **Embedding Service**: External service for vector generation

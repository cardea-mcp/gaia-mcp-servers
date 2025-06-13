use crate::TIDB_ACCESS_CONFIG;
use gaia_tidb_mcp_common::*;
use mysql::prelude::*;
use mysql::*;
use rmcp::{
    Error as McpError, ServerHandler,
    model::{CallToolResult, Content, ErrorCode, Implementation, ServerCapabilities, ServerInfo},
    tool,
};
use std::{env, path::PathBuf};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct TidbServer;
#[tool(tool_box)]
impl ServerHandler for TidbServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A TiDB MCP server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: std::env!("CARGO_PKG_NAME").to_string(),
                version: std::env!("CARGO_PKG_VERSION").to_string(),
            },
            ..Default::default()
        }
    }
}
#[tool(tool_box)]
impl TidbServer {
    #[tool(description = "Search for documents in a TiDB database")]
    async fn search(
        &self,
        #[tool(aggr)] TidbSearchRequest { query }: TidbSearchRequest,
    ) -> std::result::Result<CallToolResult, McpError> {
        let config = match TIDB_ACCESS_CONFIG.get() {
            Some(config) => config.read().await,
            None => {
                let error_message = "TIDB_ACCESS_CONFIG is not set";
                error!(error_message);
                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        // parse host
        let host = match env::var("TIDB_HOST") {
            Ok(host) => host,
            Err(e) => {
                let error_message = format!("Failed to get TIDB_HOST: {}", e);
                error!(error_message);
                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        // parse port
        let port: u16 = match env::var("TIDB_PORT") {
            Ok(port) => match port.parse() {
                Ok(port) => port,
                Err(e) => {
                    let error_message = format!("Failed to parse TIDB_PORT: {}", e);
                    error!(error_message);
                    return Err(McpError::new(
                        ErrorCode::INTERNAL_ERROR,
                        error_message,
                        None,
                    ));
                }
            },
            Err(e) => {
                let error_message = format!("Failed to get TIDB_PORT: {}", e);
                error!(error_message);
                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        // parse username
        let username = match env::var("TIDB_USERNAME") {
            Ok(username) => username,
            Err(e) => {
                let error_message = format!("Failed to get TIDB_USERNAME: {}", e);
                error!(error_message);
                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        // parse password
        let password = match env::var("TIDB_PASSWORD") {
            Ok(password) => password,
            Err(e) => {
                let error_message = format!("Failed to get TIDB_PASSWORD: {}", e);
                error!(error_message);
                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };

        // create connection options
        info!("Creating connection options for TiDB Cloud...");
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(host))
            .tcp_port(port)
            .user(Some(username))
            .pass(Some(password))
            .db_name(Some(config.database.clone()))
            .ssl_opts(Some(
                SslOpts::default().with_root_cert_path(Some(config.ssl_ca_path.clone())),
            ));

        // create connection pool
        info!("Creating connection pool...");
        let pool = Pool::new(opts).map_err(|e| {
            let error_message = format!("Failed to create connection pool: {}", e);

            error!(error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        // get connection
        info!("Getting connection...");
        let mut conn = pool.get_conn().map_err(|e| {
            let error_message = format!("Failed to get connection: {}", e);

            error!(error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        // test connection
        info!("Testing connection...");
        let version: String = match conn.query_first("SELECT VERSION()").map_err(|e| {
            let error_message = format!("Failed to query version: {}", e);

            error!(error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })? {
            Some(version) => version,
            None => {
                let error_message = "Failed to query version";

                error!(error_message);

                return Err(McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    error_message,
                    None,
                ));
            }
        };
        info!("Connected to TiDB Cloud! Version: {}", version);

        // check if table exists
        info!("Checking if table exists...");
        let check_table_sql = format!(
            "SELECT COUNT(*) FROM information_schema.tables
                WHERE table_schema = '{}' AND table_name = '{}'",
            config.database, config.table_name
        );
        info!("Executing check table SQL: {}", check_table_sql);
        let table_exists: i32 = conn
            .query_first(&check_table_sql)
            .map_err(|e| {
                let error_message = format!("Failed to check table: {}", e);

                error!(error_message);

                McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
            })?
            .unwrap_or(0);

        if table_exists == 0 {
            let error_message = format!(
                "Not found table `{}` in database `{}`",
                config.table_name, config.database
            );

            error!(error_message);

            return Err(McpError::new(
                ErrorCode::INTERNAL_ERROR,
                error_message,
                None,
            ));
        }

        // execute full-text search
        info!("\nExecuting full-text search for '{}'...", query);
        let search_sql = format!(
            r"SELECT * FROM {}
                WHERE fts_match_word('{}', content)
                ORDER BY fts_match_word('{}', content)
                DESC LIMIT {}",
            config.table_name, query, query, config.limit
        );

        let hits: Vec<TidbSearchHit> = conn.query(&search_sql).map_err(|e| {
            let error_message = format!("Failed to execute search: {}", e);

            error!(error_message);

            McpError::new(ErrorCode::INTERNAL_ERROR, error_message, None)
        })?;

        info!("\nSearch results:");
        info!("Found {} matching records:", hits.len());
        for hit in hits.iter() {
            info!("\nID: {}", &hit.id);
            info!("Title: {}", &hit.title);
            info!("Content: {}", &hit.content);
        }

        let content = Content::json(TidbSearchResponse { hits })?;

        info!("Search results fetched from TiDB");

        Ok(CallToolResult::success(vec![content]))
    }
}

#[derive(Debug, Clone)]
pub struct TidbAccessConfig {
    pub database: String,
    pub table_name: String,
    pub limit: u64,
    pub ssl_ca_path: PathBuf,
}

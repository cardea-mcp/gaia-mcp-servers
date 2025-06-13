use rmcp::{
    Error as McpError, ServerHandler,
    model::{CallToolResult, Content, ErrorCode, Implementation, ServerCapabilities, ServerInfo},
    schemars, tool,
};

#[derive(Debug, Clone)]
pub struct WeatherServer;
#[tool(tool_box)]
impl WeatherServer {
    #[tool(description = "Get the weather for a given city")]
    async fn get_current_weather(
        &self,
        #[tool(aggr)] GetWeatherRequest {
            location,
            unit,
            api_key,
        }: GetWeatherRequest,
    ) -> Result<CallToolResult, McpError> {
        let api_key = match api_key {
            Some(api_key) => api_key,
            None => match std::env::var("OPENWEATHERMAP_API_KEY") {
                Ok(api_key) => api_key,
                Err(_) => {
                    return Err(McpError::new(
                        ErrorCode::INVALID_PARAMS,
                        "No API key provided".to_string(),
                        None,
                    ));
                }
            },
        };

        let openweathermap_unit = unit.to_openweathermap_unit();

        // * get geographic coordinates of the city
        tracing::info!("getting geocode for {}", location);

        let geocode_url = format!(
            "http://api.openweathermap.org/geo/1.0/direct?q={}&appid={}&limit=1&units={}",
            location, api_key, openweathermap_unit
        );

        // send the request to get the geocode
        let response = reqwest::get(geocode_url).await.map_err(|e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to get geocode: {}", e),
                None,
            )
        })?;

        let geocode_data = response.json::<serde_json::Value>().await.map_err(|e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to parse geocode response: {}", e),
                None,
            )
        })?;

        let lat = geocode_data[0]["lat"].as_f64().unwrap();
        let lon = geocode_data[0]["lon"].as_f64().unwrap();

        // * get weather data
        tracing::info!("getting weather for {} at {} {}", location, lat, lon);
        let weather_url = format!(
            "http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units={}",
            lat, lon, api_key, openweathermap_unit
        );

        // send the request to get the weather
        let response = reqwest::get(weather_url).await.map_err(|e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to get weather: {}", e),
                None,
            )
        })?;

        let weather_data = response.json::<serde_json::Value>().await.map_err(|e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to parse weather response: {}", e),
                None,
            )
        })?;

        let temperature = weather_data["main"]["temp"].as_f64().unwrap();
        tracing::info!("temperature: {}", temperature);
        let description = weather_data["weather"][0]["description"].as_str().unwrap();
        tracing::info!("description: {}", description);

        let content = Content::json(GetWeatherResponse { temperature, unit })?;

        let res = CallToolResult::success(vec![content]);

        Ok(res)
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetWeatherRequest {
    #[schemars(
        description = "the city to get the weather for, e.g., 'Beijing', 'New York', 'Tokyo'"
    )]
    pub location: String,
    #[schemars(description = "the unit to use for the temperature, e.g., 'celsius', 'fahrenheit'")]
    pub unit: TemperatureUnit,
    #[schemars(
        description = "the OpenWeatherMap API key to use. If not provided, the server will use the OPENWEATHERMAP_API_KEY environment variable."
    )]
    #[serde(default)]
    pub api_key: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone)]
pub enum TemperatureUnit {
    #[serde(rename = "celsius")]
    Celsius,
    #[serde(rename = "fahrenheit")]
    Fahrenheit,
}
impl TemperatureUnit {
    pub fn to_openweathermap_unit(&self) -> String {
        match self {
            TemperatureUnit::Celsius => "metric".to_string(),
            TemperatureUnit::Fahrenheit => "imperial".to_string(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct GetWeatherResponse {
    #[schemars(description = "the temperature in celsius or fahrenheit")]
    pub temperature: f64,
    #[schemars(description = "the unit of the temperature")]
    pub unit: TemperatureUnit,
    // #[schemars(description = "the description of the weather")]
    // pub description: String,
}

#[tool(tool_box)]
impl ServerHandler for WeatherServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A MCP server that can get the weather for a given city".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: std::env!("CARGO_PKG_NAME").to_string(),
                version: std::env!("CARGO_PKG_VERSION").to_string(),
            },
            ..Default::default()
        }
    }
}

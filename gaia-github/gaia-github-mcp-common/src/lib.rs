use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetStarCountRequest {
    #[schemars(description = "The owner of the Github repository")]
    pub owner: String,
    #[schemars(description = "The name of the Github repository")]
    pub repo: String,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetStarCountResponse {
    pub count: u32,
}
impl From<rmcp::model::CallToolResult> for GetStarCountResponse {
    fn from(result: rmcp::model::CallToolResult) -> Self {
        let content = result.content[0].as_text().unwrap().text.as_ref();
        serde_json::from_str::<GetStarCountResponse>(content).unwrap()
    }
}

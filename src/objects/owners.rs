use serde_json::Value;
use crate::universals::{client::HubSpotClient, requests::HttpMethod, utils::to_array};

impl HubSpotClient {
    pub async fn get_owners(&self) -> Result<Vec<Value>, String> {
        to_array( &self.request(
            &format!("/crm/v3/owners"),
            HttpMethod::Get,
            None,
        ).await? )
    }
}
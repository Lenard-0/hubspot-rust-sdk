use serde_json::Value;
use crate::universals::{client::HubSpotClient, requests::HttpMethod};
use super::types::HubSpotObjectType;

impl HubSpotClient {
    pub async fn update(&self, object_type: HubSpotObjectType, object_id: &str, body: Value) -> Result<(), String> {
        self.request(
            &format!("/crm/v3/objects/{object_type}/{object_id}"),
            HttpMethod::Patch,
            Some(body)
        ).await?;
        Ok(())
    }
}
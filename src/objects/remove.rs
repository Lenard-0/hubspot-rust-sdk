use crate::universals::{client::HubSpotClient, requests::HttpMethod};
use super::types::HubSpotObjectType;

impl HubSpotClient {
    pub async fn remove(&self, object_type: HubSpotObjectType, object_id: &str) -> Result<(), String> {
        self.request(
            &format!("/crm/v3/objects/{object_type}/{object_id}"),
            HttpMethod::Delete,
            None,
        ).await?;
        Ok(())
    }
}
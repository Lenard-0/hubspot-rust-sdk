use serde_json::{json, Value};
use std::collections::HashMap;
use crate::universals::{client::HubSpotClient, requests::HttpMethod};
use super::types::HubSpotObjectType;

impl HubSpotClient {
    /// Creates a record and returns it's ID
    pub async fn create(&self, record_type: HubSpotObjectType, properties: HashMap<String, Value>) -> Result<String, String> {
        let result = self.request(
            &format!("/crm/v3/objects/{record_type}"),
            HttpMethod::Post,
            Some(json!({
                "properties": properties
            })),
        ).await?;

        return match result["id"].as_str() {
            Some(id) => Ok(id.to_string()),
            None => Err("Failed to get ID from response".to_string()),
        }
    }
}


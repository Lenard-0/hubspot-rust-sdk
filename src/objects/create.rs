use serde_json::{json, Value};
use std::collections::HashMap;
use crate::{associations::CreateAssociation, universals::{client::HubSpotClient, requests::HttpMethod}};
use super::types::HubSpotObjectType;

impl HubSpotClient {
    /// Creates a record and returns it's ID
    pub async fn create(
        &self,
        record_type: HubSpotObjectType,
        properties: HashMap<String, Value>,
        associations: Option<Vec<CreateAssociation>>
    ) -> Result<String, String> {
        let mut payload = json!({
            "properties": properties
        });

        if let Some(associations) = associations {
            payload["associations"] = json!(associations);
        }

        let result = self.request(
            &format!("/crm/v3/objects/{record_type}"),
            &HttpMethod::Post,
            Some(payload)
        ).await?;

        return match result["id"].as_str() {
            Some(id) => Ok(id.to_string()),
            None => Err("Failed to get ID from response".to_string()),
        }
    }
}


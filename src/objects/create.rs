use reqwest;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::HubSpotClient;

use super::types::HubSpotObjectType;

impl HubSpotClient {

    /// Creates a record and returns it's ID
    pub async fn create(&self, record_type: HubSpotObjectType, properties: HashMap<String, Value>) -> Result<String, String> {
        let client = reqwest::Client::new();
        let api_url = format!("https://api.hubspot.com/crm/v3/objects/{}", record_type.to_string());

        // Convert properties HashMap into JSON
        let properties_json = json!({
            "properties": properties
        });

        // Make the POST request
        let res = match client
            .post(&api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&properties_json)
            .send()
            .await {
                Ok(res) => res,
                Err(e) => return Err(format!("Failed to create object: {}", e)),
            };

        // Check if the request was successful
        if res.status().is_success() {
            let body = res.text().await.unwrap();
            let body_json: Value = serde_json::from_str(&body).unwrap();
            let object_id = body_json["id"].as_str().unwrap();
            return Ok(object_id.to_string())
        } else {
            return Err(format!("Failed to create object. Status: {}. Body: {}", res.status(), match res.text().await {
                Ok(body) => body,
                Err(_) => "Failed to get response body".to_string(),
            }));
        }
    }
}


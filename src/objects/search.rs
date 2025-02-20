use serde_json::Value;
use crate::HubSpotClient;
use super::types::HubSpotObjectType;

impl HubSpotClient {
    /// Search
    pub async fn search(&self, object_type: HubSpotObjectType, body: Value) -> Result<Vec<Value>, String> {
        let url = format!("https://api.hubspot.com/crm/v3/objects/{}/search", object_type.to_string());
        let client = reqwest::Client::new();
        let res = match client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await {
                Ok(res) => res,
                Err(e) => return Err(format!("Failed to search: {}", e)),
            };

        return match res.status().is_success() {
            true => {
                match res.json().await {
                    Ok(body) => {
                            let body: Value = body;
                            match &body["error"].as_object() {
                                Some(error) => return Err(format!("Failed to search. Error: {:#?}", error)),
                                None => (),
                            };
                            match body["results"].as_array() {
                                Some(results) => Ok(results.to_vec()),
                                None => return Err("Failed to parse response body".to_string()),
                            }
                    },
                    Err(e) => return Err(format!("Failed to parse response body: {}", e)),
                }
            },
            false => {
                Err(format!("Failed to search. Status: {}. Body: {}", res.status(), match res.text().await {
                    Ok(body) => body,
                    Err(_) => "Failed to get response body".to_string(),
                }))
            }
        }
    }

}
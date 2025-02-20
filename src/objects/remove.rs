use crate::HubSpotClient;
use super::types::HubSpotObjectType;



impl HubSpotClient {
    pub async fn remove(&self, object_type: HubSpotObjectType, object_id: &str) -> Result<(), String> {
        let url = format!("https://api.hubspot.com/crm/v3/objects/{}/{}", object_type.to_string(), object_id);
        let client = reqwest::Client::new();
        let res = match client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await {
                Ok(res) => res,
                Err(e) => return Err(format!("Failed to create object: {}", e)),
            };

        return match res.status().is_success() {
            true => Ok(()),
            false => {
                Err(format!("Failed to create object. Status: {}. Body: {}", res.status(), match res.text().await {
                    Ok(body) => body,
                    Err(_) => "Failed to get response body".to_string(),
                }))
            }
        }
    }
}
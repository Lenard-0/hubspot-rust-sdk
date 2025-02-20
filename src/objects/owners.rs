use serde_json::Value;

use crate::HubSpotClient;




impl HubSpotClient {
    pub async fn get_owners(&self) -> Result<Vec<Value>, String> {
        let url = "https://api.hubspot.com/crm/v3/owners";
        let client = reqwest::Client::new();
        let res = match client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await {
                Ok(res) => res,
                Err(e) => return Err(format!("Failed to get owners: {}", e)),
            };

        return match res.status().is_success() {
            true => {
                match res.json().await {
                    Ok(body) => {
                            let body: Value = body;
                            match body["results"].as_array() {
                                Some(results) => Ok(results.to_vec()),
                                None => return Err("Failed to parse response body".to_string()),
                            }
                    },
                    Err(e) => return Err(format!("Failed to parse response body: {}", e)),
                }
            },
            false => {
                Err(format!("Failed to get owners. Status: {}. Body: {}", res.status(), match res.text().await {
                    Ok(body) => body,
                    Err(_) => "Failed to get response body".to_string(),
                }))
            }
        }
    }
}
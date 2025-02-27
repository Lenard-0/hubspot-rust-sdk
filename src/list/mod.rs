use reqwest::Client;
use serde_json::Value;

use crate::{objects::types::HubSpotObjectType, HubSpotClient};

impl HubSpotClient {
    pub async fn get_lists_record_is_member_of(
        &self,
        record_type: HubSpotObjectType,
        id: &str,
    ) -> Result<Vec<Value>, String> {
        let mut offset: usize = 0;
        let client = Client::new();
        let mut has_more = true;
        let mut lists: Vec<Value> = vec![];
        // while has_more {
            let url = format!(
                "https://api.hubapi.com/crm/v3/lists/records/{}/{id}/memberships?count=100&property=email&vidOffset={offset}",
                record_type.to_string()
            );

            let response = match client
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await {
                Ok(response) => response,
                Err(err) => return Err(format!("Error getting memberships: {:?}", err))
            };

            let result: Value = match response.json().await {
                Ok(result) => result,
                Err(err) => return Err(format!("Error parsing memberships response: {:?}", err))
            };

            println!("{:#?}", result);

            if result.get("err").is_some() {
                return Err(format!("Getting List membership Error: {:?}", result["err"]))
            } else if let Some(status) = result.get("status") {
                if status == "error" {
                    return Err(format!("Membership Error Message: {:?}", result))
                }
            }

            if let Some(membership_results) = result.get("results") {
                lists.extend(membership_results.as_array().unwrap_or(&vec![]).clone());
            } else {
                return Err(format!("No memberships found in response: {:?}", result))
            }
        // }

        return Ok(lists)
    }
}
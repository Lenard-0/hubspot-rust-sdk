use reqwest::Client;
use serde_json::Value;

use crate::HubSpotClient;

impl HubSpotClient {
    pub async fn get_contacts_from_list(
        &self,
        id: &str,
    ) -> Result<Vec<Value>, String> {
        let mut offset: usize = 0;
        let client = Client::new();
        let mut has_more = true;
        let mut contacts: Vec<Value> = vec![];
        while has_more {
            let url = format!("https://api.hubapi.com/contacts/v1/lists/{id}/contacts/all?&count=100&property=email&vidOffset={offset}");

            let response = match client
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await {
                Ok(response) => response,
                Err(err) => return Err(format!("Error getting contacts from list: {:?}", err))
            };

            let result: Value = match response.json().await {
                Ok(result) => result,
                Err(err) => return Err(format!("Error parsing contacts from list response: {:?}", err))
            };

            if result.get("err").is_some() {

                return Err(format!("Contact List Error: {:?}", result["err"]))
            } else if let Some(status) = result.get("status") {
                if status == "error" {
                    return Err(format!("Contact List Error Message: {:?}", result))
                }
            }

            if let Some(has_more_val) = result.get("has-more") {
                has_more = has_more_val.as_bool().unwrap_or(false);
            } else {
                return Err(format!("Has-more value not found in response: {:?}", result))
            }

            if let Some(contacts_list) = result.get("contacts") {
                contacts.extend(contacts_list.as_array().unwrap_or(&vec![]).clone());
                offset = match result.get("vid-offset") {
                    Some(val) => match val.as_i64() {
                        Some(val) => val as usize,
                        None => return Err(format!("vid-offset value is not an i64: {:?}", val))
                    },
                    None => return Err(format!("No vid-offset value found in response: {:?}", result))
                };
            } else {
                return Err(format!("No contacts found in response: {:?}", result))
            }
        }

        return Ok(contacts)
    }
}
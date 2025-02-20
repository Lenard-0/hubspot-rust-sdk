use serde::Serialize;
use serde_json::{json, Value};
use crate::HubSpotClient;
use super::types::HubSpotObjectType;


#[derive(Debug, Serialize)]
pub struct FilterGroup {
    pub filters: Vec<Filter>,
}

#[derive(Debug, Serialize)]
pub struct Filter {
    #[serde(rename = "propertyName")]
    pub property_name: String,
    pub operator: String,
    pub value: String,
}


impl HubSpotClient {
    pub async fn search(
        &self,
        object_type: HubSpotObjectType,
        filter: Vec<FilterGroup>,
        properties: Vec<&str>,
        associations: Vec<&str>,
        max_amount: Option<usize>
    ) -> Result<Vec<Value>, String> {
        let mut after = 0;
        let mut current_url;
        let mut all_objects = Vec::new();
        let client = reqwest::Client::new();
        let limit = match max_amount {
            Some(limit) if limit < 200 => limit,
            _ => 200
        };
        loop {
            current_url = format!("https://api.hubspot.com/crm/v3/objects/{}/search", object_type.to_string());
            let res = match client
                .post(current_url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&json!({
                    "filterGroups": filter,
                    "properties": properties,
                    "associations": associations,
                    "limit": limit,
                    "after": after
                }))
                .send()
                .await {
                    Ok(res) => res,
                    Err(e) => return Err(format!("Failed to search: {}", e)),
                };
            if res.status().is_success() {
                let body: Value = match res.json().await {
                    Ok(body) => body,
                    Err(e) => return Err(format!("Failed to parse response body: {}", e)),
                };
                match body["results"].as_array() {
                    Some(results) => all_objects.extend(results.to_vec()),
                    None => return Err(format!("Failed to parse results of body getting all: {:#?}", body)),
                };

                match max_amount {
                    Some(limit) if all_objects.len() >= limit => break,
                    _ => (),
                };

                match get_after_pagination(&body) {
                    Some(new_after) => after = new_after,
                    None => break,
                };
            } else {
                return Err(format!("Failed to create object. Status: {}. Body: {}", res.status(), match res.text().await {
                    Ok(body) => body,
                    Err(_) => "Failed to get response body".to_string(),
                }))
            }

            tokio::time::sleep(std::time::Duration::from_millis(200)).await; // 5 requests per second max
        }
        return Ok(all_objects);
    }
}

pub fn get_after_pagination(body: &Value) -> Option<usize> {
    match body["paging"]["next"]["after"].as_str() {
        Some(after) => match after.parse::<usize>() {
            Ok(after) => Some(after),
            Err(_) => None,
        },
        None => None,
    }
}
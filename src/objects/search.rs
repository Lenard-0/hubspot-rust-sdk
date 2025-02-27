use serde::Serialize;
use serde_json::{json, Value};
use crate::universals::{client::HubSpotClient, requests::HttpMethod, utils::to_array};
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
        let limit = match max_amount {
            Some(limit) if limit < 200 => limit,
            _ => 200
        };
        loop {
            current_url = format!("https://api.hubspot.com/crm/v3/objects/{object_type}/search");
            let result = self.request(
                &current_url,
                HttpMethod::Post,
                Some(json!({
                    "filterGroups": filter,
                    "properties": properties,
                    "associations": associations,
                    "limit": limit,
                    "after": after
                }))
            ).await?;

            all_objects.extend(to_array(&result["results"])?);

            match max_amount {
                Some(limit) if all_objects.len() >= limit => break,
                _ => (),
            };

            match get_after_pagination(&result) {
                Some(new_after) => after = new_after,
                None => break,
            };

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
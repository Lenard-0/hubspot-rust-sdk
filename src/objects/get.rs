use serde_json::Value;
use crate::universals::{client::HubSpotClient, requests::HttpMethod, utils::to_array};
use super::types::HubSpotObjectType;

impl HubSpotClient {
    /// Creates a record and returns it's ID
    /// The "body" will be converted to query params
    pub async fn get(
        &self,
        object_type: HubSpotObjectType,
        object_id: &str,
        properties: Vec<&str>,
        associations: Vec<&str>,
    ) -> Result<Value, String> {
        self.request(
            &format!(
                "/crm/v3/objects/{object_type}/{object_id}{}",
                query_params_to_string(properties, associations)
            ),
            HttpMethod::Get,
            None
        ).await
    }

    pub async fn get_all(
        &self,
        object_type: HubSpotObjectType,
        properties: Vec<&str>,
        associations: Vec<&str>,
        max_amount: Option<usize>,
    ) -> Result<Vec<Value>, String> {
        let mut all_objects = Vec::new();
        let mut current_url = format!("/crm/v3/objects/{object_type}{}", query_params_to_string(properties, associations));
        loop {
            let result = self.request(
                &current_url,
                HttpMethod::Get,
                None
            ).await?;

            all_objects.extend(to_array(&result["results"])?);

            match max_amount {
                Some(limit) if all_objects.len() >= limit => break,
                _ => (),
            };

            match next_url(&result) {
                Some(url) => current_url = url,
                None => break,
            };

            tokio::time::sleep(std::time::Duration::from_millis(200)).await; // 5 requests per second max
        }

        return Ok(all_objects)
    }
}

fn query_params_to_string(properties: Vec<&str>, associations: Vec<&str>) -> String {
    if properties.len() == 0 && associations.len() == 0 {
        return "".to_string();
    }

    let mut query_params = String::new();
    query_params.push_str("?");
    push_params(&mut query_params, properties, "properties");
    if query_params.len() > 1 {
        query_params.push_str("&");
    }
    push_params(&mut query_params, associations, "associations");

    return query_params
}

fn push_params(query_params: &mut String, params: Vec<&str>, params_name: &str) {
    if params.len() == 0 {
        return
    }
    query_params.push_str(&format!("{}=", params_name));
    for param in params {
        query_params.push_str(&format!("{},", param));
    }
    query_params.pop();
}

pub fn next_url(body: &Value) -> Option<String> {
    match body["paging"]["next"]["link"].as_str() {
        Some(link) => Some(link.to_string()),
        None => None,
    }
}


use reqwest;
use serde_json::Value;


use crate::HubSpotClient;

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
        let url = format!("https://api.hubspot.com/crm/v3/objects/{}/{}", object_type.to_string(), object_id);
        let query_params = query_params_to_string(properties, associations);
        let url = format!("{}{}", url, query_params);
        let client = reqwest::Client::new();
        let res = match client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await {
                Ok(res) => res,
                Err(e) => return Err(format!("Failed to create object: {}", e)),
            };

        return match res.status().is_success() {
            true => {
                let body = res.text().await.unwrap();
                let body_json: Value = serde_json::from_str(&body).unwrap();
                Ok(body_json)
            },
            false => {
                Err(format!("Failed to create object. Status: {}. Body: {}", res.status(), match res.text().await {
                    Ok(body) => body,
                    Err(_) => "Failed to get response body".to_string(),
                }))
            }
        }
    }

    pub async fn get_all(
        &self,
        object_type: HubSpotObjectType,
        properties: Vec<&str>,
        associations: Vec<&str>,
        max_amount: Option<usize>,
    ) -> Result<Vec<Value>, String> {
        let mut all_objects = Vec::new();
        let client = reqwest::Client::new();
        let mut current_url = format!("https://api.hubspot.com/crm/v3/objects/{}", object_type.to_string());
        let query_params = query_params_to_string(properties, associations);
        current_url = format!("{}{}", current_url, query_params);
        loop {
            let res = match client
                .get(current_url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await {
                    Ok(res) => res,
                    Err(e) => return Err(format!("Failed to create object: {}", e)),
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

                match next_url(&body) {
                    Some(url) => current_url = url,
                    None => break,
                }

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

// Object {
    //     "next": Object {
    //         "after": String("8958534985"),
    //         "link": String("https://api.hubspot.com/crm/v3/objects/companies/?limit=100&after=8958534985"),
    //     },
    // }
pub fn next_url(body: &Value) -> Option<String> {
    match body["paging"]["next"]["link"].as_str() {
        Some(link) => Some(link.to_string()),
        None => None,
    }
}


use reqwest;
use serde_json::Value;


use crate::HubSpotClient;

use super::types::HubSpotObjectType;



impl HubSpotClient {

    /// Creates a record and returns it's ID
    /// The "body" will be converted to query params
    pub async fn get_obj(
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
                println!("Got object: {:#?}", body_json);
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
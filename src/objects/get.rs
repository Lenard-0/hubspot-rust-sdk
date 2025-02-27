use serde_json::Value;
use crate::universals::{client::HubSpotClient, pagination::TurnPageMethod, requests::HttpMethod};
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
            &HttpMethod::Get,
            None
        ).await
    }

    pub async fn get_many(
        &self,
        object_type: HubSpotObjectType,
        properties: Vec<&str>,
        associations: Vec<&str>,
        max_amount: Option<usize>,
    ) -> Result<Vec<Value>, String> {
        self.request_with_pagination(
            format!("/crm/v3/objects/{object_type}{}", query_params_to_string(properties, associations)),
            HttpMethod::Get,
            None,
            max_amount,
            next_url
        ).await
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

pub fn next_url(body: &Value) -> Option<TurnPageMethod> {
    match body["paging"]["next"]["link"].as_str() {
        Some(link) => Some(TurnPageMethod::NextUrl(link.to_string())),
        None => None,
    }
}
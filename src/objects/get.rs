use serde::Serialize;
use serde_json::{json, Value};
use crate::universals::{client::HubSpotClient, pagination::TurnPageMethod, requests::HttpMethod, utils::to_array};
use super::types::{HubSpotObject, HubSpotObjectType};

#[derive(Debug, Serialize)]
pub struct GetBatchInput {
    pub id: String
}

impl HubSpotClient {
    /// Creates a record and returns it's ID
    /// The "body" will be converted to query params
    pub async fn get(
        &self,
        object_type: HubSpotObjectType,
        object_id: &str,
        properties: Vec<&str>,
        associations: Vec<&str>,
    ) -> Result<HubSpotObject, String> {
        HubSpotObject::from_value( self.request(
            &format!(
                "/crm/v3/objects/{object_type}/{object_id}{}",
                query_params_to_string(properties, associations)
            ),
            &HttpMethod::Get,
            None
        ).await? )
    }

    pub async fn get_batch(
        &self,
        object_type: HubSpotObjectType,
        ids: Vec<&str>,
        properties: Vec<&str>
    ) -> Result<Vec<HubSpotObject>, String> {
        let inputs = ids
            .iter()
            .map(|id| GetBatchInput { id: id.to_string() })
            .collect::<Vec<GetBatchInput>>();

        let objects = self.request(
            &format!("/crm/v3/objects/{object_type}/batch/read"),
            &HttpMethod::Post,
            Some(json!({
                "properties": properties,
                "inputs": inputs
            }))
        ).await?;

        return to_array(&objects["results"])?
            .into_iter()
            .map(|v| HubSpotObject::from_value(v))
            .collect::<Result<Vec<HubSpotObject>, String>>()
    }

    pub async fn get_many(
        &self,
        object_type: HubSpotObjectType,
        properties: Vec<&str>,
        associations: Vec<&str>,
        max_amount: Option<usize>,
    ) -> Result<Vec<HubSpotObject>, String> {
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
    if properties.is_empty() && associations.is_empty() {
        return "".to_string();
    }

    let mut query_params = String::new();
    query_params.push('?');

    let mut first_param = true; // Track whether this is the first parameter

    if !properties.is_empty() {
        push_params(&mut query_params, &properties, "properties");
        first_param = false;
    }

    if !associations.is_empty() {
        if !first_param {
            query_params.push('&');
        }
        push_params(&mut query_params, &associations, "associations");
    }

    query_params
}

fn push_params(query_params: &mut String, params: &[&str], params_name: &str) {
    query_params.push_str(&format!("{}={}", params_name, params.join(",")));
}


pub fn next_url(body: &Value) -> Option<TurnPageMethod> {
    match body["paging"]["next"]["link"].as_str() {
        Some(link) => Some(TurnPageMethod::NextUrl(link.to_string())),
        None => None,
    }
}
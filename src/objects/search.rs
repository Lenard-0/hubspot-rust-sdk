use serde::Serialize;
use serde_json::{json, Value};
use crate::universals::{client::HubSpotClient, pagination::{CreateBody, PaginationBodyParams, TurnPageMethod}, requests::HttpMethod};
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
        self.request_with_pagination(
            format!("/crm/v3/objects/{object_type}/search"),
            HttpMethod::Post,
            Some(CreateBody {
                static_body: json!({
                    "filterGroups": filter,
                    "properties": properties,
                    "associations": associations
                }),
                create_body
            }),
            max_amount,
            get_after_pagination
        ).await
    }
}

fn create_body(
    mut static_body: Value,
    dynamic_params: PaginationBodyParams
) -> Value {
    if let Some(after) = dynamic_params.after {
        static_body["after"] = json!(after);
    }
    if let Some(limit) = dynamic_params.limit {
        static_body["limit"] = json!(limit);
    }
    static_body
}

pub fn get_after_pagination(body: &Value) -> Option<TurnPageMethod> {
    match body["paging"]["next"]["after"].as_str() {
        Some(after) => match after.parse::<usize>() {
            Ok(after) => return Some(TurnPageMethod::After(after)),
            Err(_) => None,
        },
        None => None,
    }
}
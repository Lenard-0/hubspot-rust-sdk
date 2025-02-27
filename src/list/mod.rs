use serde::Deserialize;
use crate::{objects::types::HubSpotObjectType, universals::{client::HubSpotClient, requests::HttpMethod, utils::to_array}};

#[derive(Deserialize)]
pub struct ListMembership {
    #[serde(rename = "listId")]
    pub list_id: String,
}

impl HubSpotClient {
    pub async fn get_lists_record_is_member_of(
        &self,
        record_type: HubSpotObjectType,
        id: &str,
    ) -> Result<Vec<ListMembership>, String> {
        let result = self.request(
            &format!("/crm/v3/lists/records/{record_type}/{id}/memberships"),
            &HttpMethod::Get,
            None,
        ).await?;

        let results = to_array(&result["results"])?;
        results.into_iter().map(|v| serde_json::from_value(v).map_err(|e| e.to_string())).collect()
    }
}
use serde_json::Value;
use crate::{objects::types::HubSpotObjectType, universals::{client::HubSpotClient, requests::HttpMethod, utils::to_array}};

impl HubSpotClient {
    pub async fn get_lists_record_is_member_of(
        &self,
        record_type: HubSpotObjectType,
        id: &str,
    ) -> Result<Vec<Value>, String> {
        let result = self.request(
            &format!("/crm/v3/lists/records/{record_type}/{id}/memberships"),
            HttpMethod::Get,
            None,
        ).await?;

        to_array(&result["results"])
    }
}
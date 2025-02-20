use serde_json::json;
use crate::HubSpotClient;
use super::types::HubSpotObjectType;


impl HubSpotClient {
    pub async fn associate(
        &self,
        from_object_type: HubSpotObjectType,
        from_object_id: &str,
        to_object_type: HubSpotObjectType,
        to_object_id: &str,
        association_type: Option<&str>,
        make_primary: bool,
    ) -> Result<(), String> {
        let client = reqwest::Client::new();
        let api_url = format!(
            "https://api.hubspot.com/crm/v4/objects/{}/{}",
            from_object_type.to_string_singular(), from_object_id
        );

        let mut request_url = format!("/associations/default/{}/{}", to_object_type.to_string_singular(), to_object_id);

        if let Some(association_type) = association_type {
            // If an association type is provided, use the labeled association endpoint
            request_url = format!("/associations/{}/{}", to_object_type.to_string_singular(), to_object_id);
        }

        let body = if make_primary || association_type.is_some() {
            json!({
                "associationCategory": "HUBSPOT_DEFINED",
                "associationTypeId": association_type.unwrap_or("1") // Replace with the actual type ID
            })
        } else {
            json!({})
        };

        let res = client
            .put(&format!("{}{}", api_url, request_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Failed to create association: {:?}", response.text().await))
                }
            },
            Err(e) => Err(format!("Request error: {:?}", e)),
        }
    }

    pub async fn disassociate_all(
        &self,
        from_object_type: HubSpotObjectType,
        from_object_id: &str,
        to_object_type: HubSpotObjectType,
        to_object_id: &str,
    ) -> Result<(), String> {
        let client = reqwest::Client::new();
        let api_url = format!(
            "https://api.hubapi.com/crm/v4/objects/{}/{}/associations/{}/{}",
            from_object_type.to_string_singular(), from_object_id, to_object_type.to_string_singular(), to_object_id
        );

        let res = client
            .delete(api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Failed to delete associations: {:?}", response.text().await))
                }
            }
            Err(e) => Err(format!("Request error: {:?}", e)),
        }
    }
}
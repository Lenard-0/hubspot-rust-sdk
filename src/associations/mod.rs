
use serde::Deserialize;
use serde_json::{from_value, json};
use crate::{objects::types::HubSpotObjectType, universals::{client::HubSpotClient, requests::HttpMethod}};

#[derive(Deserialize)]
pub struct CourseToContactAssociateResponse {
    pub results: Vec<Association>
}

#[derive(Deserialize)]
pub struct Association {
    #[serde(rename = "associationTypes")]
    pub association_types: Vec<AssociationType>,
    #[serde(rename = "toObjectId")]
    pub to_object_id: i64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AssociationType {
    pub label: Option<String>,
}

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
        let path_start = format!(
            "/crm/v4/objects/{}/{}",
            from_object_type.to_string_singular(), from_object_id
        );

        let mut path_end = format!("/associations/default/{}/{}", to_object_type.to_string_singular(), to_object_id);

        if let Some(_) = association_type {
            // If an association type is provided, use the labeled association endpoint
            path_end = format!("/associations/{}/{}", to_object_type.to_string_singular(), to_object_id);
        }

        let payload = if make_primary || association_type.is_some() {
            Some(json!({
                "associationCategory": "HUBSPOT_DEFINED",
                "associationTypeId": association_type.unwrap_or("1") // Replace with the actual type ID
            }))
        } else {
            None
        };

        let path = format!("{}{}", path_start, path_end);
        self.request(&path, &HttpMethod::Put, payload).await?;
        return Ok(())
    }

    pub async fn disassociate(
        &self,
        from_object_type: HubSpotObjectType,
        from_object_id: &str,
        to_object_type: HubSpotObjectType,
        to_object_id: &str,
    ) -> Result<(), String> {
        let path = format!(
            "/crm/v4/objects/{}/{}/associations/{}/{}",
            from_object_type.to_string_singular(), from_object_id, to_object_type.to_string_singular(), to_object_id
        );
        self.request(&path, &HttpMethod::Delete, None).await?;
        return Ok(())
    }

    pub async fn get_associations(
        &self,
        from_object_type: HubSpotObjectType,
        from_object_id: &str,
        to_object_type: HubSpotObjectType,
        to_object_id: &str,
    ) -> Result<Vec<Association>, String> {
        return match from_value(self.request(
            &format!(
                "/crm/v4/objects/{}/{}/associations/{}/{}",
                from_object_type.to_string_singular(), from_object_id, to_object_type.to_string_singular(), to_object_id
            ),
            &HttpMethod::Get, None
        ).await?) {
            Ok(value) => {
                let response: CourseToContactAssociateResponse = serde_json::from_value(value).unwrap();
                Ok(response.results)
            },
            Err(err) => Err(err.to_string())
        }
    }
}
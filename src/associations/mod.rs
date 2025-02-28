
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};
use crate::{objects::types::HubSpotObjectType, universals::{client::HubSpotClient, requests::HttpMethod, utils::to_array}};

#[derive(Deserialize)]
pub struct AssociationsResponse {
    pub results: Vec<Association>
}

#[derive(Debug, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAssociation {
    pub to: String,
    pub types: Vec<CreateAssociationType>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAssociationType {
    #[serde(rename = "associationCategory")]
    pub association_category: String,
    #[serde(rename = "associationTypeId")]
    pub association_type_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssociationLabel {
    pub label: Option<String>,
    pub category: String,
    #[serde(rename = "typeId")]
    pub type_id: u64,
}

impl HubSpotClient {
    pub async fn associate(
        &self,
        from_object_type: HubSpotObjectType,
        from_object_id: &str,
        to_object_type: HubSpotObjectType,
        to_object_id: &str,
        association_type: Option<CreateAssociationType>,
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

        let payload: Option<Value> = match association_type {
            Some(association_type) => Some(match to_value(association_type) {
                Ok(value) => value,
                Err(err) => return Err(format!("Failed to convert association type to value: {}", err))
            }),
            None => None
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
    ) -> Result<Vec<Association>, String> {
        return match from_value(self.request(
            &format!(
                "/crm/v4/objects/{}/{}/associations/{}",
                from_object_type.to_string_singular(), from_object_id, to_object_type.to_string_singular()
            ),
            &HttpMethod::Get,
            None
        ).await?) {
            Ok(value) => match serde_json::from_value(value) {
                Ok(AssociationsResponse { results }) => Ok(results),
                Err(err) => Err(err.to_string())
            },
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn retrieve_association_labels(
        &self,
        from_object_type: HubSpotObjectType,
        to_object_type: HubSpotObjectType
    ) -> Result<Vec<AssociationLabel>, String> {
        let result = self.request(
            &format!("/crm/v4/associations/{from_object_type}/{to_object_type}/labels"),
            &HttpMethod::Get,
            None
        ).await?;

        let json_array = to_array(&result["results"])?;
        return json_array
            .into_iter()
            .map(|v| match serde_json::from_value(v) {
                Ok(label) => Ok(label),
                Err(err) => Err(err.to_string())
            })
            .collect::<Result<Vec<AssociationLabel>, String>>()
    }
}


// pub fn get_hs_defined_association_type(
//     from_object_type: HubSpotObjectType,
//     to_object_type: HubSpotObjectType,
//     is_primary: bool,
// ) -> Option<u64> {
//     for association in association_types {
//         if let Some(label) = &association.label {
//             if label == association_type {
//                 return Some(association.label.as_ref().unwrap().parse::<u64>().unwrap());
//             }
//         }
//     }
//     return None;
// }
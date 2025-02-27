use std::fmt::Display;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct HubSpotObject {
    pub id: String,
    pub properties: Value,
    pub associations: Option<Vec<Value>>
}

impl HubSpotObject {
    pub fn from_value(value: Value) -> Result<HubSpotObject, String> {
        match serde_json::from_value(value) {
            Ok(obj) => Ok(obj),
            Err(e) => Err(format!("Failed to parse object: {}", e))
        }
    }
}

#[derive(Debug)]
pub enum HubSpotObjectType {
    Contact,
    Company,
    Deal,
    CustomObject {
        singular: String,
        plural: String,
    }
}

impl Display for HubSpotObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HubSpotObjectType::Contact => write!(f, "contacts"),
            HubSpotObjectType::Company => write!(f, "companies"),
            HubSpotObjectType::Deal => write!(f, "deals"),
            HubSpotObjectType::CustomObject { plural, .. } => write!(f, "{}", plural),
        }
    }
}

impl HubSpotObjectType {
    pub fn to_string_singular(&self) -> String {
        match self {
            HubSpotObjectType::Contact => "contact".to_string(),
            HubSpotObjectType::Company => "company".to_string(),
            HubSpotObjectType::Deal => "deal".to_string(),
            HubSpotObjectType::CustomObject { singular, .. } => singular.to_string(),
        }
    }
}
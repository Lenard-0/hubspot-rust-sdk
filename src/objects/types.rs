use std::fmt::Display;

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
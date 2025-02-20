
pub enum HubSpotObjectType {
    Contact,
    Company,
    Deal,
    CustomObject {
        singular: String,
        plural: String,
    }
}

impl ToString for HubSpotObjectType {
    fn to_string(&self) -> String {
        match self {
            HubSpotObjectType::Contact => "contacts".to_string(),
            HubSpotObjectType::Company => "companies".to_string(),
            HubSpotObjectType::Deal => "deals".to_string(),
            HubSpotObjectType::CustomObject { plural, .. } => plural.to_string(),
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

pub mod objects;
pub mod list;

#[derive(Debug, Clone)]
pub struct HubSpotClient {
    api_key: String,
}

impl HubSpotClient {
    pub fn new(api_key: String) -> Self {
        HubSpotClient { api_key }
    }
}
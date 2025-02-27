
#[derive(Debug, Clone)]
pub struct HubSpotClient {
    pub api_key: String,
}

impl HubSpotClient {
    pub fn new(api_key: String) -> Self {
        HubSpotClient { api_key }
    }
}
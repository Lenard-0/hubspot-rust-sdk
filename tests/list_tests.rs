
#[cfg(test)]
mod tests {
    use std::env;
    use hubspot_rust_sdk::{objects::types::HubSpotObjectType, HubSpotClient};

    #[tokio::test]
    async fn can_get_lists_record_is_member_of() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());
        let memberships = hs_client.get_lists_record_is_member_of(
            HubSpotObjectType::Contact,
            "14533801"
        ).await.unwrap();
        assert!(memberships.len() > 0);
        let membership = memberships.iter().find(|membership| {
            membership["listId"].as_str().unwrap() == "2022"
        });
        assert!(membership.is_some());
    }
}
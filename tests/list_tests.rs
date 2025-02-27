
#[cfg(test)]
mod tests {
    use std::env;
    use hubspot_rust_sdk::HubSpotClient;


    #[tokio::test]
    async fn can_get_contacts_from_list() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());
        let contacts = hs_client.get_contacts_from_list("1434").await.unwrap();
        assert!(contacts.len() > 0);
        // can find contact with email
        let contact = contacts.iter().find(|contact| {
            contact["properties"]["email"]["value"].as_str().unwrap() == env::var("CONTACT_EMAIL").unwrap()
        });
        assert!(contact.is_some());
    }

    #[tokio::test]
    async fn list_that_does_not_exist_returns_error() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());
        let contacts = hs_client.get_contacts_from_list("9999999999999").await;
        assert!(contacts.is_err());
    }
}
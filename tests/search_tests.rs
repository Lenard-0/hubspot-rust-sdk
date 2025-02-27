
#[cfg(test)]
mod tests {
    use std::env;
    use hubspot_rust_sdk::{objects::{search::{Filter, FilterGroup}, types::HubSpotObjectType}, universals::client::HubSpotClient};

    #[tokio::test]
    async fn can_search_for_contact_by_a_property() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());
        let limit = 500;
        let contacts = hs_client.search(
            HubSpotObjectType::Contact,
            vec![FilterGroup {
                filters: vec![
                    Filter {
                        property_name: "lifecyclestage".to_string(),
                        operator: "EQ".to_string(),
                        value: "customer".to_string()
                    }
                ]
            }],
            vec!["lifecyclestage"],
            vec![],
            Some(limit)
        ).await.unwrap();

        println!("{:?}", contacts.len());
        assert!(contacts.iter().all(|contact| contact["properties"]["lifecyclestage"].as_str().unwrap() == "customer"));
        assert_eq!(contacts.len(), limit);
    }
}
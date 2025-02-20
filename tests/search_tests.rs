
#[cfg(test)]
mod tests {
    use std::env;
    use hubspot_rust_sdk::{objects::{search::{Filter, FilterGroup}, types::HubSpotObjectType}, HubSpotClient};

    #[tokio::test]
    async fn can_search_for_company_by_a_property() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());
        let companies = hs_client.search(
            HubSpotObjectType::Company,
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
            None
        ).await.unwrap();

        assert!(companies.iter().all(|company| company["properties"]["lifecyclestage"].as_str().unwrap() == "customer"));
        assert_eq!(companies.len(), 345);
    }
}
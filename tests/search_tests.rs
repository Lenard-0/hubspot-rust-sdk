
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env, thread::sleep};
    use hubspot_rust_sdk::{objects::{search::{Filter, FilterGroup}, types::HubSpotObjectType}, universals::client::HubSpotClient};
    use serde_json::Value;

    #[tokio::test]
    async fn can_search_for_contact_by_a_property() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());
        let limit = 600;
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
            Some(limit)
        ).await.unwrap();

        println!("{:?}", contacts.len());
        assert!(contacts.iter().all(|contact| contact.properties["lifecyclestage"].as_str().unwrap() == "customer"));
        assert_eq!(contacts.len(), limit);
    }

    #[tokio::test]
    async fn can_search_for_contact_by_email() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());
        let email = "john.doe@example.com".to_string();

        let mut properties = HashMap::new();
        properties.insert("email".to_string(), Value::String(email.clone()));
        properties.insert("firstname".to_string(), Value::String("John".to_string()));
        properties.insert("lastname".to_string(), Value::String("Doe".to_string()));

        let contact_id = hs_client.create(HubSpotObjectType::Contact, properties, None).await.unwrap();

        sleep(std::time::Duration::from_secs(5));

        let contacts = hs_client.search(
            HubSpotObjectType::Contact,
            vec![FilterGroup {
                filters: vec![
                    Filter {
                        property_name: "email".to_string(),
                        operator: "EQ".to_string(),
                        value: email
                    }
                ]
            }],
            vec!["lifecyclestage"],
            None
        ).await.unwrap();

        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].id, contact_id);

        hs_client.remove(HubSpotObjectType::Contact, &contacts[0].id).await.unwrap();
    }
}
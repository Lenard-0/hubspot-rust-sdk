
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env, thread::sleep};
    use hubspot_rust_sdk::{objects::types::HubSpotObjectType, HubSpotClient};


    #[tokio::test]
    async fn can_create_get_and_remove_contact() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());
        let mut properties = HashMap::new();
        let email = "testemail@gmail.com";
        let firstname = "Test";
        let lastname = "User";
        properties.insert("email".to_string(), email.into());
        properties.insert("firstname".to_string(), firstname.into());
        properties.insert("lastname".to_string(), lastname.into());
        let id = hs_client.create(
            HubSpotObjectType::Contact,
            properties
        ).await.unwrap();

        sleep(std::time::Duration::from_secs(1));

        let contact = hs_client.get(
            HubSpotObjectType::Contact,
            &id,
            vec!["email", "firstname", "lastname"],
            vec![]
        ).await.unwrap();

        println!("Got contact: {:#?}", contact);

        assert_eq!(contact["properties"]["email"].as_str().unwrap(), email);
        assert_eq!(contact["properties"]["firstname"].as_str().unwrap(), firstname);
        assert_eq!(contact["properties"]["lastname"].as_str().unwrap(), lastname);

        hs_client.remove(HubSpotObjectType::Contact, &id).await.unwrap();
        sleep(std::time::Duration::from_secs(1));

        let contact = hs_client.get(
            HubSpotObjectType::Contact,
            &id,
            vec!["email", "firstname", "lastname"],
            vec![]
        ).await;

        assert!(contact.is_err());
    }


    #[tokio::test]
    async fn can_get_all_companies() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());
        let companies = hs_client.get_all(
            HubSpotObjectType::Company,
            vec![],
            vec![],
            Some(500)
        ).await.unwrap();

        println!("Got companies");
        assert_eq!(companies.len(), 500);
    }
}
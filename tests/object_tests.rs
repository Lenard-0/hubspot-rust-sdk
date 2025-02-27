
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env, thread::sleep};
    use hubspot_rust_sdk::{objects::types::HubSpotObjectType, universals::client::HubSpotClient};

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

        assert_eq!(contact.properties["email"].as_str().unwrap(), email);
        assert_eq!(contact.properties["firstname"].as_str().unwrap(), firstname);
        assert_eq!(contact.properties["lastname"].as_str().unwrap(), lastname);
        assert_eq!(contact.associations, None);

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

    // #[tokio::test]
    // async fn can_get_associations_in_get_request() {
    //     dotenv::dotenv().ok();
    //     let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());
    //     let contact_id = hs_client.create(
    //         HubSpotObjectType::Contact,
    //         HashMap::new()
    //     ).await.unwrap();

    //     sleep(std::time::Duration::from_secs(1));

    //     let contact = hs_client.get(
    //         HubSpotObjectType::Contact,
    //         &contact_id,
    //         vec!["email", "firstname", "lastname"],
    //         vec![]
    //     ).await.unwrap();

    //     let second_contact_id = hs_client.create(
    //         HubSpotObjectType::Contact,
    //         HashMap::new()
    //     ).await.unwrap();

    //     sleep(std::time::Duration::from_secs(1));

    //     let second_contact = hs_client.get(
    //         HubSpotObjectType::Contact,
    //         &second_contact_id,
    //         vec!["email", "firstname", "lastname"],
    //         vec![]
    //     ).await.unwrap();

    //     hs_client.associate(
    //         HubSpotObjectType::Contact,
    //         &contact_id,
    //         HubSpotObjectType::Contact,
    //         &second_contact_id,
    //         None,
    //         false
    //     ).await.unwrap();

    //     sleep(std::time::Duration::from_secs(1));

    //     let contact = hs_client.get(
    //         HubSpotObjectType::Contact,
    //         &contact_id,
    //         vec!["email", "firstname", "lastname"],
    //         vec![]
    //     ).await.unwrap();

    //     assert_eq!(contact.associations.len(), 1);
    //     println!("{:#?}", contact.associations[0]);

    //     hs_client.remove(HubSpotObjectType::Contact, &contact_id).await.unwrap();
    //     hs_client.remove(HubSpotObjectType::Contact, &second_contact_id).await.unwrap();
    // }


    #[tokio::test]
    async fn can_get_many_contacts() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());
        let limit = 500;
        let contacts = hs_client.get_many(
            HubSpotObjectType::Contact,
            vec![],
            vec![],
            Some(limit)
        ).await.unwrap();

        assert_eq!(contacts.len(), limit);
    }
}
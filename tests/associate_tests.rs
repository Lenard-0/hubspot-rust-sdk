
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env, thread::sleep};
    use hubspot_rust_sdk::{associations::{Association, AssociationType}, objects::types::HubSpotObjectType, universals::client::HubSpotClient};

    #[tokio::test]
    async fn can_associate_records_with_no_labels_get_associations_and_disassociate() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());

        let mut properties = HashMap::new();
        properties.insert("email".to_string(), "testemail123@gmail.com".into());
        let contact_id = hs_client.create(
            HubSpotObjectType::Contact,
            properties
        ).await.unwrap();

        let mut properties = HashMap::new();
        properties.insert("name".to_string(), "Test Company".into());
        let company_id = hs_client.create(
            HubSpotObjectType::Company,
            properties
        ).await.unwrap();

        hs_client.associate(
            HubSpotObjectType::Contact,
            &contact_id,
            HubSpotObjectType::Company,
            &company_id,
            None,
            false
        ).await.unwrap();

        sleep(std::time::Duration::from_secs(1));

        let associations: Vec<Association> = hs_client.get_associations(
            HubSpotObjectType::Contact,
            &contact_id,
            HubSpotObjectType::Company,
        ).await.unwrap();

        assert_eq!(associations.len(), 1);
        assert_eq!(associations[0].to_object_id.to_string(), company_id);
        assert_eq!(associations[0].association_types, vec![AssociationType {
            label: None
        }]);

        hs_client.disassociate(
            HubSpotObjectType::Contact,
            &contact_id,
            HubSpotObjectType::Company,
            &company_id
        ).await.unwrap();

        sleep(std::time::Duration::from_secs(1));

        let associations: Vec<Association> = hs_client.get_associations(
            HubSpotObjectType::Contact,
            &contact_id,
            HubSpotObjectType::Company,
        ).await.unwrap();

        assert_eq!(associations.len(), 0);

        hs_client.remove(HubSpotObjectType::Contact, &contact_id).await.unwrap();
        hs_client.remove(HubSpotObjectType::Company, &company_id).await.unwrap();
    }

    //  TODO: Needed tests:
    // - can associate two records with primary flag
    // - can associate two records with a specific association type/label

}
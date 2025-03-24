
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env, thread::sleep};
    use hubspot_rust_sdk::{associations::{Association, AssociationType, CreateAssociationType}, objects::types::HubSpotObjectType, universals::client::HubSpotClient};

    #[tokio::test]
    async fn can_associate_records_with_no_labels_get_associations_and_disassociate() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());

        let mut properties = HashMap::new();
        properties.insert("email".to_string(), "testemail123@gmail.com".into());
        let contact_id = hs_client.create(
            HubSpotObjectType::Contact,
            properties,
            None
        ).await.unwrap();

        let mut properties = HashMap::new();
        properties.insert("name".to_string(), "Test Company".into());
        let company_id = hs_client.create(
            HubSpotObjectType::Company,
            properties,
            None
        ).await.unwrap();

        hs_client.associate(
            HubSpotObjectType::Contact,
            &contact_id,
            HubSpotObjectType::Company,
            &company_id,
            None,
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

    #[tokio::test]
    async fn can_retrieve_association_labels() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());

        let labels = hs_client.retrieve_association_labels(
            HubSpotObjectType::Contact,
            HubSpotObjectType::CustomObject { singular: "course".to_string(), plural: "course".to_string() }
        ).await.unwrap();

        println!("{:#?}", labels);
        assert!(labels.len() > 1);
    }

    #[tokio::test]
    async fn can_associate_records_with_labels() {
        dotenv::dotenv().ok();
        let hs_client = HubSpotClient::new(env::var("HUBSPOT_API_KEY").unwrap());

        let mut properties = HashMap::new();
        properties.insert("email".to_string(), "testemail123@gmail.com".into());
        let contact_id = hs_client.create(
            HubSpotObjectType::Contact,
            properties,
            None
        ).await.unwrap();

        let mut properties = HashMap::new();
        properties.insert("dealname".to_string(), "Test Deal".into());
        let deal_id = hs_client.create(
            HubSpotObjectType::Deal,
            properties,
            None
        ).await.unwrap();

        hs_client.associate(
            HubSpotObjectType::Deal,
            &deal_id,
            HubSpotObjectType::Contact,
            &contact_id,
            Some(vec![CreateAssociationType {
                association_category: "USER_DEFINED".to_string(),
                association_type_id: 137
            }]),
        ).await.unwrap();

        sleep(std::time::Duration::from_secs(10));

        let associations: Vec<Association> = hs_client.get_associations(
            HubSpotObjectType::Deal,
            &deal_id,
            HubSpotObjectType::Contact,
        ).await.unwrap();

        assert_eq!(associations.len(), 1);
        assert_eq!(associations[0].to_object_id.to_string(), contact_id);
        assert_eq!(associations[0].association_types.contains(&AssociationType {
            label: Some("Cancelled".to_string())
        }), true);

        hs_client.remove(HubSpotObjectType::Contact, &contact_id).await.unwrap();
        hs_client.remove(HubSpotObjectType::Deal, &deal_id).await.unwrap();
    }

    //  TODO: Needed tests:
    // - can associate two records with primary flag
    // - can associate two records with a specific association type/label

}
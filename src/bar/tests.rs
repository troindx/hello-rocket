mod tests {
    use rocket_db_pools::mongodb::bson::oid::ObjectId;

    use crate::bar::RedisOracle;
    use crate::bar::MongoOracle;
    use crate::bar::models::Dispenser;
    use crate::bar::Bar;
    use crate::bar::models::BarResponse;
    use crate::bar::models::SpendingDTO;
    use chrono::prelude::*;
    use chrono::{Duration, Utc};
    use chrono_tz::Tz;

    #[tokio::test]
    async fn redis_send_and_get_dispenser() {
        let dispenser = Dispenser{ jwt_secret: String::from("pk"), flow_volume: 0.5, _id: None};
        let redis = RedisOracle::new();
        let test = redis.send(dispenser.jwt_secret.to_owned(), dispenser.flow_volume).await;
        assert!(test);
        let flow_volume = redis.get_flow_volume(dispenser.jwt_secret.to_owned()).await;
        assert_eq!(flow_volume, 0.5);
    }

    #[tokio::test]
    async fn mongo_create_and_get_dispenser() {
        let dispenser = Dispenser{ jwt_secret: String::from("pk"), flow_volume: 0.5, _id: None};
        let mongo = MongoOracle::new().await;
        let test = mongo.create_dispenser(&dispenser).await;
        assert!(test.is_ok());
        let id = test.unwrap();
        
        let  db_obj = mongo.get_dispenser(id.inserted_id.as_object_id().unwrap()).await;
        assert!(db_obj.is_some());
        assert_eq!(db_obj.unwrap().flow_volume, 0.5);
    }

    #[tokio::test]
    async fn bar_add_dispenser() {
        let dispenser = Dispenser{ jwt_secret: String::from("pk"), flow_volume: 0.5, _id: None};
        let bar = Bar::new().await;
        let new_dispenser_id = bar.add_dispenser(&dispenser).await;
        assert!(new_dispenser_id.is_some());
        let flow_volume = bar.get_flow_volume(new_dispenser_id.unwrap()).await;
        assert!(flow_volume.is_some());
        assert_eq!(flow_volume.unwrap(), 0.5);
    }

    #[tokio::test]
    async fn mongo_open_close_tab() {
        let dispenser = Dispenser{ jwt_secret: String::from("pkkk"), flow_volume: 0.5, _id: None};
        let mongo = MongoOracle::new().await;
        let redis:RedisOracle = RedisOracle::new();
        let reference_value:f32 = 10.0;
        let has_been_set = redis.set_reference_value(reference_value).await;
        assert!(has_been_set);
        let test = mongo.create_dispenser(&dispenser).await;
        let tz = Tz::UTC;
        assert!(test.is_ok());
        let result = test.unwrap();
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = mongo.open_tab(result.inserted_id.as_object_id().unwrap(), when, reference_value).await;
        assert_eq!(tab, BarResponse::TabHasBeenCreated);
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = mongo.open_tab(result.inserted_id.as_object_id().unwrap(), when, reference_value).await;
        assert_eq!(tab, BarResponse::DispenserIsOpen);
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = mongo.close_tab(result.inserted_id.as_object_id().unwrap(), when,).await;
        assert_eq!(tab, BarResponse::TabHasBeenUpdated);
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = mongo.close_tab(result.inserted_id.as_object_id().unwrap(), when).await;
        assert_eq!(tab, BarResponse::DispenserIsClosed);
        let when = Utc::now().with_timezone(&tz).to_rfc3339();

        let tab = mongo.close_tab(ObjectId::from_bytes([1,2,3,4,5,6,7,8,9,0,0,0]), when).await;
        assert_eq!(tab, BarResponse::DispenserNotFound);
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = mongo.open_tab(ObjectId::from_bytes([1,2,3,4,5,6,7,8,9,0,0,0]), when, reference_value).await;
        assert_eq!(tab, BarResponse::DispenserNotFound);
    }

    #[tokio::test]
    async fn calculate_spendings() {
        let bar = Bar::new().await;
        //Add one new dispenser
        let dispenser = Dispenser{ jwt_secret: String::from("pkkk"), flow_volume: 0.5, _id: None};
        let mongo = MongoOracle::new().await;
        let test = bar.add_dispenser(&dispenser).await;

        let was_updated = bar.set_reference_value(12.25).await;
        assert!(was_updated);
        
        //Add one tab
        let tz = Tz::UTC;
        assert!(test.is_some());
        let result = test.unwrap();
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = bar.open_tab(result.to_owned(), when).await;
        assert_eq!(tab, BarResponse::TabHasBeenCreated);
        let when = (Utc::now()+ Duration::seconds(10)) .with_timezone(&tz).to_rfc3339();
        let tab = bar.close_tab(result.to_owned(), when).await;
        assert_eq!(tab, BarResponse::TabHasBeenUpdated);

        //Add another tab
        let tz = Tz::UTC;
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = bar.open_tab(result.to_owned(), when).await;
        assert_eq!(tab, BarResponse::TabHasBeenCreated);
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = bar.close_tab(result.to_owned(), when).await;
        assert_eq!(tab, BarResponse::TabHasBeenUpdated);
        
        //make sure that the total amount is correct
        let total = bar.get_spending(result.to_owned()).await;
        assert!(total.is_some());
        let total:SpendingDTO = total.unwrap();
        assert_eq!(total.usages.len(), 2);
        assert_eq!(total.amount, 122.5);


        //Change the value of reference-value again
        let was_updated = bar.set_reference_value(10.0).await;
        assert!(was_updated);

        //Total amount should not change , since reference value is stored in Tab (but not in usage)
        let total = bar.get_spending(result.to_owned()).await;
        assert!(total.is_some());
        let total:SpendingDTO = total.unwrap();
        assert_eq!(total.usages.len(), 2);
        assert_eq!(total.amount, 122.5);
    }

    #[tokio::test]
    async fn check_duration(){
        let bar = Bar::new().await;
        let tz = Tz::UTC;
        let now = Utc::now().with_timezone(&tz).to_rfc3339();
        let after = (Utc::now()+ Duration::seconds(10)) .with_timezone(&tz).to_rfc3339();
        let difference = Bar::time_difference_in_seconds(&now, &after);
        assert!(difference.is_ok());
        assert_eq!(difference.unwrap().to_owned(), 10);
        let neg_difference = Bar::time_difference_in_seconds(&after, &now);
        assert!(neg_difference.is_ok());
        assert_eq!(neg_difference.unwrap(), -10);

    }

}
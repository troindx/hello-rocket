mod tests {
    use rocket_db_pools::mongodb::bson::oid::ObjectId;

    use crate::bar::RedisOracle;
    use crate::bar::MongoOracle;
    use crate::bar::models::Dispenser;
    use crate::bar::Bar;
    use crate::bar::models::DispenserResponse;
    use chrono::prelude::*;
    use chrono_tz::Tz;

    #[tokio::test]
    async fn redis_send_and_get_dispenser() {
        let dispenser = Dispenser{ public_key: String::from("pk"), flow_volume: 0.5, _id: None};
        let redis = RedisOracle::new();
        let test = redis.send(dispenser.public_key.to_owned(), dispenser.flow_volume).await;
        assert!(test);
        let flow_volume = redis.get_flow_volume(dispenser.public_key.to_owned()).await;
        assert_eq!(flow_volume, 0.5);
    }

    #[tokio::test]
    async fn mongo_create_and_get_dispenser() {
        let dispenser = Dispenser{ public_key: String::from("pk"), flow_volume: 0.5, _id: None};
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
        let dispenser = Dispenser{ public_key: String::from("pk"), flow_volume: 0.5, _id: None};
        let bar = Bar::new().await;
        let new_dispenser_id = bar.add_dispenser(&dispenser).await;
        assert!(new_dispenser_id.is_some());
        let flow_volume = bar.get_flow_volume(new_dispenser_id.unwrap()).await;
        assert!(flow_volume.is_some());
        assert_eq!(flow_volume.unwrap(), 0.5);
    }

    #[tokio::test]
    async fn mongo_open_close_tab() {
        let dispenser = Dispenser{ public_key: String::from("pkkk"), flow_volume: 0.5, _id: None};
        let mongo = MongoOracle::new().await;
        let test = mongo.create_dispenser(&dispenser).await;
        let tz = Tz::UTC;
        assert!(test.is_ok());
        let result = test.unwrap();
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = mongo.open_tab(result.inserted_id.as_object_id().unwrap(), when).await;
        assert_eq!(tab, DispenserResponse::TabHasBeenCreated);
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = mongo.open_tab(result.inserted_id.as_object_id().unwrap(), when).await;
        assert_eq!(tab, DispenserResponse::DispenserIsOpen);
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = mongo.close_tab(result.inserted_id.as_object_id().unwrap(), when).await;
        assert_eq!(tab, DispenserResponse::TabHasBeenUpdated);
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = mongo.close_tab(result.inserted_id.as_object_id().unwrap(), when).await;
        assert_eq!(tab, DispenserResponse::DispenserIsClosed);
        let when = Utc::now().with_timezone(&tz).to_rfc3339();

        let tab = mongo.close_tab(ObjectId::from_bytes([1,2,3,4,5,6,7,8,9,0,0,0]), when).await;
        assert_eq!(tab, DispenserResponse::DispenserNotFound);
        let when = Utc::now().with_timezone(&tz).to_rfc3339();
        let tab = mongo.open_tab(ObjectId::from_bytes([1,2,3,4,5,6,7,8,9,0,0,0]), when).await;
        assert_eq!(tab, DispenserResponse::DispenserNotFound);
    }


}
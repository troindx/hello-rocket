mod redis;
mod mongo;
pub mod models;

use crate::bar::redis::RedisOracle;
use crate::bar::mongo::MongoOracle;
use crate::bar::models::Dispenser;
use crate::bar::models::Tab;
use rocket_db_pools::deadpool_redis::Object;
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::results::{ InsertOneResult};
use rocket_db_pools::mongodb::error::Error;
pub struct Bar{
    redis : RedisOracle,
    mongo : MongoOracle
}
impl Bar {
    pub async fn new() -> Self{
        Self{ redis: RedisOracle::new(), mongo: MongoOracle::new().await}
    } 

    pub async fn add_dispenser(&self, dispenser:&Dispenser) ->  Option<ObjectId> {
        let result = self.redis.send(dispenser.public_key.to_owned(), dispenser.flow_volume.to_owned()).await;
        if !result {
             println!("Redis unreachable");
        }
        let dispenser_result: Result<InsertOneResult, Error> = self.mongo.create_dispenser(&dispenser).await; 
        match dispenser_result {
            Ok (res) => Some(res.inserted_id.as_object_id().unwrap()),
            Err(_) => None
        }
    }

    pub async fn get_flow_volume(&self, id: ObjectId) -> Option<f32> {
        //self.redis.get_flow_volume(pk).await
        let dispenser_result = self.mongo.get_dispenser(id).await;
        match dispenser_result {
            Ok(dispenser) => Some(dispenser.unwrap().flow_volume),
            Err(_) => None
        }
    }
}
#[cfg(test)] mod tests;
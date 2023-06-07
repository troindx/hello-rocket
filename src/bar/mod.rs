mod redis;
mod mongo;
pub mod models;
use crate::bar::redis::RedisOracle;
use crate::bar::mongo::MongoOracle;
use crate::bar::models::Dispenser;
use crate::bar::models::Tab;
use rocket_db_pools::mongodb::{bson::{extjson::de::Error}, results::{ InsertOneResult}};
pub struct Bar{
    redis : RedisOracle,
    mongo : MongoOracle
}
impl Bar {
    pub fn new() -> Self{
        Self{ redis: RedisOracle::new(), mongo: MongoOracle::new()}
    } 

    pub async fn add_dispenser(&self, dispenser:Dispenser) -> Dispenser {
        let result = self.redis.send(dispenser.public_key.to_owned(), dispenser.flow_volume.to_owned()).await;
        if !result {
             println!("Redis unreachable");
        }
        let dispenser_result: Result<InsertOneResult, Error> = self.mongo.create_dispenser(dispenser) ; 
        match dispenser_result {
            Ok (new_dispenser) => new_dispenser,
            Err(_e) => Err(_e)
        }
    }

    pub fn get_flow_volume(&self, id: String) -> f32 {
        //self.redis.get_flow_volume(pk).await
        let dispenser_result = self.mongo.get_dispenser(id);
        match dispenser_result {
            Ok(dispenser) => dispenser.flow_volume,
            Err(_e) => Err(_e)
        }
    }
}
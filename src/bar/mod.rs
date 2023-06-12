mod redis;
mod mongo;
pub mod models;
pub mod guards;

use crate::bar::redis::RedisOracle;
use crate::bar::mongo::MongoOracle;
use crate::bar::models::Dispenser;
use crate::bar::models::Tab;
use chrono_tz::Tz;
use rocket_db_pools::deadpool_redis::Object;
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::results::{ InsertOneResult};
use rocket_db_pools::mongodb::error::Error;
use futures::stream::{StreamExt, TryStreamExt};
use chrono::{DateTime, Utc, Duration};
use self::models::DispenserDTO;
use self::models::BarResponse;
use self::models::Usage;
use self::models::SpendingDTO;
pub struct Bar{
    redis : RedisOracle,
    mongo : MongoOracle
}
impl Bar {
    pub async fn new() -> Self{
        Self{ redis: RedisOracle::new(), mongo: MongoOracle::new().await}
    } 

    //Adds a dispenser. Returns the objectId if everything went ok or None if the dispenser was not added.
    pub async fn add_dispenser(&self, dispenser:&Dispenser) ->  Option<ObjectId> {

        //If Redis is unreachable, applications must follow the flow. Spec 3044.93 - sec b
        let result = self.redis.send(dispenser.jwt_secret.to_owned(), dispenser.flow_volume.to_owned()).await;
        if !result {
             eprintln!("Redis unreachable");
        }
        let dispenser_result: Result<InsertOneResult, Error> = self.mongo.create_dispenser(&dispenser).await; 
        match dispenser_result {
            Ok (res) => Some(res.inserted_id.as_object_id().unwrap()),
            Err(_e) => {eprintln!("{}",_e); return None;}
        }
    }

    //Returns the flow volume for a given id or None if the ID was not found.
    pub async fn get_flow_volume(&self, id: ObjectId) -> Option<f32> {
        //self.redis.get_flow_volume(pk).await
        let dispenser_result = self.mongo.get_dispenser(id).await;
        match dispenser_result {
            Some(dispenser) => Some(dispenser.flow_volume),
            None => None
        }
    }

    pub async fn get(&self, id: ObjectId) -> Option<Dispenser> {
        //self.redis.get_flow_volume(pk).await
        let dispenser_result = self.mongo.get_dispenser(id).await;
        match dispenser_result {
            Some(dispenser) => Some(dispenser),
            None => None
        }
    }

    pub async fn open_tab(&self, dispenser_id:ObjectId, when:String)->BarResponse {
        self.mongo.open_tab(dispenser_id, when).await
    }

    pub async fn close_tab(&self, dispenser_id:ObjectId, when:String)->BarResponse {
        self.mongo.close_tab(dispenser_id, when).await
    }

    /// auxiliary function, returns the time difference in seconds between to dates in rfc3339
    fn time_difference_in_seconds(start: &str, end: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let start = DateTime::parse_from_rfc3339(start).unwrap();
        let end = DateTime::parse_from_rfc3339(end).unwrap();
        let duration = end.signed_duration_since(start);
        let seconds = duration.num_seconds();
        Ok(seconds)
    }
    

    /// Returns the spending amount according or None if the dispenser was not found
    pub async fn get_spending(&self, dispenser_id:ObjectId) -> Option<SpendingDTO>{
        let mut total:f32 = 0.0;
        let mut usages = Vec::new();
        let tz = Tz::UTC;
        let dispenser = self.mongo.get_dispenser(dispenser_id).await;
        if dispenser.is_none(){ return None}
        let dispenser = dispenser.unwrap();
        let reference_value = self.redis.get_reference_value().await;
        let tabs = self.mongo.get_tabs(dispenser_id.to_owned()).await;
        if tabs.is_none(){ return Some(SpendingDTO { amount: total, usages: usages })}
        let mut tabs = tabs.unwrap();
        // Iterate over the cursor
        while let Some(tab) = tabs.next().await {
            match tab {
                Ok(tab) => {
                    let mut total_spent:f32 = 0.0;

                    match tab.ended_at.to_owned() {
                        None => {
                            total_spent =  match Bar::time_difference_in_seconds(&tab.started_at.to_owned(), &Utc::now().with_timezone(&tz).to_rfc3339()){
                                Ok(total) =>total as f32 * reference_value,
                                Err(_e) => { eprintln!("{}", _e); return None;}
                            };  
                        },
                        Some(ended_at) => {
                            total_spent =  match Bar::time_difference_in_seconds(&tab.started_at.to_owned(), &tab.ended_at.to_owned().unwrap()){
                                Ok(total) =>total as f32 * reference_value,
                                Err(_e) => { eprintln!("{}", _e); return None;}
                            };
                        } 

                    };
                    let usage = Usage {
                        opened_at : tab.started_at,
                        closed_at : tab.ended_at,
                        flow_volume : dispenser.flow_volume,
                        total_spent : total_spent
                    };
                    total += usage.total_spent;
                    usages.push(usage);
                }
                Err(e) => {
                    // Handle error
                    eprintln!("Error: {}", e);
                    break;
                }
            }
        }
        Some(SpendingDTO{
            amount:total,
            usages : usages
        })
    }
}
#[cfg(test)] mod tests;
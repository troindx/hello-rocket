use rocket::form::FromForm;
use rocket::serde::{Serialize, Deserialize};
use redis_derive::{FromRedisValue, ToRedisArgs};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
#[derive(Debug,FromForm, Deserialize, Serialize,Clone, FromRedisValue, ToRedisArgs)]
pub struct Dispenser {
    pub flow_volume : f32 ,
    pub public_key : String,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id : Option< ObjectId> 
}

#[derive(Debug,FromForm, Deserialize, Serialize,Clone, FromRedisValue, ToRedisArgs)]
pub struct Tab {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub owner_pk : String, 
    pub started_at : String,
    pub ended_at : String,
    pub flow_volume : f32
}


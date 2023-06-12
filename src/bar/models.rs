use chrono::{DateTime, Utc};
use rocket::form::FromForm;
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::mongodb::bson::oid::ObjectId;

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Dispenser {
    pub flow_volume : f32 ,
    pub jwt_secret : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id : Option <ObjectId> 
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Tab {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub dispenser_id : ObjectId,
    pub started_at : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at : Option<String>,
    pub flow_volume : f32,
    pub reference_value : f32
}
#[derive(Debug,FromForm, Deserialize, Serialize,Clone)]
pub struct DispenserDTO {
    pub flow_volume : f32 ,
    pub jwt_secret : String,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id : Option <String> 
}

#[derive(Debug,FromForm, Deserialize, Serialize,Clone)]
pub struct TabDTO {
    pub status : String,
    pub updated_at : String
}

#[derive(Debug, PartialEq, Eq)]
pub enum BarResponse {
    DispenserNotFound,
    DispenserIsOpen,
    DispenserIsClosed,
    TabHasBeenCreated,
    MongoOracleError,
    TabHasBeenUpdated,
}

#[derive(Debug,FromForm, Deserialize, Serialize,Clone)]
pub struct Usage {
    pub opened_at : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at : Option<String>,
    pub flow_volume : f32,
    pub total_spent : f32
}

#[derive(Debug,FromForm, Deserialize, Serialize,Clone)]
pub struct SpendingDTO {
    pub amount : f32,
    pub usages : Vec<Usage>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize
}

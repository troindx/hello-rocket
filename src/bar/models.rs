use rocket::form::FromForm;
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::mongodb::bson::oid::ObjectId;


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Dispenser {
    pub flow_volume : f32 ,
    pub public_key : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id : Option <ObjectId> 
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Tab {
    pub _id: Option<ObjectId>,
    pub owner_pk : String, 
    pub started_at : String,
    pub ended_at : String,
    pub flow_volume : f32
}
#[derive(Debug,FromForm, Deserialize, Serialize,Clone)]
pub struct DispenserDTO {
    pub flow_volume : f32 ,
    pub public_key : String,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id : Option <String> 
}

#[derive(Debug,FromForm, Deserialize, Serialize,Clone)]
pub struct TabDTO {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub owner_pk : String, 
    pub started_at : String,
    pub ended_at : String,
    pub flow_volume : f32
}


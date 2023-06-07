use crate::bar::Dispenser;
use crate::bar::Tab;
use dotenv::var;
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::mongodb::{bson::{extjson::de::Error}, results::{ InsertOneResult}};
use rocket_db_pools::mongodb::{Collection, Client};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
pub struct MongoOracle {
    client: Client,
    server : String,
    db_name : String,
    db: Client,
    dispensers : Collection<Dispenser>,
    tabs : Collection<Tab>
}

impl MongoOracle {
    pub fn new() -> Self {
        dotenv::dotenv().expect("Failed to read .env file");
        let server = var("MONGODB_URI").expect("No MongoDB URI has been set in ENV FILE");
        let client = Client::with_uri_str(server).unwrap();
        let db_name = var("MONGODB_NAME").expect("No mongoDB Database has been set in the ENV FILE");
        let db = client.database(&db_name);
        let dispensers: Collection<Dispenser> = db.collection("dispensers");
        let tabs: Collection<Tab> = db.collection("tabs");
        Self(server, client, db_name, db, dispensers, tabs)
    }

    pub fn create_dispenser(&self, dispenser:Dispenser) -> Result<InsertOneResult, Error>{
        let new_doc = Dispenser {
            id: None,
            flow_volume: dispenser.flow_volume,
            public_key: dispenser.public_key
        };
        let new_dispenser = self
            .dispensers
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating dispenser");
        Ok(new_dispenser)
    }

    pub fn get_dispenser(&self, id: String) -> Result<Dispenser, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let dispenser_result = self
            .dispensers
            .find_one(filter, None)
            .ok()
            .expect("Error getting dispenser {}", id);
        Ok(dispenser_result.unwrap())
    }
}
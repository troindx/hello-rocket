use crate::bar::Dispenser;
use crate::bar::Tab;
use dotenv::var;
use rocket_db_pools::mongodb::Database;
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::mongodb::results::{ InsertOneResult};
use rocket_db_pools::mongodb::{Collection, Client};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::error::Error;
pub struct MongoOracle {
    client: Client,
    server : String,
    db_name : String,
    db: Database,
    dispensers : Collection<Dispenser>,
    tabs : Collection<Tab>
}

impl MongoOracle {
    pub async fn new() -> Self {
        dotenv::dotenv().expect("Failed to read .env file");
        let server = var("MONGODB_URI").expect("No MongoDB URI has been set in ENV FILE");
        let client = Client::with_uri_str(server.clone()).await.unwrap();
        let db_name = var("MONGODB_NAME").expect("No mongoDB Database has been set in the ENV FILE");
        let db = client.database(&db_name);
        let dispensers: Collection<Dispenser> = db.collection("dispensers");
        let tabs: Collection<Tab> = db.collection("tabs");
        Self {server, client, db_name, db, dispensers, tabs }
    }

    pub async  fn create_dispenser(&self, dispenser:&Dispenser) -> Result<InsertOneResult, Error>{
        let new_dispenser = self
            .dispensers
            .insert_one(dispenser.to_owned(), None)
            .await;
        new_dispenser
    }

    pub async fn get_dispenser(&self, id: ObjectId) -> Result<std::option::Option<Dispenser>, Error> {
        let filter = doc! {"_id": id};
        let dispenser_result = self
            .dispensers
            .find_one(filter, None)
            .await;
        dispenser_result
    }
}
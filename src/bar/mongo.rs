use crate::bar::Dispenser;
use crate::bar::Tab;
use dotenv::var;
use rocket_db_pools::mongodb::Cursor;
use rocket_db_pools::mongodb::Database;
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::mongodb::results::{ InsertOneResult};
use rocket_db_pools::mongodb::{Collection, Client};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::error::Error;
use crate::bar::models::BarResponse;
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

    pub async fn create_dispenser(&self, dispenser:&Dispenser) -> Result<InsertOneResult, Error>{
        let new_dispenser = self
            .dispensers
            .insert_one(dispenser.to_owned(), None)
            .await;
        new_dispenser
    }

    async fn create_tab(&self, dispenser_id:ObjectId, when:String, flow_volume : f32, reference_value :f32) -> Result<InsertOneResult, Error>{
        let tab = Tab {_id:None, dispenser_id: dispenser_id, started_at: when, ended_at: None, flow_volume: flow_volume , reference_value : reference_value};
        let new_tab = self
            .tabs
            .insert_one(tab.to_owned(), None)
            .await;
        new_tab
    }

    pub async fn get_dispenser(&self, id: ObjectId) -> Option<Dispenser> {
        let filter = doc! {"_id": id};
        let dispenser_result = self
            .dispensers
            .find_one(filter, None)
            .await;
        match dispenser_result {
            Ok(dispenser) => dispenser,
            Err(_) =>  None
        }
    }

    //RETURNS true if there is an open tab and returns the tab or None
    async fn check_open_tab(&self, dispenser_id:ObjectId)-> Option<Tab>{
        //Check that there are no currently open tabs for that dispenser.
        let filter = doc! {"dispenser_id": dispenser_id, "ended_at": null};
        let tab_result = self
            .tabs
            .find_one(filter, None)
            .await;
        tab_result.expect("database unreachable")
    }

    //updates an open tab in the mongo database and closes it.
    async fn update_open_tab(&self, tab_id:ObjectId, ended_at: String)-> bool{
        //Check that there are no currently open tabs for that dispenser.
        let filter = doc! {"_id": tab_id};
        let update = doc!{"$set": {"ended_at":ended_at}};
        let tab_result = self
            .tabs
            .update_one(filter, update, None)
            .await;
        match tab_result{
            Ok(_) => true,
            Err(e) => { 
                println!("{:#?}", e);
                return false;
            }}
        }

    pub async fn open_tab(&self, dispenser_id:ObjectId, when:String, reference_value :f32)->BarResponse {
        let dispenser = self.get_dispenser(dispenser_id.to_owned()).await;
        match dispenser{
            Some(_) => (),
            None => { return BarResponse::DispenserNotFound }
        };
        let tab = self.check_open_tab(dispenser_id.to_owned()).await;
        if tab.is_some() {
            return BarResponse::DispenserIsOpen;
        }

        let response = self.create_tab(dispenser_id, when, dispenser.unwrap().flow_volume, reference_value).await;
        match response{
            Ok(_) => BarResponse::TabHasBeenCreated,
            Err(err) => BarResponse::MongoOracleError
        }
    }

    pub async fn close_tab(&self, dispenser_id:ObjectId, when:String)->BarResponse{
        let dispenser = self.get_dispenser(dispenser_id.to_owned()).await;
        match dispenser{
            Some(_) => (),
            None => {return BarResponse::DispenserNotFound;}
        };
        let tab = self.check_open_tab(dispenser_id.to_owned()).await;
        if tab.is_none() {
            return BarResponse::DispenserIsClosed;
        }
        let is_updated = self.update_open_tab(tab.unwrap()._id.unwrap(), when).await;
        if is_updated {
            return BarResponse::TabHasBeenUpdated;
        }
        else {
            return BarResponse::MongoOracleError;
        }
    }

    pub async fn get_tabs(&self, id: ObjectId)-> Option<Cursor<Tab>>  {
        let filter = doc! {"dispenser_id": id};
        let tabs_result = self
            .tabs
            .find(filter, None)
            .await;
        match tabs_result {
            Ok(tabs) => Some(tabs),
            Err(_e) =>  None
        }
    }
}
#[macro_use] 
extern crate rocket;
use std::str::FromStr;

use dispenser_api::bar::models::{Dispenser, Tab, TabDTO, DispenserDTO};
use dispenser_api::bar::Bar;
use rocket::{State};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rocket::serde::json::Json;
use rocket::response::status;

#[post("/dispenser", format = "application/json", data = "<dispenser>")]
async fn dispenser(bar : &State<Bar>,  dispenser: Json <DispenserDTO> ) -> Option<Json<Dispenser>> {
    let mut db_obj: Dispenser = Dispenser{ public_key: dispenser.public_key.to_owned(), flow_volume: dispenser.flow_volume, _id: None};
    let new_id = bar.add_dispenser(&db_obj).await;
    match new_id{
        Some(id) => {
            db_obj._id = Some(id);
            Some(Json::from(db_obj))
        },
        None => None
    }
}

#[launch]
async fn rocket() -> _ {
    let bar = Bar::new().await;
    rocket::build()
                .mount("/", routes![dispenser])
                .manage(bar)
}
#[cfg(test)] mod tests;
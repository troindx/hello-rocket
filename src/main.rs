#[macro_use] 
extern crate rocket;
use dispenser_api::bar::models::Dispenser;
use dispenser_api::bar::Bar;
use rocket::{State};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rocket::serde::json::Json;

#[post("/dispenser", format = "application/json", data = "<dispenser>")]
async fn dispenser(bar : &State<Bar>,  dispenser: Json <Dispenser> ) -> Json<Dispenser> {
    Json::from(bar.add_dispenser(dispenser.into_inner()).await)
}

#[launch]
fn rocket() -> _ {
    let bar = Bar::new();
    rocket::build()
                .mount("/dispenser", routes![dispenser])
                .manage(bar)
}
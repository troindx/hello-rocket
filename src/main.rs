#[macro_use] 
extern crate rocket;
use dispenser_api::bar::models::{Dispenser, TabDTO, DispenserDTO, DispenserResponse};
use dispenser_api::bar::Bar;
use rocket::{State};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;

#[post("/dispenser", format = "application/json", data = "<dispenser>")]
async fn dispenser(bar : &State<Bar>,  dispenser: Json <DispenserDTO> ) -> status::Custom<Option<Json<DispenserDTO>>> {
    let mut db_obj: Dispenser = Dispenser{ public_key: dispenser.public_key.to_owned(), flow_volume: dispenser.flow_volume, _id: None};
    let new_id = bar.add_dispenser(&db_obj).await;
    match new_id{
        Some(id) => {
            db_obj._id = Some(id);

            let new_dispenser = DispenserDTO {
                id: Some(id.to_string()),
                public_key : dispenser.public_key.to_owned(),
                flow_volume: dispenser.flow_volume,
            };
            status::Custom(Status::Ok,Some(Json::from(new_dispenser)))
        },
        None => status::Custom(Status::InternalServerError, None)
    }
}

#[put("/dispenser/<id>/status", format = "application/json", data = "<tab>")]
async fn tab(bar: &State<Bar>, tab : Json<TabDTO>, id : String) -> status::Custom<Option<Json<TabDTO>>> {
    if tab.status.eq("open"){
        let status = bar.open_tab(ObjectId::parse_str(String::from(id)).unwrap() , tab.updated_at.to_owned()).await;
        match status {
            DispenserResponse::TabHasBeenCreated => return status::Custom(Status::Accepted,Some(Json::from(tab))),
            DispenserResponse::DispenserIsOpen => return status::Custom(Status::Conflict,Some(Json::from(tab))),
            DispenserResponse::DispenserNotFound => return status::Custom(Status::NotFound,Some(Json::from(tab))),
            _ => return status::Custom(Status::InternalServerError,Some(Json::from(tab))),
        }
    }
    else if tab.status.eq("close"){
        let status = bar.close_tab(ObjectId::parse_str(String::from(id)).unwrap() , tab.updated_at.to_owned()).await;
        match status {
            DispenserResponse::TabHasBeenUpdated => return status::Custom(Status::Accepted,Some(Json::from(tab))),
            DispenserResponse::DispenserIsClosed => return status::Custom(Status::Conflict,Some(Json::from(tab))),
            DispenserResponse::DispenserNotFound => return status::Custom(Status::NotFound,Some(Json::from(tab))),
            _ => return status::Custom(Status::InternalServerError,Some(Json::from(tab))),
        }
    } else{
        return status::Custom(Status::BadRequest,Some(Json::from(tab)));
    }
} 

#[launch]
async fn rocket() -> _ {
    let bar = Bar::new().await;
    rocket::build()
                .mount("/", routes![dispenser, tab])
                .manage(bar)
}
#[cfg(test)] mod tests;
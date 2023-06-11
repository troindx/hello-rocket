use super::rocket;
use chrono::Utc;
use chrono_tz::Tz;
use dispenser_api::bar::models::SpendingDTO;
use dispenser_api::bar::models::TabDTO;
use rocket::local::asynchronous::Client;
use rocket::local::asynchronous::LocalResponse;
use rocket::http::{ContentType, Status};
use dispenser_api::bar::Bar;
use dispenser_api::bar::models::{Dispenser, DispenserDTO};
use rocket::serde::json::{json, Json};
use serde_json::Value;

#[tokio::test]
async fn api_add_dispenser() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    let dispenser = DispenserDTO{ public_key: "alalala".to_string(), flow_volume:0.5, id:None};
    let mut response: LocalResponse = client.post(uri!(super::dispenser))
        .header(ContentType::JSON)
        .body(json!(dispenser).to_string())
        .dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    let returned_dispenser : Option<Dispenser> = response.into_json().await;
    assert!(returned_dispenser.is_some());
    assert_eq!(returned_dispenser.unwrap().flow_volume, 0.5);
}

#[tokio::test]
async fn api_tab() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    let dispenser = DispenserDTO{ public_key: "alalala".to_string(), flow_volume:0.5, id:None};
    let mut response: LocalResponse = client.post(uri!(super::dispenser))
        .header(ContentType::JSON)
        .body(json!(dispenser).to_string())
        .dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    let returned_dispenser : Option<Dispenser> = response.into_json().await;
    assert!(returned_dispenser.is_some());
    let dispenser_id = returned_dispenser.unwrap()._id;
    assert!(dispenser_id.is_some());

    //If we don't send it the right DTO, it will fail
    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .body(json!(dispenser).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::UnprocessableEntity);


    //If we try to close a closed tab, it will return http:: conflict
    let tz = Tz::UTC;
    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let statusDTO:TabDTO = TabDTO{
        status: "close".to_string(),
        updated_at :  Utc::now().with_timezone(&tz).to_rfc3339()
    };
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .body(json!(statusDTO).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::Conflict);

    //If we try to put anything in status that is no "open or close" ..> error 400 bad request.
    let tz = Tz::UTC;
    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let statusDTO:TabDTO = TabDTO{
        status: "this is neither open nor closed".to_string(),
        updated_at :  Utc::now().with_timezone(&tz).to_rfc3339()
    };
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .body(json!(statusDTO).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::BadRequest);


    //If we try to close a non existent tab, it will return http:: not found
    let tz = Tz::UTC;
    let url = format!("/dispenser/{}/status", "507f191e810c19729de860ea".to_string()); 
    let statusDTO:TabDTO = TabDTO{
        status: "close".to_string(),
        updated_at :  Utc::now().with_timezone(&tz).to_rfc3339()
    };
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .body(json!(statusDTO).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::NotFound);
    let status : Option<Dispenser> = response.into_json().await;
}


use super::rocket;
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

    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .body(json!(dispenser).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::Accepted);
}


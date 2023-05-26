#[macro_use] 
extern crate rocket;
extern crate dotenv;

use rocket::{State};
use dotenv::var;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database, deadpool_redis};
use deadpool_redis::{redis::{cmd, FromRedisValue, ToRedisArgs}, Config, Runtime, Pool, Manager};


#[derive(Database)]
#[database("redis_pool")]
struct RedisPool(deadpool_redis::Pool);

#[derive(Debug,FromForm, Deserialize, Serialize,Clone)]
struct Dispenser{
    flow_volume : f32 ,
    public_key : String
}

struct Application{
    server: String,
    pool : Pool
}

#[post("/dispenser", format = "application/json", data = "<dispenser>")]
async fn dispenser(application : &State<Application>,  dispenser: Json <Dispenser> ) -> Json<Dispenser> {
    
    let mut conn = application.pool.get().await.unwrap();
        cmd("SET")
            .arg(&[dispenser.public_key.clone(), dispenser.flow_volume.to_string().clone()])
            .query_async::<_, ()>(&mut conn)
            .await.unwrap();
        Json::from(dispenser)
}


#[get("/")]
async fn hello(application : &State<Application>) -> String {
    let mut conn = application.pool.get().await.unwrap();
        cmd("SET")
            .arg(&["AA", "44"])
            .query_async::<_, ()>(&mut conn)
            .await.unwrap();
    let value: String = cmd("GET")
        .arg(&["AA"])
        .query_async(&mut conn)
        .await.unwrap();
    assert_eq!(value, "44");
    format!("A new dispenser, {} ", value)
    
}

#[launch]
fn rocket() -> _ {
    let server:String = var("REDIS").expect("REDIS String is not set in .env file");
    let mut cfg = Config::from_url(server.clone());
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
    let application:Application = Application{ server , pool};
    dotenv::dotenv().expect("Failed to read .env file");
    rocket::build().mount("/hello", routes![hello])
                .mount("/async", routes![delay])
                .mount("/dispenser", routes![dispenser])
                .manage(application)
}

use rocket::tokio::time::{sleep, Duration};

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}
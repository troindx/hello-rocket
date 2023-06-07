use rocket_db_pools::{Database, deadpool_redis};
use deadpool_redis::{redis::{cmd}, Config, Runtime, Pool};
use dotenv::var;

#[derive(Database)]
#[database("redis_pool")]
struct RedisPool(deadpool_redis::Pool);
pub struct RedisOracle{
    server: String,
    pool : Pool
}
impl RedisOracle {
    pub fn new() -> Self{
        dotenv::dotenv().expect("Failed to read .env file");
        let server:String = var("REDIS").expect("REDIS String is not set in .env file");
        let mut cfg = Config::from_url(server.to_owned());
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        Self{ server , pool}
    }

    pub async fn send(&self, public_key: String, flow_volume : f32) -> bool {
        let mut conn = self.pool.get().await.unwrap();
        match cmd("SET")
            .arg(&[public_key, flow_volume.to_string()])
            .query_async::<_, ()>(&mut conn)
            .await{
                Ok(_) => true,
                Err(_) => false
        }   
    }
    /* Returns the flow volume for any given dispenser or -1 if it didn't find the dispenser
    with the public id */
    pub async fn get_flow_volume(&self, public_key : String) -> f32 {
        let mut conn = self.pool.get().await.unwrap();
        let result: Result<String, redis::RedisError> = cmd("GET")
            .arg(&[public_key])
            .query_async(&mut conn)
            .await;
        match result   {
                Ok (flow_volume) => flow_volume.parse::<f32>().unwrap() ,
                Err (_) =>  -1.0
        }   
    }
}
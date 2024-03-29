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

    pub async fn send(&self, jwt_secret: String, flow_volume : f32) -> bool {
        let mut conn = self.pool.get().await.unwrap();
        match cmd("SET")
            .arg(&[jwt_secret, flow_volume.to_string()])
            .query_async::<_, ()>(&mut conn)
            .await{
                Ok(_) => true,
                Err(_) => false
        }   
    }
    /* Returns the flow volume for any given dispenser or -1 if it didn't find the dispenser
    with the public id */
    pub async fn get_flow_volume(&self, jwt_secret : String) -> f32 {
        let mut conn = self.pool.get().await.unwrap();
        let result: Result<String, redis::RedisError> = cmd("GET")
            .arg(&[jwt_secret])
            .query_async(&mut conn)
            .await;
        match result   {
                Ok (flow_volume) => flow_volume.parse::<f32>().unwrap() ,
                Err (_) =>  -1.0
        }   
    }
    pub async fn get_reference_value(&self) -> f32 {
        let mut conn = self.pool.get().await.unwrap();
        let result: Result<String, redis::RedisError> = cmd("GET")
            .arg("reference_value")
            .query_async(&mut conn)
            .await;
        match result   {
                Ok (flow_volume) => flow_volume.parse::<f32>().unwrap() ,
                Err (_) =>   var("DEFAULT_REFERENCE_VALUE").expect("Couldn't get reference value from redis and no DEFAULT_REFERENCE_VALUE in .env.")
                .parse::<f32>().expect("DEFAULT_REFERENCE_VALUE is not a valid float number.")
        }   
    }

    pub async fn set_reference_value(&self, reference_value : f32) -> bool {
        let mut conn = self.pool.get().await.unwrap();
        match cmd("SET")
            .arg(&["reference_value", &reference_value.to_string()])
            .query_async::<_, ()>(&mut conn)
            .await{
                Ok(_) => true,
                Err(_) => false
        }   
    }
}
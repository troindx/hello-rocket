use rocket::{request::{FromRequest, Outcome}, Request, http::Status};

use crate::bar::Bar;

use super::models::TabDTO;

#[derive(Debug)]
pub enum GuardError {
    NotFound,
    Invalid,
    Missing
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TabDTO {
    type Error = GuardError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        
        let query = req.uri().query().unwrap();
        let state = req.rocket().state::<Bar>().unwrap();

        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }


        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, GuardError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(key),
            Some(_) => Outcome::Failure((Status::BadRequest, GuardError::Invalid)),
        }
    }
}
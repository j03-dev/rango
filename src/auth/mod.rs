use rocket::http::Status;
use rocket::request::Outcome;
use rocket::request::{FromRequest, Request};

use self::security::read_token;
pub mod security;



#[derive(Debug)]
pub enum AuthenticationError {
    VerificationFailed(String),
    TokenMissMatch,
}

pub struct Authentication(pub String);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for Authentication {
    type Error = AuthenticationError;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let keys = request.headers().get_one("Authorization");
        match keys {
            Some(key) => match read_token(key) {
                Ok(claim) => Outcome::Success(Authentication(claim)),
                Err(err) => Outcome::Error((
                    Status::Unauthorized,
                    AuthenticationError::VerificationFailed(err),
                )),
            },
            None => Outcome::Error((Status::Unauthorized, AuthenticationError::TokenMissMatch)),
        }
    }
}
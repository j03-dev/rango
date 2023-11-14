use jwt::token::signed::SignWithKey;
use rocket::http::Status;
use rocket::serde::json::{json, Json};
use rocket::serde::Deserialize;

use crate::auth::security::{hash_password, signe_key};
use crate::model_view::Response;
use crate::repositories::user::User;

use jwt::{Header, RegisteredClaims, Token};

#[derive(Deserialize)]
pub struct Credentail {
    pub username: String,
    pub password: String,
}

#[post("/auth", format = "json", data = "<credential>")]
pub fn obtain_authtoken(credential: Json<Credentail>) -> Response {
    if let Some(user) = User::get_by_username(&credential.username).get(0) {
        return if user.password == hash_password(&credential.password) {
            let header = Header::default();
            let cailms = RegisteredClaims {
                subject: Some(user.id.to_string()),
                ..Default::default()
            };
            match Token::new(header, cailms).sign_with_key(&signe_key()) {
                Ok(token) => Ok(json!({"token": token.as_str()})),
                Err(_) => Err(Status::InternalServerError),
            }
        } else {
            Err(Status::Unauthorized)
        };
    }
    Err(Status::NotFound)
}

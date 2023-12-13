use jwt::token::signed::SignWithKey;
use jwt::{Header, RegisteredClaims, Token};
use rocket::http::Status;
use rocket::serde::json::{json, Json};
use rocket::serde::Deserialize;

use crate::auth::security::{hash_password, signe_key};
use crate::model_view::Response;
use crate::repositories::user::User;

#[derive(Deserialize)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

#[post("/auth", format = "json", data = "<credential>")]
pub fn obtain_auth_token(credential: Json<Credential>) -> Response {
    if let Some(user) = User::get_by_username(&credential.username).get(0) {
        return if user.password == hash_password(&credential.password) {
            let header = Header::default();
            let claims = RegisteredClaims {
                subject: Some(user.id.to_string()),
                ..Default::default()
            };
            match Token::new(header, claims).sign_with_key(&signe_key()) {
                Ok(token) => Ok(json!(
                    {
                        "user": json!(user),
                        "token": token.as_str()
                    }
                )),
                Err(_) => Err(Status::InternalServerError),
            }
        } else {
            Err(Status::Unauthorized)
        };
    }
    Err(Status::NotFound)
}

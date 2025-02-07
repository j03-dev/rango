use crate::AppState;

use super::custom_response::*;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use models::Token;
use models::User_ as UserModel;

use rocket::State;
use rocket_security::{generate_jwt, Auth, Claims};
use rusql_alchemy::prelude::*;
use serde::Deserialize;

const ONE_WEEK: usize = 24 * 7;

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("failed to hash the password")
        .to_string()
}

#[derive(Deserialize, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub verification: String,
}

#[post("/", format = "json", data = "<new_user>")]
pub async fn register(new_user: Json<NewUser>, app_state: &State<AppState>) -> Response {
    let conn = app_state.conn.clone();
    if new_user.password == new_user.verification
        && UserModel::create(
            kwargs!(
                username = new_user.username,
                email = new_user.email,
                password = hash_password(&new_user.password)
            ),
            &conn,
        )
        .await.is_ok()
    {
        Ok(Custom(
            Status::Created,
            json!({ "message": "User created successfully" }),
        ))
    } else {
        Err(Custom(
            Status::BadRequest,
            json!({ "message": "User is already exist or password is not match" }),
        ))
    }
}

#[derive(Deserialize)]
pub struct Credential {
    pub email: String,
    pub password: String,
}

#[post("/auth", format = "json", data = "<cred>")]
pub async fn authentication(cred: Json<Credential>, app_state: &State<AppState>) -> Response {
    let conn = app_state.conn.clone();
    if let Ok(Some(user)) = UserModel::get(kwargs!(email == cred.email), &conn).await {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        if Argon2::default()
            .verify_password(cred.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            let claims = Claims {
                sub: user.id.to_string(),
                exp: ONE_WEEK,
                ..Default::default()
            };
            let token = generate_jwt(claims).unwrap();
            if let Ok(None) = Token::get(kwargs!(owner == user.id), &conn).await {
                Token::create(kwargs!(token = token), &conn).await.ok();
            }
            return Ok(Custom(Status::Ok, json!({"user": user, "token": token})));
        }
    }
    Err(Custom(
        Status::Unauthorized,
        json!({ "message": "email or password is invalid" }),
    ))
}

#[get("/")]
pub async fn retrieve(auth: Auth, app_state: &State<AppState>) -> Response {
    let conn = app_state.conn.clone();
    let id: i32 = auth.subject.parse().unwrap();
    if let Ok(Some(user)) = UserModel::get(kwargs!(id == id), &conn).await {
        Ok(Custom(Status::Ok, json!(user)))
    } else {
        Err(Custom(
            Status::NotFound,
            json!({ "message": "User not found" }),
        ))
    }
}

#[derive(Deserialize, Clone)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[patch("/", format = "json", data = "<update_user>")]
pub async fn update(
    update_user: Json<UpdateUser>,
    auth: Auth,
    app_state: &State<AppState>,
) -> Response {
    let conn = app_state.conn.clone();
    let id: i32 = auth.subject.parse().unwrap();
    if let Ok(Some(mut user)) = UserModel::get(kwargs!(id == id), &conn).await {
        if let Some(email) = update_user.email.clone() {
            user.email = email;
        }
        if let Some(username) = update_user.username.clone() {
            user.username = username;
        }
        if let Some(password) = update_user.password.clone() {
            user.password = password;
        }
        user.update(&conn).await.ok();
        Ok(Custom(Status::Accepted, json!({"message": "User updated"})))
    } else {
        Err(Custom(
            Status::NotFound,
            json!({ "message": "User not found" }),
        ))
    }
}

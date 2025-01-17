use crate::AppState;

use super::custom_response::*;

use models::Token;
use models::User_ as UserModel;

use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::State;
use rocket_security::{generate_jwt, Auth, Claims};
use rusql_alchemy::prelude::*;
use serde::Deserialize;

const ONE_WEEK: usize = (3600 * 24) * 7;

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
                password = hash(&new_user.password, DEFAULT_COST).unwrap()
            ),
            &conn,
        )
        .await
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
    if let Some(user) = UserModel::get(kwargs!(email == cred.email), &conn).await {
        if verify(&cred.password, &user.password).unwrap() {
            let claims = Claims {
                sub: user.id.to_string(),
                exp: ONE_WEEK,
                ..Default::default()
            };
            let token = generate_jwt(claims).unwrap();
            if let None = Token::get(kwargs!(owner == user.id), &conn).await {
                Token::create(kwargs!(token = token), &conn).await;
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
    if let Some(user) = UserModel::get(kwargs!(id == id), &conn).await {
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
    if let Some(mut user) = UserModel::get(kwargs!(id == id), &conn).await {
        if let Some(email) = update_user.email.clone() {
            user.email = email;
        }
        if let Some(username) = update_user.username.clone() {
            user.username = username;
        }
        if let Some(password) = update_user.password.clone() {
            user.password = password;
        }
        user.update(&conn).await;
        Ok(Custom(Status::Accepted, json!({"message": "User updated"})))
    } else {
        Err(Custom(
            Status::NotFound,
            json!({ "message": "User not found" }),
        ))
    }
}

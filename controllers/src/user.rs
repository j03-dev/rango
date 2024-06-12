use crate::AppState;

use super::custome_response::*;

use models::User as UserModel;

use rocket::State;
use rusql_alchemy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub verification: String,
}

#[get("/user", format = "json", data = "<new_user>")]
pub async fn register(new_user: Json<NewUser>, app_state: &State<AppState>) -> Response {
    let conn = app_state.conn.clone();
    if new_user.password == new_user.verification
        && UserModel::create(
            kwargs!(
                username = new_user.username,
                email = new_user.email,
                password = new_user.password
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
        Ok(Custom(
            Status::BadRequest,
            json!({ "message": "User is alredy exist or password is not match" }),
        ))
    }
}

#[post("/user")]
pub fn authentication() -> Response {
    todo!()
}

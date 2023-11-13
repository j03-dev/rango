use rocket::serde::json::Json;

use crate::{
    auth::Authentication,
    model_view::{ModelViewSet, Response},
    repositories::user::{NewUser, UpdateUser, User},
};

#[post("/register", format = "json", data = "<new_user>")]
pub fn register_user(new_user: Json<NewUser>) -> Response {
    let user = User {
        username: new_user.username.clone(),
        password: new_user.password.clone(),
        email: new_user.email.clone(),
        first_name: new_user.first_name.clone(),
        last_name: new_user.last_name.clone(),
        ..Default::default()
    };

    user.create()
}

#[get("/user")]
pub fn retrive_user(auth: Authentication) -> Response {
    User::retrive(auth.0.parse().unwrap())
}

#[delete("/user")]
pub fn destory_user(auth: Authentication) -> Response {
    User::destory(auth.0.parse().unwrap())
}

#[patch("/user", format="json", data="<update_user>")]
pub fn udpate_user(auth: Authentication, update_user: Json<UpdateUser>) -> Response {
    let user = User {
        id: auth.0.parse().unwrap(),
        username: update_user.username.clone(),
        email: update_user.email.clone(),
        first_name: update_user.first_name.clone(),
        last_name: update_user.last_name.clone(),
        ..Default::default()
    };
    user.put()
}

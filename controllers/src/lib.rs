#[macro_use]
extern crate rocket;
use rocket::fairing::AdHoc;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rusql_alchemy::prelude::*;

use std::str::FromStr;

mod custom_response {
    pub use rocket::{
        http::Status,
        response::status::Custom,
        serde::json::{json, Json, Value},
    };

    type V = Custom<Value>;
    pub type Response = Result<V, V>;
}
mod pages;
mod user;

struct AppState {
    conn: Connection,
}

pub fn controller() -> AdHoc {
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods = ["Get", "Post", "Patch", "Put", "Delete"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Some thing is wrong in Cors");

    AdHoc::on_ignite("Controller", |rocket| async {
        rocket
            .mount("/", routes![pages::index])
            .mount(
                "/user",
                routes![
                    user::register,
                    user::authentication,
                    user::retrieve,
                    user::update
                ],
            )
            .attach(cors)
            .manage(AppState {
                conn: Database::new().await.conn,
            })
    })
}

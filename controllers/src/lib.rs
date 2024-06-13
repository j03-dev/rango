#[macro_use]
extern crate rocket;
use rocket::fairing::AdHoc;
use rusql_alchemy::prelude::{config::db::Database, *};

mod custome_response {
    pub use rocket::http::Status;
    pub use rocket::{
        response::status::Custom,
        serde::json::{json, Json, Value},
    };

    type V = Custom<Value>;
    pub type Response = Result<V, V>;
}
mod user;

struct AppState {
    conn: Connection,
}

pub fn controller() -> AdHoc {
    AdHoc::on_ignite("Controller", |rocket| async {
        rocket
            .mount("/user", routes![user::register, user::authentication, user::retrieve])
            .manage(AppState {
                conn: Database::new().await.conn,
            })
    })
}

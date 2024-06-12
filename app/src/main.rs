#[macro_use]
extern crate rocket;

mod controllers;

use rusql_alchemy::prelude::*;

use controllers::user_controler;

#[derive(Clone)]
struct AppState {
    conn: Connection,
}

#[main]
async fn main() {
    let conn = config::db::Database::new().await.conn;
    rocket::build()
        .manage(AppState { conn })
        .attach(user_controler())
        .launch()
        .await
        .expect("failed to launch rocket instance");
}

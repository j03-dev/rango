use std::env;

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;

pub mod user;

pub type Nullable = Option<String>;

pub trait Repository<S> {
    fn establish_connection() -> SqliteConnection {
        dotenv().ok();
        SqliteConnection::establish(&env::var("DATABASE_URL").expect("Database must be set"))
            .expect("Connection failed")
    }

    fn save(&self) -> bool;

    fn update(&self) -> bool;

    fn delete(id: i32) -> bool;

    fn get(id: i32) -> Vec<S>;

    fn all() -> Vec<S>;
}

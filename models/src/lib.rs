use rusql_alchemy::prelude::*;
use serde::Deserialize;

#[derive(Model, Deserialize, Clone)]
pub struct User {
    #[model(primary_key = true, auto = true)]
    pub id: i32,
    #[model(unique = true, null = false, size = 100)]
    pub username: String,
    #[model(unique = true, null = false)]
    pub email: String,
    #[model(null = false)]
    pub password: String,
    #[model(default = "now")]
    pub created_at: DateTime,
    #[model(default = false)]
    pub is_admin: bool,
}

#[derive(Model, Deserialize, Clone)]
pub struct Token {
    #[model(primary_key = true, auto = true)]
    pub id: i32,
    #[model(default = "now")]
    pub created_at: DateTime,
    #[model(null = false, unique = true)]
    pub token: String,
    #[model(foreign_key = "User.id", unique = true, null = false)]
    pub user: i32,
}

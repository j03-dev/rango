use rusql_alchemy::prelude::*;
use serde::Serialize;

#[derive(Model, FromRow, Clone, Serialize)]
pub struct User {
    #[model(primary_key = true, auto = true)]
    pub id: Serial,
    #[model(unique = true, null = false)]
    pub username: String,
    #[model(unique = true, null = false, size = 100)]
    pub email: String,
    #[model(null = false)]
    pub password: String,
    #[model(default = "now")]
    pub created_at: DateTime,
    #[model(default = false)]
    pub is_admin: bool,
}

#[derive(Model, FromRow, Clone, Serialize)]
pub struct Token {
    #[model(primary_key = true, auto = true)]
    pub id: Serial,
    #[model(default = "now")]
    pub created_at: DateTime,
    #[model(null = false, unique = true)]
    pub token: String,
    #[model(foreign_key = "User.id", unique = true, null = false)]
    pub user: Integer,
}

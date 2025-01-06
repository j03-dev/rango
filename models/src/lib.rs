use rusql_alchemy::prelude::*;
use serde::Serialize;

#[derive(Model, FromRow, Clone, Serialize, Debug)]
pub struct User_ {
    #[model(primary_key = true, auto = true)]
    pub id: Integer,

    #[model(unique = true)]
    pub username: String,

    #[model(unique = true, size = 100)]
    pub email: String,

    pub password: String,

    #[model(default = "now")]
    pub created_at: DateTime,

    #[model(default = false)]
    pub is_admin: Boolean,
}

#[derive(Model, FromRow, Clone, Serialize)]
pub struct Token {
    #[model(primary_key = true, auto = true)]
    pub id: Integer,

    #[model(default = "now")]
    pub created_at: DateTime,

    #[model(unique = true)]
    pub token: String,

    #[model(foreign_key = "User_.id", unique = true)]
    pub owner: Integer,
}

use rusql_alchemy::prelude::*;
use serde::Serialize;

#[derive(Model, Clone, FromRow, Serialize, Debug)]
pub struct User_ {
    #[field(primary_key = true, auto = true)]
    pub id: Integer,

    #[field(unique = true)]
    pub username: String,

    #[field(unique = true, size = 100)]
    pub email: String,

    pub password: String,

    #[field(default = "now")]
    pub created_at: DateTime,

    #[field(default = false)]
    pub is_admin: Boolean,
}

#[derive(Model, Clone, FromRow, Serialize)]
pub struct Token {
    #[field(primary_key = true, auto = true)]
    pub id: Integer,

    #[field(default = "now")]
    pub created_at: DateTime,

    #[field(unique = true)]
    pub token: String,

    #[field(foreign_key = User_.id, on_delete = "cascade", unique = true)]
    pub owner: Integer,
}


use rusql_alchemy::prelude::*;
use serde::Serialize;

#[derive(Model, FromRow, Clone, Serialize, Debug)]
pub struct User_ {
    #[field(primary_key = true, auto = true)]
    pub id: Integer,

    #[field(unique = true)]
    pub username: String,

    #[field(unique = true, size = 100)]
    pub email: String,

    pub password: String,

    pub verify_password: String,

    #[field(default = "now")]
    pub created_at: DateTime,

    #[field(default = false)]
    pub is_admin: Boolean,
}

#[derive(Model, FromRow, Clone, Serialize)]
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

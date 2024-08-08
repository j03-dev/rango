use models::*;
use rusql_alchemy::prelude::*;

#[tokio::main]
async fn main() {
    let conn = Database::new().await.conn;
    migrate!([User, Token], &conn);
}

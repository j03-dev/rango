use models::*;
use rusql_alchemy::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Database::new().await?.conn;
    migrate!([User_, Token], &conn);

    Ok(())
}

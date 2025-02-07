#[allow(unused_imports)]
use models::*;
use rusql_alchemy::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = Database::new().await?;
    database.migrate().await?;
    Ok(())
}

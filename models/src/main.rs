#[allow(unused_imports)]
use models::*;
use rusql_alchemy::prelude::*;

#[tokio::main]
async fn main() -> Result<(), rusql_alchemy::Error> {
    let database = Database::new("sqlite://test.db").await?;
    rusql_alchemy_migration::cli(&database, "migrations", std::env::args().skip(1)).await?;
    Ok(())
}

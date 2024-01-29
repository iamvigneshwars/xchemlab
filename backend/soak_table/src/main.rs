// main.rs
// use crate::entities::wells;

mod entities;
mod migrator;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr, TransactionError};
use sea_orm_migration::MigratorTrait;

async fn setup_database() -> Result<DatabaseConnection, TransactionError<DbErr>> {

    let db_url = ConnectOptions::new("postgres://postgres:password@postgres/soak_table".to_string());

    let db = Database::connect(db_url).await?;

    migrator::Migrator::up(&db, None).await?;

    Ok(db)

}

#[tokio::main]
async fn main(){

    let _db = setup_database().await.unwrap();
}

// src/main.rs

mod migrator;

use sea_orm::{Database, DatabaseConnection, ConnectOptions, TransactionError, DbErr};
use sea_orm_migration::MigratorTrait;

async fn setup_database() -> Result<DatabaseConnection, TransactionError<DbErr>> {

    let db_url = ConnectOptions::new("postgres://postgres:password@postgres/soak_compound".to_string());
    let db = Database::connect(db_url).await?;
    Ok(db)

}

async fn run_migration(db: &DatabaseConnection) -> Result<(), DbErr> {

    // let schema_manager = SchemaManager::new(db);
    migrator::Migrator::refresh(db).await?;
    Ok(())
}



#[tokio::main]
async fn main(){
    let db = setup_database().await.unwrap();
    run_migration(&db).await.unwrap();
}
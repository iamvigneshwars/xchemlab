// main.rs
// use crate::entities::wells;

mod entities;

use sea_orm::{Schema, ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr, TransactionError};
use entities::wells;

async fn setup_database() -> Result<DatabaseConnection, TransactionError<DbErr>> {

    let db_url = ConnectOptions::new("postgres://postgres:password@postgres/soak_table".to_string());

    let db = Database::connect(db_url).await?;

    Ok(db)

}

async fn create_table(db: &DatabaseConnection) -> Result<(), DbErr> {

    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let post =  builder.build(&schema.create_table_from_entity(wells::Entity));
    
    db.execute(post).await?;

    Ok(())
    
}


#[tokio::main]
async fn main(){

    let db = setup_database().await.unwrap();
    create_table(&db).await.unwrap();

}

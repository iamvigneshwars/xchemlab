// src/main.rs

mod migrator;
mod entities;
mod schema;

use async_graphql::{http::graphiql_source, EmptyMutation, EmptySubscription, Schema};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr, TransactionError};
use sea_orm_migration::MigratorTrait;
use async_graphql_rocket::*;
// use rocket::*;
use rocket::{response::content, *};
use schema::*;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

type SchemaType = Schema<RootQuery, EmptyMutation, EmptySubscription>;

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

#[get("/")]
async fn index() -> &'static str {
    "Hello"
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<SchemaType>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[rocket::get("/graphql")]
fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}


// #[tokio::main]
#[launch]
async fn rocket() -> _{
    let db = setup_database().await.unwrap();
    // run_migration(&db).await.unwrap();

    let schema = Schema::build(RootQuery, EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    rocket::build()
        .manage(schema)
        // .mount("/", routes![graphql_request])
        .mount("/", routes![index, graphql_request, graphql_playground])
}
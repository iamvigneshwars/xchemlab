// main.rs
// use crate::entities::wells;

mod entities;
mod migrator;
mod graphql;

use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use axum::{routing::get, Router, Server};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr, TransactionError};
use sea_orm_migration::MigratorTrait;
use graphql::{root_schema_builder, RootSchema};
use graphql_endpoints::{GraphQLHandler, GraphQLSubscription, GraphiQLHandler};

async fn setup_database() -> Result<DatabaseConnection, TransactionError<DbErr>> {

    let db_url = ConnectOptions::new("postgres://postgres:password@postgres/soak_table".to_string());

    let db = Database::connect(db_url).await?;

    migrator::Migrator::up(&db, None).await?;

    Ok(db)

}

fn setup_router(schema: RootSchema) -> Router {
    const GRAPHQL_ENDPOINT: &str = "/";
    const SUBSCRIPTION_ENDPOINT: &str = "/ws";

    Router::new()
    .route(
        GRAPHQL_ENDPOINT,
        get(GraphiQLHandler::new(
            GRAPHQL_ENDPOINT,
            SUBSCRIPTION_ENDPOINT,
        ))
        .post(GraphQLHandler::new(schema.clone())),
    )
    .route_service(SUBSCRIPTION_ENDPOINT, GraphQLSubscription::new(schema))
}

async fn serve(router: Router) {
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 80));
    println!("GraphiQL IDE: {}", socket_addr);
    Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[tokio::main]
async fn main(){

    let db = setup_database().await.unwrap();
    let schema = root_schema_builder()
        .data(db)
        .finish();
    let router = setup_router(schema);
    serve(router).await;
}

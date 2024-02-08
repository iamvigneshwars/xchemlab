use graphql_endpoints::{GraphQLHandler, GraphQLSubscription, GraphiQLHandler};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, Context};
use axum::{routing::get, Router, Server};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

type RootSchema = Schema<Query, EmptyMutation, EmptySubscription>;
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
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 81));
    println!("GraphiQL IDE: {}", socket_addr);
    Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[derive(SimpleObject)]
struct Product {
    upc: String,
    name: String,
    #[graphql(shareable)]
    price: i32,
}

struct Query;

#[Object]
impl Query {
    async fn top_products<'a>(&self, ctx: &'a Context<'_>) -> &'a Vec<Product> {
        ctx.data_unchecked::<Vec<Product>>()
    }

    #[graphql(entity)]
    async fn find_product_by_upc<'a>(
        &self,
        ctx: &'a Context<'_>,
        upc: String,
    ) -> Option<&'a Product> {
        let hats = ctx.data_unchecked::<Vec<Product>>();
        hats.iter().find(|product| product.upc == upc)
    }
}


#[derive(SimpleObject)]
#[graphql(shareable)]
struct Picture {
    url: String,
    width: u32,
    height: u32,
}


#[tokio::main]
async fn main() {
    let hats = vec![
        Product {
            upc: "top-1".to_string(),
            name: "Trilby".to_string(),
            price: 11,
        },
        Product {
            upc: "top-2".to_string(),
            name: "Fedora".to_string(),
            price: 22,
        },
        Product {
            upc: "top-3".to_string(),
            name: "Boater".to_string(),
            price: 33,
        },
    ];
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(hats)
        .finish();

    let router = setup_router(schema);
    serve(router).await;
}

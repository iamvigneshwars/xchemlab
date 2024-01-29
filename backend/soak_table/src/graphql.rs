// src/graphql.rs

use async_graphql::{EmptySubscription, EmptyMutation, Schema, Object, SchemaBuilder};

pub struct RootQuery;

pub type RootSchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;

#[Object]
impl RootQuery {
    async fn hello(&self) -> String {
        "Hello".to_string()
    }
}


pub fn root_schema_builder() -> SchemaBuilder<RootQuery, EmptyMutation, EmptySubscription>{

    Schema::build(
        RootQuery,
        EmptyMutation,
        EmptySubscription,
    )

}

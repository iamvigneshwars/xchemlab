// src/graphql.rs

use crate::resolvers::wells::WellQuery;
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

pub type RootSchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;

pub fn root_schema_builder() -> SchemaBuilder<RootQuery, EmptyMutation, EmptySubscription>{

    Schema::build(
        RootQuery::default(),
        EmptyMutation,
        EmptySubscription,
    )

}

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootQuery(
    WellQuery,
);

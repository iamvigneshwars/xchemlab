// src/graphql.rs

use crate::resolvers::soak_cmp_res::SoakQuery;
use crate::resolvers::compound_lib_rs::CompoundQuery;
use async_graphql::{EmptySubscription,EmptyMutation, MergedObject, Schema, SchemaBuilder};

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(SoakQuery, CompoundQuery);

// #[derive(Debug, Clone, MergedObject, Default)]
// pub struct Mutation(WellMutation, CompoundMutation, SoakedMutation);

pub type RootSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn root_schema_builder() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(
        Query::default(),
        EmptyMutation,
        EmptySubscription,
    )
    .enable_federation()
}

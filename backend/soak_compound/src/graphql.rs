// src/graphql.rs

use crate::resolvers::compound_lib_res::{CompoundMutation, CompoundQuery};
use crate::resolvers::soaked_well_res::{SoakedMutation, SoakedQuery};
use crate::resolvers::well_lib_res::{WellMutation, WellQuery};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(WellQuery, CompoundQuery, SoakedQuery);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Mutation(WellMutation, CompoundMutation, SoakedMutation);

pub type RootSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn root_schema_builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(
        Query::default(),
        Mutation::default(),
        EmptySubscription,
    )
    .enable_federation()
}

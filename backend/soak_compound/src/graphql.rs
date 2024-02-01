// src/graphql.rs

use crate::resolvers::compound_lib_res::{CompoundMutation, CompoundQuery};
use crate::resolvers::soaked_well_res::{SoakedMutation, SoakedQuery};
use crate::resolvers::well_lib_res::{WellMutation, WellQuery};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootQuery(WellQuery, CompoundQuery, SoakedQuery);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootMutation(WellMutation, CompoundMutation, SoakedMutation);

pub type RootSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

pub fn root_schema_builder() -> SchemaBuilder<RootQuery, RootMutation, EmptySubscription> {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription,
    )
}

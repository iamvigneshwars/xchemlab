pub mod accounts_res;

use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use accounts_res::{AccountsQuery, AccountsMutation};

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(AccountsQuery);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Mutation(AccountsMutation);

pub type RootSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn root_schema_builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).enable_federation()
}

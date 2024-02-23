pub mod reviews_res;

use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use reviews_res::{ReviewsQuery, ReviewsMutation};

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(ReviewsQuery);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Mutation(ReviewsMutation);

pub type RootSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn root_schema_builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).enable_federation()
}

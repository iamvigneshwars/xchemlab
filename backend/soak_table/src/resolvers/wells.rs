// src/resolvers/wells.rs

use async_graphql::{Context, Object};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::entities::wells;

#[derive(Debug, Clone, Default)]
pub struct WellQuery;

#[Object]
impl WellQuery{
    async fn wells(&self, ctx: &Context<'_>) -> Result<Vec<wells::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        wells::Entity::find().all(db).await
    }
}
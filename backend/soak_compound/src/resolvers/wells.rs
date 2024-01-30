// src/resolvers/wells.rs

use crate::entities::wells;
use async_graphql::{Context, Object};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};

#[derive(Debug, Clone, Default)]
pub struct WellQuery;

#[derive(Debug, Clone, Default)]
pub struct WellMutation;

#[Object]
impl WellQuery {
    async fn wells(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<wells::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        wells::Entity::find().all(db).await
    }
}

#[Object]
impl WellMutation {
    async fn add_well(
        &self,
        ctx: &Context<'_>,
        plate: String,
        pos: String,
    ) -> async_graphql::Result<wells::Model> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let well = wells::ActiveModel {
            plate: ActiveValue::Set(plate),
            pos: ActiveValue::Set(pos),
            ..Default::default()
        };
        Ok(wells::Entity::insert(well).exec_with_returning(db).await?)
    }
}

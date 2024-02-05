// src/resolvers/wells.rs

use crate::entities::well_library;
use async_graphql::{Context, Object};
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};

#[derive(Debug, Clone, Default)]
pub struct WellQuery;

#[derive(Debug, Clone, Default)]
pub struct WellMutation;

#[Object]
impl WellQuery {
    async fn wells(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<well_library::Model>> {
        subject_authorization!("xchemlab.soak_compound.read_well", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().map_err(|e| {
            async_graphql::Error::new(format!("Database connection error: {:?}", e))
        })?;
        well_library::Entity::find()
            .all(db)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to fetch all wells: {}", e)))
    }

    // async fn get_well(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<well_library::Model, DbErr> {
    async fn get_well(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> async_graphql::Result<well_library::Model> {
        subject_authorization!("xchemlab.soak_compound.read_well", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().map_err(|e| {
            async_graphql::Error::new(format!("Database connection error: {:?}", e))
        })?;
        let well =
            well_library::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::RecordNotFound(format!(
                    "Well not found with Id {} ",
                    id
                )))?;
        Ok(well)
    }
}

#[Object]
impl WellMutation {
    async fn add_well(
        &self,
        ctx: &Context<'_>,
        plate: String,
        pos: String,
    ) -> async_graphql::Result<well_library::Model> {
        subject_authorization!("xchemlab.soak_compound.write_well", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().map_err(|e| {
            async_graphql::Error::new(format!("Database connection error: {:?}", e))
        })?;
        let well = well_library::ActiveModel {
            plate: ActiveValue::Set(plate),
            pos: ActiveValue::Set(pos),
            ..Default::default()
        };

        well_library::Entity::insert(well)
            .exec_with_returning(db)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to add well: {}", e)))
    }
}

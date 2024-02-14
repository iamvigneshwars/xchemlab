// src/resolvers/compound.rs

use crate::entities::compound_library;
use async_graphql::{Context, Object};
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct CompoundQuery;

#[derive(Debug, Clone, Default)]
pub struct CompoundMutation;

#[Object]
impl CompoundQuery {
    async fn compounds(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<compound_library::Model>> {
        subject_authorization!("xchemlab.soak_compound.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        compound_library::Entity::find()
            .all(db)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to fetch all compounds: {}", e)))
    }

    async fn get_compound(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<compound_library::Model> {
        subject_authorization!("xchemlab.soak_compound.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let compound = compound_library::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Compound not found with id {}",
                id
            )))?;
        Ok(compound)
    }
}

#[Object]
impl CompoundMutation {
    async fn add_compound(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<compound_library::Model> {
        subject_authorization!("xchemlab.soak_compound.write_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let compound = compound_library::ActiveModel {
            id : ActiveValue::Set(Uuid::now_v7()),
            name: ActiveValue::Set(name),
        };

        compound_library::Entity::insert(compound)
            .exec_with_returning(db)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to add compound: {}", e)))
    }
}

// src/resolvers/compound.rs

use crate::entities::compound_library;
use async_graphql::{Context, Object};
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};

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
        let db = ctx.data::<DatabaseConnection>().map_err(|e| {
            async_graphql::Error::new(format!("Database connection error: {:?}", e))
        })?;
        compound_library::Entity::find()
            .all(db)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to fetch all compounds: {}", e)))
    }

    #[graphql(entity)]
    async fn get_compound_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)]id: i32,
    ) -> async_graphql::Result<Option<compound_library::Model>> {
        get_compound_by_id_internal(ctx, id).await
    }

    // async fn get_compound(
    //     &self,
    //     ctx: &Context<'_>,
    //     id: i32,
    // ) -> async_graphql::Result<compound_library::Model> {
    //     subject_authorization!("xchemlab.soak_compound.read_compound", ctx).await?;
    //     let db = ctx.data::<DatabaseConnection>().map_err(|e| {
    //         async_graphql::Error::new(format!("Database connection error: {:?}", e))
    //     })?;
    //     let compound = compound_library::Entity::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::RecordNotFound(format!(
    //             "Compound not found with id {}",
    //             id
    //         )))?;
    //     Ok(compound)
    // }
}

async fn get_compound_by_id_internal(
    ctx: &Context<'_>,
    id: i32,
) -> async_graphql::Result<Option<compound_library::Model>> {
    let db = ctx
        .data::<DatabaseConnection>()
        .map_err(|e| async_graphql::Error::new(format!("Datbase connection error: {:?}", e)))?;

    Ok(compound_library::Entity::find_by_id(id).one(db).await?)
}

#[Object]
impl CompoundMutation {
    async fn add_compound(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<compound_library::Model> {
        subject_authorization!("xchemlab.soak_compound.write_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().map_err(|e| {
            async_graphql::Error::new(format!("Database connection error: {:?}", e))
        })?;
        let compound = compound_library::ActiveModel {
            name: ActiveValue::Set(name),
            ..Default::default()
        };

        compound_library::Entity::insert(compound)
            .exec_with_returning(db)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to add compound: {}", e)))
    }
}

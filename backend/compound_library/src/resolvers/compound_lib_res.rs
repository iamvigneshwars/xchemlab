// src/resolvers/compound_lib_res.rs

use crate::entities::compound_library::{self, CompoundState};
use async_graphql::{Context, Object};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

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
        let db = ctx
            .data::<DatabaseConnection>()
            .map_err(|e| async_graphql::Error::new(format!("Datbase connection error: {:?}", e)))?;
        compound_library::Entity::find()
            .all(db)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to fetch all compounds: {}", e)))
    }

    async fn get_compound(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> async_graphql::Result<Option<compound_library::Model>> {
        let db = ctx
            .data::<DatabaseConnection>()
            .map_err(|e| async_graphql::Error::new(format!("Datbase connection error: {:?}", e)))?;

        Ok(compound_library::Entity::find_by_id(id).one(db).await?)
    }
}

#[Object]
impl CompoundMutation {
    async fn add_compound(
        &self,
        ctx: &Context<'_>,
        name: String,
        compound_state: CompoundState,
    ) -> async_graphql::Result<compound_library::Model> {
        let db = ctx
            .data::<DatabaseConnection>()
            .map_err(|e| async_graphql::Error::new(format!("Datbase connection error: {:?}", e)))?;

        let compound = compound_library::ActiveModel {
            name: ActiveValue::set(name),
            compound_state: ActiveValue::set(compound_state),
            ..Default::default()
        };

        compound_library::Entity::insert(compound)
            .exec_with_returning(db)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to insert compound: {}", e)))
    }
}

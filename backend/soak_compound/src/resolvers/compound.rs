// src/resolvers/compound.rs

use crate::entities::compounds;
use async_graphql::{Context, Object};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};
use opa_client::subject_authorization;


#[derive(Debug, Clone, Default)]
pub struct CompoundQuery;

#[derive(Debug, Clone, Default)]
pub struct CompoundMutation;

#[Object]
impl CompoundQuery {
    async fn compounds(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<compounds::Model>> {

        subject_authorization!("xchemlab.soak_compound.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(compounds::Entity::find().all(db).await?)
    }
}

#[Object]
impl CompoundMutation {
    async fn add_compound(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<compounds::Model> {
        
        subject_authorization!("xchemlab.soak_compound.write_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let compound = compounds::ActiveModel {
            name: ActiveValue::Set(name),
            ..Default::default()
        };

        Ok(compounds::Entity::insert(compound)
            .exec_with_returning(db)
            .await?)
    }
}

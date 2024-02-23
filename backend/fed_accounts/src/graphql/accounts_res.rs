use crate::tables::accounts;
use async_graphql::{Context, Object};
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use the_paginator::graphql::{CursorInput, ModelConnection};

#[derive(Debug, Clone, Default)]
pub struct AccountsQuery;

#[derive(Debug, Clone, Default)]
pub struct AccountsMutation;

#[Object]
impl AccountsQuery {
    async fn accounts(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<ModelConnection<accounts::Model>> {
        // subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(cursor
            .try_into_query_cursor::<accounts::Entity>()?
            .all(db)
            .await?
            .try_into_connection()?)
    }

    async fn account(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> async_graphql::Result<Option<accounts::Model>> {
        // subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(accounts::Entity::find_by_id(id).one(db).await?)
    }

    #[graphql(entity)]
    async fn find_account_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: i32,
    ) -> async_graphql::Result<Option<accounts::Model>> {
        // subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(accounts::Entity::find_by_id(id).one(db).await?)
    }
}

#[Object]
impl AccountsMutation {
    async fn add_account(
        &self,
        ctx: &Context<'_>,
        username: String, 
        id: i32,
    ) -> async_graphql::Result<accounts::Model> {
        // subject_authorization!("xchemlab.crystal_library.write_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let account = accounts::ActiveModel {
            id: ActiveValue::Set(id),
            username: ActiveValue::Set(username),
            reviewcount: ActiveValue::Set(0),
        };
        Ok(accounts::Entity::insert(account)
            .exec_with_returning(db)
            .await?)
    }
}

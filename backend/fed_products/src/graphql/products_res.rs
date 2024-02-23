use crate::tables::products;
use async_graphql::{Context, Object};
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use the_paginator::graphql::{CursorInput, ModelConnection};

#[derive(Debug, Clone, Default)]
pub struct ProductsQuery;

#[derive(Debug, Clone, Default)]
pub struct ProductsMutation;

#[Object]
impl ProductsQuery {
    async fn products(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<ModelConnection<products::Model>> {
        subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(cursor
            .try_into_query_cursor::<products::Entity>()?
            .all(db)
            .await?
            .try_into_connection()?)
    }

    async fn product(
        &self,
        ctx: &Context<'_>,
        upc: i32,
    ) -> async_graphql::Result<Option<products::Model>> {
        subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(products::Entity::find_by_id(upc).one(db).await?)
    }

    #[graphql(entity)]
    async fn find_account_by_id(
        &self,
        ctx: &Context<'_>,
        upc: i32,
    ) -> async_graphql::Result<Option<products::Model>> {
        subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(products::Entity::find_by_id(upc).one(db).await?)
    }
}

#[Object]
impl ProductsMutation {
    async fn add_account(
        &self,
        ctx: &Context<'_>,
        upc: i32, 
        name: String,
        price: i32,
    ) -> async_graphql::Result<products::Model> {
        subject_authorization!("xchemlab.crystal_library.write_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let account = products::ActiveModel {
            upc: ActiveValue::Set(upc),
            name: ActiveValue::Set(name),
            price: ActiveValue::Set(price),
        };
        Ok(products::Entity::insert(account)
            .exec_with_returning(db)
            .await?)
    }
}

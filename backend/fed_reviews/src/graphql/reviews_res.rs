use crate::tables::{reviews, accounts};
use async_graphql::{ComplexObject,SimpleObject, Context, Object};
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use the_paginator::graphql::{CursorInput, ModelConnection};

#[derive(Debug, Clone, Default)]
pub struct ReviewsQuery;

#[derive(Debug, Clone, Default)]
pub struct ReviewsMutation;

#[derive(SimpleObject)]
#[graphql(name = "accounts", complex)]
pub struct Accounts {
    pub id: i32,

}

#[ComplexObject]
impl Accounts {
    async fn review(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<reviews::Model>> {
        // subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(reviews::Entity::find()
            .filter(reviews::Column::UserId.eq(self.id))
            .all(db)
            .await?)
    }
}

#[Object]
impl ReviewsQuery {
    async fn reviews(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<ModelConnection<reviews::Model>> {
        // subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(cursor
            .try_into_query_cursor::<reviews::Entity>()?
            .all(db)
            .await?
            .try_into_connection()?)
    }

    async fn review(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> async_graphql::Result<Option<reviews::Model>> {
        // subject_authorization!("xchemlab.crystal_library.read_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(reviews::Entity::find_by_id(id).one(db).await?)
    }

    #[graphql(entity)]
    async fn get_user_by_id(&self, id: i32) -> Accounts {
        Accounts {id}
    }


}

#[Object]
impl ReviewsMutation {
    async fn add_review(
        &self,
        ctx: &Context<'_>,
        id: i32, 
        body: String,
        user_id: i32,
    ) -> async_graphql::Result<reviews::Model> {
        // subject_authorization!("xchemlab.crystal_library.write_crystal_plates", ctx).await?;
        let db = ctx.data::<DatabaseConnection>()?;
        let account = reviews::ActiveModel {
            id: ActiveValue::Set(id),
            body: ActiveValue::Set(body),
            user_id: ActiveValue::Set(user_id),
        };
        Ok(reviews::Entity::insert(account)
            .exec_with_returning(db)
            .await?)
    }
}

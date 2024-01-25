use async_graphql::{Context, Object};
use sea_orm::*;

use crate::entities::{prelude::*, *};

pub(crate) struct RootQuery;

#[Object]
impl RootQuery {
    async fn hello(&self) -> String {
        "hello".to_owned()
    }

    async fn compounds(&self, ctx: &Context<'_>) -> Result<Vec<compounds::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Compounds::find().all(db).await
    }

}

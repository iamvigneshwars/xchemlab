use crate::entities::compound_lib;
use async_graphql::{Context, Object};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};

#[derive(Debug, Clone, Default)]
pub struct CompoundQuery{
    id: i32,
}


#[Object(extends)]
impl CompoundQuery {
    #[graphql(external)]
    async fn id(&self) -> &i32 {
        &self.id
    }

}
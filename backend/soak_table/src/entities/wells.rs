// entities/wells.rs

use sea_orm::prelude::*;
use async_graphql::SimpleObject;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "wells")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub plate: String, 
    pub pos: String, 
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
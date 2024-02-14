// entities/wells.rs

use async_graphql::SimpleObject;
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(name = "well_library")]
#[sea_orm(table_name = "well_library")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment=false)]
    pub id: Uuid,
    pub plate: String,
    pub pos: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// src/entities/compounds.rs

use async_graphql::SimpleObject;
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[graphql(name = "compound_library")]
#[sea_orm(table_name = "compound_library")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment=false)]
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

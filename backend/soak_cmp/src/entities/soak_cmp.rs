// src/entities/soak_cmp.rs

use async_graphql::{Enum, SimpleObject};
use sea_orm::prelude::*;

#[derive(Clone,Copy, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "soak_cmp")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub compound_id: i32,
    #[sea_orm(column_type = "Double")]
    pub volume: f64,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation{}

impl ActiveModelBehavior for ActiveModel {}
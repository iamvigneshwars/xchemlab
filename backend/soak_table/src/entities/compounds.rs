// src/entities/compounds.rs

use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "compounds")]
pub struct Model {

    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,

}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
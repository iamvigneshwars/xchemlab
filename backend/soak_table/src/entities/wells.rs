// entities/wells.rs

// use sea_orm::{DeriveEntityModel, Eq, PartialEq};
use sea_orm::prelude::*;
use sea_orm_migration::seaql_migrations::Relation;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "wells")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub plate: String, 
    pub pos: String, 
}

impl ActiveModelBehavior for ActiveModel {}
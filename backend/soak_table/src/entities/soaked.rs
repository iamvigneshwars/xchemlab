// src/entities/soaked.rs

use sea_orm::prelude::*;
// use sea_orm_migration::seaql_migrations::Relation;
use super::{wells, compounds};

#[derive(Clone, Copy, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "soaked")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub well_id: i32,
    pub compound_id: i32,
    #[sea_orm(column_type= "Double")]
    pub volume: f32,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "compounds::Entity",
        from = "Column::CompoundId",
        to = "compounds::Column::Id"
    )]
    Compounds,
    #[sea_orm(
        belongs_to = "wells::Entity",
        from = "Column::WellId",
        to = "wells::Column::Id"
    )]
    Wells,
}

impl Related<compounds::Entity> for Entity {
    fn to() -> RelationDef{
        Relation::Compounds.def()
    }
}

impl Related<wells::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Wells.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    
}
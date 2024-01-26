//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;
use async_graphql::SimpleObject;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(complex, name = "soaked")]
#[sea_orm(table_name = "soaked")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub well_id: i32,
    pub compound_id: i32,
    #[sea_orm(column_type = "Double")]
    pub volume: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::compounds::Entity",
        from = "Column::CompoundId",
        to = "super::compounds::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Compounds,
    #[sea_orm(
        belongs_to = "super::wells::Entity",
        from = "Column::WellId",
        to = "super::wells::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Wells,
}

impl Related<super::compounds::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Compounds.def()
    }
}

impl Related<super::wells::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Wells.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

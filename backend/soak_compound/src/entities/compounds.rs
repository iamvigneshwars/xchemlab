//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;
use async_graphql::SimpleObject;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(name="compounds")]
#[sea_orm(table_name = "compounds")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::soaked::Entity")]
    Soaked,
}

impl Related<super::soaked::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Soaked.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

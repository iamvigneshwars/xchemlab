use async_graphql::SimpleObject;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationTrait, RelationDef,
};

// use crate::tables::accounts;
#[derive(Clone, Debug, DeriveEntityModel, Eq, PartialEq, SimpleObject)]
#[sea_orm(table_name = "reviews")]
#[graphql(name = "reviews")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = true)]
    id: i32,
    body: String,
    user_id: i32,
}

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

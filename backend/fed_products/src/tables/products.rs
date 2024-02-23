use async_graphql::SimpleObject;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};

#[derive(Clone, Debug, DeriveEntityModel, Eq, PartialEq, SimpleObject)]
#[sea_orm(table_name = "products")]
#[graphql(name = "products")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = false)]
    upc: i32,
    name: String,
    #[graphql(shareable)]
    price: i32,
}

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

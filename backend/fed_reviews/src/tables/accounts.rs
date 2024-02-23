use async_graphql::SimpleObject;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, RelationDef, Related, RelationTrait
};
// use crate::tables::reviews;
// #[derive(Clone, Debug, DeriveEntityModel, Eq, PartialEq, SimpleObject)]
// #[sea_orm(table_name = "accounts")]
// #[graphql(name = "accounts",complex)]
// pub struct Model {
//     #[sea_orm(primary_key,auto_increment = true)]
//     pub id: i32,
//     #[graphql(override_from ="accounts")]
//     pub reviewcount: i32,
// }

// #[allow(clippy::missing_docs_in_private_items)]
// #[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//     #[sea_orm(
//         belongs_to = "reviews::Entity",
//         from = "Column::Id",
//         to = "reviews::Column::UserId",
//     )]
//     Reviews
// }

// impl Related<reviews::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Reviews.def()
//     }
// }

// impl ActiveModelBehavior for ActiveModel {}

#[derive(SimpleObject)]
#[graphql(name = "accounts")]
pub struct Accounts {
    pub id: i32,
}
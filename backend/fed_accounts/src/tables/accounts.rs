use async_graphql::SimpleObject;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait,
};

#[derive(Clone, Debug, DeriveEntityModel, Eq, PartialEq, SimpleObject)]
#[sea_orm(table_name = "accounts")]
#[graphql(name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = false)]
    pub id: i32,
    pub username: String, 
    pub reviewcount: i32,
}

// #[derive(Clone, Debug, async_graphql::SimpleObject)]
// #[graphql(name = "accounts")]
// pub struct Accounts {
//     pub id: i32, 
//     pub username: String, 
//     pub reviewcount: i32,
// }

// impl From<Model> for Accounts {
//     fn from(model: Model) -> Self {
//         Self { id: model.id, username: model.username, reviewcount: model.reviewcount}
//     }
// }


#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

use crate::entities::soak_cmp;
use async_graphql::{Context, Object};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};

#[derive(Debug, Clone, Default)]
pub struct SoakQuery;


#[Object]
impl SoakQuery {
    async fn soaked(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<soak_cmp::Model>> {
        let db = ctx.data::<DatabaseConnection>().map_err(|e| {
            async_graphql::Error::new(format!("Database connection error: {:?}", e))
        })?;
        soak_cmp::Entity::find()
            .all(db)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to fetch all compounds: {}", e)))
    }

    #[graphql(entity)]
    async fn get_compound_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> async_graphql::Result<Option<soak_cmp::Model>> {
        get_compound_by_id_internal(ctx, id).await
    }

}

async fn get_compound_by_id_internal(
    ctx: &Context<'_>,
    id: i32,
) -> async_graphql::Result<Option<soak_cmp::Model>> {
    let db = ctx
        .data::<DatabaseConnection>()
        .map_err(|e| async_graphql::Error::new(format!("Datbase connection error: {:?}", e)))?;

    Ok(soak_cmp::Entity::find_by_id(id).one(db).await?)
}
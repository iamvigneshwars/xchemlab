use async_graphql::{Context, Object, ComplexObject};
use sea_orm::*;

use crate::entities::{prelude::*, *};

pub(crate) struct RootQuery;
pub(crate) struct RootMutation;

#[ComplexObject]
impl soaked::Model {
    async fn wells(&self, ctx: &Context<'_>) -> Result<Vec<wells::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        self.find_related(Wells).all(db).await
    }

    async fn compounds(&self, ctx: &Context<'_>) -> Result<Vec<compounds::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        self.find_related(Compounds).all(db).await

    }
}

#[ComplexObject]
impl wells::Model {
    async fn soaked(&self, ctx: &Context<'_>) -> Result<Vec<soaked::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        self.find_related(Soaked).all(db).await
    }

    async fn compounds(&self, ctx: &Context<'_>) -> Result<Vec<compounds::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        // First, find all 'soaked' entities related to this 'well'
        let soaked_records = self.find_related(Soaked).all(db).await?;

        // Then, for each 'soaked' record, find the related 'compounds'
        let mut compounds = Vec::new();
        for soaked in soaked_records {
            let related_compounds = soaked.find_related(Compounds).all(db).await?;
            compounds.extend(related_compounds);
        }

        Ok(compounds)
    }
}

#[Object]
impl RootQuery {
    async fn hello(&self) -> String {
        "hello".to_owned()
    }

    async fn compounds(&self, ctx: &Context<'_>) -> Result<Vec<compounds::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Compounds::find().all(db).await
    }

    async fn wells(&self, ctx: &Context<'_>) -> Result<Vec<wells::Model>, DbErr>  {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Wells::find().all(db).await
    }

    async fn getwellcompound(&self, ctx: &Context<'_>, id: i32) -> Result<Vec<compounds::Model>, DbErr>  {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let well = Wells::find_by_id(id).one(db).await?
            .ok_or(DbErr::RecordNotFound("Well not found".to_string()))?;

        let soaked_records = well.find_related(Soaked).all(db).await?;

        let mut compounds = Vec::new();
        for soaked in soaked_records{
            let related_comp = soaked.find_related(Compounds).all(db).await?;
            compounds.extend(related_comp);
        }

        Ok(compounds)

    }

    async fn soaked(&self, ctx: &Context<'_>) -> Result<Vec<soaked::Model>, DbErr>  {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Soaked::find().all(db).await
    }

}


#[Object]
impl RootMutation {
    async fn add_compounds(&self, ctx: &Context<'_>, name: String) -> Result<compounds::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Compounds::insert(compounds::ActiveModel{
            name: ActiveValue::Set(name),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Compounds::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|b| b.unwrap())

    }

    async fn add_wells(&self, ctx: &Context<'_>, plate: String, pos: String) -> Result<wells::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Wells::insert(wells::ActiveModel{
            plate: ActiveValue::Set(plate), 
            pos: ActiveValue::Set(pos),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Wells::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|b| b.unwrap())

    }

    async fn add_soaked(&self, ctx: &Context<'_>, well_id: i32, compound_id: i32, volume: f64) -> Result<soaked::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        let res = Soaked::insert(soaked::ActiveModel{
            well_id: ActiveValue::Set(well_id), 
            compound_id: ActiveValue::Set(compound_id),
            volume: ActiveValue::Set(volume),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Soaked::find_by_id(res.last_insert_id)
            .one(db)
            .await
            .map(|b| b.unwrap())

    }

}
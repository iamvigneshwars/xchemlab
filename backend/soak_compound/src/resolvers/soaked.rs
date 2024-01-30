// src/resolvers/soaked.rs

use crate::entities::{compounds, soaked, wells};
use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use sea_orm::{prelude::*, ActiveValue};

#[derive(SimpleObject)]
pub struct CompoundWithVolume {
    compound: compounds::Model,
    volume: f64,
}

#[derive(Debug, Clone, Default)]
pub struct SoakedQuery;

#[derive(Debug, Clone, Default)]
pub struct SoakedMutation;

#[ComplexObject]
impl soaked::Model {
    async fn wells(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<wells::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(wells::Entity).all(db).await?)
    }

    async fn compounds(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<compounds::Model>> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(compounds::Entity).all(db).await?)
    }
}

#[Object]
impl SoakedQuery {
    async fn soaked(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<soaked::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        soaked::Entity::find().all(db).await
    }

    async fn compounds_in_well(
        &self,
        ctx: &Context<'_>,
        well_id: i32,
    ) -> async_graphql::Result<Vec<CompoundWithVolume>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let soaked_wells = soaked::Entity::find()
            .filter(soaked::Column::WellId.eq(well_id))
            .all(db)
            .await?;

        let mut soaked_compounds: Vec<CompoundWithVolume> = Vec::new();
        for well in soaked_wells {
            let compound = compounds::Entity::find_by_id(well.compound_id)
                .one(db)
                .await?
                .ok_or(DbErr::RecordNotFound(format!(
                    "Compound not found with id {} ",
                    well.compound_id
                )))?;
            soaked_compounds.push(CompoundWithVolume {
                compound,
                volume: well.volume,
            });
        }

        Ok(soaked_compounds)
    }
}

#[Object]
impl SoakedMutation {
    async fn add_soaked(
        &self,
        ctx: &Context<'_>,
        wellid: i32,
        compoundid: i32,
        volume: f64,
    ) -> async_graphql::Result<soaked::Model> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let soaked = soaked::ActiveModel {
            well_id: ActiveValue::Set(wellid),
            compound_id: ActiveValue::Set(compoundid),
            volume: ActiveValue::Set(volume),
            ..Default::default()
        };

        Ok(soaked::Entity::insert(soaked)
            .exec_with_returning(db)
            .await?)
    }
}

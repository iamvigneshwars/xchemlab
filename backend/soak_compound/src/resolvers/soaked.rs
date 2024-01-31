// src/resolvers/soaked.rs

use std::collections::HashMap;
use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use sea_orm::{prelude::*, ActiveValue};
use opa_client::subject_authorization;
use crate::entities::{compounds, soaked, wells};


#[derive(Debug, Clone, Default)]
pub struct SoakedQuery;

#[derive(Debug, Clone, Default)]
pub struct SoakedMutation;

#[ComplexObject]
impl soaked::Model {
    async fn wells(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<wells::Model>> {
        subject_authorization!("xchemlab.soak_compound.read_well", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(wells::Entity).all(db).await?)

    }

    async fn compounds(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<compounds::Model>> {
        subject_authorization!("xchemlab.soak_compound.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(compounds::Entity).all(db).await?)
    }
}

#[derive(SimpleObject)]
pub struct CompoundWithVolume {
    compounds: compounds::Model,
    volume: f64,
}

#[Object]
impl SoakedQuery {

    async fn soaked(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<soaked::Model>> {
        subject_authorization!("xchemlab.soak_compound.read_soaked", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Ok(soaked::Entity::find().all(db).await?)
    }
    
    async fn compounds_in_well(
        &self,
        ctx: &Context<'_>,
        well_id: i32,
    ) -> async_graphql::Result<Vec<CompoundWithVolume>> {

        subject_authorization!("xchemlab.soak_compound.read_soaked", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let soaked_wells = soaked::Entity::find()
            .filter(soaked::Column::WellId.eq(well_id))
            .all(db)
            .await?;

        let compound_ids: Vec<i32> = soaked_wells
            .iter()
            .map(|well| well.compound_id)
            .collect();

        let compound_entities = compounds::Entity::find()
            .filter(compounds::Column::Id.is_in(compound_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|compound| (compound.id, compound))
            .collect::<HashMap<i32, compounds::Model>>();

        
        let soaked_compounds: Vec<CompoundWithVolume> = soaked_wells.into_iter().filter_map(|well| {
            let compound = compound_entities.get(&well.compound_id)?;
            Some(CompoundWithVolume{
                compounds: compound.clone(),
                volume: well.volume,
            })
        }).collect();


        // let mut soaked_compounds: Vec<CompoundWithVolume> = Vec::new();
        // for well in soaked_wells {
        //     let compound = compounds::Entity::find_by_id(well.compound_id)
        //         .one(db)
        //         .await?
        //         .ok_or(DbErr::RecordNotFound(format!(
        //             "Compound not found with id {} ",
        //             well.compound_id
        //         )))?;
        //     soaked_compounds.push(CompoundWithVolume {
        //         compounds: compound,
        //         volume: well.volume,
        //     });
        // }

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
        subject_authorization!("xchemlab.soak_compound.write_soaked", ctx).await?;
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

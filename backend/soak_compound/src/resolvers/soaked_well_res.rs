// src/resolvers/soaked.rs

use crate::entities::{compound_library, soaked_well, well_library};
use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use opa_client::subject_authorization;
use sea_orm::{prelude::*, ActiveValue};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct SoakedQuery;

#[derive(Debug, Clone, Default)]
pub struct SoakedMutation;

#[ComplexObject]
impl soaked_well::Model {
    async fn wells(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<well_library::Model>> {
        subject_authorization!("xchemlab.soak_compound.read_well", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().map_err(|e| {
            async_graphql::Error::new(format!("Database connection error: {:?}", e))
        })?;
        self.find_related(well_library::Entity)
            .all(db)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to fetch all wells: {}", e)))
    }

    async fn compounds(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<compound_library::Model>> {
        subject_authorization!("xchemlab.soak_compound.read_compound", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().map_err(|e| {
            async_graphql::Error::new(format!("Database connection error: {:?}", e))
        })?;
        // Ok(self.find_related(compound_library::Entity).all(db).await?)
        self.find_related(compound_library::Entity)
            .all(db)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to fetch all compounds: {}", e)))
    }
}

#[derive(SimpleObject)]
pub struct CompoundWithVolume {
    compounds: compound_library::Model,
    volume: f64,
}

#[Object]
impl SoakedQuery {
    async fn soaked_compounds(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<soaked_well::Model>> {
        // subject_authorization!("xchemlab.soak_compound.read_soaked", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().map_err(|e| {
            async_graphql::Error::new(format!("Database connection error: {:?}", e))
        })?;
        // Ok(soaked_well::Entity::find().all(db).await?)
        soaked_well::Entity::find().all(db).await.map_err(|e| {
            async_graphql::Error::new(format!("Failed to fetch all soaked wells: {}", e))
        })
    }

    async fn get_soaked(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> async_graphql::Result<soaked_well::Model> {
        // subject_authorization!("xchemlab.soak_compound.read_soaked", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let soaked_well =
            soaked_well::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::RecordNotFound(format!(
                    "Soaked well not found with id {}",
                    id
                )))?;
        Ok(soaked_well)
    }

    async fn get_soaked_well(
        &self,
        ctx: &Context<'_>,
        well_id: i32,
    ) -> async_graphql::Result<Vec<CompoundWithVolume>> {
        // subject_authorization!("xchemlab.soak_compound.read_soaked", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().map_err(|e| {
            async_graphql::Error::new(format!("Database connection error: {:?}", e))
        })?;
        let soaked_wells = soaked_well::Entity::find()
            .filter(soaked_well::Column::WellId.eq(well_id))
            .all(db)
            .await?;

        let compound_ids: Vec<i32> = soaked_wells.iter().map(|well| well.compound_id).collect();

        let compound_entities = compound_library::Entity::find()
            .filter(compound_library::Column::Id.is_in(compound_ids))
            .all(db)
            .await?
            .into_iter()
            .map(|compound| (compound.id, compound))
            .collect::<HashMap<i32, compound_library::Model>>();

        let soaked_compounds: Vec<CompoundWithVolume> = soaked_wells
            .into_iter()
            .filter_map(|well| {
                let compound = compound_entities.get(&well.compound_id)?;
                Some(CompoundWithVolume {
                    compounds: compound.clone(),
                    volume: well.volume,
                })
            })
            .collect();

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
    ) -> async_graphql::Result<soaked_well::Model> {
        // subject_authorization!("xchemlab.soak_compound.write_soaked", ctx).await?;
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let soaked = soaked_well::ActiveModel {
            well_id: ActiveValue::Set(wellid),
            compound_id: ActiveValue::Set(compoundid),
            volume: ActiveValue::Set(volume),
            ..Default::default()
        };

        Ok(soaked_well::Entity::insert(soaked)
            .exec_with_returning(db)
            .await?)
    }
}

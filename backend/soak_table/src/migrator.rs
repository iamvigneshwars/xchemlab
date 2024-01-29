// src/migrator.rs

// mod entities;

use crate::entities::{wells, compounds, soaked};

use sea_orm::{DbErr, DeriveMigrationName, Schema};
use sea_orm_migration::{MigrationTrait, MigratorTrait, SchemaManager};
use axum::async_trait;

pub struct Migrator;

#[derive(DeriveMigrationName)]
struct Initial;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(Initial)]
    }
}

#[async_trait]
impl MigrationTrait for Initial{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();
        let schema = Schema::new(backend);


        manager
            .create_table(schema.create_table_from_entity(wells::Entity))
            .await?;

        manager
            .create_table(schema.create_table_from_entity(compounds::Entity))
            .await?;
        
        manager
            .create_table(schema.create_table_from_entity(soaked::Entity))
            .await?;

        Ok(())

    }
}
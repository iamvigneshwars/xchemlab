// src/migrator/mod.rs

mod wells;
mod compounds;
mod soaked;

use sea_orm_migration::{MigrationTrait, MigratorTrait};
use axum::async_trait;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(wells::Migration), 
            Box::new(compounds::Migration), 
            Box::new(soaked::Migration), 
        ]
    }
}

// src/migrator/soaked.rs

use sea_orm_migration::prelude::*;
use axum::async_trait;
use super::wells::Wells;
use super::compounds::Compounds;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "soaked"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Soaked::Table)
                    .col(
                        ColumnDef::new(Soaked::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Soaked::WellId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Soaked::CompoundId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Soaked::Volume)
                            .double()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-soaked_wellId")
                            .from(Soaked::Table, Soaked::WellId)   
                            .to(Wells::Table, Wells::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-soaked_compoundId")
                            .from(Soaked::Table, Soaked::CompoundId)   
                            .to(Compounds::Table, Compounds::Id)
                    )
                    .to_owned()

            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Soaked::Table).to_owned()
            )
            .await
    }
}

#[derive(Iden)]
pub enum Soaked {
    Table,
    Id,
    WellId,
    CompoundId,
    Volume,
}

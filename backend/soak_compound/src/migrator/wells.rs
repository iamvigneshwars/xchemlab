// src/migrator/well.rs

use sea_orm_migration::prelude::*;
use axum::async_trait;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "wells"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Wells::Table)
                    .col(
                        ColumnDef::new(Wells::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Wells::Plate)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Wells::Pos)
                            .string()
                            .not_null()
                    )
                    .to_owned()

            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Wells::Table).to_owned()
            )
            .await
    }
}

#[derive(Iden)]
pub enum Wells {
    Table,
    Id,
    Plate,
    Pos,
}

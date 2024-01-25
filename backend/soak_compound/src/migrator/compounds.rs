// src/migrator/compounds.rs

use sea_orm_migration::prelude::*;
use axum::async_trait;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "compunds"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Compounds::Table)   
                    .col(
                        ColumnDef::new(Compounds::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Compounds::Name)
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
                    .table(Compounds::Table)
                    .to_owned()
            )
            .await
    }
}

#[derive(Iden)]
pub enum Compounds {
    Table, 
    Id, 
    Name,
}
use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AlprEvents::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AlprEvents::Id)
                        .string()
                        .not_null()
                        .primary_key()
                    )
                    .col(ColumnDef::new(AlprEvents::Camid)
                        .string()
                        .not_null()
                    )
                    .col( ColumnDef::new(AlprEvents::Date)
                        .date_time()
                        .not_null()
                    )
                    .col(ColumnDef::new(AlprEvents::Plate)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(AlprEvents::Image)
                        .string()
                        .not_null()
                    )
                    .col(ColumnDef::new(AlprEvents::PlateImage)
                        .string()
                        .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AlprEvents::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum AlprEvents {
    Table,
    Id,
    Camid,
    Date,
    Plate,
    Image,
    PlateImage,
}
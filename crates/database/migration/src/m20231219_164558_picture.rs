use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum Picture {
    Table,
    Id,
    PersonId,
    Type,
    Data,
    Format,
    Vector,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Picture::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Picture::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Picture::PersonId).uuid().not_null())
                    .col(ColumnDef::new(Picture::Type).tiny_integer().not_null())
                    .col(ColumnDef::new(Picture::Data).text().not_null())
                    .col(ColumnDef::new(Picture::Format).string().not_null())
                    .col(ColumnDef::new(Picture::Vector).text().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-picture-person-id")
                    .table(Picture::Table)
                    .col(Picture::PersonId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(Index::drop().name("idx-picture-person-id").to_owned())
            .await?;

        manager.drop_table(Table::drop().table(Picture::Table).to_owned())
            .await?;

        Ok(())
    }
}

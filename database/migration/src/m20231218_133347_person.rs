use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum Person {
    Table,
    Id,
    TelegramId,
    Name,
    Gender,
    Birthday,
    PrefMinAge,
    PrefMaxAge,
    PrefGenders,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Person::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Person::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Person::TelegramId).string().null())
                    .col(ColumnDef::new(Person::Name).string().null())
                    .col(ColumnDef::new(Person::Gender).tiny_integer().null())
                    .col(ColumnDef::new(Person::Birthday).small_integer().null())
                    .col(ColumnDef::new(Person::PrefMinAge).small_integer().null())
                    .col(ColumnDef::new(Person::PrefMaxAge).small_integer().null())
                    .col(ColumnDef::new(Person::PrefGenders).small_integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-telegram-id")
                    .table(Person::Table)
                    .col(Person::TelegramId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(Index::drop().name("idx-telegram-id").to_owned())
            .await?;

        manager.drop_table(Table::drop().table(Person::Table).to_owned())
            .await?;

        Ok(())
    }
}

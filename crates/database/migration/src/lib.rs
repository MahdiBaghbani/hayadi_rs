pub use sea_orm_migration::prelude::*;

mod m20231218_133347_person;
mod m20231219_164558_picture;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231218_133347_person::Migration),
            Box::new(m20231219_164558_picture::Migration),
        ]
    }
}

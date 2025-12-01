pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240101_000001_initial::Migration)]
    }
}

pub mod cli;
mod m20240101_000001_initial;

pub use cli::{run_migrations, Cli};

use anyhow::Result;
use clap::{Parser, Subcommand};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::prelude::*;

use crate::migration::Migrator;

#[derive(Parser)]
#[command(name = "aegis-migrate")]
#[command(about = "Aegis Backend Migration Tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run PostgreSQL migrations
    Up,
    /// Rollback PostgreSQL migrations
    Down,
    /// Check migration status
    Status,
    /// Reset database (DANGER)
    Reset {
        #[arg(long)]
        confirm: bool,
    },
}

pub async fn run_migrations(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Up => {
            let db = get_postgres_connection().await?;
            Migrator::up(&db, None).await?;
            println!("✅ PostgreSQL migrations completed");
        }
        Commands::Down => {
            let db = get_postgres_connection().await?;
            Migrator::down(&db, None).await?;
            println!("✅ PostgreSQL rollback completed");
        }
        Commands::Status => {
            check_migration_status().await?;
        }
        Commands::Reset { confirm } => {
            if !confirm {
                return Err(anyhow::anyhow!("Use --confirm flag to reset database"));
            }
            reset_database().await?;
        }
    }
    Ok(())
}

async fn get_postgres_connection() -> Result<DatabaseConnection> {
    let database_url = std::env::var("DATABASE_URL")
        .or_else(|_| std::env::var("AEGIS_DATABASE__URL"))
        .expect("DATABASE_URL or AEGIS_DATABASE__URL must be set");
    Ok(Database::connect(&database_url).await?)
}

async fn check_migration_status() -> Result<()> {
    let db = get_postgres_connection().await?;
    let applied = Migrator::get_applied_migrations(&db).await?;
    let pending = Migrator::get_pending_migrations(&db).await?;

    println!("PostgreSQL:");
    println!("  Applied: {}", applied.len());
    println!("  Pending: {}", pending.len());

    Ok(())
}

async fn reset_database() -> Result<()> {
    let db = get_postgres_connection().await?;
    Migrator::down(&db, None).await?;
    println!("✅ Database reset completed");
    Ok(())
}

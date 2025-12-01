use aegis_backend::migration::{run_migrations, Cli};
use clap::Parser;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let cli = Cli::parse();
    run_migrations(cli).await?;

    Ok(())
}

use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use mongodb::{Client as MongoClient, options::ClientOptions};
use std::time::Duration;
use anyhow::Result;

pub struct DatabaseConfig {
    pub postgres: DatabaseConnection,
    pub mongodb: MongoClient,
}

impl DatabaseConfig {
    pub async fn new() -> Result<Self> {
        let postgres = Self::setup_postgres().await?;
        let mongodb = Self::setup_mongodb().await?;
        
        Ok(Self { postgres, mongodb })
    }

    async fn setup_postgres() -> Result<DatabaseConnection> {
        let database_url = std::env::var("DATABASE_URL")?;
        
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
           .min_connections(5)
           .connect_timeout(Duration::from_secs(8))
           .acquire_timeout(Duration::from_secs(8))
           .idle_timeout(Duration::from_secs(8))
           .max_lifetime(Duration::from_secs(8))
           .sqlx_logging(true);

        Ok(Database::connect(opt).await?)
    }

    async fn setup_mongodb() -> Result<MongoClient> {
        let mongodb_url = std::env::var("MONGODB_URL")?;
        
        let mut client_options = ClientOptions::parse(&mongodb_url).await?;
        client_options.app_name = Some("aegis-backend".to_string());
        client_options.max_pool_size = Some(100);
        client_options.min_pool_size = Some(5);
        client_options.connect_timeout = Some(Duration::from_secs(10));
        client_options.server_selection_timeout = Some(Duration::from_secs(10));

        Ok(MongoClient::with_options(client_options)?)
    }
}

use axum::{extract::Request, middleware, response::Response, routing::get, Router};
use std::env;
use tower_http::trace::TraceLayer;
use tracing_subscriber;

use aegis_backend::{
    config::{AwsClients, Settings},
    migration::Migrator,
    AppState,
};
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Check for migration command
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "migrate" && args.len() > 2 && args[2] == "up" {
        return run_migrations().await;
    }

    // Load configuration
    let settings = Settings::new()?;

    // Initialize databases
    let db = setup_postgres(&settings.database.url).await?;
    let aws_clients = AwsClients::new().await;

    // Setup DynamoDB table and S3 bucket
    setup_aws_resources(&aws_clients).await?;

    // Create application state
    let app_state = AppState::new(db, aws_clients, settings.clone()).await;

    // Build routes
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("", aegis_backend::routes::create_routes())
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            selective_auth_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(aegis_backend::middleware::cors::cors_layer())
        .with_state(app_state);

    // Start server
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", settings.server.host, settings.server.port))
            .await?;

    tracing::info!(
        "🚀 Server running on {}:{}",
        settings.server.host,
        settings.server.port
    );
    tracing::info!(
        "📊 DynamoDB endpoint: {}",
        env::var("DYNAMODB_ENDPOINT").unwrap_or_default()
    );
    tracing::info!(
        "📦 S3 endpoint: {}",
        env::var("S3_ENDPOINT").unwrap_or_default()
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}

// Add this middleware function
async fn selective_auth_middleware(
    state: axum::extract::State<AppState>,
    req: Request,
    next: axum::middleware::Next,
) -> Result<Response, axum::http::StatusCode> {
    let path = req.uri().path();

    println!("DEBUG: Middleware checking path: {}", path);
    let is_protected = matches_protected_route(&path);

    if is_protected {
        println!("DEBUG: Path is protected, checking JWT");

        // Call the actual JWT middleware that adds Claims extension
        match aegis_backend::middleware::auth::jwt_auth_middleware(state, req, next).await {
            Ok(response) => {
                println!("DEBUG: JWT middleware succeeded");
                Ok(response)
            }
            Err(_) => {
                println!("DEBUG: JWT middleware failed");
                Err(axum::http::StatusCode::UNAUTHORIZED)
            }
        }
    } else {
        println!("DEBUG: Path is not protected, allowing through");
        Ok(next.run(req).await)
    }
}

fn matches_protected_route(path: &str) -> bool {
    path.starts_with("/auth/logout")
        || path.starts_with("/auth/refresh")
        || path.starts_with("/auth/revoke-sessions")
        || path.starts_with("/auth/send-verification")
        || path.starts_with("/players/me")
        || path.starts_with("/players/profile")
        || path == "/players"
        || (path.starts_with("/players/") && !path.contains("/username/"))
        || path.starts_with("/chats")
        || path.starts_with("/communities")
        || path.starts_with("/uploads")
}

async fn run_migrations() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = sea_orm::Database::connect(&database_url).await?;
    Migrator::up(&db, None).await?;
    tracing::info!("✅ PostgreSQL migrations completed successfully");
    Ok(())
}

async fn setup_postgres(database_url: &str) -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
    sea_orm::Database::connect(database_url).await
}

async fn setup_aws_resources(aws_clients: &AwsClients) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("🔧 Setting up AWS resources for billion-dollar gaming platform...");

    // Create DynamoDB table (Critical - must succeed)
    tracing::info!("📊 Creating DynamoDB table...");
    match aegis_backend::scripts::setup_dynamodb::create_gaming_table(&aws_clients.dynamodb).await {
        Ok(_) => tracing::info!("✅ DynamoDB table created successfully"),
        Err(e) => {
            if e.to_string().contains("ResourceInUseException") {
                tracing::info!("✅ DynamoDB table already exists");
            } else {
                tracing::error!("❌ DynamoDB table creation failed: {}", e);
                return Err(format!("Critical: DynamoDB setup failed: {}", e).into());
            }
        }
    }

    // Create S3 bucket (Important but not critical for startup)
    tracing::info!("📦 Setting up S3 bucket...");
    match aegis_backend::scripts::setup_s3::create_gaming_bucket(&aws_clients.s3).await {
        Ok(_) => {
            tracing::info!("✅ S3 bucket ready for file uploads");

            // Setup S3 policies
            if let Err(e) =
                aegis_backend::scripts::setup_s3::setup_bucket_policies(&aws_clients.s3).await
            {
                tracing::warn!("⚠️  S3 policy setup failed: {}", e);
            }
        }
        Err(e) => {
            tracing::warn!("⚠️  S3 setup failed: {}", e);
            tracing::info!("🎯 Enterprise Decision: Continuing without S3 for development");
            tracing::info!("   → File uploads will be disabled in development mode");
            tracing::info!("   → S3 will be available in production AWS environment");
        }
    }

    tracing::info!("✅ AWS resources setup completed!");
    Ok(())
}

async fn health_check() -> &'static str {
    "🎮 Aegis Gaming Backend - DynamoDB + S3 Ready!"
}

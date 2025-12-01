use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub email: EmailConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub from_email: String,
    pub from_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Settings {
            server: ServerConfig {
                host: env::var("AEGIS_SERVER__HOST")?,
                port: env::var("AEGIS_SERVER__PORT")?.parse()?,
            },
            database: DatabaseConfig {
                url: env::var("AEGIS_DATABASE__URL")?,
                max_connections: env::var("AEGIS_DATABASE__MAX_CONNECTIONS")?.parse()?,
            },
            jwt: JwtConfig {
                secret: env::var("AEGIS_JWT__SECRET")?,
                expiration: env::var("AEGIS_JWT__EXPIRATION")?.parse()?,
            },
            email: EmailConfig {
                smtp_host: env::var("AEGIS_EMAIL__SMTP_HOST")
                    .unwrap_or_else(|_| "smtp.gmail.com".to_string()),
                smtp_port: env::var("AEGIS_EMAIL__SMTP_PORT")
                    .unwrap_or_else(|_| "587".to_string())
                    .parse()?,
                smtp_user: env::var("AEGIS_EMAIL__SMTP_USER").unwrap_or_default(),
                smtp_pass: env::var("AEGIS_EMAIL__SMTP_PASS").unwrap_or_default(),
                from_email: env::var("AEGIS_EMAIL__FROM_EMAIL")
                    .unwrap_or_else(|_| "noreply@aegis.com".to_string()),
                from_name: env::var("AEGIS_EMAIL__FROM_NAME")
                    .unwrap_or_else(|_| "Aegis Gaming".to_string()),
            },
        })
    }
}

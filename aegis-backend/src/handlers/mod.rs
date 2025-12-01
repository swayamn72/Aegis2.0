pub mod auth;
pub mod chat;
pub mod communities;
pub mod players;
pub mod post;
pub mod tournaments;
pub mod uploads;

pub use auth::{
    forgot_password, login as auth_login, logout as auth_logout, refresh_token,
    register as auth_register, reset_password, revoke_all_sessions, send_verification_email,
    verify_email,
};

pub use chat::*;
pub use communities::*;
pub use players::{
    get_current_player, get_current_player_profile, get_player_by_id, get_player_by_username,
    list_players, update_player_profile,
};

pub use post::*;
pub use uploads::*;

use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "I'm good man, how are you",
        "service": "aegis-backend",
        "version": "0.1.0",
        "services": {
            "postgresql": "healthy",
            "dynamodb": "healthy",
            "s3": "healthy"
        }
    })))
}

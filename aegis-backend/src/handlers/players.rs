use crate::models::enums::GameType;
use crate::services::auth_service::Claims;
use crate::services::player_service::UpdateProfileRequest;
use crate::{utils::errors::AppError, AppState};
use axum::extract::{ConnectInfo, Extension, Path, Query};
use axum::{extract::State, http::HeaderMap, Json};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct PlayerListQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub sort_by: Option<String>, // "rating", "username", "created_at"
    pub order: Option<String>,   // "asc", "desc"
    pub game: Option<GameType>,
    pub country: Option<String>,
    pub verified_only: Option<bool>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: String,
    pub player: PlayerResponse,
}

#[derive(Serialize)]
pub struct PlayerResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct PlayerProfileResponse {
    // Core Identity
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub verified: bool,

    // Profile Information
    pub in_game_name: Option<String>,
    pub real_name: Option<String>,
    pub bio: String,
    pub profile_picture: String,
    pub age: Option<i32>,
    pub country: Option<String>,
    pub location: Option<String>,
    pub languages: Vec<String>,

    // Gaming Profile
    pub primary_game: Option<GameType>,
    pub in_game_role: Vec<String>,
    pub aegis_rating: i32,
    pub tournaments_played: i32,
    pub battles_played: i32,
    pub earnings: Decimal,

    // Team Information
    pub team_id: Option<Uuid>,
    pub team_status: Option<String>,
    pub availability: Option<String>,

    // Social Links
    pub discord_tag: String,
    pub twitch: String,
    pub youtube: String,
    pub twitter: String,

    // Preferences
    pub profile_visibility: String,
    pub card_theme: String,

    // Gamification
    pub coins: i64,
    pub check_in_streak: i32,
    pub total_check_ins: i32,
    pub last_check_in: Option<chrono::DateTime<chrono::Utc>>,

    // Metadata
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

fn extract_client_info(
    headers: &HeaderMap,
    addr: Option<SocketAddr>,
) -> (Option<String>, Option<String>) {
    let ip_address = addr.map(|a| a.ip().to_string());
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    (ip_address, user_agent)
}
pub async fn get_player_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<PlayerResponse>, AppError> {
    let requesting_user_id = Uuid::parse_str(&claims.sub)?;
    if requesting_user_id != id {
        return Err(AppError::Forbidden);
    }
    let player = state
        .player_service
        .get_by_id(id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(PlayerResponse {
        id: player.id,
        username: player.username,
        email: player.email,
    }))
}

pub async fn get_current_player(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<PlayerResponse>, AppError> {
    let player_id = Uuid::parse_str(&claims.sub)?;
    let player = state
        .player_service
        .get_by_id(player_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(PlayerResponse {
        id: player.id,
        username: player.username,
        email: player.email,
    }))
}

pub async fn get_player_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<PlayerResponse>, AppError> {
    let player = state
        .player_service
        .get_by_username(username)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(PlayerResponse {
        id: player.id,
        username: player.username,
        email: player.email,
    }))
}

// GET /api/v1/players/profile - Enhanced current player profile
pub async fn get_current_player_profile(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<PlayerProfileResponse>, AppError> {
    let player_id = Uuid::parse_str(&claims.sub)?;
    let player = state
        .player_service
        .get_by_id(player_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(PlayerProfileResponse {
        id: player.id,
        username: player.username,
        email: player.email,
        verified: player.verified,
        in_game_name: player.in_game_name,
        real_name: player.real_name,
        bio: player.bio,
        profile_picture: player.profile_picture,
        age: player.age,
        country: player.country,
        location: player.location,
        languages: player.languages,
        primary_game: player.primary_game,
        in_game_role: player.in_game_role,
        aegis_rating: player.aegis_rating,
        tournaments_played: player.tournaments_played,
        battles_played: player.battles_played,
        earnings: player.earnings,
        team_id: player.team_id,
        team_status: player.team_status,
        availability: player.availability,
        discord_tag: player.discord_tag,
        twitch: player.twitch,
        youtube: player.youtube,
        twitter: player.twitter,
        profile_visibility: player.profile_visibility,
        card_theme: player.card_theme,
        coins: player.coins,
        check_in_streak: player.check_in_streak,
        total_check_ins: player.total_check_ins,
        last_check_in: player.last_check_in,
        created_at: player.created_at,
        updated_at: player.updated_at,
    }))
}

// PUT /api/v1/players/profile - Update player profile
pub async fn update_player_profile(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<PlayerProfileResponse>, AppError> {
    let player_id = Uuid::parse_str(&claims.sub)?;
    let (ip_address, user_agent) = extract_client_info(&headers, Some(addr));

    // Enterprise validation
    if let Some(age) = payload.age {
        if age < 13 || age > 99 {
            return Err(AppError::Validation(
                "Age must be between 13 and 99".to_string(),
            ));
        }
    }

    // Rate limiting for profile updates
    if let Some(ip) = &ip_address {
        state
            .rate_limit_service
            .check_rate_limit(
                ip.clone(),
                "ip".to_string(),
                "profile_update".to_string(),
                10, // 10 updates per hour
                60,
            )
            .await?;
    }

    let updated_player = state
        .player_service
        .update_profile(player_id, payload)
        .await?;

    // Audit log
    let _ = state
        .audit_service
        .log_action(
            Some(player_id),
            Some("player".to_string()),
            Some(Uuid::parse_str(&claims.session_id)?),
            "profile_update".to_string(),
            Some("player".to_string()),
            Some(player_id),
            ip_address,
            user_agent,
            true,
            None,
            None,
            None,
        )
        .await;

    Ok(Json(PlayerProfileResponse {
        id: updated_player.id,
        username: updated_player.username,
        email: updated_player.email,
        verified: updated_player.verified,
        in_game_name: updated_player.in_game_name,
        real_name: updated_player.real_name,
        bio: updated_player.bio,
        profile_picture: updated_player.profile_picture,
        age: updated_player.age,
        country: updated_player.country,
        location: updated_player.location,
        languages: updated_player.languages,
        primary_game: updated_player.primary_game,
        in_game_role: updated_player.in_game_role,
        aegis_rating: updated_player.aegis_rating,
        tournaments_played: updated_player.tournaments_played,
        battles_played: updated_player.battles_played,
        earnings: updated_player.earnings,
        team_id: updated_player.team_id,
        team_status: updated_player.team_status,
        availability: updated_player.availability,
        discord_tag: updated_player.discord_tag,
        twitch: updated_player.twitch,
        youtube: updated_player.youtube,
        twitter: updated_player.twitter,
        profile_visibility: updated_player.profile_visibility,
        card_theme: updated_player.card_theme,
        coins: updated_player.coins,
        check_in_streak: updated_player.check_in_streak,
        total_check_ins: updated_player.total_check_ins,
        last_check_in: updated_player.last_check_in,
        created_at: updated_player.created_at,
        updated_at: updated_player.updated_at,
    }))
}

// GET /api/v1/players - Enterprise player listing
pub async fn list_players(
    State(state): State<AppState>,
    Query(query): Query<PlayerListQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let limit = query.limit.unwrap_or(20).min(100); // Max 100 per request
    let offset = query.offset.unwrap_or(0);

    // Clone query fields before moving query
    let verified_only = query.verified_only.unwrap_or(false);
    let game = query.game.clone();
    let country = query.country.clone();

    let players = state
        .player_service
        .list_players(limit, offset, query) // query is moved here
        .await?;

    let total_count = players.len() as u64;
    let has_more = total_count == limit;

    Ok(Json(serde_json::json!({
        "players": players,
        "pagination": {
            "limit": limit,
            "offset": offset,
            "count": total_count,
            "has_more": has_more
        },
        "filters_applied": {
            "verified_only": verified_only,  // Use cloned value
            "game": game,                    // Use cloned value
            "country": country               // Use cloned value
        }
    })))
}

use super::chat::ApiResponse;
use crate::services::auth_service::Claims;
use crate::{utils::errors::AppError, AppState};
use axum::extract::Extension;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateCommunityRequest {
    pub name: String,
    pub description: String,
    pub community_type: String,
}

#[derive(Deserialize)]
pub struct AddPostToCommunityRequest {
    pub post_id: String,
    pub pinned: Option<bool>,
}

pub async fn create_community(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateCommunityRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let owner_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)?;
    match state
        .community_service
        .create_community(
            payload.name,
            payload.description,
            payload.community_type,
            owner_id.to_string(),
        )
        .await
    {
        Ok(community_id) => Ok(Json(ApiResponse::success(community_id))),
        Err(e) => {
            tracing::error!("Failed to create community: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_community(
    State(state): State<AppState>,
    Path(community_id): Path<String>,
) -> Result<Json<ApiResponse<crate::models::dynamodb::Community>>, StatusCode> {
    match state.community_service.get_community(&community_id).await {
        Ok(Some(community)) => Ok(Json(ApiResponse::success(community))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get community: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn add_post_to_community(
    State(state): State<AppState>,
    Path(community_id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<AddPostToCommunityRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)?;
    let pinned = payload.pinned.unwrap_or(false);

    match state
        .community_service
        .add_post_to_community(community_id, payload.post_id, pinned, user_id.to_string())
        .await
    {
        Ok(id) => Ok(Json(ApiResponse::success(id))),
        Err(e) => {
            tracing::error!("Failed to add post to community: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_community_posts(
    State(state): State<AppState>,
    Path(community_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<crate::models::dynamodb::CommunityPost>>>, StatusCode> {
    match state
        .community_service
        .get_community_posts(&community_id)
        .await
    {
        Ok(posts) => Ok(Json(ApiResponse::success(posts))),
        Err(e) => {
            tracing::error!("Failed to get community posts: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn join_community(
    State(state): State<AppState>,
    Path((community_id, user_id)): Path<(String, String)>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    // Extract user_id from path and claims
    let requesting_user_id = Uuid::parse_str(&claims.sub)?;

    // Authorization check
    if requesting_user_id.to_string() != user_id {
        return Err(AppError::Forbidden);
    }
    match state
        .community_service
        .join_community(&community_id, &user_id)
        .await
    {
        Ok(_) => Ok(Json(ApiResponse::success(
            "Joined community successfully".to_string(),
        ))),
        Err(e) => {
            tracing::error!("Failed to join community: {}", e);
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn leave_community(
    State(state): State<AppState>,
    Path((community_id, user_id)): Path<(String, String)>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    // Extract user_id from path and claims
    let requesting_user_id = Uuid::parse_str(&claims.sub)?;

    // Authorization check
    if requesting_user_id.to_string() != user_id {
        return Err(AppError::Forbidden);
    }
    match state
        .community_service
        .leave_community(&community_id, &user_id)
        .await
    {
        Ok(_) => Ok(Json(ApiResponse::success(
            "Left community successfully".to_string(),
        ))),
        Err(e) => {
            tracing::error!("Failed to leave community: {}", e);
            Err(AppError::InternalServerError)
        }
    }
}

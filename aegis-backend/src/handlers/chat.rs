use crate::services::auth_service::Claims;
use crate::{utils::errors::AppError, AppState};
use axum::extract::Extension;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateChatRequest {
    pub name: String,
    pub chat_type: String,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub message: String,
    pub message_type: Option<String>,
}

#[derive(Deserialize)]
pub struct GetMessagesQuery {
    pub limit: Option<i32>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

pub async fn create_chat(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateChatRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let creator_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)?;
    match state
        .chat_service
        .create_chat(payload.name, payload.chat_type, creator_id.to_string()) // Use JWT user ID
        .await
    {
        Ok(chat_id) => Ok(Json(ApiResponse::success(chat_id))),
        Err(e) => {
            tracing::error!("Failed to create chat: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_chat(
    State(state): State<AppState>,
    Path(chat_id): Path<String>,
) -> Result<Json<ApiResponse<crate::models::dynamodb::Chat>>, StatusCode> {
    match state.chat_service.get_chat(&chat_id).await {
        Ok(Some(chat)) => Ok(Json(ApiResponse::success(chat))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get chat: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn send_message(
    State(state): State<AppState>,
    Path(chat_id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let sender_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)?;

    match state
        .chat_service
        .send_message(
            chat_id,
            sender_id.to_string(),
            payload.message,
            payload.message_type.unwrap_or_else(|| "text".to_string()),
        )
        .await
    {
        Ok(message_id) => Ok(Json(ApiResponse::success(message_id))),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_messages(
    State(state): State<AppState>,
    Path(chat_id): Path<String>,
    Query(params): Query<GetMessagesQuery>,
) -> Result<Json<ApiResponse<Vec<crate::models::dynamodb::ChatMessage>>>, StatusCode> {
    match state
        .chat_service
        .get_messages(&chat_id, params.limit)
        .await
    {
        Ok(messages) => Ok(Json(ApiResponse::success(messages))),
        Err(e) => {
            tracing::error!("Failed to get messages: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn join_chat(
    State(state): State<AppState>,
    Path((chat_id, user_id)): Path<(String, String)>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<String>>, AppError> {
    // Extract user_id from path and claims
    let requesting_user_id = Uuid::parse_str(&claims.sub)?;

    // Authorization check
    if requesting_user_id.to_string() != user_id {
        return Err(AppError::Forbidden);
    }
    match state.chat_service.join_chat(&chat_id, &user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(
            "Joined chat successfully".to_string(),
        ))),
        Err(e) => {
            tracing::error!("Failed to join chat: {}", e);
            Err(AppError::InternalServerError)
        }
    }
}

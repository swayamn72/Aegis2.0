use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use crate::AppState;
use super::chat::ApiResponse;

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub author: String,
    pub title: String,
    pub content: String,
    pub post_type: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize)]
pub struct AddCommentRequest {
    pub author: String,
    pub content: String,
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.post_service.create_post(
        payload.author,
        payload.title,
        payload.content,
        payload.post_type,
        payload.tags,
    ).await {
        Ok(post_id) => Ok(Json(ApiResponse::success(post_id))),
        Err(e) => {
            tracing::error!("Failed to create post: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
) -> Result<Json<ApiResponse<crate::models::dynamodb::Post>>, StatusCode> {
    match state.post_service.get_post(&post_id).await {
        Ok(Some(post)) => Ok(Json(ApiResponse::success(post))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get post: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn add_comment(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
    Json(payload): Json<AddCommentRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.post_service.add_comment(
        post_id,
        payload.author,
        payload.content,
    ).await {
        Ok(comment_id) => Ok(Json(ApiResponse::success(comment_id))),
        Err(e) => {
            tracing::error!("Failed to add comment: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_comments(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<crate::models::dynamodb::PostComment>>>, StatusCode> {
    match state.post_service.get_comments(&post_id).await {
        Ok(comments) => Ok(Json(ApiResponse::success(comments))),
        Err(e) => {
            tracing::error!("Failed to get comments: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn like_post(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.post_service.like_post(&post_id).await {
        Ok(_) => Ok(Json(ApiResponse::success("Post liked successfully".to_string()))),
        Err(e) => {
            tracing::error!("Failed to like post: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_posts_by_author(
    State(state): State<AppState>,
    Path(author_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<crate::models::dynamodb::Post>>>, StatusCode> {
    match state.post_service.get_posts_by_author(&author_id).await {
        Ok(posts) => Ok(Json(ApiResponse::success(posts))),
        Err(e) => {
            tracing::error!("Failed to get posts by author: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

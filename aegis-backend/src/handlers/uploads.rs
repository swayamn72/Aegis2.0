use super::chat::ApiResponse;
use crate::services::auth_service::Claims;
use crate::{utils::errors::AppError, AppState};
use axum::extract::Extension;
use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;

pub async fn upload_profile_picture(
    Path(user_id): Path<String>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<String>>, AppError> {
    let requesting_user_id = Uuid::parse_str(&claims.sub)?;
    if requesting_user_id.to_string() != user_id {
        return Err(AppError::Forbidden);
    }
    tracing::info!(
        "📸 Development mode: Processing profile picture upload for user: {}",
        user_id
    );

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart field error: {}", e);
        AppError::Validation("Invalid multipart data".to_string())
    })? {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            let filename = field.file_name().unwrap_or("avatar.jpg").to_string();
            let data = field.bytes().await.map_err(|e| {
                tracing::error!("Failed to read file bytes: {}", e);
                AppError::Validation("Failed to read file".to_string())
            })?;

            if data.is_empty() {
                return Ok(Json(ApiResponse::error("File is empty".to_string())));
            }

            // Development mode: Save to local filesystem
            let upload_dir = std::env::var("FILE_STORAGE_PATH")
                .unwrap_or_else(|_| "C:\\temp\\gaming-uploads".to_string());
            let user_dir = format!("{}\\profiles\\{}", upload_dir, user_id);

            // Create directory if it doesn't exist
            if let Err(e) = std::fs::create_dir_all(&user_dir) {
                tracing::error!("Failed to create upload directory: {}", e);
                return Ok(Json(ApiResponse::error(
                    "Failed to create upload directory".to_string(),
                )));
            }

            let file_path = format!("{}\\{}", user_dir, filename);

            // Save file
            if let Err(e) = std::fs::write(&file_path, &data) {
                tracing::error!("Failed to save file: {}", e);
                return Ok(Json(ApiResponse::error("Failed to save file".to_string())));
            }

            let file_url = format!("file://{}", file_path);
            tracing::info!("✅ File saved locally: {}", file_url);

            return Ok(Json(ApiResponse::success(file_url)));
        }
    }

    Ok(Json(ApiResponse::error("No file provided".to_string())))
}

pub async fn upload_chat_attachment(
    State(state): State<AppState>,
    Path(chat_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    tracing::info!("📎 Processing chat attachment upload for chat: {}", chat_id);

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart field error: {}", e);
        StatusCode::BAD_REQUEST
    })? {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            let filename = field.file_name().unwrap_or("attachment").to_string();
            let data = field.bytes().await.map_err(|e| {
                tracing::error!("Failed to read file bytes: {}", e);
                StatusCode::BAD_REQUEST
            })?;

            if data.is_empty() {
                return Ok(Json(ApiResponse::error("File is empty".to_string())));
            }

            match state
                .s3_service
                .upload_chat_attachment(&chat_id, &filename, data.to_vec())
                .await
            {
                Ok(url) => {
                    tracing::info!("✅ Chat attachment uploaded successfully: {}", url);
                    return Ok(Json(ApiResponse::success(url)));
                }
                Err(e) => {
                    tracing::error!("❌ S3 upload failed: {}", e);
                    return Ok(Json(ApiResponse::error(format!("Upload failed: {}", e))));
                }
            }
        }
    }

    Ok(Json(ApiResponse::error("No file provided".to_string())))
}

pub async fn get_presigned_url(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.s3_service.get_presigned_url(&key, 3600).await {
        Ok(url) => Ok(Json(ApiResponse::success(url))),
        Err(e) => {
            tracing::error!("Failed to generate presigned URL: {}", e);
            Ok(Json(ApiResponse::error(format!(
                "Failed to generate URL: {}",
                e
            ))))
        }
    }
}

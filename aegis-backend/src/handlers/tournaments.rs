use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::AppState;

pub async fn get_tournaments(
    State(_state): State<AppState>,
) -> Result<Json<Value>, axum::http::StatusCode> {
    Ok(Json(json!({
        "tournaments": [],
        "total": 0
    })))
}
    
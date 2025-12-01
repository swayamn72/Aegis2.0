pub mod api;

use axum::Router;
use crate::AppState;

pub fn create_routes() -> Router<AppState> {
    api::create_routes()
}

use axum::http::{HeaderName, HeaderValue, Method};
use std::env;
use tower_http::cors::CorsLayer;

pub fn cors_layer() -> CorsLayer {
    let allowed_origins =
        env::var("ALLOWED_ORIGINS").expect("ALLOWED_ORIGINS environment variable must be set");

    let mut cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
            HeaderName::from_static("x-timezone"),
        ])
        .allow_credentials(true);

    // Add each origin from environment variable
    for origin in allowed_origins.split(',') {
        if let Ok(header_value) = origin.trim().parse::<HeaderValue>() {
            cors = cors.allow_origin(header_value);
        }
    }

    cors
}

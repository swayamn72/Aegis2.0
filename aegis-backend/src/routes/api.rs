use crate::{handlers, AppState};
use axum::{
    routing::{get, post, put},
    Router,
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        // ========================================
        // PUBLIC AUTH ENDPOINTS (No JWT Required)
        // ========================================
        .route("/auth/login", post(handlers::auth_login))
        .route("/auth/register", post(handlers::auth_register))
        .route("/auth/forgot-password", post(handlers::forgot_password))
        .route(
            "/auth/reset-password/:token",
            post(handlers::reset_password),
        )
        .route("/auth/verify-email/:token", post(handlers::verify_email))
        // ========================================
        // PROTECTED AUTH ENDPOINTS (JWT Required)
        // ========================================
        .route("/auth/logout", post(handlers::auth_logout))
        .route("/auth/refresh", post(handlers::refresh_token))
        .route("/auth/revoke-sessions", post(handlers::revoke_all_sessions))
        .route(
            "/auth/send-verification",
            post(handlers::send_verification_email),
        )
        // ========================================
        // PROTECTED PLAYER ENDPOINTS (JWT Required)
        // ========================================
        .route("/players/me", get(handlers::get_current_player))
        .route(
            "/players/profile",
            get(handlers::get_current_player_profile),
        )
        .route("/players/profile", put(handlers::update_player_profile))
        .route("/players", get(handlers::list_players))
        .route("/players/:id", get(handlers::get_player_by_id))
        .route(
            "/players/username/:username",
            get(handlers::get_player_by_username),
        )
        // ========================================
        // PROTECTED SOCIAL ENDPOINTS (JWT Required)
        // ========================================
        .route("/chats", post(handlers::create_chat))
        .route("/chats/:chat_id", get(handlers::get_chat))
        .route("/chats/:chat_id/messages", post(handlers::send_message))
        .route("/chats/:chat_id/messages", get(handlers::get_messages))
        .route("/chats/:chat_id/join/:user_id", post(handlers::join_chat))
        .route("/communities", post(handlers::create_community))
        .route("/communities/:community_id", get(handlers::get_community))
        .route(
            "/communities/:community_id/posts",
            post(handlers::add_post_to_community),
        )
        .route(
            "/communities/:community_id/posts",
            get(handlers::get_community_posts),
        )
        .route(
            "/communities/:community_id/join/:user_id",
            post(handlers::join_community),
        )
        .route(
            "/communities/:community_id/leave/:user_id",
            post(handlers::leave_community),
        )
        // ========================================
        // PROTECTED UPLOAD ENDPOINTS (JWT Required)
        // ========================================
        .route(
            "/uploads/profile/:user_id",
            post(handlers::upload_profile_picture),
        )
        .route(
            "/uploads/chat/:chat_id",
            post(handlers::upload_chat_attachment),
        )
        .route("/uploads/presigned/:key", get(handlers::get_presigned_url))
}

/// These routes require valid JWT authentication and active session
// pub fn protected_routes() -> Vec<&'static str> {
//     vec![
//         // ========================================
//         // AUTHENTICATED USER MANAGEMENT
//         // ========================================
//         "/auth/logout",
//         "/auth/refresh",
//         "/auth/revoke-sessions",
//         "/auth/send-verification",
//         // ========================================
//         // PLAYER DATA ACCESS (PII/SENSITIVE)
//         // ========================================
//         "/players/me",                 // Current user's data
//         "/players/profile",            // Current user's profile
//         "/players",                    // All players list (sensitive)
//         "/players/:id",                // Individual player data
//         "/players/username/:username", // Player lookup by username
//         // ========================================
//         // SOCIAL FEATURES (USER CONTEXT REQUIRED)
//         // ========================================
//         "/chats",                                    // Create chat
//         "/chats/:chat_id",                           // View chat
//         "/chats/:chat_id/messages",                  // Chat messages
//         "/chats/:chat_id/join/:user_id",             // Join chat
//         "/communities",                              // Create community
//         "/communities/:community_id",                // View community
//         "/communities/:community_id/posts",          // Community posts
//         "/communities/:community_id/join/:user_id",  // Join community
//         "/communities/:community_id/leave/:user_id", // Leave community
//         // ========================================
//         // FILE OPERATIONS (SECURITY CRITICAL)
//         // ========================================
//         "/uploads/profile/:user_id", // Profile picture upload
//         "/uploads/chat/:chat_id",    // Chat file upload
//         "/uploads/presigned/:key",   // S3 presigned URLs
//     ]
// }

/// Enterprise Security Classification: Public Routes
/// These routes are accessible without authentication
pub fn public_routes() -> Vec<&'static str> {
    vec![
        // ========================================
        // AUTHENTICATION ENTRY POINTS
        // ========================================
        "/auth/login",                 // User login
        "/auth/register",              // User registration
        "/auth/forgot-password",       // Password reset request
        "/auth/reset-password/:token", // Password reset completion
        "/auth/verify-email/:token",   // Email verification completion
    ]
}

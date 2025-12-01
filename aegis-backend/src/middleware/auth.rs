use crate::models::enums::ApprovalStatus;
use crate::services::auth_service::Claims;
use crate::utils::errors::AppError;
use crate::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

pub async fn jwt_auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    println!("DEBUG: JWT middleware - extracting token");

    let token = match extract_token_from_request(&request) {
        Ok(t) => {
            println!("DEBUG: Token extracted successfully (length: {})", t.len());
            t
        }
        Err(e) => {
            println!("DEBUG: Token extraction failed: {:?}", e);
            return Err(e);
        }
    };

    println!("DEBUG: JWT middleware - verifying JWT");
    let claims = match state.auth_service.verify_jwt(&token) {
        Ok(c) => {
            println!(
                "DEBUG: JWT verification successful, session_id: {}",
                c.session_id
            );
            c
        }
        Err(e) => {
            println!("DEBUG: JWT verification failed: {:?}", e);
            return Err(e);
        }
    };

    println!(
        "DEBUG: JWT middleware - validating session: {}",
        claims.session_id
    );
    let session = match state
        .session_service
        .validate_session(&claims.session_id)
        .await
    {
        Ok(Some(s)) => {
            println!(
                "DEBUG: Session validation successful, user_id: {}",
                s.user_id
            );
            s
        }
        Ok(None) => {
            println!("DEBUG: Session not found in database");
            return Err(AppError::Unauthorized);
        }
        Err(e) => {
            println!("DEBUG: Session validation error: {:?}", e);
            return Err(e);
        }
    };

    println!("DEBUG: JWT middleware - checking user match");
    if session.user_id.to_string() != claims.sub {
        println!(
            "DEBUG: User ID mismatch - session: {}, claims: {}",
            session.user_id, claims.sub
        );
        return Err(AppError::Unauthorized);
    }

    println!("DEBUG: JWT middleware - all checks passed");
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

fn extract_token_from_request(request: &Request) -> Result<String, AppError> {
    // Try Authorization header first
    if let Some(auth_header) = request.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Ok(token.to_string());
            }
        }
    }

    // Try cookie as fallback
    if let Some(cookie_header) = request.headers().get("cookie") {
        if let Ok(cookies) = cookie_header.to_str() {
            for cookie in cookies.split(';') {
                let cookie = cookie.trim();
                if let Some(token) = cookie.strip_prefix("token=") {
                    return Ok(token.to_string());
                }
            }
        }
    }

    Err(AppError::Unauthorized)
}

// Admin-only middleware
pub async fn admin_only_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)?;

    if claims.user_type != "admin" {
        return Err(AppError::Unauthorized);
    }

    // Verify admin is still active
    let admin_id = claims
        .sub
        .parse()
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;
    let admin = state
        .admin_service
        .get_by_id(admin_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !admin.is_active {
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(request).await)
}

// Organization-only middleware
pub async fn organization_only_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)?;

    if claims.user_type != "organization" {
        return Err(AppError::Unauthorized);
    }

    // Verify organization is approved
    let org_id = claims
        .sub
        .parse()
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;
    let org = state
        .organization_service
        .get_by_id(org_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if org.approval_status != ApprovalStatus::Approved {
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(request).await)
}

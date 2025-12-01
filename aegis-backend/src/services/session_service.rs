use crate::models::postgres::{user_session, UserSession};
use crate::utils::errors::AppError;
use chrono::{Duration, Utc};
use sea_orm::{sea_query::Expr, *};
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionService {
    db: DatabaseConnection,
}

impl SessionService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_session(
        &self,
        user_id: Uuid,
        user_type: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<user_session::Model, AppError> {
        let now = Utc::now();
        let session_token = Uuid::new_v4().to_string();
        let refresh_token = Uuid::new_v4().to_string();

        let new_session = user_session::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            session_token: Set(session_token),
            refresh_token: Set(refresh_token),
            user_type: Set(user_type),
            ip_address: Set(ip_address),
            user_agent: Set(user_agent),
            device_fingerprint: Set(None),
            expires_at: Set(now + Duration::days(30)),
            revoked: Set(false),
            revoked_at: Set(None),
            revoked_reason: Set(None),
            last_activity: Set(now),
            created_at: Set(now),
            updated_at: Set(now),
        };

        Ok(new_session.insert(&self.db).await?)
    }

    pub async fn validate_session(
        &self,
        session_id: &str, // Changed parameter name for clarity
    ) -> Result<Option<user_session::Model>, AppError> {
        let session_uuid = session_id
            .parse::<Uuid>()
            .map_err(|_| AppError::Validation("Invalid session ID format".to_string()))?;

        let session = UserSession::find()
            .filter(user_session::Column::Id.eq(session_uuid)) // ✅ FIXED: Use ID instead of SessionToken
            .filter(user_session::Column::Revoked.eq(false))
            .filter(user_session::Column::ExpiresAt.gt(Utc::now()))
            .one(&self.db)
            .await?;

        Ok(session)
    }

    pub async fn revoke_session_by_id(&self, session_id: &str) -> Result<(), AppError> {
        let session_uuid = session_id
            .parse::<Uuid>()
            .map_err(|_| AppError::Validation("Invalid session ID format".to_string()))?;

        UserSession::update_many()
            .col_expr(
                user_session::Column::Revoked,
                Expr::value(Value::Bool(Some(true))),
            )
            .col_expr(
                user_session::Column::RevokedAt,
                Expr::value(Value::ChronoDateTimeUtc(Some(Box::new(Utc::now())))),
            )
            .col_expr(
                user_session::Column::UpdatedAt,
                Expr::value(Value::ChronoDateTimeUtc(Some(Box::new(Utc::now())))),
            )
            .filter(user_session::Column::Id.eq(session_uuid))
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn revoke_session(&self, session_token: &str) -> Result<(), AppError> {
        UserSession::update_many()
            .col_expr(
                user_session::Column::Revoked,
                Expr::value(Value::Bool(Some(true))),
            )
            .col_expr(
                user_session::Column::RevokedAt,
                Expr::value(Value::ChronoDateTimeUtc(Some(Box::new(Utc::now())))),
            )
            .col_expr(
                user_session::Column::UpdatedAt,
                Expr::value(Value::ChronoDateTimeUtc(Some(Box::new(Utc::now())))),
            )
            .filter(user_session::Column::SessionToken.eq(session_token))
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn revoke_all_user_sessions(&self, user_id: Uuid) -> Result<(), AppError> {
        UserSession::update_many()
            .col_expr(
                user_session::Column::Revoked,
                Expr::value(Value::Bool(Some(true))),
            )
            .col_expr(
                user_session::Column::RevokedAt,
                Expr::value(Value::ChronoDateTimeUtc(Some(Box::new(Utc::now())))),
            )
            .col_expr(
                user_session::Column::UpdatedAt,
                Expr::value(Value::ChronoDateTimeUtc(Some(Box::new(Utc::now())))),
            )
            .filter(user_session::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn refresh_session(
        &self,
        refresh_token: &str,
    ) -> Result<Option<user_session::Model>, AppError> {
        let session = UserSession::find()
            .filter(user_session::Column::RefreshToken.eq(refresh_token))
            .filter(user_session::Column::Revoked.eq(false))
            .filter(user_session::Column::ExpiresAt.gt(Utc::now()))
            .one(&self.db)
            .await?;

        if let Some(s) = session {
            // Create new session
            let new_session = self
                .create_session(s.user_id, s.user_type, s.ip_address, s.user_agent)
                .await?;

            // Revoke old session
            self.revoke_session(&s.session_token).await?;

            Ok(Some(new_session))
        } else {
            Ok(None)
        }
    }
}

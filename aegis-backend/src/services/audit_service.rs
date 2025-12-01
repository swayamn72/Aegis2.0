use crate::models::postgres::{audit_log, AuditLog};
use crate::utils::errors::AppError;
use chrono::Utc;
use sea_orm::*;
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuditService {
    db: DatabaseConnection,
}

impl AuditService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn log_action(
        &self,
        user_id: Option<Uuid>,
        user_type: Option<String>,
        session_id: Option<Uuid>,
        action: String,
        resource: Option<String>,
        resource_id: Option<Uuid>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        success: bool,
        failure_reason: Option<String>,
        request_id: Option<String>,
        details: Option<Value>,
    ) -> Result<audit_log::Model, AppError> {
        let new_log = audit_log::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            user_type: Set(user_type),
            session_id: Set(session_id),
            action: Set(action),
            resource: Set(resource),
            resource_id: Set(resource_id),
            ip_address: Set(ip_address),
            user_agent: Set(user_agent),
            success: Set(success),
            failure_reason: Set(failure_reason),
            request_id: Set(request_id),
            details: Set(details.unwrap_or_default()),
            created_at: Set(Utc::now()),
        };

        Ok(new_log.insert(&self.db).await?)
    }

    pub async fn log_auth_attempt(
        &self,
        email: &str,
        success: bool,
        ip_address: Option<String>,
        user_agent: Option<String>,
        failure_reason: Option<String>,
    ) -> Result<(), AppError> {
        self.log_action(
            None,
            None,
            None,
            "login_attempt".to_string(),
            Some("auth".to_string()),
            None,
            ip_address,
            user_agent,
            success,
            failure_reason,
            None,
            Some(serde_json::json!({"email": email})),
        )
        .await?;
        Ok(())
    }

    pub async fn log_user_action(
        &self,
        user_id: Uuid,
        user_type: String,
        action: String,
        resource: Option<String>,
        success: bool,
        ip_address: Option<String>,
    ) -> Result<(), AppError> {
        self.log_action(
            Some(user_id),
            Some(user_type),
            None,
            action,
            resource,
            None,
            ip_address,
            None,
            success,
            None,
            None,
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn get_user_activity(
        &self,
        user_id: Uuid,
        limit: u64,
    ) -> Result<Vec<audit_log::Model>, AppError> {
        Ok(AuditLog::find()
            .filter(audit_log::Column::UserId.eq(user_id))
            .order_by_desc(audit_log::Column::CreatedAt)
            .limit(limit)
            .all(&self.db)
            .await?)
    }

    pub async fn get_security_events(&self, hours: i64) -> Result<Vec<audit_log::Model>, AppError> {
        let since = Utc::now() - chrono::Duration::hours(hours);

        Ok(AuditLog::find()
            .filter(audit_log::Column::CreatedAt.gte(since))
            .filter(audit_log::Column::Success.eq(false))
            .order_by_desc(audit_log::Column::CreatedAt)
            .all(&self.db)
            .await?)
    }
}

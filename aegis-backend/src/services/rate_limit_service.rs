use crate::models::postgres::{rate_limit, RateLimit};
use crate::utils::errors::AppError;
use chrono::{Duration, Utc};
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct RateLimitService {
    db: DatabaseConnection,
}

impl RateLimitService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn check_rate_limit(
        &self,
        identifier: String,
        identifier_type: String,
        action: String,
        max_attempts: i32,
        window_minutes: i64,
    ) -> Result<bool, AppError> {
        let window_start = Utc::now() - Duration::minutes(window_minutes);

        // Find existing rate limit record
        let existing = RateLimit::find()
            .filter(rate_limit::Column::Identifier.eq(&identifier))
            .filter(rate_limit::Column::IdentifierType.eq(&identifier_type))
            .filter(rate_limit::Column::Action.eq(&action))
            .one(&self.db)
            .await?;

        match existing {
            Some(record) => {
                // Check if currently blocked
                if let Some(blocked_until) = record.blocked_until {
                    if blocked_until > Utc::now() {
                        return Err(AppError::RateLimited);
                    }
                }

                // Check if within current window
                if record.window_start > window_start {
                    if record.attempts >= max_attempts {
                        // Block for next window
                        let mut update: rate_limit::ActiveModel = record.into();
                        update.blocked_until =
                            Set(Some(Utc::now() + Duration::minutes(window_minutes)));
                        update.updated_at = Set(Utc::now());
                        RateLimit::update(update).exec(&self.db).await?;
                        return Err(AppError::RateLimited);
                    } else {
                        // Increment attempts - store attempts value before move
                        let current_attempts = record.attempts;
                        let mut update: rate_limit::ActiveModel = record.into();
                        update.attempts = Set(current_attempts + 1);
                        update.updated_at = Set(Utc::now());
                        RateLimit::update(update).exec(&self.db).await?;
                        return Ok(true);
                    }
                } else {
                    // New window, reset
                    let mut update: rate_limit::ActiveModel = record.into();
                    update.attempts = Set(1);
                    update.window_start = Set(Utc::now());
                    update.blocked_until = Set(None);
                    update.updated_at = Set(Utc::now());
                    RateLimit::update(update).exec(&self.db).await?;
                    return Ok(true);
                }
            }
            None => {
                // Create new record
                let new_record = rate_limit::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    identifier: Set(identifier),
                    identifier_type: Set(identifier_type),
                    action: Set(action),
                    attempts: Set(1),
                    window_start: Set(Utc::now()),
                    blocked_until: Set(None),
                    created_at: Set(Utc::now()),
                    updated_at: Set(Utc::now()),
                };
                new_record.insert(&self.db).await?;
                return Ok(true);
            }
        }
    }

    pub async fn is_blocked(
        &self,
        identifier: String,
        identifier_type: String,
        action: String,
    ) -> Result<bool, AppError> {
        let record = RateLimit::find()
            .filter(rate_limit::Column::Identifier.eq(identifier))
            .filter(rate_limit::Column::IdentifierType.eq(identifier_type))
            .filter(rate_limit::Column::Action.eq(action))
            .one(&self.db)
            .await?;

        if let Some(r) = record {
            if let Some(blocked_until) = r.blocked_until {
                return Ok(blocked_until > Utc::now());
            }
        }
        Ok(false)
    }

    pub async fn reset_rate_limit(
        &self,
        identifier: String,
        identifier_type: String,
        action: String,
    ) -> Result<(), AppError> {
        RateLimit::delete_many()
            .filter(rate_limit::Column::Identifier.eq(identifier))
            .filter(rate_limit::Column::IdentifierType.eq(identifier_type))
            .filter(rate_limit::Column::Action.eq(action))
            .exec(&self.db)
            .await?;
        Ok(())
    }
}

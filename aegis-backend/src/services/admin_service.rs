use crate::models::enums::AdminRole;
use crate::models::postgres::{admin, Admin};
use crate::services::auth_service::AuthService;
use crate::utils::errors::AppError;
use anyhow::Result;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct AdminService {
    db: DatabaseConnection,
    auth_service: AuthService,
}

impl AdminService {
    pub fn new(db: DatabaseConnection, auth_service: AuthService) -> Self {
        Self { db, auth_service }
    }

    pub async fn authenticate(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<(admin::Model, String)>, AppError> {
        let admin = Admin::find()
            .filter(admin::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        match admin {
            Some(a) => {
                if !a.is_active {
                    return Ok(None);
                }

                if let Some(lock_until) = a.lock_until {
                    if lock_until > chrono::Utc::now() {
                        return Ok(None);
                    }
                }

                if self.auth_service.verify_password(&password, &a.password)? {
                    // Return empty string for token - auth handler will generate it
                    Ok(Some((a, String::new())))
                } else {
                    self.increment_login_attempts(a.id).await?;
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<admin::Model>, AppError> {
        Ok(Admin::find_by_id(id).one(&self.db).await?)
    }

    pub async fn create_admin(
        &self,
        username: String,
        email: String,
        password: String,
        role: AdminRole,
        permissions: serde_json::Value,
    ) -> Result<admin::Model, AppError> {
        let hashed_password = self.auth_service.hash_password(&password)?;
        let now = chrono::Utc::now();

        let new_admin = admin::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(username),
            email: Set(email),
            password: Set(hashed_password),
            role: Set(role),
            permissions: Set(permissions),
            is_active: Set(true),
            last_login: Set(None),
            login_attempts: Set(0),
            lock_until: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        Ok(new_admin.insert(&self.db).await?)
    }

    pub async fn has_permission(&self, admin_id: Uuid, permission: &str) -> Result<bool, AppError> {
        let admin = self.get_by_id(admin_id).await?;
        match admin {
            Some(a) => {
                if let Some(perm_value) = a.permissions.get(permission) {
                    Ok(perm_value.as_bool().unwrap_or(false))
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }

    async fn increment_login_attempts(&self, admin_id: Uuid) -> Result<(), AppError> {
        let admin = Admin::find_by_id(admin_id).one(&self.db).await?;
        if let Some(a) = admin {
            let mut admin_update: admin::ActiveModel = a.into();
            let new_attempts = admin_update.login_attempts.as_ref() + 1;
            admin_update.login_attempts = Set(new_attempts);

            if new_attempts >= 5 {
                admin_update.lock_until =
                    Set(Some(chrono::Utc::now() + chrono::Duration::hours(1)));
            }

            Admin::update(admin_update).exec(&self.db).await?;
        }
        Ok(())
    }
    pub async fn get_by_email(&self, email: String) -> Result<Option<admin::Model>, AppError> {
        Ok(Admin::find()
            .filter(admin::Column::Email.eq(email))
            .one(&self.db)
            .await?)
    }

    pub async fn update_password(
        &self,
        user_id: Uuid,
        hashed_password: String,
    ) -> Result<bool, AppError> {
        if let Some(admin) = Admin::find_by_id(user_id).one(&self.db).await? {
            let mut admin_update: admin::ActiveModel = admin.into();
            admin_update.password = Set(hashed_password);
            admin_update.updated_at = Set(chrono::Utc::now());
            Admin::update(admin_update).exec(&self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

use crate::models::postgres::{api_key, ApiKey};
use crate::services::auth_service::AuthService;
use crate::utils::errors::AppError;
use chrono::Utc;
use sea_orm::{sea_query::Expr, *};
use uuid::Uuid;

#[derive(Clone)]
pub struct ApiKeyService {
    db: DatabaseConnection,
    auth_service: AuthService,
}

impl ApiKeyService {
    pub fn new(db: DatabaseConnection, auth_service: AuthService) -> Self {
        Self { db, auth_service }
    }

    pub async fn create_api_key(
        &self,
        name: String,
        owner_id: Uuid,
        owner_type: String,
        scopes: Vec<String>,
        rate_limit_per_hour: Option<i32>,
        expires_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<(api_key::Model, String), AppError> {
        let key_id = format!(
            "ak_{}",
            Uuid::new_v4().to_string().replace("-", "")[..16].to_lowercase()
        );
        let secret_key = Uuid::new_v4().to_string().replace("-", "");
        let full_key = format!("{}_{}", key_id, secret_key);

        // ✅ ENTERPRISE: Use AuthService for consistent hashing
        let key_hash = self.auth_service.hash_password(&secret_key)?;

        let new_key = api_key::ActiveModel {
            id: Set(Uuid::new_v4()),
            key_id: Set(key_id),
            key_hash: Set(key_hash),
            name: Set(name),
            owner_id: Set(owner_id),
            owner_type: Set(owner_type),
            scopes: Set(scopes),
            rate_limit_per_hour: Set(rate_limit_per_hour.unwrap_or(1000)),
            expires_at: Set(expires_at),
            last_used_at: Set(None),
            is_active: Set(true),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };

        let api_key_model = new_key.insert(&self.db).await?;
        Ok((api_key_model, full_key))
    }

    pub async fn validate_api_key(
        &self,
        full_key: &str,
    ) -> Result<Option<api_key::Model>, AppError> {
        let parts: Vec<&str> = full_key.split('_').collect();
        if parts.len() != 3 || parts[0] != "ak" {
            return Ok(None);
        }

        let key_id = format!("{}_{}", parts[0], parts[1]);
        let secret = parts[2];

        let api_key = ApiKey::find()
            .filter(api_key::Column::KeyId.eq(key_id))
            .filter(api_key::Column::IsActive.eq(true))
            .one(&self.db)
            .await?;

        if let Some(key) = api_key {
            // Check expiration
            if let Some(expires_at) = key.expires_at {
                if expires_at < Utc::now() {
                    return Ok(None);
                }
            }

            // ✅ ENTERPRISE: Use AuthService for consistent verification
            if self.auth_service.verify_password(secret, &key.key_hash)? {
                // Update last used
                let mut update: api_key::ActiveModel = key.clone().into();
                update.last_used_at = Set(Some(Utc::now()));
                ApiKey::update(update).exec(&self.db).await?;

                return Ok(Some(key));
            }
        }

        Ok(None)
    }

    pub async fn revoke_api_key(&self, key_id: String) -> Result<(), AppError> {
        ApiKey::update_many()
            .col_expr(api_key::Column::IsActive, Expr::value(false))
            .col_expr(api_key::Column::UpdatedAt, Expr::value(Utc::now()))
            .filter(api_key::Column::KeyId.eq(key_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    pub async fn list_owner_keys(
        &self,
        owner_id: Uuid,
        owner_type: String,
    ) -> Result<Vec<api_key::Model>, AppError> {
        Ok(ApiKey::find()
            .filter(api_key::Column::OwnerId.eq(owner_id))
            .filter(api_key::Column::OwnerType.eq(owner_type))
            .filter(api_key::Column::IsActive.eq(true))
            .order_by_desc(api_key::Column::CreatedAt)
            .all(&self.db)
            .await?)
    }

    pub async fn check_scope(&self, api_key: &api_key::Model, required_scope: &str) -> bool {
        api_key.scopes.contains(&required_scope.to_string())
            || api_key.scopes.contains(&"*".to_string())
    }
}

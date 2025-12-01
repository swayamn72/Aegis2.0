use crate::models::enums::{ApprovalStatus, GameType};
use crate::models::postgres::{organization, Organization};
use crate::services::auth_service::{AuthService, UserType};
use crate::utils::errors::AppError;
use anyhow::Result;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct OrganizationService {
    db: DatabaseConnection,
    auth_service: AuthService,
}

impl OrganizationService {
    pub fn new(db: DatabaseConnection, auth_service: AuthService) -> Self {
        Self { db, auth_service }
    }

    pub async fn create_organization(
        &self,
        org_name: String,
        owner_name: String,
        email: String,
        password: String,
        country: String,
        description: String,
    ) -> Result<(organization::Model, String), AppError> {
        let existing = Organization::find()
            .filter(organization::Column::Email.eq(&email))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::Validation("Email already exists".to_string()));
        }

        let hashed_password = self.auth_service.hash_password(&password)?;
        let now = chrono::Utc::now();

        let new_org = organization::ActiveModel {
            id: Set(Uuid::new_v4()),
            org_name: Set(org_name),
            owner_name: Set(owner_name),
            email: Set(email),
            password: Set(hashed_password),
            country: Set(country),
            headquarters: Set(None),
            description: Set(description),
            logo: Set(String::new()),
            established_date: Set(now),
            active_games: Set(Vec::<GameType>::new()),
            total_earnings: Set(rust_decimal::Decimal::ZERO),
            contact_phone: Set(String::new()),
            discord: Set(String::new()),
            twitter: Set(String::new()),
            twitch: Set(String::new()),
            youtube: Set(String::new()),
            website: Set(String::new()),
            linkedin: Set(String::new()),
            profile_visibility: Set("public".to_string()),
            approval_status: Set(ApprovalStatus::Pending),
            approved_by: Set(None),
            approval_date: Set(None),
            rejection_reason: Set(None),
            email_verified: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let org = new_org.insert(&self.db).await?;
        let token = self.auth_service.generate_jwt(
            org.id,
            UserType::Organization,
            None,
            Uuid::new_v4().to_string(),
        )?;

        Ok((org, token))
    }

    pub async fn authenticate(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<(organization::Model, String)>, AppError> {
        let org = Organization::find()
            .filter(organization::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        match org {
            Some(o) => {
                if self.auth_service.verify_password(&password, &o.password)? {
                    let token = self.auth_service.generate_jwt(
                        o.id,
                        UserType::Organization,
                        None,
                        Uuid::new_v4().to_string(),
                    )?;
                    Ok(Some((o, token)))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<organization::Model>, AppError> {
        Ok(Organization::find_by_id(id).one(&self.db).await?)
    }

    pub async fn approve_organization(
        &self,
        org_id: Uuid,
        admin_id: Uuid,
    ) -> Result<organization::Model, AppError> {
        let org = Organization::find_by_id(org_id).one(&self.db).await?;
        match org {
            Some(o) => {
                let mut org_update: organization::ActiveModel = o.into();
                org_update.approval_status = Set(ApprovalStatus::Approved);
                org_update.approved_by = Set(Some(admin_id));
                org_update.approval_date = Set(Some(chrono::Utc::now()));
                org_update.updated_at = Set(chrono::Utc::now());

                Organization::update(org_update).exec(&self.db).await?;
                self.get_by_id(org_id).await?.ok_or(AppError::NotFound)
            }
            None => Err(AppError::NotFound),
        }
    }

    pub async fn reject_organization(
        &self,
        org_id: Uuid,
        reason: String,
    ) -> Result<organization::Model, AppError> {
        let org = Organization::find_by_id(org_id).one(&self.db).await?;
        match org {
            Some(o) => {
                let mut org_update: organization::ActiveModel = o.into();
                org_update.approval_status = Set(ApprovalStatus::Rejected);
                org_update.rejection_reason = Set(Some(reason));
                org_update.updated_at = Set(chrono::Utc::now());

                Organization::update(org_update).exec(&self.db).await?;
                self.get_by_id(org_id).await?.ok_or(AppError::NotFound)
            }
            None => Err(AppError::NotFound),
        }
    }
    // Add these methods to OrganizationService impl block in organization_service.rs

    pub async fn get_by_email(
        &self,
        email: String,
    ) -> Result<Option<organization::Model>, AppError> {
        Ok(Organization::find()
            .filter(organization::Column::Email.eq(email))
            .one(&self.db)
            .await?)
    }

    pub async fn update_password(
        &self,
        user_id: Uuid,
        hashed_password: String,
    ) -> Result<bool, AppError> {
        if let Some(org) = Organization::find_by_id(user_id).one(&self.db).await? {
            let mut org_update: organization::ActiveModel = org.into();
            org_update.password = Set(hashed_password);
            org_update.updated_at = Set(chrono::Utc::now());
            Organization::update(org_update).exec(&self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn verify_email(&self, user_id: Uuid) -> Result<bool, AppError> {
        if let Some(org) = Organization::find_by_id(user_id).one(&self.db).await? {
            let mut org_update: organization::ActiveModel = org.into();
            org_update.email_verified = Set(true);
            org_update.updated_at = Set(chrono::Utc::now());
            Organization::update(org_update).exec(&self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

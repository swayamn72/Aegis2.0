use crate::models::postgres::{reward, Reward};
use crate::utils::errors::AppError;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct RewardService {
    db: DatabaseConnection,
}

impl RewardService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_active_rewards(&self) -> Result<Vec<reward::Model>, AppError> {
        Ok(Reward::find()
            .filter(reward::Column::IsActive.eq(true))
            .all(&self.db)
            .await?)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<reward::Model>, AppError> {
        Ok(Reward::find_by_id(id).one(&self.db).await?)
    }
}

use crate::models::postgres::{battle, Battle};
use crate::utils::errors::AppError;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct BattleService {
    db: DatabaseConnection,
}

impl BattleService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_by_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<battle::Model>, AppError> {
        Ok(Battle::find()
            .filter(battle::Column::Tournament.eq(tournament_id))
            .order_by_asc(battle::Column::BattleNumber)
            .all(&self.db)
            .await?)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<battle::Model>, AppError> {
        Ok(Battle::find_by_id(id).one(&self.db).await?)
    }
}

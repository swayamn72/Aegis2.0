use crate::models::postgres::{transaction, Transaction};
use crate::utils::errors::AppError;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct TransactionService {
    db: DatabaseConnection,
}

impl TransactionService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_player_transactions(
        &self,
        player_id: Uuid,
    ) -> Result<Vec<transaction::Model>, AppError> {
        Ok(Transaction::find()
            .filter(transaction::Column::PlayerId.eq(player_id))
            .order_by_desc(transaction::Column::CreatedAt)
            .all(&self.db)
            .await?)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<transaction::Model>, AppError> {
        Ok(Transaction::find_by_id(id).one(&self.db).await?)
    }
}

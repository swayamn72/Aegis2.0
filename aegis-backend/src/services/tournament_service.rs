use crate::models::postgres::{tournament, Tournament};
use crate::utils::errors::AppError;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct TournamentService {
    db: DatabaseConnection,
}

impl TournamentService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_tournaments(&self) -> Result<Vec<tournament::Model>, AppError> {
        Ok(Tournament::find().all(&self.db).await?)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<tournament::Model>, AppError> {
        Ok(Tournament::find_by_id(id).one(&self.db).await?)
    }

    pub async fn get_by_status(&self, status: String) -> Result<Vec<tournament::Model>, AppError> {
        Ok(Tournament::find()
            .filter(tournament::Column::Status.eq(status))
            .all(&self.db)
            .await?)
    }

    pub async fn get_featured(&self) -> Result<Vec<tournament::Model>, AppError> {
        Ok(Tournament::find()
            .filter(tournament::Column::Featured.eq(true))
            .filter(tournament::Column::Visibility.eq("public"))
            .all(&self.db)
            .await?)
    }

    pub async fn create_tournament(
        &self,
        tournament_name: String,
        game_title: String,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        organizer_id: Option<Uuid>,
    ) -> Result<tournament::Model, AppError> {
        let new_tournament = tournament::ActiveModel {
            id: Set(Uuid::new_v4()),
            tournament_name: Set(tournament_name),
            game_title: Set(game_title),
            start_date: Set(start_date),
            end_date: Set(end_date),
            submitted_by: Set(organizer_id),
            submitted_at: Set(Some(chrono::Utc::now())),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        Ok(new_tournament.insert(&self.db).await?)
    }
}

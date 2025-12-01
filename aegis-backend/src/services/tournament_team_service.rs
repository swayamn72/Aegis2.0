use crate::models::postgres::{tournament_team, TournamentTeam};
use crate::utils::errors::AppError;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct TournamentTeamService {
    db: DatabaseConnection,
}

impl TournamentTeamService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn join_tournament(
        &self,
        tournament_id: Uuid,
        team_id: Uuid,
        qualified_through: Option<String>,
    ) -> Result<tournament_team::Model, AppError> {
        let new_entry = tournament_team::ActiveModel {
            id: Set(Uuid::new_v4()),
            tournament_id: Set(tournament_id),
            team_id: Set(team_id),
            qualified_through: Set(qualified_through),
            joined_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        Ok(new_entry.insert(&self.db).await?)
    }

    pub async fn get_tournament_teams(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<tournament_team::Model>, AppError> {
        Ok(TournamentTeam::find()
            .filter(tournament_team::Column::TournamentId.eq(tournament_id))
            .all(&self.db)
            .await?)
    }

    pub async fn get_team_tournaments(
        &self,
        team_id: Uuid,
    ) -> Result<Vec<tournament_team::Model>, AppError> {
        Ok(TournamentTeam::find()
            .filter(tournament_team::Column::TeamId.eq(team_id))
            .all(&self.db)
            .await?)
    }
}

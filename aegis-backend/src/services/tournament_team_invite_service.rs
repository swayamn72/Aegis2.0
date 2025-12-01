use crate::models::enums::InviteStatus;
use crate::models::postgres::{tournament_team_invite, TournamentTeamInvite};
use crate::utils::errors::AppError;
use sea_orm::{sea_query::Expr, *};
use uuid::Uuid;

#[derive(Clone)]
pub struct TournamentTeamInviteService {
    db: DatabaseConnection,
}

impl TournamentTeamInviteService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_invite(
        &self,
        tournament_id: Uuid,
        team_id: Uuid,
        organizer_id: Uuid,
        phase: String,
        message: Option<String>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<tournament_team_invite::Model, AppError> {
        let new_invite = tournament_team_invite::ActiveModel {
            id: Set(Uuid::new_v4()),
            tournament: Set(tournament_id),
            team: Set(team_id),
            organizer: Set(organizer_id),
            phase: Set(phase),
            message: Set(message),
            expires_at: Set(expires_at.unwrap_or_else(|| {
                chrono::Utc::now() + chrono::Duration::days(7) // Default 7 days expiry
            })),
            status: Set(InviteStatus::Pending),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        };

        Ok(new_invite.insert(&self.db).await?)
    }

    pub async fn get_team_invites(
        &self,
        team_id: Uuid,
    ) -> Result<Vec<tournament_team_invite::Model>, AppError> {
        Ok(TournamentTeamInvite::find()
            .filter(tournament_team_invite::Column::Team.eq(team_id))
            .filter(tournament_team_invite::Column::Status.eq(InviteStatus::Pending))
            .all(&self.db)
            .await?)
    }

    pub async fn accept_invite(&self, invite_id: Uuid) -> Result<(), AppError> {
        TournamentTeamInvite::update_many()
            .col_expr(
                tournament_team_invite::Column::Status,
                Expr::value(InviteStatus::Accepted),
            )
            .col_expr(
                tournament_team_invite::Column::UpdatedAt,
                Expr::value(chrono::Utc::now()),
            )
            .filter(tournament_team_invite::Column::Id.eq(invite_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }
}

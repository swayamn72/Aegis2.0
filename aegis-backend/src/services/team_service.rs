use crate::models::postgres::{team, Team};
use crate::utils::errors::AppError;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct TeamService {
    db: DatabaseConnection,
}

impl TeamService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_team(
        &self,
        team_name: String,
        team_tag: Option<String>,
        captain_id: Uuid,
        organization_id: Option<Uuid>,
    ) -> Result<team::Model, AppError> {
        let new_team = team::ActiveModel {
            id: Set(Uuid::new_v4()),
            team_name: Set(team_name),
            team_tag: Set(team_tag),
            captain: Set(Some(captain_id)),
            organization_id: Set(organization_id),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        Ok(new_team.insert(&self.db).await?)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<team::Model>, AppError> {
        Ok(Team::find_by_id(id).one(&self.db).await?)
    }

    pub async fn get_by_organization(&self, org_id: Uuid) -> Result<Vec<team::Model>, AppError> {
        Ok(Team::find()
            .filter(team::Column::OrganizationId.eq(org_id))
            .all(&self.db)
            .await?)
    }
}

use crate::models::enums::{ApprovalStatus, GameType};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "organizations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub org_name: String,
    pub owner_name: String,
    pub email: String,
    pub password: String,
    pub country: String,
    pub headquarters: Option<String>,
    pub description: String,
    pub logo: String,
    pub established_date: ChronoDateTimeUtc,
    pub active_games: Vec<GameType>,
    pub total_earnings: Decimal,
    pub contact_phone: String,
    pub discord: String,
    pub twitter: String,
    pub twitch: String,
    pub youtube: String,
    pub website: String,
    pub linkedin: String,
    pub profile_visibility: String,
    pub approval_status: ApprovalStatus,
    pub approved_by: Option<Uuid>,
    pub approval_date: Option<ChronoDateTimeUtc>,
    pub rejection_reason: Option<String>,
    pub email_verified: bool,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::team::Entity")]
    Teams,
    #[sea_orm(has_many = "super::tournament_team_invite::Entity")]
    TournamentTeamInvites,
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teams.def()
    }
}

impl Related<super::tournament_team_invite::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TournamentTeamInvites.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

use crate::models::enums::{ApprovalStatus, TournamentStatus};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tournaments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub tournament_name: String,
    pub short_name: Option<String>,
    pub slug: Option<String>,
    pub game_title: String,
    pub tier: String,
    pub region: String,
    pub sub_region: Option<String>,
    pub organizer: Json,
    pub sponsors: Json,
    pub announcement_date: Option<ChronoDateTimeUtc>,
    pub is_open_for_all: bool,
    pub registration_start_date: Option<ChronoDateTimeUtc>,
    pub registration_end_date: Option<ChronoDateTimeUtc>,
    pub start_date: ChronoDateTimeUtc,
    pub end_date: ChronoDateTimeUtc,
    pub status: TournamentStatus,
    pub format: Option<String>,
    pub format_details: Option<String>,
    pub slots: Json,
    pub participating_teams: Json,
    pub phases: Json,
    pub final_standings: Json,
    pub prize_pool: Json,
    pub statistics: Json,
    pub awards: Json,
    pub media: Json,
    pub stream_links: Json,
    pub social_media: Json,
    pub description: Option<String>,
    pub ruleset_document: Option<String>,
    pub website_link: Option<String>,
    pub game_settings: Json,
    pub visibility: String,
    pub featured: bool,
    pub verified: bool,
    pub parent_series: Option<Uuid>,
    pub qualifies_for: Json,
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub external_ids: Json,
    pub approval_status: ApprovalStatus,
    pub submitted_by: Option<Uuid>,
    pub submitted_at: Option<ChronoDateTimeUtc>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<ChronoDateTimeUtc>,
    pub rejected_by: Option<Uuid>,
    pub rejected_at: Option<ChronoDateTimeUtc>,
    pub rejection_reason: Option<String>,
    pub pending_invitations: Json,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tournament_team::Entity")]
    TournamentTeams,
    #[sea_orm(has_many = "super::battle::Entity")]
    Battles,
    #[sea_orm(has_many = "super::tournament_team_invite::Entity")]
    TournamentTeamInvites,
    #[sea_orm(has_many = "super::transaction::Entity")]
    Transactions,
}

impl Related<super::tournament_team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TournamentTeams.def()
    }
}

impl Related<super::battle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Battles.def()
    }
}

impl Related<super::tournament_team_invite::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TournamentTeamInvites.def()
    }
}

impl Related<super::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transactions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

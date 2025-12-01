use crate::models::enums::{GameType, TeamStatus};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub team_name: String,
    pub team_tag: Option<String>,
    pub logo: String,
    pub captain: Option<Uuid>,
    pub primary_game: GameType,
    pub region: String,
    pub country: Option<String>,
    pub bio: String,
    pub established_date: ChronoDateTimeUtc,
    pub total_earnings: Decimal,
    pub aegis_rating: i32,
    pub organization_id: Option<Uuid>,
    pub discord: String,
    pub twitter: String,
    pub twitch: String,
    pub youtube: String,
    pub website: String,
    pub profile_visibility: String,
    pub status: TeamStatus,
    pub looking_for_players: bool,
    pub open_roles: Vec<String>,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::Captain",
        to = "super::player::Column::Id"
    )]
    Captain,
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrganizationId",
        to = "super::organization::Column::Id"
    )]
    Organization,
    #[sea_orm(has_many = "super::player::Entity")]
    Players,
    #[sea_orm(has_many = "super::tournament_team::Entity")]
    TournamentTeams,
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Players.def()
    }
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::tournament_team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TournamentTeams.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

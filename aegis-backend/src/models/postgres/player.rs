use crate::models::enums::GameType;
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "players")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub username: String,
    pub in_game_name: Option<String>,
    pub real_name: Option<String>,
    pub email: String,
    pub password: String,
    pub verified: bool,
    pub country: Option<String>,
    pub bio: String,
    pub profile_picture: String,
    pub primary_game: Option<GameType>,
    pub earnings: Decimal,
    pub in_game_role: Vec<String>,
    pub location: Option<String>,
    pub age: Option<i32>,
    pub languages: Vec<String>,
    pub aegis_rating: i32,
    pub tournaments_played: i32,
    pub battles_played: i32,
    pub qualified_events: bool,
    pub qualified_event_details: Vec<String>,
    pub team_status: Option<String>,
    pub team_id: Option<Uuid>,
    pub availability: Option<String>,
    pub discord_tag: String,
    pub twitch: String,
    pub youtube: String,
    pub twitter: String,
    pub profile_visibility: String,
    pub card_theme: String,
    pub coins: i64,
    pub last_check_in: Option<ChronoDateTimeUtc>,
    pub check_in_streak: i32,
    pub total_check_ins: i32,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::team::Entity",
        from = "Column::TeamId",
        to = "super::team::Column::Id"
    )]
    Team,
    #[sea_orm(has_many = "super::player_game_stats::Entity")]
    PlayerGameStats,
    #[sea_orm(has_many = "super::transaction::Entity")]
    Transactions,
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl Related<super::player_game_stats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlayerGameStats.def()
    }
}

impl Related<super::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transactions.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}

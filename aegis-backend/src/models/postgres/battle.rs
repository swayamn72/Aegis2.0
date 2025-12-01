use crate::models::enums::BattleStatus;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "battles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub battle_number: i32,
    pub tournament: Uuid,
    pub tournament_phase: Option<String>,
    pub scheduled_start_time: ChronoDateTimeUtc,
    pub status: BattleStatus,
    pub map: Option<String>,
    pub participating_groups: Vec<String>,
    pub participating_teams: Json,
    pub battle_stats: Json,
    pub stream_urls: Json,
    pub room_credentials: Json,
    pub points_system: Json,
    pub tags: Vec<String>,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tournament::Entity",
        from = "Column::Tournament",
        to = "super::tournament::Column::Id"
    )]
    Tournament,
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tournament.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

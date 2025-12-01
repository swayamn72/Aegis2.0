use crate::models::enums::GameType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "player_game_stats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub player_id: Uuid,
    pub game_type: GameType,
    pub rank_tier: Option<String>,
    pub battles_played: i32,
    pub wins: i32,
    pub kills: i32,
    pub game_specific_stats: Json,
    pub last_updated: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::PlayerId",
        to = "super::player::Column::Id"
    )]
    Player,
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

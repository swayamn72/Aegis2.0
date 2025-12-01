use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "rate_limits")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub identifier: String,
    pub identifier_type: String,
    pub action: String,
    pub attempts: i32,
    pub window_start: ChronoDateTimeUtc,
    pub blocked_until: Option<ChronoDateTimeUtc>,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

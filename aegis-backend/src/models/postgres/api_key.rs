use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "api_keys")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub key_id: String,
    pub key_hash: String,
    pub name: String,
    pub owner_id: Uuid,
    pub owner_type: String,
    pub scopes: Vec<String>,
    pub rate_limit_per_hour: i32,
    pub expires_at: Option<ChronoDateTimeUtc>,
    pub last_used_at: Option<ChronoDateTimeUtc>,
    pub is_active: bool,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

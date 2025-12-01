use crate::models::enums::InviteStatus;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tournament_team_invites")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub tournament: Uuid,
    pub team: Uuid,
    pub phase: String,
    pub organizer: Uuid,
    pub status: InviteStatus,
    pub message: Option<String>,
    pub expires_at: ChronoDateTimeUtc,
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
    #[sea_orm(
        belongs_to = "super::team::Entity",
        from = "Column::Team",
        to = "super::team::Column::Id"
    )]
    Team,
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::Organizer",
        to = "super::organization::Column::Id"
    )]
    Organization,
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tournament.def()
    }
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

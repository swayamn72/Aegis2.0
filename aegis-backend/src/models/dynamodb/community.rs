use super::entity::GameEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Community {
    pub id: String,
    pub name: String,
    pub description: String,
    pub community_type: String,  // "public", "private", "tournament"
    pub owner: String,           // Player UUID
    pub moderators: Vec<String>, // Player UUIDs
    pub members: Vec<String>,    // Player UUIDs
    pub member_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunityPost {
    pub id: String,
    pub community_id: String,
    pub post_id: String,
    pub pinned: bool,
    pub added_by: String,
    pub created_at: String,
}

impl Community {
    pub fn to_entity(&self) -> Result<GameEntity, serde_json::Error> {
        GameEntity::new("community", &format!("COMMUNITY#{}", self.id), "METADATA")
            .with_gsi(&format!("OWNER#{}", self.owner), &self.created_at)
            .with_data(self)
    }
}

impl CommunityPost {
    pub fn to_entity(&self) -> Result<GameEntity, serde_json::Error> {
        GameEntity::new(
            "community_post",
            &format!("COMMUNITY#{}", self.community_id),
            &format!("POST#{}", self.post_id),
        )
        .with_gsi(&format!("POST#{}", self.post_id), &self.created_at)
        .with_data(self)
    }
}

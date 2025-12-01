use serde::{Deserialize, Serialize};
use super::entity::GameEntity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: String,
    pub author: String,                // Player UUID
    pub title: String,
    pub content: String,
    pub post_type: String,             // "general", "tournament", "team"
    pub tags: Vec<String>,
    pub likes: i32,
    pub comments_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostComment {
    pub id: String,
    pub post_id: String,
    pub author: String,                // Player UUID
    pub content: String,
    pub created_at: String,
}

impl Post {
    pub fn to_entity(&self) -> Result<GameEntity, serde_json::Error> {
        GameEntity::new("post", &format!("POST#{}", self.id), "METADATA")
            .with_gsi(&format!("USER#{}", self.author), &self.created_at)
            .with_data(self)
    }
}

impl PostComment {
    pub fn to_entity(&self) -> Result<GameEntity, serde_json::Error> {
        GameEntity::new(
            "comment",
            &format!("POST#{}", self.post_id),
            &format!("COMMENT#{}", self.created_at)
        )
        .with_gsi(&format!("USER#{}", self.author), &self.created_at)
        .with_data(self)
    }
}

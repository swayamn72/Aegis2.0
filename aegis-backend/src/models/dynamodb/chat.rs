use super::entity::GameEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
    pub id: String,
    pub chat_type: String, // "general", "team", "tournament"
    pub name: String,
    pub participants: Vec<String>, // Player UUIDs
    pub created_by: String,        // Player UUID
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub id: String,
    pub chat_id: String,
    pub sender: String, // Player UUID
    pub message: String,
    pub message_type: String, // "text", "image", "file"
    pub timestamp: String,
}

impl Chat {
    pub fn to_entity(&self) -> Result<GameEntity, serde_json::Error> {
        GameEntity::new("chat", &format!("CHAT#{}", self.id), "METADATA")
            .with_gsi(&format!("CHAT_TYPE#{}", self.chat_type), &self.created_at)
            .with_data(self)
    }

    pub fn from_entity(entity: &GameEntity) -> Result<Self, serde_json::Error> {
        serde_json::from_value(entity.data.clone())
    }
}

impl ChatMessage {
    pub fn to_entity(&self) -> Result<GameEntity, serde_json::Error> {
        GameEntity::new(
            "message",
            &format!("CHAT#{}", self.chat_id),
            &format!("MSG#{}", self.timestamp),
        )
        .with_gsi(&format!("USER#{}", self.sender), &self.timestamp)
        .with_data(self)
    }
}

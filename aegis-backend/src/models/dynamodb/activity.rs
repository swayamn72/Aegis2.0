use serde::{Deserialize, Serialize};
use super::entity::GameEntity;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityLog {
    pub id: String,
    pub user_id: String,               // Player UUID
    pub activity_type: String,         // "login", "match", "tournament_join"
    pub description: String,
    pub metadata: serde_json::Value,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TryoutChat {
    pub id: String,
    pub chat_type: String,             // "team_tryout", "tournament_group"
    pub team_id: Option<String>,       // Team UUID
    pub tournament_id: Option<String>, // Tournament UUID
    pub participants: Vec<String>,     // Player UUIDs
    pub created_at: String,
    pub updated_at: String,
}

impl ActivityLog {
    pub fn to_entity(&self) -> Result<GameEntity, serde_json::Error> {
        GameEntity::new(
            "activity",
            &format!("USER#{}", self.user_id),
            &format!("ACTIVITY#{}", self.timestamp)
        )
        .with_gsi(&format!("TYPE#{}", self.activity_type), &self.timestamp)
        .with_data(self)
    }
}

impl TryoutChat {
    pub fn to_entity(&self) -> Result<GameEntity, serde_json::Error> {
        let pk = match (&self.team_id, &self.tournament_id) {
            (Some(team_id), _) => format!("TEAM#{}", team_id),
            (_, Some(tournament_id)) => format!("TOURNAMENT#{}", tournament_id),
            _ => format!("TRYOUT#{}", self.id),
        };
        
        GameEntity::new("tryout_chat", &pk, &format!("CHAT#{}", self.id))
            .with_gsi(&format!("TYPE#{}", self.chat_type), &self.created_at)
            .with_data(self)
    }
}

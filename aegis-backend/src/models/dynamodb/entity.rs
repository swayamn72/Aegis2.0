use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEntity {
    pub pk: String,              // Partition Key
    pub sk: String,              // Sort Key
    pub entity_type: String,     // "chat", "post", "community", etc.
    pub gsi1_pk: Option<String>, // GSI1 Partition Key
    pub gsi1_sk: Option<String>, // GSI1 Sort Key
    pub data: serde_json::Value, // Entity-specific data
    pub created_at: String,      // ISO 8601 timestamp
    pub updated_at: String,      // ISO 8601 timestamp
}

impl GameEntity {
    pub fn new(entity_type: &str, pk: &str, sk: &str) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            pk: pk.to_string(),
            sk: sk.to_string(),
            entity_type: entity_type.to_string(),
            gsi1_pk: None,
            gsi1_sk: None,
            data: serde_json::Value::Null,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn with_gsi(mut self, gsi1_pk: &str, gsi1_sk: &str) -> Self {
        self.gsi1_pk = Some(gsi1_pk.to_string());
        self.gsi1_sk = Some(gsi1_sk.to_string());
        self
    }

    pub fn with_data<T: Serialize>(mut self, data: &T) -> Result<Self, serde_json::Error> {
        self.data = serde_json::to_value(data)?;
        Ok(self)
    }
}

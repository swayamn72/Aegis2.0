use anyhow::Result;
use uuid::Uuid;
use crate::models::dynamodb::{Chat, ChatMessage};
use super::{Repository, DynamoRepository};

#[derive(Clone)]
pub struct ChatRepository {
    dynamo: DynamoRepository,
}

impl ChatRepository {
    pub fn new(dynamo: DynamoRepository) -> Self {
        Self { dynamo }
    }

    pub async fn create_chat(&self, mut chat: Chat) -> Result<String> {
        if chat.id.is_empty() {
            chat.id = Uuid::new_v4().to_string();
        }
        
        let entity = chat.to_entity()?;
        self.dynamo.put_item(&entity).await?;
        Ok(chat.id)
    }

    pub async fn get_chat(&self, chat_id: &str) -> Result<Option<Chat>> {
        let pk = format!("CHAT#{}", chat_id);
        if let Some(entity) = self.dynamo.get_item(&pk, "METADATA").await? {
            Ok(Some(Chat::from_entity(&entity)?))
        } else {
            Ok(None)
        }
    }

    pub async fn add_message(&self, mut message: ChatMessage) -> Result<String> {
        if message.id.is_empty() {
            message.id = Uuid::new_v4().to_string();
        }
        
        let entity = message.to_entity()?;
        self.dynamo.put_item(&entity).await?;
        Ok(message.id)
    }

    pub async fn get_messages(&self, chat_id: &str, limit: Option<i32>) -> Result<Vec<ChatMessage>> {
        let pk = format!("CHAT#{}", chat_id);
        let entities = self.dynamo.query_by_pk(&pk).await?;
        
        let mut messages: Vec<ChatMessage> = Vec::new();
        for entity in entities {
            if entity.entity_type == "message" {
                messages.push(serde_json::from_value(entity.data)?);
            }
        }
        
        // Sort by timestamp (newest first)
        messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if let Some(limit) = limit {
            messages.truncate(limit as usize);
        }
        
        Ok(messages)
    }

    pub async fn get_chats_by_type(&self, chat_type: &str) -> Result<Vec<Chat>> {
        let gsi1_pk = format!("CHAT_TYPE#{}", chat_type);
        let entities = self.dynamo.query_gsi1(&gsi1_pk, None).await?;
        
        let mut chats = Vec::new();
        for entity in entities {
            if entity.entity_type == "chat" {
                chats.push(Chat::from_entity(&entity)?);
            }
        }
        
        Ok(chats)
    }
}

#[async_trait::async_trait]
impl Repository<Chat> for ChatRepository {
    async fn create(&self, chat: &Chat) -> Result<String> {
        self.create_chat(chat.clone()).await
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Chat>> {
        self.get_chat(id).await
    }

    async fn update(&self, chat: &Chat) -> Result<()> {
        let entity = chat.to_entity()?;
        self.dynamo.put_item(&entity).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let pk = format!("CHAT#{}", id);
        self.dynamo.delete_item(&pk, "METADATA").await
    }
}

use anyhow::Result;
use crate::models::dynamodb::{Chat, ChatMessage};
use crate::repositories::{ChatRepository, Repository};

#[derive(Clone)]
pub struct ChatService {
    chat_repo: ChatRepository,
}

impl ChatService {
    pub fn new(chat_repo: ChatRepository) -> Self {
        Self { chat_repo }
    }

    pub async fn create_chat(&self, name: String, chat_type: String, created_by: String) -> Result<String> {
        let chat = Chat {
            id: String::new(), // Will be generated in repository
            chat_type,
            name,
            participants: vec![created_by.clone()],
            created_by,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        self.chat_repo.create_chat(chat).await
    }

    pub async fn get_chat(&self, chat_id: &str) -> Result<Option<Chat>> {
        self.chat_repo.get_chat(chat_id).await
    }

    pub async fn send_message(&self, chat_id: String, sender: String, message: String, message_type: String) -> Result<String> {
        let chat_message = ChatMessage {
            id: String::new(), // Will be generated in repository
            chat_id,
            sender,
            message,
            message_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        self.chat_repo.add_message(chat_message).await
    }

    pub async fn get_messages(&self, chat_id: &str, limit: Option<i32>) -> Result<Vec<ChatMessage>> {
        self.chat_repo.get_messages(chat_id, limit).await
    }

    pub async fn get_chats_by_type(&self, chat_type: &str) -> Result<Vec<Chat>> {
        self.chat_repo.get_chats_by_type(chat_type).await
    }

    pub async fn join_chat(&self, chat_id: &str, user_id: &str) -> Result<()> {
        if let Some(mut chat) = self.chat_repo.get_chat(chat_id).await? {
            if !chat.participants.contains(&user_id.to_string()) {
                chat.participants.push(user_id.to_string());
                chat.updated_at = chrono::Utc::now().to_rfc3339();
                self.chat_repo.update(&chat).await?;
            }
        }
        Ok(())
    }
}

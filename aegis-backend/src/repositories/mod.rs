pub mod dynamodb_repository;
pub mod chat_repository;
pub mod post_repository;
pub mod community_repository;

pub use dynamodb_repository::DynamoRepository;
pub use chat_repository::ChatRepository;
pub use post_repository::PostRepository;
pub use community_repository::CommunityRepository;

use anyhow::Result;

#[async_trait::async_trait]
pub trait Repository<T> {
    async fn create(&self, item: &T) -> Result<String>;
    async fn get_by_id(&self, id: &str) -> Result<Option<T>>;
    async fn update(&self, item: &T) -> Result<()>;
    async fn delete(&self, id: &str) -> Result<()>;
}

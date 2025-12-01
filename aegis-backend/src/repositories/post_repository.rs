use anyhow::Result;
use uuid::Uuid;
use crate::models::dynamodb::{Post, PostComment};
use super::{Repository, DynamoRepository};

#[derive(Clone)]
pub struct PostRepository {
    dynamo: DynamoRepository,
}

impl PostRepository {
    pub fn new(dynamo: DynamoRepository) -> Self {
        Self { dynamo }
    }

    pub async fn create_post(&self, mut post: Post) -> Result<String> {
        if post.id.is_empty() {
            post.id = Uuid::new_v4().to_string();
        }
        
        let entity = post.to_entity()?;
        self.dynamo.put_item(&entity).await?;
        Ok(post.id)
    }

    pub async fn get_post(&self, post_id: &str) -> Result<Option<Post>> {
        let pk = format!("POST#{}", post_id);
        if let Some(entity) = self.dynamo.get_item(&pk, "METADATA").await? {
            Ok(Some(serde_json::from_value(entity.data)?))
        } else {
            Ok(None)
        }
    }

    pub async fn add_comment(&self, mut comment: PostComment) -> Result<String> {
        if comment.id.is_empty() {
            comment.id = Uuid::new_v4().to_string();
        }
        
        let entity = comment.to_entity()?;
        self.dynamo.put_item(&entity).await?;
        Ok(comment.id)
    }

    pub async fn get_comments(&self, post_id: &str) -> Result<Vec<PostComment>> {
        let pk = format!("POST#{}", post_id);
        let entities = self.dynamo.query_by_pk(&pk).await?;
        
        let mut comments: Vec<PostComment> = Vec::new();
        for entity in entities {
            if entity.entity_type == "comment" {
                comments.push(serde_json::from_value(entity.data)?);
            }
        }
        
        // Sort by timestamp (oldest first)
        comments.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(comments)
    }

    pub async fn get_posts_by_author(&self, author_id: &str) -> Result<Vec<Post>> {
        let gsi1_pk = format!("USER#{}", author_id);
        let entities = self.dynamo.query_gsi1(&gsi1_pk, None).await?;
        
        let mut posts = Vec::new();
        for entity in entities {
            if entity.entity_type == "post" {
                posts.push(serde_json::from_value(entity.data)?);
            }
        }
        
        Ok(posts)
    }
}

#[async_trait::async_trait]
impl Repository<Post> for PostRepository {
    async fn create(&self, post: &Post) -> Result<String> {
        self.create_post(post.clone()).await
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Post>> {
        self.get_post(id).await
    }

    async fn update(&self, post: &Post) -> Result<()> {
        let entity = post.to_entity()?;
        self.dynamo.put_item(&entity).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let pk = format!("POST#{}", id);
        self.dynamo.delete_item(&pk, "METADATA").await
    }
}

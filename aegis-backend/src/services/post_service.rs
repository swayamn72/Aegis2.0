use anyhow::Result;
use crate::models::dynamodb::{Post, PostComment};
use crate::repositories::{PostRepository, Repository};

#[derive(Clone)]
pub struct PostService {
    post_repo: PostRepository,
}

impl PostService {
    pub fn new(post_repo: PostRepository) -> Self {
        Self { post_repo }
    }

    pub async fn create_post(&self, author: String, title: String, content: String, post_type: String, tags: Vec<String>) -> Result<String> {
        let post = Post {
            id: String::new(), // Will be generated in repository
            author,
            title,
            content,
            post_type,
            tags,
            likes: 0,
            comments_count: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        self.post_repo.create_post(post).await
    }

    pub async fn get_post(&self, post_id: &str) -> Result<Option<Post>> {
        self.post_repo.get_post(post_id).await
    }

    pub async fn add_comment(&self, post_id: String, author: String, content: String) -> Result<String> {
        let comment = PostComment {
            id: String::new(), // Will be generated in repository
            post_id: post_id.clone(),
            author,
            content,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let comment_id = self.post_repo.add_comment(comment).await?;

        // Update comment count
        if let Some(mut post) = self.post_repo.get_post(&post_id).await? {
            post.comments_count += 1;
            post.updated_at = chrono::Utc::now().to_rfc3339();
            self.post_repo.update(&post).await?;
        }

        Ok(comment_id)
    }

    pub async fn get_comments(&self, post_id: &str) -> Result<Vec<PostComment>> {
        self.post_repo.get_comments(post_id).await
    }

    pub async fn like_post(&self, post_id: &str) -> Result<()> {
        if let Some(mut post) = self.post_repo.get_post(post_id).await? {
            post.likes += 1;
            post.updated_at = chrono::Utc::now().to_rfc3339();
            self.post_repo.update(&post).await?;
        }
        Ok(())
    }

    pub async fn get_posts_by_author(&self, author_id: &str) -> Result<Vec<Post>> {
        self.post_repo.get_posts_by_author(author_id).await
    }
}

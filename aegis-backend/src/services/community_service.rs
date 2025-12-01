use crate::models::dynamodb::{Community, CommunityPost};
use crate::repositories::{CommunityRepository, Repository};
use anyhow::Result;

#[derive(Clone)]
pub struct CommunityService {
    community_repo: CommunityRepository,
}

impl CommunityService {
    pub fn new(community_repo: CommunityRepository) -> Self {
        Self { community_repo }
    }

    pub async fn create_community(
        &self,
        name: String,
        description: String,
        community_type: String,
        owner: String,
    ) -> Result<String> {
        let community = Community {
            id: String::new(), // Will be generated in repository
            name,
            description,
            community_type,
            owner: owner.clone(),
            moderators: vec![owner], // Owner is also a moderator
            members: Vec::new(),
            member_count: 0,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        self.community_repo.create_community(community).await
    }

    pub async fn get_community(&self, community_id: &str) -> Result<Option<Community>> {
        self.community_repo.get_community(community_id).await
    }

    pub async fn add_post_to_community(
        &self,
        community_id: String,
        post_id: String,
        pinned: bool,
        added_by: String,
    ) -> Result<String> {
        let community_post = CommunityPost {
            id: uuid::Uuid::new_v4().to_string(),
            community_id,
            post_id,
            pinned,
            added_by,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        self.community_repo
            .add_post_to_community(community_post)
            .await
    }

    pub async fn get_community_posts(&self, community_id: &str) -> Result<Vec<CommunityPost>> {
        self.community_repo.get_community_posts(community_id).await
    }

    pub async fn join_community(&self, community_id: &str, user_id: &str) -> Result<()> {
        if let Some(mut community) = self.community_repo.get_community(community_id).await? {
            if !community.members.contains(&user_id.to_string()) {
                community.members.push(user_id.to_string());
                community.member_count += 1;
                community.updated_at = chrono::Utc::now().to_rfc3339();
                self.community_repo.update(&community).await?;
            }
        }
        Ok(())
    }

    pub async fn leave_community(&self, community_id: &str, user_id: &str) -> Result<()> {
        if let Some(mut community) = self.community_repo.get_community(community_id).await? {
            if let Some(pos) = community.members.iter().position(|x| x == user_id) {
                community.members.remove(pos);
                community.member_count -= 1;
                community.updated_at = chrono::Utc::now().to_rfc3339();
                self.community_repo.update(&community).await?;
            }
        }
        Ok(())
    }

    pub async fn get_communities_by_owner(&self, owner_id: &str) -> Result<Vec<Community>> {
        self.community_repo.get_communities_by_owner(owner_id).await
    }
}

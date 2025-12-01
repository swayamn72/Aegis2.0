use super::{DynamoRepository, Repository};
use crate::models::dynamodb::{Community, CommunityPost};
use anyhow::Result;
use uuid::Uuid;

#[derive(Clone)]
pub struct CommunityRepository {
    dynamo: DynamoRepository,
}

impl CommunityRepository {
    pub fn new(dynamo: DynamoRepository) -> Self {
        Self { dynamo }
    }

    pub async fn create_community(&self, mut community: Community) -> Result<String> {
        if community.id.is_empty() {
            community.id = Uuid::new_v4().to_string();
        }

        let entity = community.to_entity()?;
        self.dynamo.put_item(&entity).await?;
        Ok(community.id)
    }

    pub async fn get_community(&self, community_id: &str) -> Result<Option<Community>> {
        let pk = format!("COMMUNITY#{}", community_id);
        if let Some(entity) = self.dynamo.get_item(&pk, "METADATA").await? {
            Ok(Some(serde_json::from_value(entity.data)?))
        } else {
            Ok(None)
        }
    }

    pub async fn add_post_to_community(&self, community_post: CommunityPost) -> Result<String> {
        let entity = community_post.to_entity()?;
        self.dynamo.put_item(&entity).await?;
        Ok(community_post.id)
    }

    pub async fn get_community_posts(&self, community_id: &str) -> Result<Vec<CommunityPost>> {
        let pk = format!("COMMUNITY#{}", community_id);
        let entities = self.dynamo.query_by_pk(&pk).await?;

        let mut posts = Vec::new();
        for entity in entities {
            if entity.entity_type == "community_post" {
                posts.push(serde_json::from_value(entity.data)?);
            }
        }

        Ok(posts)
    }

    pub async fn get_communities_by_owner(&self, owner_id: &str) -> Result<Vec<Community>> {
        let gsi1_pk = format!("OWNER#{}", owner_id);
        let entities = self.dynamo.query_gsi1(&gsi1_pk, None).await?;

        let mut communities = Vec::new();
        for entity in entities {
            if entity.entity_type == "community" {
                communities.push(serde_json::from_value(entity.data)?);
            }
        }

        Ok(communities)
    }
}

#[async_trait::async_trait]
impl Repository<Community> for CommunityRepository {
    async fn create(&self, community: &Community) -> Result<String> {
        self.create_community(community.clone()).await
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Community>> {
        self.get_community(id).await
    }

    async fn update(&self, community: &Community) -> Result<()> {
        let entity = community.to_entity()?;
        self.dynamo.put_item(&entity).await
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let pk = format!("COMMUNITY#{}", id);
        self.dynamo.delete_item(&pk, "METADATA").await
    }
}

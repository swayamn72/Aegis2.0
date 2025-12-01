use crate::models::dynamodb::GameEntity;
use anyhow::Result;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use serde_dynamo::{from_item, to_item};
use std::collections::HashMap;

#[derive(Clone)]
pub struct DynamoRepository {
    client: Client,
    table_name: String,
}

impl DynamoRepository {
    pub fn new(client: Client) -> Self {
        let table_name = std::env::var("DYNAMODB_TABLE_NAME")
            .unwrap_or_else(|_| "aegis_gaming_table".to_string());

        Self { client, table_name }
    }

    pub async fn put_item(&self, entity: &GameEntity) -> Result<()> {
        let item = to_item(entity)?;

        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_item(&self, pk: &str, sk: &str) -> Result<Option<GameEntity>> {
        let result = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(pk.to_string()))
            .key("sk", AttributeValue::S(sk.to_string()))
            .send()
            .await?;

        match result.item {
            Some(item) => Ok(Some(from_item(item)?)),
            None => Ok(None),
        }
    }

    pub async fn query_by_pk(&self, pk: &str) -> Result<Vec<GameEntity>> {
        let result = self
            .client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("pk = :pk")
            .expression_attribute_values(":pk", AttributeValue::S(pk.to_string()))
            .send()
            .await?;

        let mut entities = Vec::new();
        if let Some(items) = result.items {
            for item in items {
                entities.push(from_item(item)?);
            }
        }
        Ok(entities)
    }

    pub async fn query_gsi1(
        &self,
        gsi1_pk: &str,
        gsi1_sk_prefix: Option<&str>,
    ) -> Result<Vec<GameEntity>> {
        let mut query = self
            .client
            .query()
            .table_name(&self.table_name)
            .index_name("GSI1")
            .key_condition_expression("gsi1_pk = :gsi1_pk");

        let mut expression_values = HashMap::new();
        expression_values.insert(
            ":gsi1_pk".to_string(),
            AttributeValue::S(gsi1_pk.to_string()),
        );

        if let Some(prefix) = gsi1_sk_prefix {
            query = query
                .key_condition_expression("gsi1_pk = :gsi1_pk AND begins_with(gsi1_sk, :prefix)");
            expression_values.insert(":prefix".to_string(), AttributeValue::S(prefix.to_string()));
        }

        let result = query
            .set_expression_attribute_values(Some(expression_values))
            .send()
            .await?;

        let mut entities = Vec::new();
        if let Some(items) = result.items {
            for item in items {
                entities.push(from_item(item)?);
            }
        }
        Ok(entities)
    }

    pub async fn delete_item(&self, pk: &str, sk: &str) -> Result<()> {
        self.client
            .delete_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(pk.to_string()))
            .key("sk", AttributeValue::S(sk.to_string()))
            .send()
            .await?;

        Ok(())
    }
}

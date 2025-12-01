use aws_sdk_dynamodb::{
    types::{
        AttributeDefinition, BillingMode, GlobalSecondaryIndex, KeySchemaElement, KeyType,
        Projection, ProjectionType, ScalarAttributeType,
    },
    Client,
};

pub async fn create_gaming_table(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let table_name = std::env::var("DYNAMODB_TABLE_NAME")
        .unwrap_or_else(|_| "aegis_gaming_table".to_string());

    // 🎯 Enterprise Check: Does table already exist?
    match client.describe_table().table_name(&table_name).send().await {
        Ok(_) => {
            println!("✅ DynamoDB table '{}' already exists - skipping creation", table_name);
            return Ok(());
        }
        Err(_) => {
            println!("📊 Creating DynamoDB table '{}'...", table_name);
            // Table doesn't exist, proceed with creation
        }
    }

    // Create table only if it doesn't exist
    client
        .create_table()
        .table_name(&table_name)
        .billing_mode(BillingMode::PayPerRequest)
        // Primary Key
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(KeyType::Hash)
                .build()?,
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("sk")
                .key_type(KeyType::Range)
                .build()?,
        )
        // Attribute Definitions
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(ScalarAttributeType::S)
                .build()?,
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("sk")
                .attribute_type(ScalarAttributeType::S)
                .build()?,
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("gsi1_pk")
                .attribute_type(ScalarAttributeType::S)
                .build()?,
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("gsi1_sk")
                .attribute_type(ScalarAttributeType::S)
                .build()?,
        )
        // GSI for queries
        .global_secondary_indexes(
            GlobalSecondaryIndex::builder()
                .index_name("GSI1")
                .key_schema(
                    KeySchemaElement::builder()
                        .attribute_name("gsi1_pk")
                        .key_type(KeyType::Hash)
                        .build()?,
                )
                .key_schema(
                    KeySchemaElement::builder()
                        .attribute_name("gsi1_sk")
                        .key_type(KeyType::Range)
                        .build()?,
                )
                .projection(
                    Projection::builder()
                        .projection_type(ProjectionType::All)
                        .build(),
                )
                .build()?,
        )
        .send()
        .await?;

    println!("✅ DynamoDB table '{}' created successfully", table_name);
    Ok(())
}

use aws_config::{BehaviorVersion, Region};
use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_s3::Client as S3Client;
use std::env;

#[derive(Clone)]
pub struct AwsClients {
    pub dynamodb: DynamoClient,
    pub s3: S3Client,
}

impl AwsClients {
    pub async fn new() -> Self {
        let config = if is_local_development() {
            let region = env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string());
            aws_config::defaults(BehaviorVersion::latest())
                .region(Region::new(region))
                .endpoint_url("http://localhost:4566")
                .load()
                .await
        } else {
            let region = env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string());
            aws_config::defaults(BehaviorVersion::latest())
                .region(Region::new(region))
                .load()
                .await
        };

        Self {
            dynamodb: DynamoClient::new(&config),
            s3: S3Client::new(&config),
        }
    }
}

fn is_local_development() -> bool {
    env::var("DYNAMODB_ENDPOINT")
        .unwrap_or_default()
        .contains("localhost")
}

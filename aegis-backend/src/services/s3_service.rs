use anyhow::Result;
use aws_sdk_s3::{primitives::ByteStream, Client};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct S3Service {
    client: Client,
    bucket_name: String,
}

impl S3Service {
    pub async fn upload_with_retry(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: &str,
        max_retries: u32,
    ) -> Result<String> {
        let mut last_error = None;

        for attempt in 1..=max_retries {
            match self.upload_file(key, data.clone(), content_type).await {
                Ok(url) => return Ok(url),
                Err(e) => {
                    tracing::warn!("S3 upload attempt {} failed: {}", attempt, e);
                    last_error = Some(e);

                    if attempt < max_retries {
                        let delay = Duration::from_secs(2_u64.pow(attempt - 1)); // Exponential backoff
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap())
    }

    pub fn new(client: Client) -> Self {
        let bucket_name =
            std::env::var("S3_BUCKET_NAME").unwrap_or_else(|_| "aegis-gaming-assets".to_string());

        Self {
            client,
            bucket_name,
        }
    }

    pub async fn health_check(&self) -> Result<()> {
        // Simple health check - verify bucket exists
        self.client
            .head_bucket()
            .bucket(&self.bucket_name)
            .send()
            .await?;
        Ok(())
    }

    // ... rest of your existing methods stay the same
    pub async fn upload_file(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<String> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .send()
            .await?;

        Ok(format!("s3://{}/{}", self.bucket_name, key))
    }

    pub async fn upload_profile_picture(
        &self,
        user_id: &str,
        data: Vec<u8>,
        file_extension: &str,
    ) -> Result<String> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let key = format!(
            "profiles/{}/avatar_{}.{}",
            user_id, timestamp, file_extension
        );
        let content_type = match file_extension {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            _ => "application/octet-stream",
        };

        self.upload_file(&key, data, content_type).await
    }

    pub async fn upload_chat_attachment(
        &self,
        chat_id: &str,
        filename: &str,
        data: Vec<u8>,
    ) -> Result<String> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let key = format!("chats/{}/attachments/{}_{}", chat_id, timestamp, filename);
        let content_type = self.get_content_type_from_filename(filename);

        self.upload_file(&key, data, &content_type).await
    }

    pub async fn upload_tournament_media(
        &self,
        tournament_id: &str,
        filename: &str,
        data: Vec<u8>,
    ) -> Result<String> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let key = format!(
            "tournaments/{}/media/{}_{}",
            tournament_id, timestamp, filename
        );
        let content_type = self.get_content_type_from_filename(filename);

        self.upload_file(&key, data, &content_type).await
    }

    pub async fn get_presigned_url(&self, key: &str, expires_in_secs: u64) -> Result<String> {
        let presigning_config = aws_sdk_s3::presigning::PresigningConfig::expires_in(
            std::time::Duration::from_secs(expires_in_secs),
        )?;

        let presigned_request = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .presigned(presigning_config)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    pub async fn delete_file(&self, key: &str) -> Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await?;

        Ok(())
    }

    fn get_content_type_from_filename(&self, filename: &str) -> String {
        match filename
            .split('.')
            .last()
            .unwrap_or("")
            .to_lowercase()
            .as_str()
        {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "pdf" => "application/pdf",
            "mp4" => "video/mp4",
            "mp3" => "audio/mpeg",
            "txt" => "text/plain",
            "json" => "application/json",
            _ => "application/octet-stream",
        }
        .to_string()
    }
}

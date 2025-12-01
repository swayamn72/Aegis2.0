use std::time::Duration;
use tokio::time::sleep;

pub struct LocalStackMonitor;

impl LocalStackMonitor {
    pub async fn monitor_health() {
        tokio::spawn(async {
            loop {
                match Self::check_localstack_health().await {
                    Ok(healthy) => {
                        if !healthy {
                            tracing::warn!("🔄 LocalStack unhealthy, waiting for recovery...");
                        } else {
                            tracing::debug!("✅ LocalStack healthy");
                        }
                    }
                    Err(e) => {
                        tracing::error!("❌ LocalStack health check failed: {}", e);
                    }
                }
                sleep(Duration::from_secs(30)).await;
            }
        });
    }

    async fn check_localstack_health() -> Result<bool, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .get("http://localhost:4566/_localstack/health")
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await?;
            let dynamodb_status = body["services"]["dynamodb"].as_str().unwrap_or("unavailable");
            let s3_status = body["services"]["s3"].as_str().unwrap_or("unavailable");
            
            Ok(dynamodb_status == "running" && s3_status == "running")
        } else {
            Ok(false)
        }
    }
}

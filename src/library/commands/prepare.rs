use crate::{library::secrets_sources, models::full_config::FullConfig};

/// Sync secrets and set environment variables
pub async fn prepare(config: &FullConfig, secrets: &serde_json::Value) {
    secrets_sources::sync(config, secrets).await;
}

use crate::{
    library::secrets_sources,
    models::{config::Config, project_config::ProjectConfig},
};

/// Sync secrets and set environment variables
pub async fn prepare(project_config: &ProjectConfig, config: &Config, secrets: &serde_json::Value) {
    secrets_sources::sync(project_config, config, secrets).await;
}

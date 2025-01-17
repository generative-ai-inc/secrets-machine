use std::collections::HashMap;

use crate::{library::secrets_sources, models::full_config::FullConfig};

/// Sync secrets and set environment variables
pub async fn prepare(
    config: &FullConfig,
    mocked_keyring_env_vars_map: Option<HashMap<String, (String, String), std::hash::RandomState>>,
) {
    secrets_sources::sync(config, mocked_keyring_env_vars_map).await;
}

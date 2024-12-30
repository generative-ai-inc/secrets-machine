use crate::{
    library::utils::env_vars,
    models::config::{Config, SecretsSource},
};

use super::bitwarden;

pub async fn check(config: &Config, secrets: &serde_json::Value) {
    for secrets_source in &config.secrets_sources {
        match secrets_source {
            SecretsSource::Bitwarden(credentials) => {
                env_vars::make_sure_exists(Some(secrets), &credentials.access_token_name).await;
                bitwarden::check_installation().await;
            }
        }
    }
}

pub async fn sync(
    config: &Config,
    secrets: &serde_json::Value,
    env_vars: &mut Vec<(String, String, String)>,
) {
    for secrets_source in &config.secrets_sources {
        match secrets_source {
            SecretsSource::Bitwarden(credentials) => {
                let bw_env_vars = bitwarden::get_env_variables(Some(credentials), secrets).await;

                for (key, value) in bw_env_vars {
                    env_vars.push((key, value, "bitwarden".to_string()));
                }
            }
        }
    }
}

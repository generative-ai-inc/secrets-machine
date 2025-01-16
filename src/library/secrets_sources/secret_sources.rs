use std::collections::HashMap;

use crate::{
    library::utils::env_vars,
    models::{
        config::{Config, SecretsSource},
        project_config::ProjectConfig,
    },
};

use super::{aliases, bitwarden, keyring};

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

pub async fn sync(project_config: &ProjectConfig, config: &Config, secrets: &serde_json::Value) {
    let vars_iter = std::env::vars();

    let mut original_env_vars: HashMap<String, String> = HashMap::new();

    for (key, value) in vars_iter {
        original_env_vars.insert(key, value);
    }

    let mut env_vars: Vec<(String, String, String)> = Vec::new();

    let keyring_env_vars = keyring::get_env_variables(secrets).await;

    env_vars.extend(keyring_env_vars);

    for secrets_source in &config.secrets_sources {
        match secrets_source {
            SecretsSource::Bitwarden(credentials) => {
                let bw_env_vars = bitwarden::get_env_variables(Some(credentials), secrets).await;

                env_vars.extend(bw_env_vars);
            }
        }
    }

    // Set the environment variables to the current process
    env_vars::set(&env_vars);

    let alias_env_vars = aliases::add(project_config).await;

    // Set the aliases to the current process
    env_vars::set(&alias_env_vars);

    let mut complete_env_vars = env_vars.clone();
    complete_env_vars.extend(alias_env_vars);

    if config.general.print_secrets_table {
        env_vars::print_variables_box(original_env_vars, &complete_env_vars).await;
    }
}

use std::{collections::HashMap, env, error::Error};

use crate::{
    library::{
        secrets_sources,
        utils::{env_vars, logging},
    },
    models::config::Config,
};

/// Sync secrets and set environment variables
///
/// # Errors
/// - If the secrets map is not found
/// - If the environment variables fail to be set
pub async fn prepare(config: &Config, secrets: &serde_json::Value) -> Result<(), Box<dyn Error>> {
    let vars_iter = env::vars();

    let mut original_env_vars: HashMap<String, String> = HashMap::new();

    for (key, value) in vars_iter {
        original_env_vars.insert(key, value);
    }

    let mut env_vars: Vec<(String, String, String)> = Vec::new();

    let Some(secrets_map) = secrets.as_object() else {
        return Err(Box::from("Secrets map not found"));
    };

    // Add keyring secrets to the environment variables
    for (key, value) in secrets_map {
        if let Some(value) = value.as_str() {
            env_vars.push((key.to_string(), value.to_string(), "keyring".to_string()));
        } else {
            logging::error(&format!("Failed to set secret {key} from keyring")).await;
        }
    }

    secrets_sources::sync(config, secrets, &mut env_vars).await;

    env_vars::set(&env_vars);

    if config.general.print_secrets_table {
        env_vars::print_variables_box(original_env_vars, &env_vars).await;
    }

    Ok(())
}

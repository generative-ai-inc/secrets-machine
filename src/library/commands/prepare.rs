use std::{collections::HashMap, env};

use crate::{
    library::{secrets_sources, utils::env_vars},
    models::config::Config,
};

/// Prepare the environment for the service to run.
/// - Add features
/// - Set the environment variables
pub async fn prepare(config: &Config, secrets: &serde_json::Value) {
    let vars_iter = env::vars();

    let mut original_env_vars: HashMap<String, String> = HashMap::new();

    for (key, value) in vars_iter {
        original_env_vars.insert(key, value);
    }

    let mut env_vars: Vec<(String, String, String)> = Vec::new();

    // Add keyring secrets to the environment variables
    for (key, value) in secrets.as_object().unwrap() {
        env_vars.push((
            key.to_string(),
            value.as_str().unwrap().to_string(),
            "keyring".to_string(),
        ));
    }

    secrets_sources::start(config, secrets, &mut env_vars).await;

    env_vars::set(&env_vars).await;

    env_vars::print_variables_box(original_env_vars, &env_vars).await;
}

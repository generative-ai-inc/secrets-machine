use std::collections::HashMap;

use crate::library::utils::{env_vars, logging};
use crate::models::full_config::FullConfig;

pub async fn add(
    config: &FullConfig,
    local_env_vars_map: &HashMap<String, (String, String), std::hash::RandomState>,
    process_env_vars_map: &HashMap<String, (String, String), std::hash::RandomState>,
    sources_env_vars_map: &HashMap<String, (String, String), std::hash::RandomState>,
    keyring_env_vars_map: &HashMap<String, (String, String), std::hash::RandomState>,
) -> HashMap<String, (String, String)> {
    let mut aliases_env_vars_map: HashMap<String, (String, String)> = HashMap::new();
    for (key, value) in &config.aliases {
        let Some(alias_value) = env_vars::get_from_all(
            value,
            &HashMap::with_capacity(0),
            local_env_vars_map,
            process_env_vars_map,
            keyring_env_vars_map,
            sources_env_vars_map,
        ) else {
            logging::warn(&format!(
                "Environment variable {value} not found while adding aliases. Skipping alias {key} -> {value}"
            ))
            .await;
            continue;
        };

        aliases_env_vars_map.insert(
            key.to_string(),
            (alias_value.to_string(), format!("aliased to {value}")),
        );
    }

    aliases_env_vars_map
}

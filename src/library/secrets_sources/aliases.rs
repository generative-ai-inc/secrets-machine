use crate::library::utils::logging;
use crate::models::full_config::FullConfig;

pub async fn add(full_config: &FullConfig) -> Vec<(String, String, String)> {
    let mut env_vars: Vec<(String, String, String)> = Vec::new();
    for (key, value) in &full_config.aliases {
        let Ok(alias_value) = std::env::var(value) else {
            logging::warn(&format!(
                "Environment variable {value} not found while adding aliases. Skipping alias {key} -> {value}"
            ))
            .await;
            continue;
        };

        env_vars.push((key.to_string(), alias_value, format!("aliased to {value}")));
    }

    env_vars
}

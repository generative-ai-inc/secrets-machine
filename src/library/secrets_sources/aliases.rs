use crate::library::utils::logging;
use crate::models::project_config::ProjectConfig;

pub async fn add(project_config: &ProjectConfig) -> Vec<(String, String, String)> {
    let mut env_vars: Vec<(String, String, String)> = Vec::new();
    for (key, value) in &project_config.aliases {
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

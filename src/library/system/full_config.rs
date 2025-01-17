use std::collections::HashMap;

use crate::{
    library::utils::logging,
    models::{
        full_config::FullConfig, project_config::ProjectConfig, secret_source::SecretsSource,
        user_config::UserConfig,
    },
};

async fn merge_secrets_sources(
    project_secrets_sources: &[SecretsSource],
    user_secrets_sources: &[SecretsSource],
) -> Vec<SecretsSource> {
    let mut full_secrets_sources = project_secrets_sources.to_vec();
    let mut replaced_sources = 0;
    for (index, source) in user_secrets_sources.iter().enumerate() {
        if full_secrets_sources.contains(source) {
            // Remove the source from the full secrets sources
            replaced_sources += 1;
            full_secrets_sources.remove(index);
            full_secrets_sources.push(source.clone());
        }

        full_secrets_sources.push(source.clone());
    }
    if replaced_sources > 0 {
        logging::warn(&format!(
            "Replaced {replaced_sources} secret sources defined in user config",
        ))
        .await;
    }
    full_secrets_sources
}

fn replace_hash_map(
    initial_map: &HashMap<String, String>,
    override_map: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut new_map = initial_map.clone();
    for (key, value) in override_map {
        new_map.insert(key.clone(), value.clone());
    }
    new_map
}

#[must_use]
pub async fn get(project_config: &ProjectConfig, user_config: &UserConfig) -> FullConfig {
    FullConfig {
        general: user_config.general.clone(),
        secrets_sources: merge_secrets_sources(
            &user_config.secrets_sources,
            &project_config.secrets_sources,
        )
        .await,
        commands: replace_hash_map(&user_config.commands, &project_config.commands),
        pre_commands: replace_hash_map(&user_config.pre_commands, &project_config.pre_commands),
        aliases: replace_hash_map(&user_config.aliases, &project_config.aliases),
    }
}

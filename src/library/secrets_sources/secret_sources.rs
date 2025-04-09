use std::collections::{BTreeMap, HashMap};

use crate::{
    library::utils::env_vars,
    models::{full_config::FullConfig, secret_source::SecretsSource},
};

use super::{aliases, bitwarden, keyring, local, process};

pub async fn check(config: &FullConfig) {
    let mut check_bitwarden_installation = false;

    for secrets_source in &config.secrets_sources {
        match secrets_source {
            SecretsSource::Bitwarden(_) => {
                check_bitwarden_installation = true;
            }
        }
    }

    if check_bitwarden_installation {
        bitwarden::check_installation().await;
    }
}

fn update_btree_map(
    env_vars_map: &mut BTreeMap<String, (String, String)>,
    new_env_vars: &HashMap<String, (String, String)>,
) {
    for (key, value) in new_env_vars {
        env_vars_map.insert(key.clone(), value.clone());
    }
}

fn update_hash_map(
    env_vars_map: &mut HashMap<String, (String, String)>,
    new_env_vars: &HashMap<String, (String, String)>,
) {
    for (key, value) in new_env_vars {
        env_vars_map.insert(key.clone(), value.clone());
    }
}

/// Get the process environment variables that are also defined in other sources
fn get_clean_process_env_vars(
    aliases_env_vars: &HashMap<String, (String, String)>,
    local_env_vars: &HashMap<String, (String, String)>,
    process_env_vars: &HashMap<String, (String, String)>,
    sources_env_vars: &HashMap<String, (String, String)>,
    keyring_env_vars: &HashMap<String, (String, String)>,
) -> HashMap<String, (String, String)> {
    let mut clean_process_env_vars = HashMap::new();

    for key in keyring_env_vars.keys() {
        if let Some(value) = process_env_vars.get(key) {
            clean_process_env_vars.insert(key.clone(), value.clone());
        }
    }

    for key in sources_env_vars.keys() {
        if let Some(value) = process_env_vars.get(key) {
            clean_process_env_vars.insert(key.clone(), value.clone());
        }
    }

    // From here below, other sources override the process environment variables

    for (key, value) in local_env_vars {
        if process_env_vars.get(key).is_some() {
            clean_process_env_vars.insert(key.clone(), value.clone());
        }
    }

    for (key, value) in aliases_env_vars {
        if process_env_vars.get(key).is_some() {
            clean_process_env_vars.insert(key.clone(), value.clone());
        }
    }

    clean_process_env_vars
}

fn produce_env_vars_map(
    aliases_env_vars: &HashMap<String, (String, String)>,
    local_env_vars: &HashMap<String, (String, String)>,
    process_env_vars: &HashMap<String, (String, String)>,
    sources_env_vars: &HashMap<String, (String, String)>,
    keyring_env_vars: &HashMap<String, (String, String)>,
) -> BTreeMap<String, (String, String)> {
    let mut env_vars_map: BTreeMap<String, (String, String)> = BTreeMap::new();

    let clean_process_env_vars = get_clean_process_env_vars(
        aliases_env_vars,
        local_env_vars,
        process_env_vars,
        sources_env_vars,
        keyring_env_vars,
    );

    update_btree_map(&mut env_vars_map, keyring_env_vars);
    update_btree_map(&mut env_vars_map, sources_env_vars);
    update_btree_map(&mut env_vars_map, &clean_process_env_vars);
    update_btree_map(&mut env_vars_map, local_env_vars);
    update_btree_map(&mut env_vars_map, aliases_env_vars);

    env_vars_map
}

pub async fn sync(
    config: &FullConfig,
    mocked_keyring_env_vars_map: Option<HashMap<String, (String, String), std::hash::RandomState>>,
) -> BTreeMap<String, (String, String)> {
    let local_env_vars_map = local::get_env_variables();
    let process_env_vars_map = process::get_env_variables();
    let keyring_env_vars_map = match mocked_keyring_env_vars_map {
        Some(mocked_keyring_env_vars) => mocked_keyring_env_vars,
        None => keyring::get_env_variables().await,
    };

    let mut sources_env_vars_map: HashMap<String, (String, String)> = HashMap::new();
    for secrets_source in &config.secrets_sources {
        match secrets_source {
            SecretsSource::Bitwarden(credentials) => {
                let bw_env_vars = bitwarden::get_env_variables(
                    credentials,
                    &local_env_vars_map,
                    &process_env_vars_map,
                    &keyring_env_vars_map,
                )
                .await;

                update_hash_map(&mut sources_env_vars_map, &bw_env_vars);
            }
        }
    }

    let aliases_env_vars = aliases::add(
        config,
        &local_env_vars_map,
        &process_env_vars_map,
        &sources_env_vars_map,
        &keyring_env_vars_map,
    )
    .await;

    let env_vars_map = produce_env_vars_map(
        &aliases_env_vars,
        &local_env_vars_map,
        &process_env_vars_map,
        &sources_env_vars_map,
        &keyring_env_vars_map,
    );

    if config.general.print_secrets_table {
        env_vars::print_variables_box(&env_vars_map).await;
    }

    env_vars_map
}

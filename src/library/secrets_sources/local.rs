use dotenvy::{EnvLoader, EnvSequence};
use std::collections::HashMap;
#[must_use]
pub fn get_env_variables() -> HashMap<String, (String, String)> {
    let env_map_res = EnvLoader::new().sequence(EnvSequence::InputOnly).load();

    let mut env_vars_map: HashMap<String, (String, String)> = HashMap::new();

    let Ok(env_map) = env_map_res else {
        return env_vars_map;
    };

    for (key, value) in env_map {
        env_vars_map.insert(key, (value, ".env".to_string()));
    }

    env_vars_map
}

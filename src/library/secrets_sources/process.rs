use std::collections::HashMap;

#[must_use]
pub fn get_env_variables() -> HashMap<String, (String, String)> {
    let vars_iter = std::env::vars();

    let mut env_vars_map: HashMap<String, (String, String)> = HashMap::new();

    for (key, value) in vars_iter {
        env_vars_map.insert(key, (value, "process".to_string()));
    }

    env_vars_map
}

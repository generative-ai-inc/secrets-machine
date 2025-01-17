use std::collections::HashMap;

use crate::library::utils::logging;
use keyring::Entry;
use serde_json::json;

pub async fn get_secrets() -> serde_json::Value {
    // Attempt to get the secrets from the keyring
    let entry_result = Entry::new("secrets-machine", "default");
    let entry = match entry_result {
        Ok(entry) => entry,
        Err(e) => {
            logging::error(&format!(
                "Failed to create keyring entry while getting secrets: {e}"
            ))
            .await;
            return json!({});
        }
    };
    let password = entry.get_password();

    match password {
        Ok(password) => {
            let deserialization_result: Result<serde_json::Value, serde_json::Error> =
                serde_json::from_str(&password);
            match deserialization_result {
                Ok(secrets) => secrets,
                Err(e) => {
                    logging::error(&format!("Failed to deserialize secrets: {e}")).await;
                    json!({})
                }
            }
        }
        Err(_) => {
            json!({})
        }
    }
}

pub async fn set_secret(secrets: serde_json::Value) {
    let serialization_result = serde_json::to_string(&secrets);
    let secrets_str = match serialization_result {
        Ok(secrets_str) => secrets_str,
        Err(e) => {
            logging::error(&format!("Failed to serialize secrets: {e}")).await;
            return;
        }
    };

    let entry_result = Entry::new("secrets-machine", "default");
    let entry = match entry_result {
        Ok(entry) => entry,
        Err(e) => {
            logging::error(&format!(
                "Failed to create keyring entry while setting secret: {e}"
            ))
            .await;
            return;
        }
    };
    let res = entry.set_password(&secrets_str);

    if res.is_err() {
        logging::error("Could not save secrets in keyring").await;
    }
}

pub async fn get_env_variables() -> HashMap<String, (String, String)> {
    let keyring_secrets = get_secrets().await;

    let mut env_vars_map: HashMap<String, (String, String)> = HashMap::new();

    let Some(secrets_map) = keyring_secrets.as_object() else {
        logging::error("Secrets map not found").await;
        std::process::exit(1);
    };

    // Add keyring secrets to the environment variables
    for (key, value) in secrets_map {
        if let Some(value) = value.as_str() {
            env_vars_map.insert(key.to_string(), (value.to_string(), "keyring".to_string()));
        } else {
            logging::error(&format!("Failed to set secret {key} from keyring")).await;
        }
    }

    env_vars_map
}

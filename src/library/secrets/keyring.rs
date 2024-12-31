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

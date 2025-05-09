use std::{collections::HashMap, env, error::Error, path::PathBuf};

use sm::{
    library::system::{full_config, project_config, user_config},
    models::full_config::FullConfig,
};
use tokio::fs;

/// Sets up the test environment
///
/// # Errors
#[allow(dead_code)]
pub async fn setup(
    use_bw: bool,
) -> Result<(FullConfig, HashMap<String, (String, String)>), Box<dyn Error>> {
    env::set_var("TEST_ENV_VAR", "beautiful");
    // Make test_results directory
    if let Err(e) = fs::create_dir_all("tests/test_results").await {
        return Err(Box::from(format!(
            "Failed to create test_results directory: {e}"
        )));
    }
    let mut project_config_path = PathBuf::from("tests/assets/secrets_machine.toml");

    if use_bw {
        project_config_path = PathBuf::from("tests/assets/secrets_machine_bw.toml");
    }

    let project_config = project_config::parse(project_config_path).await;

    let user_config_path = PathBuf::from("tests/assets/user_config.toml");
    let user_config = user_config::parse(Some(user_config_path)).await;
    let full_config = full_config::get(&project_config, &user_config).await;

    let Ok(mocked_keyring_env_vars_map) = get_mock_secrets().await else {
        return Err(Box::from("Failed to get mock secrets"));
    };
    Ok((full_config, mocked_keyring_env_vars_map))
}

/// Cleans up the test environment
///
/// # Errors
/// - If the test results directory cannot be deleted
#[allow(dead_code)]
pub async fn teardown() {
    // Delete the test file
    let _ = fs::remove_dir_all("tests/test_results").await;
}

/// Sets up the test environment
///
/// # Errors
/// - If the file cannot be read
/// - If the file value does not match the expected value
#[allow(dead_code)]
pub async fn assert_text_result(
    test_name: &str,
    expected_value: &str,
) -> Result<(), Box<dyn Error>> {
    let file_path = format!("tests/test_results/{test_name}.txt");
    // Make test_results directory
    match fs::read_to_string(file_path).await {
        Ok(value) => {
            if value.trim() == expected_value.trim() {
                Ok(())
            } else {
                Err(Box::from(format!(
                    "File value does not match expected value.\nExpected: {expected_value}\nActual: {value}"
                )))
            }
        }
        Err(e) => Err(Box::from(format!("Failed to read file: {e}"))),
    }
}

/// Gets the mock secrets
///
/// # Errors
/// - If the file cannot be read
/// - If the file cannot be parsed as JSON
///
/// # Panics
/// - If the file cannot be read
/// - If the file cannot be parsed as JSON
#[allow(dead_code)]
pub async fn get_mock_secrets() -> Result<HashMap<String, (String, String)>, Box<dyn Error>> {
    match fs::read_to_string("tests/assets/secrets.json").await {
        Ok(value) => {
            let secrets: serde_json::Value = serde_json::from_str(&value)?;

            let mut secrets_map = HashMap::new();
            for (key, value) in secrets.as_object().unwrap() {
                let secret_value = value.as_str().unwrap();

                let secret_name = key.to_string();
                secrets_map.insert(
                    secret_name,
                    (secret_value.to_string(), "keyring".to_string()),
                );
            }

            Ok(secrets_map)
        }
        Err(e) => Err(Box::from(format!("Failed to read file: {e}"))),
    }
}

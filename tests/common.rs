use std::{env, error::Error, path::PathBuf};

use sm::{
    library::system::{commands_config, config},
    models::{commands_config::CommandsConfig, config::Config},
};
use tokio::fs;

/// Sets up the test environment
///
/// # Errors
#[allow(dead_code)]
pub async fn setup() -> Result<(CommandsConfig, Config, serde_json::Value), Box<dyn Error>> {
    env::set_var("TEST_ENV_VAR", "beautiful");
    // Make test_results directory
    if let Err(e) = fs::create_dir_all("tests/test_results").await {
        return Err(Box::from(format!(
            "Failed to create test_results directory: {e}"
        )));
    }
    let commands_config_path = PathBuf::from("tests/assets/secrets_machine.toml");
    let commands_config = commands_config::parse(commands_config_path).await;

    let config_path = PathBuf::from("tests/assets/secrets_machine.toml");
    let config = config::parse(Some(config_path)).await;

    let Ok(secrets) = get_mock_secrets().await else {
        return Err(Box::from("Failed to get mock secrets"));
    };
    Ok((commands_config, config, secrets))
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
#[allow(dead_code)]
pub async fn get_mock_secrets() -> Result<serde_json::Value, Box<dyn Error>> {
    match fs::read_to_string("tests/assets/secrets.json").await {
        Ok(value) => {
            let secrets: serde_json::Value = serde_json::from_str(&value)?;
            Ok(secrets)
        }
        Err(e) => Err(Box::from(format!("Failed to read file: {e}"))),
    }
}

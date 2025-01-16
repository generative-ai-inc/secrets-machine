use std::path::PathBuf;

use tokio::fs::{self};

use crate::{library::utils::logging, models::project_config::ProjectConfig};

/// Checks that the config file is set up correctly
pub async fn parse(config_path: PathBuf) -> ProjectConfig {
    // Read the TOML file
    let toml_content = fs::read_to_string(config_path).await;

    let toml_content = match toml_content {
        Ok(content) => content,
        Err(e) => {
            logging::error(&format!("Error reading commands config file: {e}")).await;
            std::process::exit(1);
        }
    };

    // Parse the TOML content
    let config_result = toml::from_str(&toml_content);

    let config: ProjectConfig = match config_result {
        Ok(parsed_config) => parsed_config,
        Err(e) => {
            logging::error(&format!("Error parsing commands config file: {e}")).await;
            std::process::exit(1);
        }
    };

    config
}

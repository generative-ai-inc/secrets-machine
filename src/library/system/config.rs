use std::path::PathBuf;

use tokio::fs::{self};

use crate::{library::utils::logging, models::config::Config};

async fn create_default_config(config_path: PathBuf) -> Config {
    let config = Config::default();
    let toml_content = toml::to_string(&config).unwrap();
    fs::write(&config_path, toml_content).await.unwrap();
    config
}

/// Checks that the config file is set up correctly
pub async fn parse(config_path: Option<PathBuf>) -> Config {
    let config_path = match config_path {
        Some(path) => path,
        None => {
            let home_dir = dirs::home_dir().expect("Failed to get home directory");
            home_dir.join(".config/secrets-machine/config.toml")
        }
    };

    // Read the TOML file
    let toml_content = fs::read_to_string(&config_path).await;

    let toml_content = match toml_content {
        Ok(content) => content,
        Err(_) => {
            logging::info(&format!(
                "Creating default config file at {}",
                config_path.display()
            ))
            .await;
            return create_default_config(config_path).await;
        }
    };

    // Parse the TOML content
    let config_result: Result<Config, toml::de::Error> = toml::from_str(&toml_content);

    let config: Config = match config_result {
        Ok(parsed_config) => {
            if toml_content.is_empty() {
                return create_default_config(config_path).await;
            }

            parsed_config
        }

        Err(e) => {
            logging::error(&format!("Error parsing config file: {}", e)).await;
            std::process::exit(1);
        }
    };

    config
}

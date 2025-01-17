use crate::{library::utils::logging, models::full_config::FullConfig};

/// Check that the command is in the configuration
pub async fn check(full_config: &FullConfig, command: &String) {
    if !full_config.commands.contains_key(command) {
        logging::error(&format!("Command {command} not found in the config")).await;
        std::process::exit(1);
    }
}

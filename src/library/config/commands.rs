use crate::{library::utils::logging, models::project_config::ProjectConfig};

/// Check that the command is in the configuration
pub async fn check(commands_config: &ProjectConfig, command: &String) {
    if !commands_config.commands.contains_key(command) {
        logging::error(&format!("Command {command} not found in the config")).await;
        std::process::exit(1);
    }
}

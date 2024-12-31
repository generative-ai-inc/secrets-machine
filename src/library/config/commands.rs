use crate::{library::utils::logging, models::commands_config::CommandsConfig};

/// Check that the command is in the configuration
pub async fn check(commands_config: &CommandsConfig, command: &String) {
    if !commands_config.commands.contains_key(command) {
        logging::error(&format!("Command {command} not found in the config")).await;
        std::process::exit(1);
    }
}

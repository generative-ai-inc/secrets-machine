use std::{collections::HashMap, error::Error};

use tokio::process::Command;

use crate::{
    library::{
        secrets_sources,
        system::command,
        utils::{env_vars, logging},
    },
    models::full_config::FullConfig,
};

/// Runs a command specified in the config file with the secrets machine
///
/// # Errors
/// - If the command is not found in the config file
/// - If the command fails to execute
/// - If the function fails to get the output of the command
/// - If the function fails to kill the process
pub async fn run(
    config: &FullConfig,
    command_name: &str,
    command_args: &str,
    mocked_keyring_env_vars_map: Option<HashMap<String, (String, String), std::hash::RandomState>>,
) -> Result<(), Box<dyn Error>> {
    let env_vars_map = secrets_sources::sync(config, mocked_keyring_env_vars_map).await;

    if let Some(pre_command) = config.pre_commands.get(command_name) {
        let result = command::run(pre_command).await;
        match result {
            Ok(output) => {
                logging::info(&format!("Output: {output}")).await;
                logging::info("✅ Pre command completed successfully").await;
            }
            Err(e) => {
                logging::error(e.to_string().as_str().trim()).await;
                logging::error("🛑 Failed to run pre command").await;
                return Err(e);
            }
        }
    }

    let Some(command) = config.commands.get(command_name) else {
        return Err(Box::from("Command not found"));
    };

    let full_command = format!("{command} {command_args}");
    logging::nl().await;
    logging::print_color(logging::BG_GREEN, " Running command ").await;
    logging::info(&format!(
        "Running: {}",
        env_vars::replace(&env_vars_map, &full_command, true).await
    ))
    .await;

    let Ok(child) = Command::new("sh")
        .arg("-c")
        .arg(&full_command)
        .envs(
            env_vars_map
                .iter()
                .map(|(key, value)| (key.as_str(), value.0.as_str())),
        )
        .spawn()
    else {
        return Err(Box::from("Failed to execute command"));
    };

    let Some(pid) = child.id() else {
        return Err(Box::from("Failed to get child pid"));
    };
    let handle = child.wait_with_output();

    tokio::spawn(async move {
        logging::nl().await;
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                logging::info("👍 Shutting down gracefully...").await;
            }
            Err(e) => {
                logging::error(&format!("🛑 Failed to listen for Ctrl+C: {e}")).await;
            }
        }
        match Command::new("kill").arg(pid.to_string()).status().await {
            Ok(_) => {
                logging::info("✅ All processes have been terminated.").await;
            }
            Err(e) => {
                logging::error(&format!("🛑 Failed to kill process: {e}")).await;
            }
        }
    });

    match handle.await {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(Box::from("Command failed"))
            }
        }
        Err(e) => Err(Box::from(format!(
            "🛑 Failed to wait for command execution: {e}"
        ))),
    }
}

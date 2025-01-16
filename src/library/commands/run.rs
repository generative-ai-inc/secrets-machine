use std::error::Error;

use tokio::process::Command;

use crate::{
    library::{
        system::command,
        utils::{env_vars, logging},
    },
    models::{config::Config, project_config::ProjectConfig},
};

use super::prepare;

/// Runs a command specified in the config file with the secrets machine
///
/// # Errors
/// - If the command is not found in the config file
/// - If the command fails to execute
/// - If the function fails to get the output of the command
/// - If the function fails to kill the process
pub async fn run(
    project_config: ProjectConfig,
    config: Config,
    secrets: serde_json::Value,
    command_name: String,
    command_args: String,
) -> Result<(), Box<dyn Error>> {
    prepare(&project_config, &config, &secrets).await;

    if let Some(pre_command) = project_config.pre_commands.get(&command_name) {
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

    let Some(command) = project_config.commands.get(&command_name) else {
        return Err(Box::from("Command not found"));
    };

    let full_command = format!("{command} {command_args}");
    logging::nl().await;
    logging::print_color(logging::BG_GREEN, " Running command ").await;
    logging::info(&format!(
        "Running: {}",
        env_vars::replace(&full_command, true).await
    ))
    .await;

    let Ok(child) = Command::new("sh").arg("-c").arg(&full_command).spawn() else {
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

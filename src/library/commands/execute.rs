use std::{collections::HashMap, env, error::Error};

use tokio::process::Command;

use crate::{
    library::{
        secrets_sources,
        utils::{env_vars, logging},
    },
    models::full_config::FullConfig,
};

/// Executes a command with the secrets machine
///
/// # Errors
/// - When `return_output` is true:
///   - If the command fails to execute
///   - If function fails to get the output of the command
///   - If function fails to kill the process
///
/// - When `return_output` is false:
///   - Never
pub async fn execute(
    config: &FullConfig,
    command_to_run: &str,
    mocked_keyring_env_vars_map: Option<HashMap<String, (String, String), std::hash::RandomState>>,
) -> Result<(), Box<dyn Error>> {
    let env_vars_map = secrets_sources::sync(config, mocked_keyring_env_vars_map).await;

    logging::nl().await;
    logging::print_color(logging::BG_GREEN, " Executing command ").await;
    logging::info(&format!(
        "Executing: {}",
        env_vars::replace(&env_vars_map, command_to_run, true).await
    ))
    .await;

    // Get the default shell from the SHELL environment variable
    let default_shell = match env::var("SHELL") {
        Ok(shell) => shell,
        Err(_) => "/bin/sh".to_string(),
    };

    let Ok(child) = Command::new(default_shell)
        .arg("-c")
        .arg(command_to_run)
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

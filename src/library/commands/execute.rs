use std::env;

use tokio::process::Command;

use crate::{
    library::utils::{env_vars, logging},
    models::config::Config,
};

use super::prepare;

pub async fn execute(config: Config, secrets: serde_json::Value, command_to_run: &str) {
    prepare(&config, &secrets).await;

    logging::nl().await;
    logging::print_color(logging::BG_GREEN, " Executing command ").await;
    logging::info(&format!(
        "Running: {}",
        env_vars::replace_env_vars(command_to_run, true).await
    ))
    .await;

    // Get the default shell from the SHELL environment variable
    let default_shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

    let child: tokio::process::Child = Command::new(default_shell)
        .arg("-c")
        .arg(command_to_run)
        .envs(env::vars())
        .spawn()
        .expect("Failed to execute command");

    let pid = child.id().expect("Failed to get child pid");
    let handle = child.wait_with_output();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        logging::nl().await;
        logging::info("👍 Shutting down gracefully...").await;
        let result = Command::new("kill").arg(&pid.to_string()).status().await;

        match result {
            Ok(_) => {
                logging::info("✅ All processes have been terminated.").await;
                std::process::exit(0);
            }
            Err(e) => {
                logging::error(&format!("🛑 Failed to kill process: {}", e)).await;
                std::process::exit(1);
            }
        }
    });

    let output = handle.await;

    match output {
        Ok(output) => {
            if output.status.success() {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }
        Err(e) => {
            logging::error(&format!("🛑 Failed to wait for command execution: {}", e)).await;
            std::process::exit(1);
        }
    }
}

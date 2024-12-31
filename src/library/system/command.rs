use std::{env, error::Error};

use tokio::process::Command;

/// Runs a command and returns the output
///
/// # Errors
/// - If the command fails to execute
/// - If the function fails to get the output of the command
pub async fn run(command_str: &str) -> Result<String, Box<dyn Error>> {
    // Get the default shell from the SHELL environment variable
    let default_shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

    let run_results = Command::new(default_shell)
        .arg("-c")
        .arg(command_str)
        .envs(env::vars())
        .output()
        .await;

    match run_results {
        Ok(output) => {
            if output.status.success() {
                return Ok(String::from_utf8_lossy(&output.stdout).to_string());
            }
            Err(Box::from(String::from_utf8_lossy(&output.stderr)))
        }
        Err(e) => Err(Box::from(format!("🛑 Failed to check result: {e}"))),
    }
}

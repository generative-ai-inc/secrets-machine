use crate::library::utils::{env_vars, logging};
use crate::models::config::BitwardenCredentials;
use regex::Regex;
use tokio::process::Command;

pub async fn check_installation() {
    let bws_version_result = Command::new("bws").arg("--version").output().await;

    match bws_version_result {
        Ok(output) => {
            logging::info(&format!(
                "Bitwarden Secrets Manager CLI version: {}",
                String::from_utf8_lossy(&output.stdout).trim()
            ))
            .await;
            return;
        }
        Err(_) => {
            logging::error("Bitwarden Secrets Manager CLI not found via `bws` command. Go to https://github.com/bitwarden/sdk-sm/tree/main/crates/bws and follow the instructions to install it.").await;
            std::process::exit(1);
        }
    }
}

pub async fn get_env_variables(
    credentials: Option<&BitwardenCredentials>,
    secrets: &serde_json::Value,
) -> Vec<(String, String)> {
    let env_var_name = match credentials {
        Some(credentials) => credentials.access_token_name.as_str(),
        None => "BWS_ACCESS_TOKEN",
    };
    let access_token = env_vars::get_from_all(secrets, env_var_name).await;

    let bitwarden_result = Command::new("bws")
        .args(&[
            "secret",
            "list",
            "--output",
            "env",
            "--access-token",
            &access_token,
        ])
        .output()
        .await;

    match bitwarden_result {
        Ok(bitwarden_output) => {
            if bitwarden_output.status.success() {
                let re = Regex::new(r#"^([A-Z0-9_]+)="(.+)""#).unwrap();

                let env_vars_str = String::from_utf8_lossy(&bitwarden_output.stdout);
                let mut env_vars: Vec<(String, String)> = Vec::new();

                for line in env_vars_str.lines() {
                    if let Some(caps) = re.captures(line) {
                        let key = &caps[1];
                        let value = &caps[2];

                        env_vars.push((key.to_string(), value.to_string()));
                    }
                }

                env_vars
            } else {
                logging::error(&format!(
                    "🛑 Failed to retrieve bitwarden environment variables: {}",
                    String::from_utf8_lossy(&bitwarden_output.stderr)
                ))
                .await;
                std::process::exit(1);
            }
        }
        Err(e) => {
            logging::error(&format!(
                "🛑 Failed to retrieve bitwarden environment variables: {}",
                e
            ))
            .await;
            std::process::exit(1);
        }
    }
}

use std::collections::HashMap;

use crate::library::system;
use crate::library::utils::{env_vars, logging};
use crate::models::secret_source::BitwardenCredentials;
use regex::Regex;
use tokio::process::Command;

pub async fn check_installation() {
    if let Ok(output_string) = system::command::run("bws --version").await {
        logging::info(&format!(
            "Bitwarden Secrets Manager CLI version: {}",
            output_string.trim()
        ))
        .await;
    } else {
        logging::error("Bitwarden Secrets Manager CLI not found via `bws` command. Go to https://github.com/bitwarden/sdk-sm/tree/main/crates/bws and follow the instructions to install it.").await;
        std::process::exit(1);
    }
}

pub async fn get_env_variables(
    credentials: &BitwardenCredentials,
    local_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
    process_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
    keyring_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
) -> HashMap<String, (String, String)> {
    let env_var_name = credentials.access_token_name.as_str();
    let access_token = env_vars::get_or_exit(
        env_var_name,
        &HashMap::with_capacity(0),
        local_env_vars,
        process_env_vars,
        keyring_env_vars,
        &HashMap::with_capacity(0),
    )
    .await;

    let bitwarden_result = Command::new("bws")
        .args([
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
                let Ok(re) = Regex::new(r#"^([A-Z0-9_]+)="(.+)""#) else {
                    logging::error(
                        "Failed to create regex while retrieving bitwarden environment variables",
                    )
                    .await;
                    return HashMap::new();
                };

                let env_vars_str = String::from_utf8_lossy(&bitwarden_output.stdout);
                let mut env_vars: HashMap<String, (String, String)> = HashMap::new();

                for line in env_vars_str.lines() {
                    if let Some(caps) = re.captures(line) {
                        let key = &caps[1];
                        let value = &caps[2];

                        env_vars.insert(
                            key.to_string(),
                            (value.to_string(), "bitwarden".to_string()),
                        );
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
                "🛑 Failed to retrieve bitwarden environment variables: {e}"
            ))
            .await;
            std::process::exit(1);
        }
    }
}

use std::io::Write;

use crate::library::utils::logging;

/// It will ask the user to input a secret in the terminal and return it.
pub async fn ask_for_secret(name: &str) -> String {
    let mut secret = String::new();
    while secret.is_empty() {
        print!("Please enter the value for secret {name}: ");
        if let Err(e) = std::io::stdout().flush() {
            logging::error(&format!("Failed to flush stdout: {e}")).await;
            continue;
        }
        let secret_result = rpassword::read_password();
        match secret_result {
            Ok(passed_secret) => {
                secret = passed_secret;
                break;
            }
            Err(e) => {
                logging::error(&format!("Failed to read secret value: {e}")).await;
            }
        }
    }
    secret
}

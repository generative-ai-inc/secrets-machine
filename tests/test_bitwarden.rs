use std::path::PathBuf;

use serde_json::json;
use sm::{
    library::system::config,
    models::config::{BitwardenCredentials, SecretsSource},
};

use sm::library::secrets_sources::bitwarden;

#[tokio::test]
async fn test_legacy_bitwarden_parse_env_vars() {
    dotenv::dotenv().ok();

    let credentials = BitwardenCredentials {
        access_token_name: "BWS_ACCESS_TOKEN".to_string(),
    };

    let env_vars = bitwarden::get_env_variables(Some(&credentials), &json!({})).await;

    assert!(!env_vars.is_empty());
}

#[tokio::test]
async fn test_bitwarden_parse_env_vars() {
    let config_path = PathBuf::from("tests/assets/config.toml");
    let config = config::parse(Some(config_path)).await;

    dotenv::dotenv().ok();

    for secrets_source in &config.secrets_sources {
        match secrets_source {
            SecretsSource::Bitwarden(credentials) => {
                let env_vars = bitwarden::get_env_variables(Some(credentials), &json!({})).await;
                assert!(!env_vars.is_empty());
            }
        }
    }
}

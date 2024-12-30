use std::path::PathBuf;

use sm::library::system::config;

#[tokio::test]
async fn test_parse_config() {
    let config_path = PathBuf::from("tests/assets/secrets_machine.toml");
    let _ = config::parse(Some(config_path)).await;
}

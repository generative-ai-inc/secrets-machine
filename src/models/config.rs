use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct BitwardenCredentials {
    pub access_token_name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case", tag = "name")]
pub enum SecretsSource {
    Bitwarden(BitwardenCredentials),
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PythonpathFeature {
    pub env_file_path: String,
    pub pythonpath_value: String,
}

fn default_secrets_sources() -> Vec<SecretsSource> {
    vec![SecretsSource::Bitwarden(BitwardenCredentials {
        access_token_name: "BWS_ACCESS_TOKEN".to_string(),
    })]
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    #[serde(default = "default_secrets_sources")]
    pub secrets_sources: Vec<SecretsSource>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            secrets_sources: default_secrets_sources(),
        }
    }
}

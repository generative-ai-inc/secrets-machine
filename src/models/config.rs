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

fn default_general() -> General {
    General {
        print_secrets_table: true,
    }
}

fn default_secrets_sources() -> Vec<SecretsSource> {
    vec![]
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct General {
    pub print_secrets_table: bool,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    #[serde(default = "default_general")]
    pub general: General,

    #[serde(default = "default_secrets_sources")]
    pub secrets_sources: Vec<SecretsSource>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: default_general(),
            secrets_sources: vec![SecretsSource::Bitwarden(BitwardenCredentials {
                access_token_name: "BWS_ACCESS_TOKEN".to_string(),
            })],
        }
    }
}

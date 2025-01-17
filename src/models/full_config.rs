use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    general_settings::General,
    secret_source::{BitwardenCredentials, SecretsSource},
};

fn default_general() -> General {
    General {
        print_secrets_table: true,
    }
}

fn default_secrets_sources() -> Vec<SecretsSource> {
    vec![]
}

fn default_commands() -> HashMap<String, String> {
    HashMap::new()
}

fn default_pre_commands() -> HashMap<String, String> {
    HashMap::new()
}

fn default_aliases() -> HashMap<String, String> {
    HashMap::new()
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct FullConfig {
    #[serde(default = "default_general")]
    pub general: General,

    #[serde(default = "default_secrets_sources")]
    pub secrets_sources: Vec<SecretsSource>,

    #[serde(default = "default_commands")]
    pub commands: HashMap<String, String>,

    #[serde(default = "default_pre_commands")]
    pub pre_commands: HashMap<String, String>,

    #[serde(default = "default_aliases")]
    pub aliases: HashMap<String, String>,
}

impl Default for FullConfig {
    fn default() -> Self {
        FullConfig {
            general: default_general(),
            secrets_sources: vec![SecretsSource::Bitwarden(BitwardenCredentials {
                access_token_name: "BWS_ACCESS_TOKEN".to_string(),
            })],
            commands: HashMap::new(),
            pre_commands: HashMap::new(),
            aliases: HashMap::new(),
        }
    }
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{general_settings::General, secret_source::SecretsSource};

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
pub struct UserConfig {
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

impl Default for UserConfig {
    fn default() -> Self {
        UserConfig {
            general: default_general(),
            secrets_sources: vec![],
            commands: HashMap::new(),
            pre_commands: HashMap::new(),
            aliases: HashMap::new(),
        }
    }
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::secret_source::SecretsSource;

fn default_commands() -> HashMap<String, String> {
    HashMap::new()
}

fn default_pre_commands() -> HashMap<String, String> {
    HashMap::new()
}

fn default_aliases() -> HashMap<String, String> {
    HashMap::new()
}

fn default_secrets_sources() -> Vec<SecretsSource> {
    vec![]
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectConfig {
    #[serde(default = "default_commands")]
    pub commands: HashMap<String, String>,

    #[serde(default = "default_pre_commands")]
    pub pre_commands: HashMap<String, String>,

    #[serde(default = "default_aliases")]
    pub aliases: HashMap<String, String>,

    #[serde(default = "default_secrets_sources")]
    pub secrets_sources: Vec<SecretsSource>,
}

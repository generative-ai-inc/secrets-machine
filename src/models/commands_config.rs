use std::collections::HashMap;

use serde::{Deserialize, Serialize};

fn default_commands() -> HashMap<String, String> {
    HashMap::new()
}

fn default_pre_commands() -> HashMap<String, String> {
    HashMap::new()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CommandsConfig {
    #[serde(default = "default_commands")]
    pub commands: HashMap<String, String>,

    #[serde(default = "default_pre_commands")]
    pub pre_commands: HashMap<String, String>,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct BitwardenCredentials {
    pub access_token_name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case", tag = "name")]
pub enum SecretsSource {
    Bitwarden(BitwardenCredentials),
}

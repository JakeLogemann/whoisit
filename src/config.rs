use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub whois: WhoisConfig,
    pub ip_lists: Vec<IPList>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WhoisConfig {
    #[serde(default)]
    pub disabled: bool,
    pub primary_server: String,
    pub secondary_server: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IPList {
    #[serde(default)]
    pub disabled: bool,
    pub name: String,
    pub asns: Vec<i32>,
}

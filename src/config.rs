use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;
use tokio::fs;
use tokio::prelude::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub whois: WhoisConfig,
    pub ip_lists: Vec<IPList>,
}

impl Config {
    pub async fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Config> {
        // fill the config buffer (cfg_buf) with the contents of config.toml
        let mut cfg_buf = Vec::new();
        let mut cfg_file = fs::File::open(path).await?;
        cfg_file.read_to_end(&mut cfg_buf).await?;
        // parse the config object from the config buffer created above.
        let cfg: Config = toml::from_slice(&cfg_buf)?;
        return Ok(cfg);
    }
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

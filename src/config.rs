use crate::{bgpview, prelude::*};
use futures::{stream, StreamExt}; // 0.3.5
use serde::{Deserialize, Serialize};
use std::{io, path::Path};
use tokio::{fs, prelude::*};

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct IPList {
  pub name: String,
  pub ips:  Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WhoisConfig {
  #[serde(default)]
  pub enabled: bool,
  pub backend: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
  pub whois:    WhoisConfig,
  pub ip_lists: Vec<IPListConfig>,
}

impl Config {
  /// attempt to read the config from a toml file at given path.
  pub async fn from_path(path: impl AsRef<Path>) -> Result<Config, Box<dyn std::error::Error>> {
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
pub struct IPListConfig {
  /// if set, this alias is ignored entirely.
  #[serde(default)]
  pub disabled: bool,
  /// unique identifier for this alias
  pub name:     String,
  /// "automated systems" numbers (aka "ASNs").
  #[serde(default)]
  pub asns:     Vec<i32>,
  /// domains which should be resolved and aliased.
  #[serde(default)]
  pub domains:  Vec<String>,
}

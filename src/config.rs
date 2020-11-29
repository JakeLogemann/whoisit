use crate::whois::Config as WhoisConfig;
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
pub struct Config {
  pub whois:    WhoisConfig,
  pub ip_lists: Vec<IPListConfig>,
}

impl Config {
  /// attempt to read the config from a toml file at given path.
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

impl IPListConfig {
  pub async fn resolve_asns(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let ip_set: dashmap::DashSet<String> = dashmap::DashSet::new();

    // create an unordered streaming buffer of ASN prefix requests.
    stream::iter(self.asns.clone())
      .map(|asn_id| async move { WhoisConfig::lookup_asn_prefixes(asn_id).await })
      .buffer_unordered(2_usize)
      .for_each(|res: Result<Vec<String>, surf::Error>| async {
        match res {
          Ok(ips) => ips.iter().for_each(|ip| {
            ip_set.insert(ip.clone());
          }),
          Err(e) => eprintln!("FATAL: {:?}", e),
        }
      })
      .await;

    Ok(ip_set.iter().map(|v| v.clone()).collect::<Vec<_>>())
  }
}

use crate::{ip_list::IPList, prelude::*};

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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WhoisConfig {
  #[serde(default)]
  pub enabled: bool,
  pub backend: String,
}

impl Default for WhoisConfig {
  fn default() -> Self {
    Self {
      enabled: true,
      backend: "bgpview".to_string(),
    }
  }
}

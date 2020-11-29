use crate::prelude::*;

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
pub struct WhoisConfig {
  #[serde(default)]
  pub enabled: bool,
  pub backend: String,
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

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct IPList {
  name: String,
  ips:  Vec<IpNet>,
}

impl IPList {
  /// add an IP address to the list.
  pub fn new(name: impl AsRef<str>) -> Self {
    let mut ls = Self::default();
    ls.name = name.as_ref().to_string();
    ls
  }

  /// get the name of this list.
  pub fn name(&self) -> String { self.name.clone() }

  /// get the ips contained in this list.
  pub fn ips(&self) -> Vec<IpNet> { self.ips.clone() }

  /// get the ips contained in this list.
  pub fn ip_strs(&self) -> Vec<String> { self.ips.iter().map(|ip| ip.to_string()).collect() }

  /// add an IP address, represented as a string-like to the list.
  pub fn add_ip_str(&mut self, ipnet_str: String) -> color_eyre::eyre::Result<()> {
    self.ips.push(ipnet_str.parse::<IpNet>()?);
    Ok(())
  }

  /// add an IP address to the list.
  pub fn add_ip(&mut self, ipaddr: &IpAddr) -> color_eyre::eyre::Result<()> {
    self.ips.push(ipaddr.clone().into());
    Ok(())
  }

  /// add an IP address to the list.
  pub fn add_ip_net(&mut self, ipnet: &IpNet) -> color_eyre::eyre::Result<()> {
    self.ips.push(ipnet.clone());
    Ok(())
  }
}

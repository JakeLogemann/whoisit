use crate::prelude::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct IPList {
  name:   String,
  ipnets: BTreeSet<IpNet>,
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

  /// add an IP address, represented as a string-like to the list.
  pub fn add_ip_str(&mut self, ipnet_str: impl AsRef<str>) -> AnyResult<()> {
    self.add_ipnet(ipnet_str.as_ref().to_string().parse::<IpNet>()?)
  }

  /// add an IP address to the list.
  pub fn add_ip<N: Clone+Into<IpNet>>(&mut self, ipaddr: &N) -> AnyResult<()> {
    let ipnet: IpNet = ipaddr.clone().into();
    self.add_ipnet(ipnet)
  }

  /// add an IP address to the list.
  pub fn add_ipnet(&mut self, ipnet: IpNet) -> AnyResult<()> {
    self.ipnets.insert(ipnet);
    Ok(())
  }

  /// get the ips contained in this list.
  pub fn ipnets_stringified(&self) -> Vec<String> {
    self.ipnets_vec().iter().map(|ip| ip.to_string()).collect()
  }

  /// get the [ip networks][`IpNet`] contained in this list.
  pub fn ipnets_vec(&self) -> Vec<IpNet> {
    IpNet::aggregate(&self.ipnets.iter().cloned().collect_vec())
  }
}

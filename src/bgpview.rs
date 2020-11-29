//! bgpview.io response types
//!
//! Generated by: [QuickType.io](https://app.quicktype.io/)
use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct ASNPrefixes {
  pub status:         String,
  pub status_message: String,
  pub data:           Data,
  #[serde(rename = "@meta")]
  pub meta:           Meta,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
  pub ipv4_prefixes: Vec<IpvPrefix>,
  pub ipv6_prefixes: Vec<IpvPrefix>,
}

#[derive(Serialize, Deserialize)]
pub struct IpvPrefix {
  pub prefix:       String,
  pub ip:           String,
  pub cidr:         i64,
  pub roa_status:   RoaStatus,
  pub name:         String,
  pub description:  String,
  pub country_code: String,
  pub parent:       Parent,
}

#[derive(Serialize, Deserialize)]
pub struct Parent {
  pub prefix:            String,
  pub ip:                String,
  pub cidr:              i64,
  pub rir_name:          RirName,
  pub allocation_status: AllocationStatus,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
  pub time_zone:      String,
  pub api_version:    i64,
  pub execution_time: String,
}

#[derive(Serialize, Deserialize)]
pub enum AllocationStatus {
  #[serde(rename = "unknown")]
  Unknown,
}

#[derive(Serialize, Deserialize)]
pub enum RirName {
  #[serde(rename = "AfriNIC")]
  AfriNic,
  #[serde(rename = "APNIC")]
  Apnic,
  #[serde(rename = "RIPE")]
  Ripe,
  #[serde(rename = "ARIN")]
  Arin,
  #[serde(rename = "Lacnic")]
  Lacnic,
}

#[derive(Serialize, Deserialize)]
pub enum RoaStatus {
  None,
}

pub async fn lookup_asn_prefixes(asn_id: i32) -> Result<Vec<String>, surf::Error> {
  let mut ip_list = vec![];

  let response = surf::get(&self::asn_prefixes_url(asn_id))
    .send()
    .await?
    .body_json::<ASNPrefixes>()
    .await?;

  response
    .data
    .ipv4_prefixes
    .iter()
    .for_each(|v4| ip_list.push(v4.prefix.clone()));
  response
    .data
    .ipv6_prefixes
    .iter()
    .for_each(|v6| ip_list.push(v6.prefix.clone()));
  Ok(ip_list)
}

pub fn asn_prefixes_url(asn_id: i32) -> String {
  format!("https://api.bgpview.io/asn/{}/prefixes", asn_id)
}

pub async fn resolve_asns(asns: Vec<i32>) -> Result<Vec<String>, Box<dyn StdError>> {
  let ip_set: DashSet<String> = DashSet::new();

  // create an unordered streaming buffer of ASN prefix requests.
  stream::iter(asns)
    .map(|asn_id| async move { self::lookup_asn_prefixes(asn_id).await })
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

  let ip_list = ip_set
    .iter()
    .map(|v| v.to_string())
    .sorted()
    .collect::<Vec<_>>();

  Ok(ip_list)
}

/// convert a [ip list config][`IPListConfig`] into an [ip list][`IPList`].
pub async fn generate_ip_list(
  li: IPListConfig,
) -> Result<IPList, Box<dyn std::error::Error+'static>> {
  info!("generate \"{}\" ip list", &li.name);
  let mut ip_list = IPList::new(&li.name);

  info!("resolve ASNs for ip list \"{}\"", &li.name);
  for ip in self::resolve_asns(li.asns.clone()).await?.iter() {
    ip_list.add_ip_str(ip.to_string())?;
  }

  Ok(ip_list)
}

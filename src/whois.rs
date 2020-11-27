use crate::bgpview::ASNPrefixes;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub enabled: bool,
    pub backend: String,
}

impl Config {
    pub async fn lookup_asn_prefixes(asn_id: i32) -> Result<Vec<String>, surf::Error> {
        let mut ip_list = vec![];

        let response = surf::get(&Self::asn_prefixes_url(asn_id)?)
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

    pub fn asn_prefixes_url(asn_id: i32) -> anyhow::Result<String> {
        Ok(format!("https://api.bgpview.io/asn/{}/prefixes", asn_id))
    }
}

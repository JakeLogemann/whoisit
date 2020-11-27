use futures::{stream, StreamExt}; // 0.3.5

#[allow(unused_imports)]
use ::{
    maxminddb::Reader,
    net2::{TcpBuilder, TcpListenerExt, TcpStreamExt, UdpBuilder, UdpSocketExt},
    std::collections::HashMap,
    std::env,
    std::io,
    std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream, UdpSocket},
    tokio::fs,
    tokio::prelude::*,
};

pub mod bgpview;
pub mod config;
pub mod whois;
use whois::Config as WhoisConfig;

const CONCURRENT_REQUESTS: usize = 2;

pub type IPList = Vec<String>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfgfile = env::args().nth(1).unwrap_or("config.toml".to_string());
    let cfg = config::Config::from_path(cfgfile).await?;
    let ip_lists: dashmap::DashMap<String, IPList> = dashmap::DashMap::new();

    for li in cfg.ip_lists.iter() {
        let ip_set: dashmap::DashSet<String> = dashmap::DashSet::new();

        // create an unordered streaming buffer of ASN prefix requests.
        stream::iter(li.asns.clone())
            .map(|asn_id| async move { WhoisConfig::lookup_asn_prefixes(asn_id).await })
            .buffer_unordered(CONCURRENT_REQUESTS)
            .for_each(|res: Result<IPList, surf::Error>| async {
                match res {
                    Ok(ips) => ips.iter().for_each(|ip| {
                        ip_set.insert(ip.clone());
                    }),
                    Err(e) => eprintln!("FATAL: {:?}", e),
                }
            })
            .await;
        // insert this ip list into the final result.
        ip_lists.insert(
            li.name.clone(),
            ip_set.iter().map(|v| v.clone()).collect::<Vec<_>>(),
        );
    }

    let mut res = HashMap::new();
    ip_lists.iter().for_each(|e| {
        res.insert(e.key().clone(), e.value().clone());
    });
    println!("{}", serde_json::to_string_pretty(&res)?);

    Ok(())
}

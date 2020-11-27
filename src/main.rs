use futures::{stream, StreamExt}; // 0.3.5

#[allow(unused_imports)]
use ::{
    maxminddb::Reader,
    net2::{TcpBuilder, TcpListenerExt, TcpStreamExt, UdpBuilder, UdpSocketExt},
    std::env,
    std::collections::HashMap,
    std::io,
    std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream, UdpSocket},
    tokio::fs,
    tokio::prelude::*,
};

pub mod bgpview;
pub mod config;

const CONCURRENT_REQUESTS: usize = 2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfgfile = env::args().nth(1).unwrap_or("config.toml".to_string());
    let cfg = config::Config::from_path(cfgfile).await?;
    let client = surf::Client::new();
    let ip_lists: dashmap::DashMap<String, Vec<String>> = dashmap::DashMap::new();

    for li in cfg.ip_lists.iter() {
      let ip_set: dashmap::DashSet<String> = dashmap::DashSet::new();

      // create an unordered streaming buffer of ASN prefix requests.
      let asn_lists = stream::iter(li.asns.clone()).map(|asn_id| {
        let client = &client;
        let url = format!("https://api.bgpview.io/asn/{}/prefixes", asn_id);
        async move {
            client.get(&url).send().await?.body_json::<bgpview::BgpView>().await
        }
      }).buffer_unordered(CONCURRENT_REQUESTS);
      
      asn_lists
        .for_each(|res: Result<bgpview::BgpView, surf::Error>| async {
          match res {
            Ok(view) => {
              for pfx in view.data.ipv4_prefixes {
                ip_set.insert(pfx.prefix);
              }
            },
            Err(e) => eprintln!("FATAL: {:?}", e),
          }

        })
        .await;
      ip_lists.insert(li.name.clone(), ip_set.iter().map(|v| v.clone()).collect::<Vec<_>>());
    }

    let mut res = HashMap::new();
    for kvpair in ip_lists.iter() {
      let (list_name, list_ips) = kvpair.pair();
      res.insert(list_name.clone(), list_ips.iter().map(|ip| ip.clone()).collect::<Vec<_>>());
    }
    println!("{}", serde_json::to_string_pretty(&res)?);

    Ok(())
}

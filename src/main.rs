
pub mod prelude {
  #[allow(unused_imports)]
  pub use ::{
    futures::{stream, StreamExt},
    maxminddb::Reader,
    net2::{TcpBuilder, TcpListenerExt, TcpStreamExt, UdpBuilder, UdpSocketExt},
    std::collections::HashMap,
    std::env,
    std::io,
    std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream, UdpSocket},
    ipnet::*,
    tokio::fs,
    tokio::prelude::*,
    trust_dns_resolver::Resolver as DNSResolver,
    trust_dns_resolver::config::ResolverOpts as DNSResolverOpts,
    trust_dns_resolver::config::ResolverConfig as DNSResolverConfig,
    dashmap::DashMap,
    dashmap::DashSet,
  };
}
use prelude::*;

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
    let mut ip_lists = HashMap::new();

    // add each list item to the ip lists
    for li in cfg.ip_lists.iter(){
      let name = li.name.clone();

      println!("# Fetching ASN Prefixes {} ...", &name);
      let mut ip_list: Vec<String> = li.resolve_asns().await?;

      println!("# Resolving Domains: {} ...", &name);
      let mut resolver_opts = DNSResolverOpts::default();
      let resolver = DNSResolver::new(DNSResolverConfig::cloudflare(), resolver_opts)?;
      for domain in li.domains.iter() {
        let domain_ips = resolver.lookup_ip(domain.as_str())?;
        for ip in domain_ips.into_iter() {
          ip_list.push(ip.to_string());
        }
      } 

      let res_file = format!("target/{}.txt", &name);
      fs::write(&res_file, &format!("{}\n", ip_list.join("\n"))).await?;

      ip_lists.insert(name, ip_list);
    }

    fs::write("last-run.json", &serde_json::to_string_pretty(&ip_lists)?).await?;

    Ok(())
}

pub mod prelude;
use prelude::*;

pub mod bgpview;
pub mod config;
pub mod whois;
use whois::Config as WhoisConfig;

const CONCURRENT_REQUESTS: usize = 2;

pub type IPList = Vec<String>;

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
  use tracing_subscriber::{fmt, prelude::*, EnvFilter};
  let default_log_level = EnvFilter::try_new("info");

  if cfg!(debug) {
    let default_log_level = EnvFilter::try_new("debug");
    std::env::set_var("RUST_LIB_BACKTRACE", "1");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("DEV_MODE", "1");
  }

  let fmt_layer = fmt::layer().with_target(false);
  let filter_layer = EnvFilter::try_from_default_env()
    .or_else(|_| default_log_level)
    .unwrap();
  tracing_subscriber::registry()
    .with(filter_layer)
    .with(fmt_layer)
    .init();

  color_eyre::install()?;
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  setup_logging()?;

  let cfgfile = env::args().nth(1).unwrap_or("config.toml".to_string());
  let cfg = config::Config::from_path(cfgfile).await?;
  let mut ip_lists = HashMap::new();

  // add each list item to the ip lists
  for li in cfg.ip_lists.iter() {
    let name = li.name.clone();

    info!("Fetching ASN Prefixes {} ...", &name);
    let mut ip_list: Vec<String> = li.resolve_asns().await?;

    info!("domains {}", &name);
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

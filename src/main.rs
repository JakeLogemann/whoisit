pub mod prelude;
use prelude::*;

pub mod bgpview;
pub mod config;
use config::{IPList, IPListConfig};

const CONCURRENT_REQUESTS: usize = 2;

/// global, singleton, sync-only, DNS resolver.
pub static RESOLVER: Lazy<DNSResolver> = Lazy::new(|| {
  let resolver_opts = DNSResolverOpts::default();
  let resolver_config = DNSResolverConfig::cloudflare();
  let resolver = DNSResolver::new(resolver_config, resolver_opts).expect("can resolve dns domains");
  resolver
});

/// initialize the logger and the error reporting hook.
fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
  use tracing_subscriber::{
    filter::{Directive, EnvFilter},
    fmt,
    prelude::*,
  };
  let default_log_level = EnvFilter::try_new("info");

  if cfg!(debug) {
    let default_log_level = EnvFilter::try_new("debug");
    std::env::set_var("RUST_LIB_BACKTRACE", "1");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("DEV_MODE", "1");
  }

  let fmt_layer = fmt::layer().with_target(false);
  let filter_layer = EnvFilter::try_from_default_env()
    .or_else(|_| default_log_level)?
    .add_directive("whoisit=debug".parse()?)
    .add_directive("surf=warn".parse()?);
  tracing_subscriber::registry()
    .with(filter_layer)
    .with(fmt_layer)
    .init();

  color_eyre::install()?;
  Ok(())
}

/// load the configuration from file.
pub async fn load_config() -> Result<config::Config, Box<dyn std::error::Error>> {
  let cfgfile = env::args().nth(1).unwrap_or("config.toml".to_string());
  let cfg = config::Config::from_path(cfgfile).await?;
  Ok(cfg)
}

#[tokio::main]
/// main program entrypoint function
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  setup_logging()?;
  let cfg = load_config().await?;
  let (tx, mut rx) = mpsc::channel(10);

  // add each list item to the ip lists
  for li in cfg.ip_lists.clone().into_iter() {
    tokio::spawn({
      // Each task needs its own `tx` handle to send messages. This is done by
      // cloning the original handle.
      let tx = tx.clone();
      let fut_ip_list = bgpview::generate_ip_list(li.clone());
      async move {
        let mut ip_list = fut_ip_list.await.unwrap();

        info!("resolve domains for ip list \"{}\"", &li.name);
        for domain in li.domains.iter() {
          for ip in RESOLVER.lookup_ip(domain.as_str()).unwrap().into_iter() {
            debug!(
              "domain \"{}\" in ip list \"{}\" resolves to {}",
              &domain.as_str(),
              &li.name,
              &ip
            );
            ip_list.add_ip(&ip).unwrap();
          }
        }
        info!("resolved DNS domains for ip list \"{}\"", &li.name);

        tx.send(ip_list).await.unwrap();
      }
    });
  }

  // The `rx` half of the channel returns `None` once **all** `tx` clones
  // drop. To ensure `None` is returned, drop the handle owned by the
  // current task. If this `tx` handle is not dropped, there will always
  // be a single outstanding `tx` handle.
  drop(tx);

  // create the output directory.
  let output_dir = "target/ip_lists";
  fs::create_dir_all(output_dir).await?;

  // save each incoming ip list to a file in given directory.
  while let Some(ip_list) = rx.recv().await {
    // write the JSON file.
    let json_file = format!("{}/{}.json", &output_dir, &ip_list.name());
    fs::write(&json_file, &serde_json::to_string_pretty(&ip_list)?).await?;
    info!(
      "ip list {name} saved to {json_file}",
      name = &ip_list.name(),
      json_file = &json_file
    );
    let txt_file = format!("{}/{}.txt", &output_dir, &ip_list.name());
    fs::write(
      &txt_file,
      &ip_list.ips().iter().map(|ip| ip.to_string()).join("\n"),
    )
    .await?;
    info!(
      "ip list {name} saved to {txt_file}",
      name = &ip_list.name(),
      txt_file = &txt_file
    );
  }

  Ok(())
}

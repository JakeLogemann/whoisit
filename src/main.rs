#[allow(unused_imports)]
use ::{
    actix::actors::resolver,
    actix::prelude::*,
    maxminddb::Reader,
    net2::{TcpBuilder, TcpListenerExt, TcpStreamExt, UdpBuilder, UdpSocketExt},
    std::io,
    std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream, UdpSocket},
    tokio::fs,
    tokio::prelude::*,
};

pub mod config;
use config::Config;

#[tokio::main]
async fn main() -> io::Result<()> {
    // fill the config buffer (cfg_buf) with the contents of config.toml
    let mut cfg_buf = Vec::new();
    let mut cfg_file = fs::File::open("config.toml").await?;
    cfg_file.read_to_end(&mut cfg_buf).await?;
    // parse the config object from the config buffer created above.
    let cfg: Config = toml::from_slice(&cfg_buf)?;
    for l in cfg.ip_lists.iter() {
        println!("{}", l.name);
    }
    Ok(())
}

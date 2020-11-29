//! common development prelude
#![allow(unused_imports)]

pub use dashmap::{DashMap, DashSet};
pub use futures::{stream, StreamExt};
pub use ipnet::*;
pub use maxminddb::Reader;
pub use net2::{TcpBuilder, TcpListenerExt, TcpStreamExt, UdpBuilder, UdpSocketExt};
pub use std::{
  collections::HashMap,
  env,
  io,
  net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream, UdpSocket},
};
pub use tokio::{fs, prelude::*};
pub use trust_dns_resolver::{
  config::{ResolverConfig as DNSResolverConfig, ResolverOpts as DNSResolverOpts},
  Resolver as DNSResolver,
};

pub use tracing::{
  debug,
  debug_span,
  error,
  error_span,
  info,
  info_span,
  span,
  warn,
  warn_span,
  Level,
};

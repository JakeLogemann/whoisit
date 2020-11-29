//! common development prelude
#![allow(unused_imports)]

pub use dashmap::{DashMap, DashSet};
pub use futures::{stream, StreamExt};
pub use ipnet::*;
pub use itertools::Itertools;
pub use net2::{TcpBuilder, TcpListenerExt, TcpStreamExt, UdpBuilder, UdpSocketExt};
pub use serde::{Deserialize, Serialize};
pub use std::{
  collections::HashMap,
  env,
  error::Error as StdError,
  path::Path,
  io,
  net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpListener, TcpStream, UdpSocket},
};
pub use tokio::{fs, prelude::*, sync::mpsc};
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

pub use once_cell::sync::{Lazy, OnceCell};

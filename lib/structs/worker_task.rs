use crate::server::ServerConfig;

use std::{net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;

pub struct WorkerTask {
  pub lookup_socket: Arc<UdpSocket>,
  pub socket: Arc<UdpSocket>,
  pub config: Arc<ServerConfig>,

  pub payload: Vec<u8>,
  pub source: SocketAddr,
  pub debug: bool,
}

impl WorkerTask {
  pub fn new(
    socket: Arc<UdpSocket>,
    lookup_socket: Arc<UdpSocket>,
    config: Arc<ServerConfig>,
    payload: Vec<u8>,
    source: SocketAddr,
    debug: bool,
  ) -> Self {
    Self { lookup_socket, socket, config, payload, source, debug }
  }
}

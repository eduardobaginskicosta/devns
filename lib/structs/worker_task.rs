use crate::structs::ServerConfig;

use std::{net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;

pub struct WorkerTask {
  lookup_socket: Arc<UdpSocket>,
  socket: Arc<UdpSocket>,
  config: Arc<ServerConfig>,

  payload: Vec<u8>,
  source: SocketAddr,
  debug: bool,
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

  pub fn lookup_socket(&self) -> &Arc<UdpSocket> {
    &self.lookup_socket
  }
  pub fn socket(&self) -> &Arc<UdpSocket> {
    &self.socket
  }
  pub fn config(&self) -> &Arc<ServerConfig> {
    &self.config
  }
  pub fn payload(&self) -> &[u8] {
    &self.payload
  }
  pub fn source(&self) -> &SocketAddr {
    &self.source
  }
  pub fn debug(&self) -> bool {
    self.debug
  }
}

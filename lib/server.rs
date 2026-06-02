use crate::{
  log_debug,
  structs::{ServerConfig, WorkerTask},
  workers::worker_pool,
};

use local_ip_address::local_ip;
use std::{
  io::{Error, ErrorKind},
  net::{IpAddr, Ipv4Addr},
  sync::Arc,
  time::Duration,
};
use tokio::{
  net::UdpSocket,
  spawn,
  sync::mpsc::{Sender, channel},
};

// === CONSTANTS ===

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(20);
const BUFFER_SIZE: usize = 1280;

// === DNS SERVER ===

#[derive(Debug)]
pub struct DnsServer {
  pub config: ServerConfig,

  lookup_client: UdpSocket,
  worker_tx: Sender<WorkerTask>,
  socket: UdpSocket,
}

impl DnsServer {
  // * shared socket setup
  async fn bind_socket(addr: &str) -> Result<UdpSocket, Error> {
    UdpSocket::bind(addr).await.map_err(Into::into)
  }

  async fn create_sockets(port: u16) -> Result<(UdpSocket, UdpSocket), Error> {
    let host_ip: IpAddr = local_ip().map_err(|e| Error::new(ErrorKind::Other, e))?;
    let lookup_client: UdpSocket = UdpSocket::bind("0.0.0.0:0").await?;
    let socket: UdpSocket = UdpSocket::bind((host_ip, port)).await?;
    Ok((lookup_client, socket))
  }

  // * new instance
  pub async fn new(config: ServerConfig) -> Result<Self, Error> {
    let (lookup_client, socket) = Self::create_sockets(config.port).await?;
    let (worker_tx, worker_rx) = channel::<WorkerTask>(config.max_messages);

    spawn(async move {
      worker_pool(worker_rx, config.max_workers, config.max_messages, config.debug).await;
    });

    Ok(Self { config, lookup_client, worker_tx, socket })
  }

  // * start server
  pub async fn start(self) -> Result<(), Error> {
    let lookup_client: Arc<UdpSocket> = Arc::new(self.lookup_client);
    let socket: Arc<UdpSocket> = Arc::new(self.socket);
    let config: Arc<ServerConfig> = Arc::new(self.config);

    let mut buffer: [u8; BUFFER_SIZE] = [0u8; BUFFER_SIZE];
    loop {
      match socket.recv_from(&mut buffer).await {
        Ok((len, src)) => {
          let data: Vec<u8> = buffer[..len].to_vec();
          let socket: Arc<UdpSocket> = Arc::clone(&socket);
          let lookup: Arc<UdpSocket> = Arc::clone(&lookup_client);
          let config: Arc<ServerConfig> = Arc::clone(&config);
          let debug: bool = config.debug;

          if let Err(e) = self
            .worker_tx
            .send(WorkerTask::new(socket, lookup, config, data, src, debug))
            .await
          {
            if debug {
              log_debug!("Error handling query from {src}: {e}");
            }
          }
        },
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => continue,
        Err(e) => {
          if config.debug {
            log_debug!("Socket error: {e}");
          }
        },
      }
    }
  }
}

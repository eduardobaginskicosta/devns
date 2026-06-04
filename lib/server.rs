use crate::{
  log_debug, structs::WorkerTask, utils::startup_banner, workers::worker_pool,
};

use local_ip_address::local_ip;
use std::{
  io::{Error, ErrorKind},
  net::{IpAddr, Ipv4Addr, Ipv6Addr},
  path::Path,
  pin::Pin,
  sync::Arc,
};
use tokio::{
  fs::read_to_string,
  net::UdpSocket,
  spawn,
  sync::mpsc::{Sender, channel},
};

// === CONSTANTS ===

const DEFAULT_MAX_MESSAGES: usize = 20;
const DEFAULT_MAX_WORKERS: usize = 10;

// === DNS ZONE ===

#[derive(Debug, Clone)]
pub struct DnsZone {
  pub nameserves: Vec<Ipv4Addr>,
  pub ipv4_addrs: Vec<Ipv4Addr>,
  pub ipv6_addrs: Vec<Ipv6Addr>,
  pub domains: Vec<String>,
  pub ttl: u32,
  pub mx: String,
}

impl DnsZone {
  pub fn new() -> Self {
    Self {
      nameserves: vec![],
      ipv4_addrs: vec![],
      ipv6_addrs: vec![],
      domains: vec![],
      ttl: 3600,
      mx: "mail.localhost".to_string(),
    }
  }

  // * resolve ips
  fn resolve_ipv4(value: &str) -> Ipv4Addr {
    match value.trim() {
      "$LOCALHOST" => Ipv4Addr::LOCALHOST,
      other => other.parse().unwrap_or(Ipv4Addr::UNSPECIFIED),
    }
  }

  fn resolve_ipv6(value: &str) -> Ipv6Addr {
    match value.trim() {
      "&LOCALHOST" => Ipv6Addr::LOCALHOST,
      other => other.parse().unwrap_or(Ipv6Addr::UNSPECIFIED),
    }
  }

  // * load from zone file
  pub async fn from_file<P: AsRef<Path>>(path: P) -> Self {
    let content: String = read_to_string(path).await.expect("Failed to read zone file");
    let mut zone: DnsZone = DnsZone::new();

    for line in content.lines() {
      let line: &str = line.trim();
      if line.is_empty() || line.starts_with('#') {
        continue;
      }

      if let Some(rest) = line.strip_prefix("@") {
        let parts: Vec<&str> = rest.splitn(2, ':').collect();
        if parts.len() < 2 {
          continue;
        }

        let key: String = parts[0].trim().to_uppercase();
        let value: &str = parts[1].trim();

        match key.as_str() {
          "ZONE" => value
            .split(',')
            .map(|v| v.trim().to_string())
            .for_each(|domain| zone.domains.push(domain)),
          "TTL" => zone.ttl = value.parse().unwrap_or(3600),
          "NS" => value.split(',').map(|v| v.trim().to_string()).for_each(|ns| {
            let ip: Ipv4Addr = Self::resolve_ipv4(&ns);
            zone.nameserves.push(ip);
          }),
          "MX" => zone.mx = value.trim().to_string(),
          "AAAA" => value.split(',').for_each(|domain| {
            let ip: Ipv6Addr = Self::resolve_ipv6(domain);
            zone.ipv6_addrs.push(ip);
          }),
          "A" => value.split(',').for_each(|domain| {
            let ip: Ipv4Addr = Self::resolve_ipv4(domain);
            zone.ipv4_addrs.push(ip);
          }),
          _ => {},
        }
      }
    }
    zone
  }

  // * load zones from folder
  pub async fn scan_dir(path: &Path, zones: &mut Vec<DnsZone>) {
    let mut entries = match tokio::fs::read_dir(path).await {
      Ok(e) => e,
      Err(e) => {
        eprintln!("Failed to read dir {:?}: {}", path, e);
        return;
      },
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
      let path = entry.path();

      if path.is_dir() {
        let fut: Pin<Box<dyn Future<Output = ()>>> =
          Box::pin(Self::scan_dir(&path, zones));
        fut.await;
      } else if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        if file_name.starts_with("zone.") {
          let zone = Self::from_file(&path).await;
          zones.push(zone);
        }
      }
    }
  }

  pub async fn from_dir<P: AsRef<Path>>(path: P) -> Vec<DnsZone> {
    let mut zones: Vec<DnsZone> = vec![];
    let _ = Self::scan_dir(path.as_ref(), &mut zones).await;
    zones
  }
}

// === SERVER CONFIG ===

#[derive(Debug, Clone)]
pub struct ServerConfig {
  pub nameservers: Vec<Ipv4Addr>, // lookup nameservers
  pub zones: Vec<DnsZone>,        // custom dns zones
  pub max_messages: usize,
  pub max_workers: usize,
  pub debug: bool,
  pub port: u16,
}

impl ServerConfig {
  pub fn new(
    nameservers: Vec<Ipv4Addr>,
    max_messages: usize,
    max_workers: usize,
  ) -> Self {
    Self {
      nameservers: nameservers,
      zones: vec![],
      max_messages: max_messages.max(DEFAULT_MAX_MESSAGES),
      max_workers: max_workers.max(DEFAULT_MAX_WORKERS),
      debug: false,
      port: 53,
    }
  }
}

// === DNS SERVER ===

#[derive(Debug)]
pub struct DnsServer {
  pub config: ServerConfig,

  lookup_client: UdpSocket,
  worker_tx: Sender<WorkerTask>,
  socket: UdpSocket,
}

impl DnsServer {
  // * create sockets
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
    let config: Arc<ServerConfig> = Arc::new(self.config.clone());

    startup_banner(local_ip().map_err(|e| Error::new(ErrorKind::Other, e))?, self.config);

    const BUFFER_SIZE: usize = 1280;
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

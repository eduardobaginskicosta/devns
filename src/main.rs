use dns_core::{DnsServer, DnsZone, ServerConfig, actions::ROOT_SERVERS};
use std::{
  env::{current_dir, var},
  io::Error,
  net::Ipv4Addr,
  path::PathBuf,
};

// === ENV RUNTIME ===

struct EnvRuntime {
  pub nameservers: Vec<Ipv4Addr>,
  pub max_messages: usize,
  pub max_workers: usize,
  pub debug: bool,
  pub port: u16,
}

impl EnvRuntime {
  // * safe generic parsers
  fn parse_env<T, F>(key: &str, default: T, parser: F) -> T
  where
    F: FnOnce(String) -> Option<T>,
  {
    var(key).ok().and_then(parser).unwrap_or(default)
  }

  // * safe parsers
  fn parse_bool(key: &str, default: bool) -> bool {
    Self::parse_env(key, default, |v| match v.as_str() {
      "1" | "true" | "TRUE" | "True" => Some(true),
      "0" | "false" | "FALSE" | "False" => Some(false),
      _ => Some(true),
    })
  }

  fn parse_usize(key: &str, default: usize) -> usize {
    Self::parse_env(key, default, |v| v.parse().ok())
  }

  fn parse_u16(key: &str, default: u16) -> u16 {
    Self::parse_env(key, default, |v| v.parse().ok())
  }

  fn parse_dns_servers(default: Vec<Ipv4Addr>) -> Vec<Ipv4Addr> {
    std::env::var("DNS_SERVERS")
      .ok()
      .map(|v| {
        v.split(';').map(str::trim).filter_map(|s| s.parse::<Ipv4Addr>().ok()).collect()
      })
      .unwrap_or_else(|| default)
  }

  // * safe clamps
  fn clamp_usize(v: usize, min: usize, max: usize) -> usize {
    v.max(min).min(max)
  }

  fn clamp_u16(v: u16, min: u16, max: u16) -> u16 {
    v.max(min).min(max)
  }

  // * builder
  pub fn from_env() -> Self {
    Self {
      nameservers: Self::parse_dns_servers(ROOT_SERVERS.to_vec()),
      max_messages: Self::clamp_usize(Self::parse_usize("MAX_MESSAGES", 20), 1, 10_000),
      max_workers: Self::clamp_usize(Self::parse_usize("MAX_WORKERS", 10), 1, 256),
      debug: Self::parse_bool("DEBUG_MODE", false),
      port: Self::clamp_u16(Self::parse_u16("PORT", 53), 53, 9000),
    }
  }
}

// === GET CONFIG PATH ===

fn get_config_path() -> PathBuf {
  let cwd = current_dir().expect("Failed to get current working directory");
  cwd.join("config")
}

// === APP MAIN ===

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Error> {
  let env_runtime: EnvRuntime = EnvRuntime::from_env();
  let config: ServerConfig = ServerConfig {
    nameservers: env_runtime.nameservers,
    max_messages: env_runtime.max_messages,
    max_workers: env_runtime.max_workers,
    debug: env_runtime.debug,
    port: env_runtime.port,
    zones: DnsZone::from_dir(get_config_path()).await,
  };

  let server: DnsServer = DnsServer::new(config).await?;
  server.start().await
}

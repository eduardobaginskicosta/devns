use dns_core::{DnsServer, DnsZone, ServerConfig};
use std::{env::current_dir, io::Error, path::PathBuf};

fn get_config_path() -> PathBuf {
  let cwd = current_dir().expect("Failed to get current working directory");
  cwd.join("config")
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Error> {
  let mut config: ServerConfig = ServerConfig::new(vec![], 20, 10);
  config.zones = DnsZone::from_dir(get_config_path()).await;
  let server: DnsServer = DnsServer::new(config).await?;
  server.start().await
}

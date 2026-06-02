use core::{DnsServer, structs::ServerConfig};
use std::io::Error;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Error> {
  let mut config: ServerConfig = ServerConfig::new(vec![], 20, 10);
  let mut server: DnsServer = DnsServer::new(config).await?;
}

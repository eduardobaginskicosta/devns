use crate::structs::{LookupRecord, LookupRecordError};
use std::net::{Ipv4Addr, Ipv6Addr};

// === CONSTANTS ===

const DEFAULT_MAX_MESSAGES: usize = 20;
const DEFAULT_MAX_WORKERS: usize = 10;

// === SERVER CONFIG ===

#[derive(Debug, Clone)]
pub struct ServerConfig {
  pub nameservers: Vec<Ipv4Addr>, // lookup nameservers
  pub lookup: Vec<LookupRecord>,  // custom lookup records
  pub max_messages: usize,
  pub max_workers: usize,
  pub debug: bool,
  pub port: u16,
}

impl Default for ServerConfig {
  fn default() -> Self {
    Self {
      nameservers: vec![],
      lookup: vec![],
      max_messages: DEFAULT_MAX_MESSAGES,
      max_workers: DEFAULT_MAX_WORKERS,
      debug: false,
      port: 53,
    }
  }
}

impl ServerConfig {
  pub fn new(
    nameservers: Vec<Ipv4Addr>,
    max_messages: usize,
    max_workers: usize,
  ) -> Self {
    Self {
      nameservers: nameservers,
      lookup: Self::default().lookup,
      max_messages: max_messages.max(1),
      max_workers: max_workers.max(1),
      debug: Self::default().debug,
      port: Self::default().port,
    }
  }

  // * builder
  fn add_record(
    &mut self,
    domains: Vec<String>,
    ipv4: Vec<Ipv4Addr>,
    ipv6: Vec<Ipv6Addr>,
  ) -> Result<&mut Self, LookupRecordError> {
    if !domains.is_empty() {
      let record: LookupRecord = LookupRecord::new(domains, ipv4, ipv6)?;
      self.lookup.push(record);
    }
    Ok(self)
  }

  pub fn look_at(
    &mut self,
    domain: String,
    ipv4: Vec<Ipv4Addr>,
    ipv6: Vec<Ipv6Addr>,
  ) -> &mut Self {
    self.add_record(vec![domain], ipv4, ipv6)?;
  }
}

use std::{
  fmt::Display,
  net::{Ipv4Addr, Ipv6Addr},
};

// === LOOKUP RECORD ERROR ===

#[derive(Debug)]
pub enum LookupRecordError {
  EmptyAddresses,
  EmptyDomains,
}

impl std::error::Error for LookupRecordError {}
impl Display for LookupRecordError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::EmptyAddresses => write!(f, "at least one IPV4 or IPV6 address is required"),
      Self::EmptyDomains => write!(f, "domains list cannot be empty"),
    }
  }
}

// === LOOKUP RECORD ===

#[derive(Debug, Clone)]
pub struct LookupRecord {
  ipv6_addrs: Vec<Ipv6Addr>,
  ipv4_addrs: Vec<Ipv4Addr>,
  domains: Vec<String>,
}

impl LookupRecord {
  pub fn new(
    domains: Vec<String>,
    ipv4: Vec<Ipv4Addr>,
    ipv6: Vec<Ipv6Addr>,
  ) -> Result<Self, LookupRecordError> {
    if domains.is_empty() {
      return Err(LookupRecordError::EmptyDomains);
    }

    if ipv4.is_empty() && ipv6.is_empty() {
      return Err(LookupRecordError::EmptyAddresses);
    }

    Ok(Self { ipv6_addrs: ipv6, ipv4_addrs: ipv4, domains })
  }

  pub fn domains(&self) -> &[String] {
    &self.domains
  }
  pub fn ipv4(&self) -> &[Ipv4Addr] {
    &self.ipv4_addrs
  }
  pub fn ipv6(&self) -> &[Ipv6Addr] {
    &self.ipv6_addrs
  }
}

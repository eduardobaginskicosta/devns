use crate::{
  enums::{BytePacketError, QueryType},
  records::*,
  structs::BytePacketBuffer,
};

use std::{
  fmt::Display,
  net::{Ipv4Addr, Ipv6Addr},
};

#[derive(PartialEq, Eq, Debug, Clone, Hash, PartialOrd, Ord)]
pub enum DnsRecord {
  Unknown { data_len: u16, domain: String, qtype: u16, ttl: u32 },

  AAAA { address: Ipv6Addr, domain: String, ttl: u32 },
  A { address: Ipv4Addr, domain: String, ttl: u32 },

  MX { priority: u16, domain: String, host: String, ttl: u32 },
  NS { domain: String, host: String, ttl: u32 },

  CNAME { domain: String, host: String, ttl: u32 },
}

impl DnsRecord {
  // * safe read
  pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord, BytePacketError> {
    let mut domain: String = String::new();
    buffer.read_qname(&mut domain)?;

    let qtype_num: u16 = buffer.read_u16()?;
    let qtype: QueryType = QueryType::from(qtype_num);
    buffer.read_u16()?;

    let ttl: u32 = buffer.read_u32()?;
    let data_len: u16 = buffer.read_u16()?;

    match qtype {
      QueryType::UNKNOWN(_) => {
        buffer.step(data_len as usize)?;
        Ok(DnsRecord::Unknown { data_len, domain, qtype: qtype_num, ttl })
      },
      QueryType::AAAA => make_aaaa(buffer, domain, ttl),
      QueryType::A => make_a(buffer, domain, ttl),
      QueryType::MX | QueryType::NS | QueryType::CNAME => {
        make_mnc(buffer, qtype, domain, ttl)
      },
    }
  }

  // * safe write
  pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<usize, BytePacketError> {
    let start_pos: usize = buffer.pos();
    match self {
      Self::A { address, domain, ttl } => write_a(buffer, address, domain, *ttl)?,
      Self::AAAA { address, domain, ttl } => write_aaaa(buffer, address, domain, *ttl)?,
      Self::MX { priority, domain, host, ttl } => {
        write_mx(buffer, *priority, domain, host, *ttl)?
      },
      Self::NS { domain, host, ttl } => {
        write_cname_ns(buffer, domain, host, QueryType::NS, *ttl)?
      },
      Self::CNAME { domain, host, ttl } => {
        write_cname_ns(buffer, domain, host, QueryType::CNAME, *ttl)?
      },
      Self::Unknown { .. } => return Err(BytePacketError::UnknownRecordError),
    }
    Ok(buffer.pos() - start_pos)
  }
}

impl Display for DnsRecord {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::A { address, domain, ttl } => {
        write!(f, "A: {domain} | {address} | ttl={ttl}")
      },
      Self::AAAA { address, domain, ttl } => {
        write!(f, "AAAA: {domain} | {address} | ttl={ttl}")
      },
      Self::MX { priority, domain, host, ttl } => {
        write!(f, "MX: {domain} | {host} | priority={priority} | ttl={ttl}")
      },
      Self::NS { domain, host, ttl } => write!(f, "NS: {domain} | {host} | ttl={ttl}"),
      Self::CNAME { domain, host, ttl } => {
        write!(f, "CNAME: {domain} | {host} | ttl={ttl}")
      },
      Self::Unknown { domain, qtype, ttl, .. } => {
        write!(f, "UNKNOWN: {domain} | qtype={qtype} | ttl={ttl} ")
      },
    }
  }
}

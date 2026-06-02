use crate::{
  enums::{BytePacketError, DnsRecord, QueryType},
  structs::BytePacketBuffer,
};

use std::net::{Ipv4Addr, Ipv6Addr};

// === MAKE A RECORD ===

pub fn make_a(
  buffer: &mut BytePacketBuffer,
  domain: String,
  ttl: u32,
) -> Result<DnsRecord, BytePacketError> {
  Ok(DnsRecord::A { address: Ipv4Addr::from(buffer.read_u32()?), domain, ttl })
}

// === MAKE AAAA RECORD ===

pub fn make_aaaa(
  buffer: &mut BytePacketBuffer,
  domain: String,
  ttl: u32,
) -> Result<DnsRecord, BytePacketError> {
  let [a, b, c, d] = std::array::from_fn(|_| buffer.read_u32());
  let [a, b, c, d] = [a?, b?, c?, d?];

  Ok(DnsRecord::AAAA {
    address: Ipv6Addr::new(
      (a >> 0x10) as u16,
      (a & 0xFFFF) as u16,
      (b >> 0x10) as u16,
      (b & 0xFFFF) as u16,
      (c >> 0x10) as u16,
      (c & 0xFFFF) as u16,
      (d >> 0x10) as u16,
      (d & 0xFFFF) as u16,
    ),
    domain,
    ttl,
  })
}

// === MAKE MX, NS & CNAME RECORD ===

pub fn make_mnc(
  buffer: &mut BytePacketBuffer,
  qtype: QueryType,
  domain: String,
  ttl: u32,
) -> Result<DnsRecord, BytePacketError> {
  let read_host = |buf: &mut BytePacketBuffer| -> Result<String, BytePacketError> {
    let mut host: String = String::new();
    buf.read_qname(&mut host)?;
    Ok(host)
  };

  match qtype {
    QueryType::MX => {
      let priority: u16 = buffer.read_u16()?;
      Ok(DnsRecord::MX { priority: priority, domain, host: read_host(buffer)?, ttl })
    },
    QueryType::NS => Ok(DnsRecord::NS { domain, host: read_host(buffer)?, ttl }),
    QueryType::CNAME => Ok(DnsRecord::CNAME { domain, host: read_host(buffer)?, ttl }),
    _ => Err(BytePacketError::InvalidQueryType(u16::from(qtype))),
  }
}

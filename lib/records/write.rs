use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{
  enums::{BytePacketError, QueryType},
  structs::BytePacketBuffer,
};

// === HELPER - WRITE HEADER ===

fn write_header(
  buffer: &mut BytePacketBuffer,
  domain: &str,
  qtype: QueryType,
  ttl: u32,
) -> Result<(), BytePacketError> {
  buffer.write_qname(domain)?;
  buffer.write_u16(u16::from(qtype))?;
  buffer.write_u16(0x1)?;
  buffer.write_u32(ttl)
}

// === HELPER - WRITE RAW DATA ===

fn write_rdata<F>(
  buffer: &mut BytePacketBuffer,
  write_fn: F,
) -> Result<(), BytePacketError>
where
  F: FnOnce(&mut BytePacketBuffer) -> Result<(), BytePacketError>,
{
  let pos: usize = buffer.pos();
  buffer.write_u16(0x0)?;
  write_fn(buffer)?;

  let size: usize = buffer.pos() - (pos + 2);
  buffer.set_u16(pos, size as u16)
}

// === WRITE A RECORD ===

pub fn write_a(
  buffer: &mut BytePacketBuffer,
  addr: &Ipv4Addr,
  domain: &str,
  ttl: u32,
) -> Result<(), BytePacketError> {
  write_header(buffer, domain, QueryType::A, ttl)?;
  buffer.write_u16(0x4)?;
  addr.octets().iter().try_for_each(|octet| buffer.write(*octet))
}

// === WRITE AAAA RECORD ===

pub fn write_aaaa(
  buffer: &mut BytePacketBuffer,
  addr: &Ipv6Addr,
  domain: &str,
  ttl: u32,
) -> Result<(), BytePacketError> {
  write_header(buffer, domain, QueryType::AAAA, ttl)?;
  buffer.write_u16(0x10)?;
  addr.segments().iter().try_for_each(|segment| buffer.write_u16(*segment))
}

// === WRITE MX RECORD ===

pub fn write_mx(
  buffer: &mut BytePacketBuffer,
  priority: u16,
  domain: &str,
  host: &str,
  ttl: u32,
) -> Result<(), BytePacketError> {
  write_header(buffer, domain, QueryType::MX, ttl)?;
  write_rdata(buffer, |buf| {
    buf.write_u16(priority)?;
    buf.write_qname(host)
  })
}

// === WRITE CNAME & NS RECORD ===

pub fn write_cname_ns(
  buffer: &mut BytePacketBuffer,
  domain: &str,
  host: &str,
  qtype: QueryType,
  ttl: u32,
) -> Result<(), BytePacketError> {
  write_header(buffer, domain, qtype, ttl)?;
  write_rdata(buffer, |buf| buf.write_qname(host))
}

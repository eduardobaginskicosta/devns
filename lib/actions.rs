use crate::{
  DnsZone,
  enums::{BytePacketError, DnsRecord, QueryType, ResultCode},
  server::ServerConfig,
  structs::{BytePacketBuffer, DnsHeader, DnsPacket, DnsQuestion},
  utils::{handle_lookup, send_response},
};

use std::{
  future::Future,
  net::{IpAddr, Ipv4Addr, SocketAddr},
  pin::Pin,
};
use tokio::net::UdpSocket;

// === CONSTANTS ===

const DOOM_ID: u16 = 0x29A; // yes, it's a Doom game reference
pub const ROOT_SERVERS: &[Ipv4Addr] = &[
  Ipv4Addr::new(198, 41, 0, 4), // a.root-servers.net
  Ipv4Addr::new(1, 1, 1, 1),    // one.one.one.one (cloudflare)
  Ipv4Addr::new(1, 0, 0, 1),    // one.one.one.one (cloudflare)
];

// === BUILD DNS PACKET ===

pub fn build_dns_packet(
  zone: &DnsZone,
  domain: String,
) -> Result<DnsPacket, BytePacketError> {
  let ttl: u32 = zone.ttl.min(0xE10); // 1 hour
  let mut packet: DnsPacket = DnsPacket::default();

  packet.questions.push(DnsQuestion::new(domain.clone(), QueryType::A));
  packet.authorities.push(DnsRecord::NS {
    domain: domain.clone(),
    host: format!("ns1.{domain}"),
    ttl: ttl,
  });
  packet.resources.push(DnsRecord::MX {
    priority: 10,
    domain: domain.clone(),
    host: format!("mail.{domain}"),
    ttl: ttl,
  });

  for &address in &zone.ipv4_addrs {
    packet.answers.push(DnsRecord::A { domain: domain.clone(), address, ttl: ttl });
  }
  for &address in &zone.ipv6_addrs {
    packet.answers.push(DnsRecord::AAAA { domain: domain.clone(), address, ttl: ttl });
  }

  packet.header.questions = packet.questions.len() as u16;
  packet.header.answers = packet.answers.len() as u16;
  packet.header.authoritative_entries = packet.authorities.len() as u16;
  packet.header.resource_entries = packet.resources.len() as u16;

  packet.header.id = DOOM_ID;
  packet.header.response = true;
  packet.header.recursion_available = true;
  packet.header.rescode = ResultCode::NoError;

  Ok(packet)
}

// === LOOKUP ===

pub async fn lookup(
  socket: &UdpSocket,
  qname: &str,
  qtype: QueryType,
  server: SocketAddr,
) -> Result<DnsPacket, BytePacketError> {
  let mut packet: DnsPacket = DnsPacket::default();
  packet.header.id = DOOM_ID;
  packet.header.questions = 0x1;
  packet.header.recursion_desired = true;
  packet.questions.push(DnsQuestion::new(qname.to_string(), qtype));

  let mut req_buffer: BytePacketBuffer = BytePacketBuffer::new();
  packet.write(&mut req_buffer)?;
  socket.send_to(&req_buffer.buffer[..req_buffer.position], &server).await?;

  let mut res_buffer: BytePacketBuffer = BytePacketBuffer::new();
  socket.recv_from(&mut res_buffer.buffer).await?;

  DnsPacket::try_from(&mut res_buffer)
}

// === RECURSIVE LOOKUP ===

pub fn recursive_lookup<'a>(
  socket: &'a UdpSocket,
  mut servers: Vec<Ipv4Addr>,
  qname: &'a str,
  qtype: QueryType,
) -> Pin<Box<dyn Future<Output = Result<DnsPacket, BytePacketError>> + Send + 'a>> {
  Box::pin(async move {
    while let Some(current_ns) = servers.pop() {
      let mut ns_ip: Ipv4Addr = current_ns;

      loop {
        let server = SocketAddr::new(IpAddr::V4(ns_ip), 53);
        let response = lookup(socket, qname, qtype, server).await?;

        let resolved =
          !response.answers.is_empty() && response.header.rescode == ResultCode::NoError;
        let nxdomain = response.header.rescode == ResultCode::NxDomain;

        if resolved || nxdomain {
          return Ok(response);
        }

        if let Some(new_ns_ip) = response.get_resolved_ns(qname) {
          ns_ip = new_ns_ip;
          continue;
        }

        let Some(new_ns_name) = response.get_unresolved_ns(qname) else {
          return Ok(response);
        };

        let recursive =
          recursive_lookup(socket, ROOT_SERVERS.to_vec(), new_ns_name, QueryType::A)
            .await?;

        match recursive.get_random_a() {
          Some(new_ip) => ns_ip = new_ip,
          None => return Ok(response),
        }
      }
    }

    Err(BytePacketError::LookupFailed)
  })
}

// === HANDLE QUERY ===

pub async fn handle_query(
  config: &ServerConfig,
  client_socket: &UdpSocket,
  socket: &UdpSocket,
  buffer: Vec<u8>,
  source: SocketAddr,
  debug: bool,
) -> Result<(), BytePacketError> {
  let mut req_buffer: BytePacketBuffer = BytePacketBuffer::new();
  req_buffer.buffer[..buffer.len()].copy_from_slice(&buffer);

  let request: DnsPacket = DnsPacket::try_from(&mut req_buffer)?;
  let mut response: DnsPacket = DnsPacket::new();

  response.header.id = request.header.id;
  response.header.recursion_desired = true;
  response.header.recursion_available = true;
  response.header.response = true;

  let Some(question) = request.questions.first() else {
    response.header.rescode = ResultCode::FormError;
    return send_response(socket, &mut response, source).await;
  };

  if let Some(mut result) = handle_lookup(&config, question, &mut response, debug) {
    return send_response(socket, &mut result, source).await;
  };

  match recursive_lookup(
    client_socket,
    if config.nameservers.is_empty() {
      ROOT_SERVERS.to_vec()
    } else {
      config.nameservers.clone()
    },
    &question.name,
    question.qtype,
  )
  .await
  {
    Ok(mut result) => {
      response.header = DnsHeader {
        rescode: result.header.rescode,
        questions: result.questions.len() as u16,
        answers: result.answers.len() as u16,
        authoritative_entries: response.authorities.len() as u16,
        resource_entries: response.resources.len() as u16,
        ..response.header
      };

      response.questions.push(question.clone());
      response.answers.append(&mut result.answers);
      response.authorities.append(&mut result.authorities);
      response.resources.append(&mut result.resources);
    },
    Err(_) => response.header.rescode = ResultCode::ServerFail,
  }
  send_response(socket, &mut response, source).await
}

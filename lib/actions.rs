use crate::{
  enums::{BytePacketError, DnsRecord, QueryType, ResultCode},
  structs::{BytePacketBuffer, DnsPacket, DnsQuestion, LookupRecord, ServerConfig},
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
const ROOT_SERVERS: &[Ipv4Addr] = &[
  Ipv4Addr::new(198, 41, 0, 4), // a.root-servers.net
  Ipv4Addr::new(1, 1, 1, 1),    // one.one.one.one (cloudflare)
  Ipv4Addr::new(1, 0, 0, 1),    // one.one.one.one (cloudflare)
];

// === BUILD DNS PACKET ===

pub fn build_dns_packet(record: &LookupRecord) -> Result<DnsPacket, BytePacketError> {
  const DEFAULT_TTL: u32 = 0xE10; // 1 hour

  let has_ipv4: bool = !record.ipv4().is_empty();
  let has_ipv6: bool = !record.ipv6().is_empty();

  let num_domains: usize = record.domains().len();
  let num_questions: usize = (has_ipv4 as usize + has_ipv6 as usize) * num_domains;
  let num_answers: usize = num_domains * (record.ipv4().len() + record.ipv6().len());

  let mut packet: DnsPacket = DnsPacket::default();
  packet.header().set_id(DOOM_ID); // doom reference?
  packet.header().set_recursion_desired(true);
  packet.header().set_recursion_available(true);
  packet.header().set_authoritative_answer(true);
  packet.header().set_authed_data(true);
  packet.header().set_response(true);
  packet.header().set_rescode(ResultCode::NoError);
  packet.header().set_questions(num_questions as u16);
  packet.header().set_answers(num_answers as u16);
  packet.header().set_authoritative_entries(num_domains as u16);
  packet.header().set_resource_entries(num_domains as u16);

  for domain in record.domains() {
    if has_ipv4 {
      packet.questions().push(DnsQuestion::new(domain.clone(), QueryType::A));
    }
    if has_ipv6 {
      packet.questions().push(DnsQuestion::new(domain.clone(), QueryType::AAAA));
    }

    packet.answers().extend(record.ipv4().iter().map(|&address| DnsRecord::A {
      address,
      domain: domain.clone(),
      ttl: DEFAULT_TTL,
    }));
    packet.answers().extend(record.ipv6().iter().map(|&address| DnsRecord::AAAA {
      address,
      domain: domain.clone(),
      ttl: DEFAULT_TTL,
    }));

    packet.authorities().push(DnsRecord::NS {
      domain: domain.clone(),
      host: format!("ns1.{domain}"),
      ttl: DEFAULT_TTL,
    });
    packet.resources().push(DnsRecord::MX {
      domain: domain.clone(),
      host: format!("mail.{domain}"),
      priority: 10,
      ttl: DEFAULT_TTL,
    });
  }
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
  packet.header().set_id(DOOM_ID);
  packet.header().set_questions(0x1);
  packet.header().set_recursion_desired(true);
  packet.questions().push(DnsQuestion::new(qname.to_string(), qtype));

  let mut req_buffer: BytePacketBuffer = BytePacketBuffer::new();
  packet.write(&mut req_buffer)?;

  let buffer_pos: usize = req_buffer.pos();
  socket.send_to(&req_buffer.buffer()[..buffer_pos], &server).await?;

  let mut res_buffer: BytePacketBuffer = BytePacketBuffer::new();
  socket.recv_from(res_buffer.buffer()).await?;

  DnsPacket::try_from(&mut res_buffer)
}

// === RECURSIVE LOOKUP ===

pub fn recursive_lookup<'a>(
  socket: &'a UdpSocket,
  servers: Vec<Ipv4Addr>,
  qname: &'a str,
  qtype: QueryType,
) -> Pin<Box<dyn Future<Output = Result<DnsPacket, BytePacketError>> + Send + 'a>> {
  Box::pin(async move {
    let mut servers = servers;

    while let Some(current_ns) = servers.pop() {
      let mut ns_ip = current_ns;

      loop {
        let server = SocketAddr::new(IpAddr::V4(ns_ip), 53);
        let mut response = lookup(socket, qname, qtype, server).await?;

        let resolved = !response.answers().is_empty()
          && *response.header().rescode() == ResultCode::NoError;
        let nxdomain = *response.header().rescode() == ResultCode::NxDomain;

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
  req_buffer.buffer()[..buffer.len()].copy_from_slice(&buffer);

  let mut request: DnsPacket = DnsPacket::try_from(&mut req_buffer)?;
  let mut response: DnsPacket = DnsPacket::new();

  response.header().set_id(*request.header().id());
  response.header().set_recursion_desired(true);
  response.header().set_recursion_available(true);
  response.header().set_response(true);

  let Some(question) = request.questions().first_mut() else {
    response.header().set_rescode(ResultCode::FormError);
    return send_response(socket, &mut response, source).await;
  };

  let qname: &str = &question.name().to_string();
  let qtype: QueryType = *question.qtype();
  let mut _question: DnsQuestion = question.clone();

  if let Some(_) = handle_lookup(config, &mut _question, &mut response, debug) {
    return send_response(socket, &mut response, source).await;
  };

  match recursive_lookup(client_socket, config.nameservers.clone(), qname, qtype).await {
    Ok(mut result) => {
      response.questions().push(question.clone());
      response.header().set_rescode(*result.header().rescode());
      response.answers().append(&mut result.answers());
      response.authorities().append(&mut result.authorities());
      response.resources().append(&mut result.resources());
    },
    Err(_) => response.header().set_rescode(ResultCode::ServerFail),
  }

  send_response(socket, &mut response, source).await
}

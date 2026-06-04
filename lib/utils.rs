use crate::{
  actions::build_dns_packet,
  enums::{BytePacketError, ResultCode},
  log_debug,
  server::ServerConfig,
  structs::{BytePacketBuffer, DnsPacket, DnsQuestion},
};

use std::{
  fmt::Display,
  net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::net::UdpSocket;

// === SEND RESPONSE ===

pub async fn send_response(
  socket: &UdpSocket,
  response: &mut DnsPacket,
  source: SocketAddr,
) -> Result<(), BytePacketError> {
  let mut res_buffer: BytePacketBuffer = BytePacketBuffer::new();
  response.write(&mut res_buffer)?;

  let data: &[u8] = res_buffer.get_range(0, res_buffer.pos())?;
  socket.send_to(data, source).await?;
  Ok(())
}

// === HANDLE LOOKUP ===

pub fn handle_lookup(
  config: &ServerConfig,
  question: &DnsQuestion,
  response: &mut DnsPacket,
  debug: bool,
) -> Option<DnsPacket> {
  let req_domain: String = question.name.to_lowercase();
  for zone in &config.zones {
    let matches: bool = zone.domains.iter().any(|d| {
      let d = d.trim_end_matches('.').to_lowercase();
      req_domain == d
        || req_domain.strip_suffix(&d).map(|s| s.ends_with('.')).unwrap_or(false)
    });

    if !matches {
      continue;
    }

    let is_blocked: bool = !zone.ipv4_addrs.iter().any(|ip| ip.octets() != [0; 4])
      && !zone.ipv6_addrs.iter().any(|ip| ip.segments() != [0; 8]);

    if is_blocked {
      if debug {
        log_debug!("Blocked request: {} (Domain Blocking System)", question.name);
      }
      response.header.rescode = ResultCode::Refused;
      return Some(response.clone());
    }

    if let Ok(mut result) = build_dns_packet(zone, req_domain.clone()) {
      result.questions = vec![question.clone()];
      result.header.id = response.header.id;
      result.header.recursion_desired = response.header.recursion_desired;
      result.header.recursion_available = response.header.recursion_available;
      result.header.response = response.header.response;
      return Some(result);
    }
  }
  None
}

// === STARTUP BANNER ===

pub fn startup_banner(ip: IpAddr, config: ServerConfig) {
  let format_ipv4_list = |ips: &[Ipv4Addr]| -> String {
    let mut out = String::new();
    for (i, ip) in ips.iter().enumerate() {
      if i > 0 {
        out.push_str(", ");
      }
      out.push_str(&ip.to_string());
    }
    out
  };

  let line = |label: &str, value: &dyn Display| {
    println!("# {:<16}: {}", label, value);
  };

  const TITLE: &str = "DevNS (Development Name Server)";
  const AUTHOR: &str = "Eduardo Baginski Costa <eduardobcosta1234@gmail.com>";
  const LICENSE: &str = "BSD-3-Clause";
  const DONATE: &str = "https://ko-fi.com/baginskistudio";
  const REPO: &str = "https://github.com/eduardobaginskicosta/devns";

  line("dns.title", &TITLE);
  line("dns.author", &AUTHOR);
  line("dns.license", &LICENSE);
  line("dns.donate", &DONATE);
  line("dns.repo", &REPO);
  line("dns.max.messages", &config.max_messages);
  line("dns.max.workers", &config.max_workers);
  line("dns.nameservers", &format_ipv4_list(&config.nameservers));
  line("dns.debug", &config.debug);
  line("dns.bind", &ip);
  line("dns.port", &config.port);
  line("zones.amount", &config.zones.len());
}

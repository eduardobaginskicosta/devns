use crate::{
  actions::build_dns_packet,
  enums::{BytePacketError, ResultCode},
  log_debug,
  structs::{BytePacketBuffer, DnsPacket, DnsQuestion, ServerConfig},
};

use std::net::SocketAddr;
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
  socket.send_to(data, &source).await?;
  Ok(())
}

// === HANDLE LOOKUP ===

pub fn handle_lookup(
  config: &ServerConfig,
  question: &mut DnsQuestion,
  response: &mut DnsPacket,
  debug: bool,
) -> Option<DnsPacket> {
  let req_domain: String = question.name().to_lowercase();

  for record in &config.lookup {
    let matches: bool = record
      .domains()
      .iter()
      .map(|d| d.trim_end_matches('.').to_lowercase())
      .any(|d| req_domain.ends_with(&format!(".{d}")));

    if !matches {
      continue;
    }

    let is_blocked: bool = !record.ipv4().iter().any(|ip| ip.octets() != [0; 4])
      && !record.ipv6().iter().any(|ip| ip.segments() != [0; 8]);

    if is_blocked {
      if debug {
        log_debug!("Blocked request: {} (Domain Blocking System)", question.name());
      }
      response.header().set_rescode(ResultCode::Refused);
      return Some(response.clone());
    }

    if let Ok(mut result) = build_dns_packet(record) {
      result.questions().push(question.clone());
      result.header().set_id(*response.header().id());
      result.header().set_recursion_desired(*response.header().recursion_desired());
      result.header().set_recursion_available(*response.header().recursion_available());
      result.header().set_response(*response.header().response());
      return Some(result);
    }
  }
  None
}

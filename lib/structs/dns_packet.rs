use std::net::Ipv4Addr;

use crate::{
  enums::{BytePacketError, DnsRecord},
  structs::{BytePacketBuffer, DnsHeader, DnsQuestion},
};

#[derive(Debug, Clone, Default)]
pub struct DnsPacket {
  pub header: DnsHeader,
  pub questions: Vec<DnsQuestion>,
  pub answers: Vec<DnsRecord>,
  pub authorities: Vec<DnsRecord>,
  pub resources: Vec<DnsRecord>,
}

impl DnsPacket {
  pub fn new() -> Self {
    Self::default()
  }

  // * safe write
  pub fn write(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), BytePacketError> {
    self.header.questions = self.questions.len() as u16;
    self.header.answers = self.answers.len() as u16;
    self.header.authoritative_entries = self.authorities.len() as u16;
    self.header.resource_entries = self.resources.len() as u16;

    self.header.write(buffer)?;

    self.questions.iter_mut().try_for_each(|q| q.write(buffer))?;
    self.answers.iter_mut().try_for_each(|r| r.write(buffer).map(|_| ()))?;
    self.authorities.iter_mut().try_for_each(|r| r.write(buffer).map(|_| ()))?;
    self.resources.iter_mut().try_for_each(|r| r.write(buffer).map(|_| ()))?;
    Ok(())
  }

  // * queries
  pub fn get_random_a(&self) -> Option<Ipv4Addr> {
    self.answers.iter().find_map(|record| match record {
      DnsRecord::A { address, .. } => Some(*address),
      _ => None,
    })
  }

  pub fn get_ns<'a>(
    &'a self,
    qname: &'a str,
  ) -> impl Iterator<Item = (&'a str, &'a str)> {
    self.authorities.iter().filter_map(move |record| match record {
      DnsRecord::NS { domain, host, .. } if qname.ends_with(domain.as_str()) => {
        Some((domain.as_str(), host.as_str()))
      },
      _ => None,
    })
  }

  pub fn get_resolved_ns(&self, qname: &str) -> Option<Ipv4Addr> {
    self.get_ns(qname).find_map(|(_, host)| {
      self
        .resources
        .iter()
        .filter_map(|record| match record {
          DnsRecord::A { address, domain, .. } if domain == host => Some(*address),
          _ => None,
        })
        .next()
    })
  }

  pub fn get_unresolved_ns<'a>(&'a self, qname: &'a str) -> Option<&'a str> {
    self.get_ns(qname).map(|(_, host)| host).next()
  }
}

impl TryFrom<&mut BytePacketBuffer> for DnsPacket {
  type Error = BytePacketError;
  fn try_from(value: &mut BytePacketBuffer) -> Result<Self, Self::Error> {
    let mut packet: Self = Self::default();
    packet.header.read(value)?;

    let read_question = |_| {
      let mut question: DnsQuestion =
        DnsQuestion::new(String::new(), crate::enums::QueryType::UNKNOWN(0));
      question.read(value)?;
      Ok(question)
    };

    packet.questions = (0..packet.header.questions)
      .map(read_question)
      .collect::<Result<_, BytePacketError>>()?;

    packet.answers = (0..packet.header.answers)
      .map(|_| DnsRecord::read(value))
      .collect::<Result<_, _>>()?;

    packet.authorities = (0..packet.header.authoritative_entries)
      .map(|_| DnsRecord::read(value))
      .collect::<Result<_, _>>()?;

    packet.resources = (0..packet.header.resource_entries)
      .map(|_| DnsRecord::read(value))
      .collect::<Result<_, _>>()?;

    Ok(packet)
  }
}

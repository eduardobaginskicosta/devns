use crate::{
  enums::{BytePacketError, QueryType},
  structs::BytePacketBuffer,
};

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct DnsQuestion {
  qtype: QueryType,
  name: String,
}

impl DnsQuestion {
  pub fn new(name: String, qtype: QueryType) -> Self {
    Self { qtype, name }
  }

  // * members
  pub fn qtype(&mut self) -> &mut QueryType {
    &mut self.qtype
  }
  pub fn name(&mut self) -> &mut str {
    &mut self.name
  }

  // * safe read
  pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), BytePacketError> {
    buffer.read_qname(&mut self.name)?;
    self.qtype = QueryType::from(buffer.read_u16()?);
    buffer.read_u16()?;
    Ok(())
  }

  // * safe write
  pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<(), BytePacketError> {
    buffer.write_qname(&self.name)?;
    buffer.write_u16(u16::from(self.qtype))?;
    buffer.write_u16(0x1)
  }
}

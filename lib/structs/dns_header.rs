use crate::{
  enums::{BytePacketError, ResultCode},
  structs::BytePacketBuffer,
};

#[derive(Debug, Clone, Default)]
pub struct DnsHeader {
  id: u16,

  authoritative_answer: bool,
  recursion_desired: bool,
  truncated_message: bool,
  response: bool,
  opcode: u8,

  recursion_available: bool,
  checking_disabled: bool,
  authed_data: bool,
  rescode: ResultCode,
  z: bool,

  authoritative_entries: u16,
  resource_entries: u16,
  questions: u16,
  answers: u16,
}

impl DnsHeader {
  pub fn new() -> Self {
    Self {
      id: 0,
      authoritative_answer: false,
      recursion_desired: false,
      truncated_message: false,
      response: false,
      opcode: 0,
      recursion_available: false,
      checking_disabled: false,
      authed_data: false,
      rescode: ResultCode::NoError,
      z: false,
      authoritative_entries: 0,
      resource_entries: 0,
      questions: 0,
      answers: 0,
    }
  }

  // * members
  pub fn id(&mut self) -> &u16 {
    &self.id
  }
  pub fn authoritative_answer(&mut self) -> &bool {
    &self.authoritative_answer
  }
  pub fn recursion_desired(&mut self) -> &bool {
    &self.recursion_desired
  }
  pub fn truncated_message(&mut self) -> &bool {
    &self.truncated_message
  }
  pub fn response(&mut self) -> &bool {
    &self.response
  }
  pub fn opcode(&mut self) -> &u8 {
    &self.opcode
  }
  pub fn recursion_available(&mut self) -> &bool {
    &self.recursion_available
  }
  pub fn checking_disabled(&mut self) -> &bool {
    &self.checking_disabled
  }
  pub fn authed_data(&mut self) -> &bool {
    &self.authed_data
  }
  pub fn rescode(&mut self) -> &ResultCode {
    &self.rescode
  }
  pub fn z(&mut self) -> &bool {
    &self.z
  }
  pub fn authoritative_entries(&mut self) -> &u16 {
    &self.authoritative_entries
  }
  pub fn resource_entries(&mut self) -> &u16 {
    &self.resource_entries
  }
  pub fn questions(&mut self) -> &u16 {
    &self.questions
  }
  pub fn answers(&mut self) -> &u16 {
    &self.answers
  }

  // * setters
  pub fn set_id(&mut self, v: u16) {
    self.id = v
  }
  pub fn set_authoritative_answer(&mut self, v: bool) {
    self.authoritative_answer = v
  }
  pub fn set_recursion_desired(&mut self, v: bool) {
    self.recursion_desired = v
  }
  pub fn set_truncated_message(&mut self, v: bool) {
    self.truncated_message = v
  }
  pub fn set_response(&mut self, v: bool) {
    self.response = v
  }
  pub fn set_opcode(&mut self, v: u8) {
    self.opcode = v
  }
  pub fn set_recursion_available(&mut self, v: bool) {
    self.recursion_available = v
  }
  pub fn set_checking_disabled(&mut self, v: bool) {
    self.checking_disabled = v
  }
  pub fn set_authed_data(&mut self, v: bool) {
    self.authed_data = v
  }
  pub fn set_rescode(&mut self, v: ResultCode) {
    self.rescode = v
  }
  pub fn set_z(&mut self, v: bool) {
    self.z = v
  }
  pub fn set_authoritative_entries(&mut self, v: u16) {
    self.authoritative_entries = v
  }
  pub fn set_resource_entries(&mut self, v: u16) {
    self.resource_entries = v
  }
  pub fn set_questions(&mut self, v: u16) {
    self.questions = v
  }
  pub fn set_answers(&mut self, v: u16) {
    self.answers = v
  }

  // * safe read
  pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), BytePacketError> {
    self.id = buffer.read_u16()?;

    let flags: u16 = buffer.read_u16()?;
    let a: u8 = (flags >> 0x8) as u8;
    let b: u8 = (flags & 0xFF) as u8;

    self.recursion_desired = a & 0x1 != 0;
    self.truncated_message = a & 0x02 != 0;
    self.authoritative_answer = a & 0x04 != 0;
    self.opcode = (a >> 0x03) & 0x0F;
    self.response = a & 0x80 != 0;

    self.rescode = ResultCode::from(b);
    self.checking_disabled = b & 0x10 != 0;
    self.authed_data = b & 0x20 != 0;
    self.z = b & 0x40 != 0;
    self.recursion_available = b & 0x80 != 0;

    self.questions = buffer.read_u16()?;
    self.answers = buffer.read_u16()?;
    self.authoritative_entries = buffer.read_u16()?;
    self.resource_entries = buffer.read_u16()?;

    Ok(())
  }

  // * safe write
  pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<(), BytePacketError> {
    let a: u8 = (self.recursion_desired as u8)
      | ((self.truncated_message as u8) << 0x01)
      | ((self.authoritative_answer as u8) << 0x02)
      | (self.opcode << 0x03)
      | ((self.response as u8) << 0x07);

    let b: u8 = (self.rescode as u8)
      | ((self.checking_disabled as u8) << 0x04)
      | ((self.authed_data as u8) << 0x05)
      | ((self.z as u8) << 0x06)
      | ((self.recursion_available as u8) << 0x07);

    buffer.write_u16(self.id)?;
    buffer.write(a)?;
    buffer.write(b)?;
    buffer.write_u16(self.questions)?;
    buffer.write_u16(self.answers)?;
    buffer.write_u16(self.authoritative_entries)?;
    buffer.write_u16(self.resource_entries)
  }
}
